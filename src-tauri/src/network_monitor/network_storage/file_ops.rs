use std::fs;
use std::path::PathBuf;
use chrono::{NaiveDate, Utc, TimeZone};
use super::types::DailyNetworkSummary;

pub fn load_daily_summary(storage_dir: &PathBuf, date: &str) -> Result<DailyNetworkSummary, String> {
    let file_path = storage_dir.join(format!("network-{}.json", date));
    
    if !file_path.exists() {
        return Err("Daily summary not found".to_string());
    }
    
    let content = fs::read_to_string(&file_path)
        .map_err(|e| format!("Failed to read daily summary: {}", e))?;
    
    let summary: DailyNetworkSummary = serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse daily summary: {}", e))?;
    
    Ok(summary)
}

pub fn save_daily_summary(storage_dir: &PathBuf, backup_dir: &PathBuf, date: &str, summary: &DailyNetworkSummary) -> Result<(), String> {
    let file_path = storage_dir.join(format!("network-{}.json", date));
    
    // Create backup before saving if file exists
    if file_path.exists() {
        if let Err(e) = super::backup::create_backup(storage_dir, backup_dir, date) {
            eprintln!("Warning: Failed to create backup before saving: {}", e);
        }
    }
    
    let content = serde_json::to_string_pretty(summary)
        .map_err(|e| format!("Failed to serialize daily summary: {}", e))?;
    
    // Atomic save using temporary file
    let temp_path = storage_dir.join(format!("network-{}.json.tmp", date));
    
    fs::write(&temp_path, &content)
        .map_err(|e| format!("Failed to write temporary file: {}", e))?;
    
    // Verify the file was written correctly
    let _ = fs::read_to_string(&temp_path)
        .map_err(|e| format!("Failed to verify temporary file: {}", e))?;
    
    // Atomically replace the original file
    fs::rename(&temp_path, &file_path)
        .map_err(|e| format!("Failed to finalize save: {}", e))?;
    
    Ok(())
}

pub fn get_date_range_data(storage_dir: &PathBuf, start_date: &str, end_date: &str) -> Result<Vec<DailyNetworkSummary>, String> {
    let start = NaiveDate::parse_from_str(start_date, "%Y-%m-%d")
        .map_err(|e| format!("Invalid start date format: {}", e))?;
    let end = NaiveDate::parse_from_str(end_date, "%Y-%m-%d")
        .map_err(|e| format!("Invalid end date format: {}", e))?;
    
    let mut results = Vec::new();
    let mut current_date = start;
    
    while current_date <= end {
        let date_str = current_date.format("%Y-%m-%d").to_string();
        if let Ok(summary) = load_daily_summary(storage_dir, &date_str) {
            results.push(summary);
        } else {
            // Create empty summary for missing dates
            results.push(DailyNetworkSummary {
                date: date_str,
                sessions: Vec::new(),
                total_incoming_bytes: 0,
                total_outgoing_bytes: 0,
                total_duration: 0,
                unique_hosts: 0,
                unique_services: 0,
            });
        }
        current_date = current_date.succ_opt().ok_or("Date overflow")?;
    }
    
    Ok(results)
}

pub fn cleanup_old_data(storage_dir: &PathBuf, days_to_keep: u32) -> Result<(), String> {
    let cutoff_date = Utc::now().checked_sub_signed(chrono::Duration::days(days_to_keep as i64)).ok_or("Date calculation overflow")?;
    
    let entries = fs::read_dir(storage_dir)
        .map_err(|e| format!("Failed to read storage directory: {}", e))?;
    
    for entry in entries {
        let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
        let file_name = entry.file_name();
        let file_name_str = file_name.to_string_lossy();
        
        if file_name_str.starts_with("network-") && file_name_str.ends_with(".json") {
            // Extract date from filename: network-YYYY-MM-DD.json
            if let Some(date_part) = file_name_str.strip_prefix("network-").and_then(|s| s.strip_suffix(".json")) {
                if let Ok(file_date) = NaiveDate::parse_from_str(date_part, "%Y-%m-%d") {
                    if let Some(file_datetime) = Utc.from_local_datetime(&file_date.and_hms_opt(0, 0, 0).unwrap()).single() {
                        if file_datetime < cutoff_date {
                            if let Err(e) = fs::remove_file(entry.path()) {
                                eprintln!("Failed to remove old network data file: {}", e);
                            } else {
                                println!("Removed old network data: {}", file_name_str);
                            }
                        }
                    }
                }
            }
        }
    }
    
    Ok(())
}

pub fn get_available_dates(storage_dir: &PathBuf) -> Result<Vec<String>, String> {
    let entries = fs::read_dir(storage_dir)
        .map_err(|e| format!("Failed to read storage directory: {}", e))?;
    
    let mut dates = Vec::new();
    
    for entry in entries {
        let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
        let file_name = entry.file_name();
        let file_name_str = file_name.to_string_lossy();
        
        if file_name_str.starts_with("network-") && file_name_str.ends_with(".json") {
            if let Some(date_part) = file_name_str.strip_prefix("network-").and_then(|s| s.strip_suffix(".json")) {
                dates.push(date_part.to_string());
            }
        }
    }
    
    dates.sort();
    Ok(dates)
}
