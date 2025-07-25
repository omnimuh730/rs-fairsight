use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use chrono::{DateTime, Utc, NaiveDate, TimeZone};
use crate::traffic_monitor::{TrafficData, NetworkHost, ServiceInfo};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkSession {
    pub adapter_name: String,
    pub start_time: u64,
    pub end_time: Option<u64>,
    pub total_incoming_bytes: u64,
    pub total_outgoing_bytes: u64,
    pub total_incoming_packets: u64,
    pub total_outgoing_packets: u64,
    pub duration: u64,
    pub traffic_data: Vec<TrafficData>,
    pub top_hosts: Vec<NetworkHost>,
    pub top_services: Vec<ServiceInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyNetworkSummary {
    pub date: String, // YYYY-MM-DD format
    pub sessions: Vec<NetworkSession>,
    pub total_incoming_bytes: u64,
    pub total_outgoing_bytes: u64,
    pub total_duration: u64,
    pub unique_hosts: usize,
    pub unique_services: usize,
}

pub struct NetworkStorageManager {
    storage_dir: PathBuf,
    backup_dir: PathBuf,
}

impl NetworkStorageManager {
    pub fn new() -> Result<Self, String> {
        // Use the same base directory as activity logging: C:\fairsight-log
        // Network data will be stored in C:\fairsight-network-log
        let storage_dir = std::path::Path::new("C:\\fairsight-network-log");
        let backup_dir = std::path::Path::new("C:\\fairsight-network-backup");
        
        if !storage_dir.exists() {
            fs::create_dir_all(&storage_dir)
                .map_err(|e| format!("Failed to create network storage directory: {}", e))?;
        }
        
        if !backup_dir.exists() {
            fs::create_dir_all(&backup_dir)
                .map_err(|e| format!("Failed to create network backup directory: {}", e))?;
        }
        
        Ok(Self { 
            storage_dir: storage_dir.to_path_buf(),
            backup_dir: backup_dir.to_path_buf(),
        })
    }

    pub fn save_session(&self, session: &NetworkSession) -> Result<(), String> {
        let date = DateTime::<Utc>::from_timestamp(session.start_time as i64, 0)
            .ok_or("Invalid timestamp")?
            .format("%Y-%m-%d")
            .to_string();
        
        let mut daily_summary = self.load_daily_summary(&date).unwrap_or_else(|_| {
            DailyNetworkSummary {
                date: date.clone(),
                sessions: Vec::new(),
                total_incoming_bytes: 0,
                total_outgoing_bytes: 0,
                total_duration: 0,
                unique_hosts: 0,
                unique_services: 0,
            }
        });

        // Add new session
        daily_summary.sessions.push(session.clone());
        
        // Update totals
        daily_summary.total_incoming_bytes += session.total_incoming_bytes;
        daily_summary.total_outgoing_bytes += session.total_outgoing_bytes;
        daily_summary.total_duration += session.duration;
        
        // Calculate unique hosts and services
        let mut all_hosts = std::collections::HashSet::new();
        let mut all_services = std::collections::HashSet::new();
        
        for sess in &daily_summary.sessions {
            for host in &sess.top_hosts {
                all_hosts.insert(&host.ip);
            }
            for service in &sess.top_services {
                all_services.insert(format!("{}:{}", service.protocol, service.port));
            }
        }
        
        daily_summary.unique_hosts = all_hosts.len();
        daily_summary.unique_services = all_services.len();

        // Save updated daily summary
        self.save_daily_summary(&date, &daily_summary)
    }

    pub fn load_daily_summary(&self, date: &str) -> Result<DailyNetworkSummary, String> {
        let file_path = self.storage_dir.join(format!("network-{}.json", date));
        
        if !file_path.exists() {
            return Err("Daily summary not found".to_string());
        }
        
        let content = fs::read_to_string(&file_path)
            .map_err(|e| format!("Failed to read daily summary: {}", e))?;
        
        let summary: DailyNetworkSummary = serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse daily summary: {}", e))?;
        
        Ok(summary)
    }

    fn save_daily_summary(&self, date: &str, summary: &DailyNetworkSummary) -> Result<(), String> {
        let file_path = self.storage_dir.join(format!("network-{}.json", date));
        
        // Create backup before saving if file exists
        if file_path.exists() {
            if let Err(e) = self.create_backup(date) {
                eprintln!("Warning: Failed to create backup before saving: {}", e);
            }
        }
        
        let content = serde_json::to_string_pretty(summary)
            .map_err(|e| format!("Failed to serialize daily summary: {}", e))?;
        
        // Atomic save using temporary file
        let temp_path = self.storage_dir.join(format!("network-{}.json.tmp", date));
        
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

    pub fn get_date_range_data(&self, start_date: &str, end_date: &str) -> Result<Vec<DailyNetworkSummary>, String> {
        let start = NaiveDate::parse_from_str(start_date, "%Y-%m-%d")
            .map_err(|e| format!("Invalid start date format: {}", e))?;
        let end = NaiveDate::parse_from_str(end_date, "%Y-%m-%d")
            .map_err(|e| format!("Invalid end date format: {}", e))?;
        
        let mut results = Vec::new();
        let mut current_date = start;
        
        while current_date <= end {
            let date_str = current_date.format("%Y-%m-%d").to_string();
            if let Ok(summary) = self.load_daily_summary(&date_str) {
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

    pub fn cleanup_old_data(&self, days_to_keep: u32) -> Result<(), String> {
        let cutoff_date = Utc::now()
            .checked_sub_signed(chrono::Duration::days(days_to_keep as i64))
            .ok_or("Date calculation overflow")?;
        
        let entries = fs::read_dir(&self.storage_dir)
            .map_err(|e| format!("Failed to read storage directory: {}", e))?;
        
        for entry in entries {
            let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
            let file_name = entry.file_name();
            let file_name_str = file_name.to_string_lossy();
            
            if file_name_str.starts_with("network-") && file_name_str.ends_with(".json") {
                // Extract date from filename: network-YYYY-MM-DD.json
                if let Some(date_part) = file_name_str.strip_prefix("network-").and_then(|s| s.strip_suffix(".json")) {
                    if let Ok(file_date) = NaiveDate::parse_from_str(date_part, "%Y-%m-%d") {
                        let file_datetime = Utc.from_utc_datetime(&file_date.and_hms_opt(0, 0, 0).unwrap());
                        
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
        
        Ok(())
    }

    pub fn get_available_dates(&self) -> Result<Vec<String>, String> {
        let entries = fs::read_dir(&self.storage_dir)
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

    /// Create a backup of network data files
    pub fn create_backup(&self, date: &str) -> Result<(), String> {
        let source_file = self.storage_dir.join(format!("network-{}.json", date));
        
        if !source_file.exists() {
            return Ok(()); // Nothing to backup
        }
        
        // Create backup with timestamp
        let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
        let backup_name = format!("network-{}.json.backup_{}", date, timestamp);
        let temp_backup = self.backup_dir.join(format!("{}.tmp", backup_name));
        let final_backup = self.backup_dir.join(backup_name);
        
        // First copy to temporary file
        fs::copy(&source_file, &temp_backup)
            .map_err(|e| format!("Failed to create temporary backup: {}", e))?;
        
        // Verify the backup by reading it back
        let _ = fs::read(&temp_backup)
            .map_err(|e| format!("Failed to verify backup: {}", e))?;
        
        // Atomically rename to final backup
        fs::rename(&temp_backup, &final_backup)
            .map_err(|e| format!("Failed to finalize backup: {}", e))?;
        
        // Keep only the 5 most recent backups for this date
        self.cleanup_old_backups(date, 5)?;
        
        println!("Network data backup created for date: {}", date);
        Ok(())
    }

    /// Clean up old backup files, keeping only the most recent ones
    fn cleanup_old_backups(&self, date: &str, keep_count: usize) -> Result<(), String> {
        let mut backups = Vec::new();
        
        if let Ok(entries) = fs::read_dir(&self.backup_dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let file_name = entry.file_name();
                    let file_name_str = file_name.to_string_lossy();
                    
                    if file_name_str.starts_with(&format!("network-{}.json.backup_", date)) {
                        if let Ok(metadata) = entry.metadata() {
                            if let Ok(modified) = metadata.modified() {
                                backups.push((entry.path(), modified));
                            }
                        }
                    }
                }
            }
        }
        
        // Sort by modification time (newest first)
        backups.sort_by(|a, b| b.1.cmp(&a.1));
        
        // Remove old backups beyond keep_count
        for (path, _) in backups.into_iter().skip(keep_count) {
            let _ = fs::remove_file(path);
        }
        
        Ok(())
    }

    /// Restore from the most recent valid backup for a specific date
    pub fn restore_from_backup(&self, date: &str) -> Result<(), String> {
        // Find the most recent backup for this date
        let mut backups = Vec::new();
        
        if let Ok(entries) = fs::read_dir(&self.backup_dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let entry_file_name = entry.file_name();
                    let entry_file_name_str = entry_file_name.to_string_lossy();
                    
                    if entry_file_name_str.starts_with(&format!("network-{}.json.backup_", date)) {
                        if let Ok(metadata) = entry.metadata() {
                            if let Ok(modified) = metadata.modified() {
                                backups.push((entry.path(), modified));
                            }
                        }
                    }
                }
            }
        }
        
        if backups.is_empty() {
            return Err(format!("No backup files found for date: {}", date));
        }
        
        // Sort by modification time (newest first)
        backups.sort_by(|a, b| b.1.cmp(&a.1));
        
        // Restore from the most recent backup
        let (backup_path, _) = &backups[0];
        let restore_path = self.storage_dir.join(format!("network-{}.json", date));
        let temp_restore = self.storage_dir.join(format!("network-{}.json.tmp", date));
        
        // Copy backup to temporary file
        fs::copy(backup_path, &temp_restore)
            .map_err(|e| format!("Failed to copy backup: {}", e))?;
        
        // Verify the restore file
        let _ = fs::read(&temp_restore)
            .map_err(|e| format!("Failed to verify restore file: {}", e))?;
        
        // Atomically replace the original
        fs::rename(&temp_restore, &restore_path)
            .map_err(|e| format!("Failed to finalize restore: {}", e))?;
        
        println!("Network data restored from backup for date: {}", date);
        Ok(())
    }

    /// Perform daily cleanup of old backup files (not today's backups)
    pub fn daily_backup_cleanup(&self) -> Result<(), String> {
        let today = chrono::Local::now().format("%Y-%m-%d").to_string();
        let cutoff_date = chrono::Local::now() - chrono::Duration::days(7); // Keep backups for 7 days
        
        if let Ok(entries) = fs::read_dir(&self.backup_dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let file_name = entry.file_name();
                    let file_name_str = file_name.to_string_lossy();
                    
                    // Skip today's backups
                    if file_name_str.contains(&today) {
                        continue;
                    }
                    
                    // Extract timestamp from backup filename
                    if file_name_str.contains(".backup_") {
                        if let Some(timestamp_part) = file_name_str.split(".backup_").nth(1) {
                            if let Ok(backup_date) = chrono::NaiveDateTime::parse_from_str(timestamp_part, "%Y%m%d_%H%M%S") {
                                let backup_datetime = chrono::Local.from_local_datetime(&backup_date);
                                if let Some(backup_datetime) = backup_datetime.single() {
                                    if backup_datetime < cutoff_date {
                                        if let Err(e) = fs::remove_file(entry.path()) {
                                            eprintln!("Failed to remove old network backup: {}", e);
                                        } else {
                                            println!("Removed old network backup: {}", file_name_str);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        
        Ok(())
    }
}

// Global storage manager instance
lazy_static::lazy_static! {
    pub static ref NETWORK_STORAGE: NetworkStorageManager = {
        NetworkStorageManager::new().expect("Failed to initialize network storage")
    };
}
