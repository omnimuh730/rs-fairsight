//Example Request
//http://192.168.9.111:7930/aggregate?startDate=2025-04-01&endDate=2025-04-04

#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

mod encryption;
mod file_utils;
mod hooks;
mod time_tracker;
mod web_server;
mod commands;
mod app_state;
mod ui_setup;
mod health_monitor;
mod logger;
mod network_monitor;
mod traffic_monitor;
mod network_storage;
mod persistent_state;
#[cfg(target_os = "macos")]
mod macos_utils;

use chrono::Local;
#[cfg(target_os = "windows")]
use std::path::Path;
use tauri::Manager;
use tauri_plugin_autostart::MacosLauncher;

use crate::encryption::KEY;
use crate::file_utils::{is_log_file_valid, load_backup};
use crate::hooks::setup_hooks;
use crate::time_tracker::initialize_time_tracking;
use crate::network_monitor::get_default_network_adapter;
use crate::traffic_monitor::get_or_create_monitor;
use crate::web_server::start_web_server;
use crate::commands::{greet, sync_time_data, aggregate_week_activity_logs, get_health_status, get_all_logs, get_recent_logs_limited, clear_all_logs, get_network_adapters_command, start_network_monitoring, stop_network_monitoring, get_network_stats, is_network_monitoring, get_network_history, get_available_network_dates, cleanup_old_network_data, create_network_backup, restore_network_backup, cleanup_network_backups, get_adapter_persistent_state, get_lifetime_stats, check_unexpected_shutdown, get_current_network_totals, request_network_permissions, check_network_permissions_status};
use crate::ui_setup::{setup_tray_and_window_events, handle_window_event};
use crate::health_monitor::initialize_health_monitoring;
use crate::persistent_state::get_persistent_state_manager;

