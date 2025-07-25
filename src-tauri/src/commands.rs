use crate::time_tracker::aggregate_log_results;
use crate::health_monitor::HEALTH_MONITOR;
use crate::logger::{get_logs, get_recent_logs, clear_logs, LogEntry};
use crate::network_monitor::{get_network_adapters, NetworkAdapter};
use crate::traffic_monitor::{get_or_create_monitor, MonitoringStats};
use crate::network_storage::{NETWORK_STORAGE, DailyNetworkSummary};

#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
pub fn sync_time_data(report_date: &str) -> String {
    match aggregate_log_results(report_date) {
        Ok(result) => result,
        Err(e) => format!("Error: {}", e),
    }
}

#[tauri::command]
pub fn aggregate_week_activity_logs(data_list: Vec<String>) -> Vec<String> {
    let mut logdb_list = Vec::with_capacity(data_list.len());

    for (_i, s) in data_list.into_iter().enumerate() {
        let styled = format!("rs-fairsight({}).txt", s);
        let result = aggregate_log_results(&styled)
            .unwrap_or_else(|e| format!("Error aggregating {}: {}", styled, e));
        logdb_list.push(result);
    }

    logdb_list
}

#[tauri::command]
pub fn get_health_status() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    
    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
        
    let last_activity = HEALTH_MONITOR.get_last_activity_time();
    
    if last_activity == 0 {
        "No activity tracked yet".to_string()
    } else {
        let seconds_since_activity = current_time - last_activity;
        if seconds_since_activity < 60 {
            "Time tracking is working normally".to_string()
        } else if seconds_since_activity < 600 {
            format!("Last activity {} seconds ago", seconds_since_activity)
        } else {
            format!("Warning: No activity for {} seconds", seconds_since_activity)
        }
    }
}

#[tauri::command]
pub fn get_all_logs() -> Vec<LogEntry> {
    get_logs()
}

#[tauri::command]
pub fn get_recent_logs_limited(count: usize) -> Vec<LogEntry> {
    get_recent_logs(count)
}

#[tauri::command]
pub fn clear_all_logs() -> String {
    clear_logs();
    "Logs cleared successfully".to_string()
}

#[tauri::command]
pub fn get_network_adapters_command() -> Result<Vec<NetworkAdapter>, String> {
    get_network_adapters()
}

#[tauri::command]
pub async fn start_network_monitoring(adapter_name: String) -> Result<String, String> {
    let monitor = get_or_create_monitor(&adapter_name);
    match monitor.start_monitoring().await {
        Ok(_) => Ok(format!("Started monitoring adapter: {}", adapter_name)),
        Err(e) => Err(e),
    }
}

#[tauri::command]
pub fn stop_network_monitoring(adapter_name: String) -> Result<String, String> {
    let monitor = get_or_create_monitor(&adapter_name);
    monitor.stop_monitoring();
    Ok(format!("Stopped monitoring adapter: {}", adapter_name))
}

#[tauri::command]
pub fn get_network_stats(adapter_name: String) -> Result<MonitoringStats, String> {
    let monitor = get_or_create_monitor(&adapter_name);
    Ok(monitor.get_stats())
}

#[tauri::command]
pub fn is_network_monitoring(adapter_name: String) -> bool {
    let monitor = get_or_create_monitor(&adapter_name);
    monitor.is_monitoring()
}

#[tauri::command]
pub fn get_network_history(start_date: String, end_date: String) -> Result<Vec<DailyNetworkSummary>, String> {
    NETWORK_STORAGE.get_date_range_data(&start_date, &end_date)
}

#[tauri::command]
pub fn get_available_network_dates() -> Result<Vec<String>, String> {
    NETWORK_STORAGE.get_available_dates()
}

#[tauri::command]
pub fn cleanup_old_network_data(days_to_keep: u32) -> Result<String, String> {
    NETWORK_STORAGE.cleanup_old_data(days_to_keep)?;
    Ok(format!("Cleaned up network data older than {} days", days_to_keep))
}
