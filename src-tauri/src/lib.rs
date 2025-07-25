// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

pub mod encryption;
pub mod file_utils;
pub mod hooks;
pub mod time_tracker;
pub mod web_server;
pub mod commands;
pub mod app_state;
pub mod ui_setup;
pub mod health_monitor;
pub mod logger;
pub mod network_monitor;

#[cfg(target_os = "macos")]
pub mod macos_utils;

pub fn run() {
    println!("Running rust_fairsight_lib..."); // Non-Tauri background task
}
