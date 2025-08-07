use std::path::PathBuf;
use chrono::{Local, TimeZone};

use super::types::{NetworkSession, DailyNetworkSummary};
use super::utils::get_platform_directories;
use super::file_ops::{load_daily_summary, save_daily_summary, get_date_range_data, cleanup_old_data, get_available_dates};
use super::backup::{create_backup, restore_from_backup, daily_backup_cleanup};
use super::consolidation::{consolidate_sessions, calculate_unique_counts};

#[cfg(target_os = "macos")]
use super::consolidation::calculate_macos_totals;

pub struct NetworkStorageManager {
    storage_dir: PathBuf,
    backup_dir: PathBuf,
}

impl NetworkStorageManager {
    pub fn new() -> Result<Self, String> {
        let (storage_dir, backup_dir) = get_platform_directories()?;
        Ok(Self {
            storage_dir,
            backup_dir,
        })
    }

    pub fn save_session(&self, session: &NetworkSession) -> Result<(), String> {
        let start = Local.timestamp_opt(session.start_time as i64, 0)
            .single()
            .ok_or("Invalid start timestamp")?;
        let end = session.end_time.map(|et| Local.timestamp_opt(et as i64, 0).single()).flatten();
        let end = end.unwrap_or(start);

        self.process_session_across_days(session, start, end)
    }

