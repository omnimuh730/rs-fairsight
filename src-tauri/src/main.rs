//Example Request
//http://192.168.9.111:7930/aggregate?startDate=2025-04-01&endDate=2025-04-04

#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

mod activity_monitor;
mod network_monitor;
mod utils;

use chrono::Local;
#[cfg(target_os = "windows")]
use std::path::Path;
use tauri::Manager;

use crate::activity_monitor::initialize_time_tracking;
use crate::network_monitor::{
    persistent_state::get_persistent_state_manager,
};
use crate::utils::{
    commands::{
        aggregate_week_activity_logs, check_network_permissions_status,
        check_unexpected_shutdown, clear_all_logs, cleanup_network_backups,
        cleanup_old_network_data, create_network_backup, get_adapter_persistent_state,
        get_all_logs, get_available_network_dates, get_current_network_totals,
        get_health_status, get_lifetime_stats, get_network_adapters_command, get_network_history,
        get_network_stats, get_recent_logs_limited, greet, is_network_monitoring,
        request_network_permissions, restore_network_backup, start_network_monitoring,
        stop_network_monitoring, sync_time_data,
    },
    encryption::KEY,
    file_utils::{is_log_file_valid, load_backup},
    health_monitor::initialize_health_monitoring,
    hooks::setup_hooks,
    ui_setup::{handle_window_event, setup_tray_and_window_events},
    web_server::start_web_server,
};

// Global flag to prevent duplicate auto-start attempts
static AUTO_START_COMPLETED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

#[cfg(target_os = "macos")]
use dirs;
fn main() {
    // Initialize logging first
    crate::log_info!("main", "Application starting...");
    
    // On Windows, try to load the bundled Npcap DLLs
    #[cfg(target_os = "windows")]
    {
        if let Err(e) = crate::utils::dll_loader::ensure_npcap_dlls_loaded() {
            crate::log_error!("main", "Failed to load Npcap DLLs: {}", e);
        }
    }
    
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

    // (Moved) Auto-start network monitoring will now be handled in the Tauri .setup closure below

    builder
        .setup(|app| {
            setup_tray_and_window_events(app)?;

            // Auto-start network monitoring after Tauri is fully initialized
            tauri::async_runtime::spawn(async {
                // Wait a bit for the application to fully initialize (optional, can be tuned)
                tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

                // Prevent duplicate auto-start attempts
                if AUTO_START_COMPLETED.swap(true, std::sync::atomic::Ordering::SeqCst) {
                    println!("ℹ️  Auto-start already completed, skipping duplicate attempt");
                    return;
                }

                // Start comprehensive monitoring on all suitable adapters
                match crate::utils::commands::start_comprehensive_monitoring().await {
                    Ok(msg) => println!("✅ Auto-started comprehensive network monitoring: {}", msg),
                    Err(e) => eprintln!("❌ Failed to auto-start comprehensive monitoring: {}", e),
                }
            });

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

