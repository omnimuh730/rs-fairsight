pub mod app_state;
pub mod commands;
pub mod dependency_checker;
pub mod dll_loader;
pub mod encryption;
pub mod file_utils;
pub mod health_monitor;
pub mod hooks;
pub mod logger;
pub mod macos_utils;
pub mod ui_setup;
pub mod web_server;

pub fn run() {
    println!("Running rust_fairsight_lib..."); // Non-Tauri background task
}