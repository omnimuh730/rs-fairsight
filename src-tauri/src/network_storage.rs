use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use chrono::{Utc, NaiveDate, TimeZone};
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
        let start = chrono::Local.timestamp_opt(session.start_time as i64, 0)
            .single()
            .ok_or("Invalid start timestamp")?;
        let end = session.end_time.map(|et| chrono::Local.timestamp_opt(et as i64, 0).single()).flatten();
        let end = end.unwrap_or(start);

        let mut current_start = start;
        let current_end = end;
        let mut remaining_duration = session.duration;
        let mut remaining_in_bytes = session.total_incoming_bytes;
        let mut remaining_out_bytes = session.total_outgoing_bytes;
        let mut remaining_in_packets = session.total_incoming_packets;
        let mut remaining_out_packets = session.total_outgoing_packets;
        let traffic_data = session.traffic_data.clone();
        let top_hosts = session.top_hosts.clone();
        let top_services = session.top_services.clone();

        let mut result = Ok(());

        // Helper to calculate seconds between two DateTimes
        fn seconds_between(a: chrono::DateTime<chrono::Local>, b: chrono::DateTime<chrono::Local>) -> u64 {
            if b > a {
                (b.timestamp() - a.timestamp()) as u64
            } else {
                0
            }
        }

        // Split at each midnight boundary
        while current_start.date_naive() < current_end.date_naive() {
            let next_midnight = current_start.date_naive().succ_opt().unwrap().and_hms_opt(0, 0, 0).unwrap();
            let next_midnight_dt = chrono::Local.from_local_datetime(&next_midnight).single().unwrap();
            let part_duration = seconds_between(current_start, next_midnight_dt);
            let part_in_bytes = (session.total_incoming_bytes as f64 * part_duration as f64 / session.duration as f64).round() as u64;
            let part_out_bytes = (session.total_outgoing_bytes as f64 * part_duration as f64 / session.duration as f64).round() as u64;
            let part_in_packets = (session.total_incoming_packets as f64 * part_duration as f64 / session.duration as f64).round() as u64;
            let part_out_packets = (session.total_outgoing_packets as f64 * part_duration as f64 / session.duration as f64).round() as u64;

            let date = current_start.format("%Y-%m-%d").to_string();
            let mut daily_summary = self.load_daily_summary(&date).unwrap_or_else(|_| DailyNetworkSummary {
                date: date.clone(),
                sessions: Vec::new(),
                total_incoming_bytes: 0,
                total_outgoing_bytes: 0,
                total_duration: 0,
                unique_hosts: 0,
                unique_services: 0,
            });
            let split_session = NetworkSession {
                adapter_name: session.adapter_name.clone(),
                start_time: current_start.timestamp() as u64,
                end_time: Some(next_midnight_dt.timestamp() as u64),
                total_incoming_bytes: part_in_bytes,
                total_outgoing_bytes: part_out_bytes,
                total_incoming_packets: part_in_packets,
                total_outgoing_packets: part_out_packets,
                duration: part_duration,
                traffic_data: Vec::new(), // Not splitting traffic_data for now
                top_hosts: top_hosts.clone(),
                top_services: top_services.clone(),
            };
            daily_summary.sessions.push(split_session);
            daily_summary.total_incoming_bytes += part_in_bytes;
            daily_summary.total_outgoing_bytes += part_out_bytes;
            daily_summary.total_duration += part_duration;
            if daily_summary.sessions.len() > 100 {
                daily_summary = self.consolidate_sessions(daily_summary)?;
            }
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
            if let Err(e) = self.save_daily_summary(&date, &daily_summary) {
                result = Err(e);
            }
            // Move to next day
            current_start = next_midnight_dt;
            remaining_duration = remaining_duration.saturating_sub(part_duration);
            remaining_in_bytes = remaining_in_bytes.saturating_sub(part_in_bytes);
            remaining_out_bytes = remaining_out_bytes.saturating_sub(part_out_bytes);
            remaining_in_packets = remaining_in_packets.saturating_sub(part_in_packets);
            remaining_out_packets = remaining_out_packets.saturating_sub(part_out_packets);
        }
        // Last part (or only part if not spanning days)
        let date = current_start.format("%Y-%m-%d").to_string();
        let mut daily_summary = self.load_daily_summary(&date).unwrap_or_else(|_| DailyNetworkSummary {
            date: date.clone(),
            sessions: Vec::new(),
            total_incoming_bytes: 0,
            total_outgoing_bytes: 0,
            total_duration: 0,
            unique_hosts: 0,
            unique_services: 0,
        });
        let split_session = NetworkSession {
            adapter_name: session.adapter_name.clone(),
            start_time: current_start.timestamp() as u64,
            end_time: Some(current_end.timestamp() as u64),
            total_incoming_bytes: remaining_in_bytes,
            total_outgoing_bytes: remaining_out_bytes,
            total_incoming_packets: remaining_in_packets,
            total_outgoing_packets: remaining_out_packets,
            duration: remaining_duration,
            traffic_data,
            top_hosts,
            top_services,
        };
        daily_summary.sessions.push(split_session);
        if cfg!(target_os = "macos") {
            // Deduplicate by unique host and service for macOS
            use std::collections::HashMap;
            let mut host_map: HashMap<String, u64> = HashMap::new();
            let mut service_map: HashMap<String, u64> = HashMap::new();
            let mut total_in_bytes = 0u64;
            let mut total_out_bytes = 0u64;
            for sess in &daily_summary.sessions {
                for host in &sess.top_hosts {
                    let entry = host_map.entry(host.ip.clone()).or_insert(0);
                    if *entry == 0 {
                        total_in_bytes += host.incoming_bytes;
                        total_out_bytes += host.outgoing_bytes;
                    }
                    *entry += 1;
                }
                for service in &sess.top_services {
                    let key = format!("{}:{}", service.protocol, service.port);
                    let entry = service_map.entry(key).or_insert(0);
                    if *entry == 0 {
                        // Only count bytes for unique service
                        // If you want to sum bytes, you can add service.bytes here
                    }
                    *entry += 1;
                }
            }
            daily_summary.total_incoming_bytes = total_in_bytes;
            daily_summary.total_outgoing_bytes = total_out_bytes;
            daily_summary.total_duration = daily_summary.sessions.iter().map(|s| s.duration).sum();
            daily_summary.unique_hosts = host_map.len();
            daily_summary.unique_services = service_map.len();
        } else {
            daily_summary.total_incoming_bytes += remaining_in_bytes;
            daily_summary.total_outgoing_bytes += remaining_out_bytes;
            daily_summary.total_duration += remaining_duration;
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
        }
        if let Err(e) = self.save_daily_summary(&date, &daily_summary) {
            result = Err(e);
        }
        result
    }

    fn consolidate_sessions(&self, mut summary: DailyNetworkSummary) -> Result<DailyNetworkSummary, String> {
        // Group sessions by adapter and time windows (30-minute chunks)
        let mut consolidated_sessions = Vec::new();
        let time_window = 1800; // 30 minutes
        
        // Sort sessions by start time
        summary.sessions.sort_by_key(|s| s.start_time);
        
        // Group sessions by adapter
        let mut adapter_groups: std::collections::HashMap<String, Vec<NetworkSession>> = std::collections::HashMap::new();
        for session in summary.sessions {
            adapter_groups.entry(session.adapter_name.clone()).or_insert_with(Vec::new).push(session);
        }
        
        for (adapter, sessions) in adapter_groups {
            let mut current_group = Vec::new();
            let mut last_time_window = 0;
            
            for session in sessions {
                let time_window_start = (session.start_time / time_window) * time_window;
                
                if time_window_start != last_time_window && !current_group.is_empty() {
                    // Consolidate the current group
                    consolidated_sessions.push(self.merge_sessions(&adapter, &current_group)?);
                    current_group.clear();
                }
                
                current_group.push(session);
                last_time_window = time_window_start;
            }
            
            if !current_group.is_empty() {
                consolidated_sessions.push(self.merge_sessions(&adapter, &current_group)?);
            }
        }
        
        summary.sessions = consolidated_sessions;
        
        println!("📊 Consolidated sessions: reduced from {} to {} sessions", 
            summary.sessions.len() + 100, // Original count before consolidation
            summary.sessions.len());
        
        Ok(summary)
    }
    
    fn merge_sessions(&self, adapter_name: &str, sessions: &[NetworkSession]) -> Result<NetworkSession, String> {
        if sessions.is_empty() {
            return Err("Cannot merge empty sessions".to_string());
        }
        
        let first_session = &sessions[0];
        let last_session = &sessions[sessions.len() - 1];
        
        let total_incoming: u64 = sessions.iter().map(|s| s.total_incoming_bytes).sum();
        let total_outgoing: u64 = sessions.iter().map(|s| s.total_outgoing_bytes).sum();
        let total_incoming_packets: u64 = sessions.iter().map(|s| s.total_incoming_packets).sum();
        let total_outgoing_packets: u64 = sessions.iter().map(|s| s.total_outgoing_packets).sum();
        let total_duration: u64 = sessions.iter().map(|s| s.duration).sum();
        
        // Merge hosts and services (keep top entries)
        let mut all_hosts = Vec::new();
        let mut all_services = Vec::new();
        
        for session in sessions {
            all_hosts.extend(session.top_hosts.iter().cloned());
            all_services.extend(session.top_services.iter().cloned());
        }
        
        // Deduplicate and sort hosts
        all_hosts.sort_by(|a, b| {
            let total_a = a.incoming_bytes + a.outgoing_bytes;
            let total_b = b.incoming_bytes + b.outgoing_bytes;
            total_b.cmp(&total_a)
        });
        all_hosts.truncate(10);
        
        // Deduplicate and sort services
        all_services.sort_by(|a, b| b.bytes.cmp(&a.bytes));
        all_services.truncate(10);
        
        Ok(NetworkSession {
            adapter_name: adapter_name.to_string(),
            start_time: first_session.start_time,
            end_time: last_session.end_time,
            total_incoming_bytes: total_incoming,
            total_outgoing_bytes: total_outgoing,
            total_incoming_packets: total_incoming_packets,
            total_outgoing_packets: total_outgoing_packets,
            duration: total_duration,
            traffic_data: Vec::new(), // Clear detailed traffic data for consolidated sessions
            top_hosts: all_hosts,
            top_services: all_services,
        })
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

    pub fn save_daily_summary(&self, date: &str, summary: &DailyNetworkSummary) -> Result<(), String> {
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