    fn process_session_across_days(
        &self, 
        session: &NetworkSession, 
        start: chrono::DateTime<Local>, 
        end: chrono::DateTime<Local>
    ) -> Result<(), String> {
        let mut current_start = start;
        let current_end = end;
        let mut remaining_duration = session.duration;
        let mut remaining_in_bytes = session.total_incoming_bytes;
        let mut remaining_out_bytes = session.total_outgoing_bytes;
        let mut remaining_in_packets = session.total_incoming_packets;
        let mut remaining_out_packets = session.total_outgoing_packets;
        let traffic_data = session.traffic_data.clone();
        let _top_hosts = session.top_hosts.clone();
        let top_services = session.top_services.clone();

        let mut result = Ok(());

        // Split at each midnight boundary
        while current_start.date_naive() < current_end.date_naive() {
            let next_midnight = current_start.date_naive().succ_opt().unwrap().and_hms_opt(0, 0, 0).unwrap();
            let next_midnight_dt = Local.from_local_datetime(&next_midnight).single().unwrap();
            let part_duration = seconds_between(current_start, next_midnight_dt);
            
            let (part_in_bytes, part_out_bytes, part_in_packets, part_out_packets) = 
                calculate_proportional_split(session, part_duration);

            if let Err(e) = self.save_day_part(
                session, 
                current_start, 
                next_midnight_dt, 
                part_duration, 
                part_in_bytes, 
                part_out_bytes, 
                part_in_packets, 
                part_out_packets,
                &Vec::new(), // Empty traffic data for partial sessions
                &top_services
            ) {
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
        if let Err(e) = self.save_day_part(
            session,
            current_start,
            current_end,
            remaining_duration,
            remaining_in_bytes,
            remaining_out_bytes,
            remaining_in_packets,
            remaining_out_packets,
            &traffic_data,
            &top_services
        ) {
            result = Err(e);
        }

        result
    }

    fn save_day_part(
        &self,
        session: &NetworkSession,
        start: chrono::DateTime<Local>,
        end: chrono::DateTime<Local>,
        duration: u64,
        in_bytes: u64,
        out_bytes: u64,
        in_packets: u64,
        out_packets: u64,
        traffic_data: &[crate::traffic_monitor::TrafficData],
        top_services: &[crate::traffic_monitor::ServiceInfo]
    ) -> Result<(), String> {
        // Safety check: ensure no session duration exceeds 24 hours for a single day
        const MAX_SECONDS_PER_DAY: u64 = 24 * 60 * 60; // 86400 seconds
        if duration > MAX_SECONDS_PER_DAY {
            eprintln!("⚠️  Warning: Session duration ({}) exceeds 24 hours for single day. Capping at 24 hours.", duration);
            // Cap the duration at 24 hours to prevent display issues
            let capped_duration = MAX_SECONDS_PER_DAY;
            // Adjust bytes proportionally
            let ratio = capped_duration as f64 / duration as f64;
            let capped_in_bytes = (in_bytes as f64 * ratio).round() as u64;
            let capped_out_bytes = (out_bytes as f64 * ratio).round() as u64;
            let capped_in_packets = (in_packets as f64 * ratio).round() as u64;
            let capped_out_packets = (out_packets as f64 * ratio).round() as u64;
            
            return self.save_day_part(
                session,
                start,
                end,
                capped_duration,
                capped_in_bytes,
                capped_out_bytes,
                capped_in_packets,
                capped_out_packets,
                traffic_data,
                top_services
            );
        }

        let date = start.format("%Y-%m-%d").to_string();
        let mut daily_summary = load_daily_summary(&self.storage_dir, &date)
            .unwrap_or_else(|_| DailyNetworkSummary {
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
            start_time: start.timestamp() as u64,
            end_time: Some(end.timestamp() as u64),
            total_incoming_bytes: in_bytes,
            total_outgoing_bytes: out_bytes,
            total_incoming_packets: in_packets,
            total_outgoing_packets: out_packets,
            duration,
            traffic_data: traffic_data.to_vec(),
            top_hosts: session.top_hosts.clone(),
            top_services: top_services.to_vec(),
        };

        daily_summary.sessions.push(split_session);

        // Consolidate if too many sessions
        if daily_summary.sessions.len() > 100 {
            daily_summary.sessions = consolidate_sessions(daily_summary.sessions, &session.adapter_name)?;
        }

        self.update_daily_summary_totals(&mut daily_summary);
        save_daily_summary(&self.storage_dir, &self.backup_dir, &date, &daily_summary)
    }

    fn update_daily_summary_totals(&self, daily_summary: &mut DailyNetworkSummary) {
        #[cfg(target_os = "macos")]
        {
            let (total_in, total_out, total_duration) = calculate_macos_totals(&daily_summary.sessions);
            daily_summary.total_incoming_bytes = total_in;
            daily_summary.total_outgoing_bytes = total_out;
            daily_summary.total_duration = total_duration;
            let (unique_hosts, unique_services) = calculate_unique_counts(&daily_summary.sessions);
            daily_summary.unique_hosts = unique_hosts;
            daily_summary.unique_services = unique_services;
        }
        #[cfg(not(target_os = "macos"))]
        {
            daily_summary.total_incoming_bytes = daily_summary.sessions.iter().map(|s| s.total_incoming_bytes).sum();
            daily_summary.total_outgoing_bytes = daily_summary.sessions.iter().map(|s| s.total_outgoing_bytes).sum();
            daily_summary.total_duration = daily_summary.sessions.iter().map(|s| s.duration).sum();
            let (unique_hosts, unique_services) = calculate_unique_counts(&daily_summary.sessions);
            daily_summary.unique_hosts = unique_hosts;
            daily_summary.unique_services = unique_services;
        }
    }

    // Re-export file operations
    pub fn load_daily_summary(&self, date: &str) -> Result<DailyNetworkSummary, String> {
        load_daily_summary(&self.storage_dir, date)
    }

    pub fn get_date_range_data(&self, start_date: &str, end_date: &str) -> Result<Vec<DailyNetworkSummary>, String> {
        get_date_range_data(&self.storage_dir, start_date, end_date)
    }

    pub fn cleanup_old_data(&self, days_to_keep: u32) -> Result<(), String> {
        cleanup_old_data(&self.storage_dir, days_to_keep)
    }

    pub fn get_available_dates(&self) -> Result<Vec<String>, String> {
        get_available_dates(&self.storage_dir)
    }

    // Re-export backup operations
    pub fn create_backup(&self, date: &str) -> Result<(), String> {
        create_backup(&self.storage_dir, &self.backup_dir, date)
    }

    pub fn restore_from_backup(&self, date: &str) -> Result<(), String> {
        restore_from_backup(&self.storage_dir, &self.backup_dir, date)
    }

    pub fn daily_backup_cleanup(&self) -> Result<(), String> {
        daily_backup_cleanup(&self.backup_dir)
    }
}

// Helper functions
fn seconds_between(a: chrono::DateTime<Local>, b: chrono::DateTime<Local>) -> u64 {
    if b > a {
        (b.timestamp() - a.timestamp()) as u64
    } else {
        0
    }
}

fn calculate_proportional_split(session: &NetworkSession, part_duration: u64) -> (u64, u64, u64, u64) {
    let ratio = part_duration as f64 / session.duration as f64;
    let part_in_bytes = (session.total_incoming_bytes as f64 * ratio).round() as u64;
    let part_out_bytes = (session.total_outgoing_bytes as f64 * ratio).round() as u64;
    let part_in_packets = (session.total_incoming_packets as f64 * ratio).round() as u64;
    let part_out_packets = (session.total_outgoing_packets as f64 * ratio).round() as u64;
    (part_in_bytes, part_out_bytes, part_in_packets, part_out_packets)
}