#[cfg(target_os = "macos")]
use dirs;
fn main() {
    // Initialize logging first
    crate::log_info!("main", "Application starting...");
    
    let mut builder = tauri::Builder::default();

    #[cfg(desktop)]
    {
        builder = builder.plugin(
            tauri_plugin_single_instance::init(|app, _args, _cwd| {
                let _ = app.get_webview_window("main").expect("no main window").set_focus();
            })
        );
    }
    
    // Initialize time tracking
    initialize_time_tracking();
    crate::log_info!("main", "Time tracking initialized");
    
    // Initialize health monitoring
    initialize_health_monitoring();
    crate::log_info!("main", "Health monitoring initialized");

    // Initialize backup and validation with better error handling
    #[cfg(target_os = "windows")]
    {
        let log_dir = Path::new("C:\\fairsight-log");
        let backup_dir = Path::new("C:\\fairsight-backup");
        let current_date = Local::now().format("%Y-%m-%d").to_string();
        let file_name = format!("rs-fairsight({}).txt", current_date);
        let log_file_path = log_dir.join(&file_name);

        // Ensure directories exist
        if let Err(e) = std::fs::create_dir_all(log_dir) {
            eprintln!("Warning: Failed to create log directory: {}", e);
        }
        if let Err(e) = std::fs::create_dir_all(backup_dir) {
            eprintln!("Warning: Failed to create backup directory: {}", e);
        }

        if log_file_path.exists() && !is_log_file_valid(&log_file_path, &KEY) {
            match load_backup(backup_dir, log_dir, &file_name) {
                Ok(_) => println!("Successfully restored from backup at startup"),
                Err(e) => eprintln!("Warning: Failed to restore from backup: {}", e),
            }
        } else {
            println!("Log file is valid or doesn't exist at startup");
        }
    }
    #[cfg(target_os = "macos")]
    {
        let home_dir = match dirs::home_dir() {
            Some(dir) => dir,
            None => {
                eprintln!("Error: Could not find home directory");
                return;
            }
        };
        let log_dir = home_dir.join("Documents").join("rs-fairsight");
        let backup_dir = home_dir.join("Documents").join("rs-fairsight-backup");
        let current_date = Local::now().format("%Y-%m-%d").to_string();
        let file_name = format!("rs-fairsight({}).txt", current_date);
        let log_file_path = log_dir.join(&file_name);

        // Ensure directories exist
        if let Err(e) = std::fs::create_dir_all(&log_dir) {
            eprintln!("Warning: Failed to create log directory: {}", e);
        }
        if let Err(e) = std::fs::create_dir_all(&backup_dir) {
            eprintln!("Warning: Failed to create backup directory: {}", e);
        }

        if log_file_path.exists() && !is_log_file_valid(&log_file_path, &KEY) {
            match load_backup(&backup_dir, &log_dir, &file_name) {
                Ok(_) => println!("Successfully restored from backup at startup"),
                Err(e) => eprintln!("Warning: Failed to restore from backup: {}", e),
            }
        }
    }

    // Set up hooks in a background thread
    setup_hooks();

    // Start web server in background
    std::thread::spawn(|| {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(start_web_server());
    });

    // Auto-start network monitoring on application startup
    std::thread::spawn(|| {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            // Wait a bit for the application to fully initialize
            tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
            
            // Check macOS network permissions before starting monitoring
            #[cfg(target_os = "macos")]
            {
                use crate::macos_utils::{check_bpf_permissions, get_permission_instructions};
                
                println!("🔐 Checking macOS network permissions...");
                match check_bpf_permissions() {
                    Ok(_) => {
                        println!("✅ macOS network permissions verified");
                    }
                    Err(e) => {
                        println!("❌ macOS network permission error: {}", e);
                        println!("{}", get_permission_instructions());
                        println!("🔄 Network monitoring will continue in simulation mode until permissions are granted");
                    }
                }
            }
            
            match get_default_network_adapter() {
                Ok(adapter_name) => {
                    let monitor = get_or_create_monitor(&adapter_name);
                    match monitor.start_monitoring().await {
                        Ok(_) => println!("🚀 Auto-started network monitoring on adapter: {}", adapter_name),
                        Err(e) => eprintln!("❌ Failed to auto-start network monitoring: {}", e),
                    }
                }
                Err(e) => eprintln!("⚠️  No suitable network adapter found for auto-start: {}", e),
            }
        });
    });

    builder
        .plugin(
            tauri_plugin_autostart::init(
                MacosLauncher::LaunchAgent,
                None
            )
        )
        .setup(|app| {
            setup_tray_and_window_events(app)?;
            
            // Check for unexpected shutdown and warn if needed
            match get_persistent_state_manager().was_unexpected_shutdown() {
                Ok(true) => {
                    println!("⚠️  Detected unexpected shutdown - some network data may have been lost");
                }
                Ok(false) => {
                    println!("✅ Clean shutdown detected - data integrity maintained");
                }
                Err(e) => {
                    eprintln!("❌ Failed to check shutdown state: {}", e);
                }
            }
            
            Ok(())
        })
        .on_window_event(|window, event| {
            handle_window_event(window, event);
            
            // Handle clean shutdown on various window events
            match event {
                tauri::WindowEvent::CloseRequested { .. } => {
                    println!("🔄 Application closing - marking clean shutdown...");
                    if let Err(e) = get_persistent_state_manager().mark_clean_shutdown() {
                        eprintln!("⚠️  Failed to mark clean shutdown: {}", e);
                    }
                }
                tauri::WindowEvent::Destroyed => {
                    println!("🔄 Window destroyed - ensuring clean shutdown...");
                    if let Err(e) = get_persistent_state_manager().mark_clean_shutdown() {
                        eprintln!("⚠️  Failed to mark clean shutdown on destroy: {}", e);
                    }
                }
                _ => {}
            }
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(
            tauri::generate_handler![greet, sync_time_data, aggregate_week_activity_logs, get_health_status, get_all_logs, get_recent_logs_limited, clear_all_logs, get_network_adapters_command, start_network_monitoring, stop_network_monitoring, get_network_stats, is_network_monitoring, get_network_history, get_available_network_dates, cleanup_old_network_data, create_network_backup, restore_network_backup, cleanup_network_backups, get_adapter_persistent_state, get_lifetime_stats, check_unexpected_shutdown, get_current_network_totals, request_network_permissions, check_network_permissions_status]
        )
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

