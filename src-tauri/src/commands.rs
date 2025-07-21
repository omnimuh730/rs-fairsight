use crate::time_tracker::aggregate_log_results;
use crate::health_monitor::HEALTH_MONITOR;
use crate::logger::{get_logs, get_recent_logs, clear_logs, LogEntry};

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
