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
#[cfg(target_os = "macos")]
mod macos_utils;

use chrono::Local;
use std::path::Path;
use tauri::Manager;
use tauri_plugin_autostart::MacosLauncher;

use crate::encryption::KEY;
use crate::file_utils::{is_log_file_valid, load_backup};
use crate::hooks::setup_hooks;
use crate::time_tracker::initialize_time_tracking;
use crate::web_server::start_web_server;
use crate::commands::{greet, sync_time_data, aggregate_week_activity_logs};
use crate::ui_setup::{setup_tray_and_window_events, handle_window_event};

#[cfg(target_os = "macos")]
use dirs;
fn main() {
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

    // Initialize backup and validation
    #[cfg(target_os = "windows")]
    {
        let log_dir = Path::new("C:\\fairsight-log");
        let backup_dir = Path::new("C:\\fairsight-backup");
        let current_date = Local::now().format("%Y-%m-%d").to_string();
        let file_name = format!("rs-fairsight({}).txt", current_date);
        let log_file_path = log_dir.join(&file_name);

        if log_file_path.exists() && !is_log_file_valid(&log_file_path, &KEY) {
            let _ = load_backup(backup_dir, log_dir, &file_name);
            println!("Saved to backup at startup");
        } else {
            println!("File invalid or not found at main function");
        }
    }
    #[cfg(target_os = "macos")]
    {
        let home_dir = dirs::home_dir().expect("Could not find home directory");
        let log_dir = home_dir.join("Documents").join("rs-fairsight");
        let backup_dir = home_dir.join("Documents").join("rs-fairsight-backup");
        let current_date = Local::now().format("%Y-%m-%d").to_string();
        let file_name = format!("rs-fairsight({}).txt", current_date);
        let log_file_path = log_dir.join(&file_name);

        if log_file_path.exists() && !is_log_file_valid(&log_file_path, &KEY) {
            let _ = load_backup(&backup_dir, &log_dir, &file_name);
        }
    }

    // Set up hooks in a background thread
    setup_hooks();

    // Start web server in background
    std::thread::spawn(|| {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(start_web_server());
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
            Ok(())
        })
        .on_window_event(|window, event| {
            handle_window_event(window, event);
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(
            tauri::generate_handler![greet, sync_time_data, aggregate_week_activity_logs]
        )
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

