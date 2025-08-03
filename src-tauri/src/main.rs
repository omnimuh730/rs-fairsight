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
mod system_verification;
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
use crate::network_monitor::{get_default_network_adapter, get_monitoring_adapters};
use crate::traffic_monitor::get_or_create_monitor;
use crate::web_server::start_web_server;
use crate::commands::{greet, sync_time_data, aggregate_week_activity_logs, get_health_status, get_comprehensive_health_status, get_all_logs, get_recent_logs_limited, clear_all_logs, get_network_adapters_command, get_monitoring_adapters_command, start_network_monitoring, start_comprehensive_monitoring, stop_network_monitoring, stop_comprehensive_monitoring, refresh_and_restart_monitoring, get_network_stats, get_comprehensive_network_stats, is_network_monitoring, is_comprehensive_monitoring_active, get_network_history, get_available_network_dates, cleanup_old_network_data, create_network_backup, restore_network_backup, cleanup_network_backups, get_adapter_persistent_state, get_lifetime_stats, check_unexpected_shutdown, get_current_network_totals, request_network_permissions, check_network_permissions_status, verify_system_dependencies};
use crate::ui_setup::{setup_tray_and_window_events, handle_window_event};
use crate::health_monitor::initialize_health_monitoring;
use crate::persistent_state::get_persistent_state_manager;
use crate::system_verification::verify_system_requirements;

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

    // Verify system requirements for deployment compatibility
    println!("🔍 Verifying system requirements for network monitoring...");
    let verification_results = verify_system_requirements();
    let mut all_checks_passed = true;
    
    for (check_name, result) in &verification_results {
        match result {
            Ok(msg) => {
                println!("✅ {}: {}", check_name, msg);
                crate::log_info!("system_verification", "{}: {}", check_name, msg);
            }
            Err(err) => {
                println!("❌ {}: {}", check_name, err);
                crate::log_error!("system_verification", "{}: {}", check_name, err);
                all_checks_passed = false;
            }
        }
    }
    
    if all_checks_passed {
        println!("✅ All system requirements verified - app should work on deployment");
    } else {
        println!("⚠️  Some system requirements failed - app may have issues on other machines");
        println!("💡 Consider running the bundling process or checking deployment documentation");
    }

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
            
            // Start comprehensive monitoring with automatic problematic adapter filtering
            match get_monitoring_adapters() {
                Ok(adapters) => {
                    if adapters.is_empty() {
                        println!("⚠️  No suitable adapters found for monitoring");
                        return;
                    }
                    
                    println!("🚀 Auto-starting comprehensive monitoring on {} filtered adapters", adapters.len());
                    
                    let mut started_adapters = Vec::new();
                    let mut failed_adapters = Vec::new();
                    
                    for adapter_name in adapters {
                        let monitor = get_or_create_monitor(&adapter_name);
                        match monitor.start_monitoring().await {
                            Ok(_) => {
                                started_adapters.push(adapter_name);
                            }
                            Err(e) => {
                                // Skip problematic adapters silently during auto-start
                                failed_adapters.push(format!("{}:{}", adapter_name, e));
                            }
                        }
                    }
                    
                    if !started_adapters.is_empty() {
                        println!("✅ Auto-started monitoring on {} adapters: {:?}", started_adapters.len(), started_adapters);
                        if !failed_adapters.is_empty() {
                            println!("⏭️  Skipped {} problematic adapters during auto-start", failed_adapters.len());
                        }
                    } else {
                        println!("❌ Failed to start monitoring on any adapters during auto-start");
                    }
                }
                Err(e) => {
                    println!("⚠️  Failed to get monitoring adapters for auto-start: {}", e);
                }
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
            
            // Auto-start comprehensive network monitoring with packet deduplication
            let _app_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                println!("🔍 Auto-starting comprehensive network monitoring...");
                
                // Wait a moment for app to fully initialize
                tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                
                // Check network permissions on macOS first
                #[cfg(target_os = "macos")]
                {
                    use std::process::Command;
                    
                    crate::log_info!("network_permissions", "Checking macOS network monitoring permissions...");
                    
                    match Command::new("tcpdump").arg("-D").output() {
                        Ok(output) => {
                            if !output.status.success() {
                                let stderr = String::from_utf8_lossy(&output.stderr);
                                let stdout = String::from_utf8_lossy(&output.stdout);
                                
                                crate::log_error!("network_permissions", "tcpdump permission check failed - stdout: {}, stderr: {}", stdout, stderr);
                                
                                if stderr.contains("permission") || stderr.contains("Operation not permitted") {
                                    let error_msg = "Network monitoring requires administrator privileges on macOS";
                                    crate::log_error!("network_permissions", "{}", error_msg);
                                    println!("❌ {}", error_msg);
                                    println!("💡 Please allow network access in System Preferences → Security & Privacy → Privacy → Developer Tools");
                                    println!("   Or run the application with elevated permissions");
                                    
                                    // Log specific guidance
                                    crate::log_info!("network_permissions", "To fix: Open System Preferences → Security & Privacy → Privacy → Developer Tools → Enable for this app");
                                    return;
                                } else {
                                    crate::log_warning!("network_permissions", "tcpdump failed but not due to permissions - continuing with monitoring attempt");
                                }
                            } else {
                                crate::log_info!("network_permissions", "✅ Network permissions verified on macOS");
                                println!("✅ Network permissions verified on macOS");
                            }
                        },
                        Err(e) => {
                            crate::log_warning!("network_permissions", "Could not verify network permissions (tcpdump not available): {}. Continuing anyway...", e);
                            println!("⚠️  Could not verify network permissions: {}. Continuing anyway...", e);
                        }
                    }
                }
                
                match get_monitoring_adapters() {
                    Ok(adapters) => {
                        if !adapters.is_empty() {
                            crate::log_info!("network_monitoring", "Found {} suitable adapters for comprehensive monitoring: {:?}", adapters.len(), adapters);
                            println!("🚀 Starting comprehensive monitoring on {} adapters with packet deduplication", adapters.len());
                            
                            let mut started_adapters = Vec::new();
                            let mut failed_adapters = Vec::new();
                            
                            for adapter_name in adapters {
                                crate::log_info!("network_monitoring", "Attempting to start monitoring on adapter: {}", adapter_name);
                                
                                let monitor = get_or_create_monitor(&adapter_name);
                                match monitor.start_monitoring().await {
                                    Ok(_) => {
                                        crate::log_info!("network_monitoring", "✅ Successfully started monitoring on adapter: {}", adapter_name);
                                        started_adapters.push(adapter_name);
                                    }
                                    Err(e) => {
                                        crate::log_error!("network_monitoring", "❌ Failed to start monitoring on adapter '{}': {}", adapter_name, e);
                                        failed_adapters.push(format!("{}: {}", adapter_name, e));
                                    }
                                }
                            }
                            
                            if !started_adapters.is_empty() {
                                crate::log_info!("network_monitoring", "Comprehensive monitoring successfully started on {} adapters: {:?}", started_adapters.len(), started_adapters);
                                println!("✅ Auto-started comprehensive monitoring on {} adapters: {:?}", 
                                    started_adapters.len(), started_adapters);
                                
                                if !failed_adapters.is_empty() {
                                    crate::log_warning!("network_monitoring", "Some adapters failed to start: {:?}", failed_adapters);
                                    println!("⚠️  Some adapters failed to start: {:?}", failed_adapters);
                                }
                                
                                println!("🔄 Packet deduplication active - monitoring all adapters without duplicate counting");
                            } else {
                                crate::log_error!("network_monitoring", "Failed to start monitoring on any adapters: {:?}", failed_adapters);
                                println!("❌ Failed to start monitoring on any adapters: {:?}", failed_adapters);
                            }
                        } else {
                            crate::log_warning!("network_monitoring", "No suitable adapters found for comprehensive monitoring");
                            println!("⚠️  No suitable adapters found for comprehensive monitoring");
                        }
                    }
                    Err(e) => {
                        crate::log_error!("network_monitoring", "Failed to get monitoring adapters: {}", e);
                        println!("❌ Failed to get monitoring adapters: {}", e);
                        
                        // Fallback to single adapter monitoring
                        crate::log_info!("network_monitoring", "Attempting fallback to single adapter monitoring...");
                        println!("🔄 Attempting fallback to single adapter monitoring...");
                        
                        match get_default_network_adapter() {
                            Ok(adapter_name) => {
                                crate::log_info!("network_monitoring", "Fallback: Found default adapter: {}", adapter_name);
                                let monitor = get_or_create_monitor(&adapter_name);
                                match monitor.start_monitoring().await {
                                    Ok(_) => {
                                        crate::log_info!("network_monitoring", "✅ Fallback: Successfully started monitoring single adapter: {}", adapter_name);
                                        println!("✅ Fallback: Started monitoring single adapter: {}", adapter_name);
                                    }
                                    Err(e) => {
                                        crate::log_error!("network_monitoring", "❌ Fallback failed for adapter '{}': {}", adapter_name, e);
                                        println!("❌ Fallback failed: {}", e);
                                    }
                                }
                            }
                            Err(e) => {
                                crate::log_error!("network_monitoring", "❌ Could not find any network adapter: {}", e);
                                println!("❌ Could not find any network adapter: {}", e);
                            }
                        }
                    }
                }
            });
            
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
            tauri::generate_handler![greet, sync_time_data, aggregate_week_activity_logs, get_health_status, get_comprehensive_health_status, get_all_logs, get_recent_logs_limited, clear_all_logs, get_network_adapters_command, get_monitoring_adapters_command, start_network_monitoring, start_comprehensive_monitoring, stop_network_monitoring, stop_comprehensive_monitoring, refresh_and_restart_monitoring, get_network_stats, get_comprehensive_network_stats, is_network_monitoring, is_comprehensive_monitoring_active, get_network_history, get_available_network_dates, cleanup_old_network_data, create_network_backup, restore_network_backup, cleanup_network_backups, get_adapter_persistent_state, get_lifetime_stats, check_unexpected_shutdown, get_current_network_totals, request_network_permissions, check_network_permissions_status, verify_system_dependencies]
        )
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

