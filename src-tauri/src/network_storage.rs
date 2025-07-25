use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{PathBuf};
use chrono::{DateTime, Utc, NaiveDate, TimeZone};
use crate::traffic_monitor::{MonitoringStats, TrafficData, NetworkHost, ServiceInfo};

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
}

impl NetworkStorageManager {
    pub fn new() -> Result<Self, String> {
        let home_dir = dirs::home_dir().ok_or("Could not find home directory")?;
        let storage_dir = home_dir.join("fairsight-network-log");
        
        if !storage_dir.exists() {
            fs::create_dir_all(&storage_dir)
                .map_err(|e| format!("Failed to create network storage directory: {}", e))?;
        }
        
        Ok(Self { storage_dir })
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
        
        let content = serde_json::to_string_pretty(summary)
            .map_err(|e| format!("Failed to serialize daily summary: {}", e))?;
        
        fs::write(&file_path, content)
            .map_err(|e| format!("Failed to save daily summary: {}", e))?;
        
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
}

// Global storage manager instance
lazy_static::lazy_static! {
    pub static ref NETWORK_STORAGE: NetworkStorageManager = {
        NetworkStorageManager::new().expect("Failed to initialize network storage")
    };
}
