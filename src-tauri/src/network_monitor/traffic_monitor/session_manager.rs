use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
use parking_lot::RwLock;
use chrono::{Local, Datelike};

use crate::network_monitor::network_storage::{NETWORK_STORAGE, NetworkSession};
use crate::network_monitor::persistent_state::get_persistent_state_manager;
use super::types::{MonitoringStats, TrafficData};
use super::monitor::TRAFFIC_MONITORS;

pub async fn save_periodic_session(
    adapter_name: &str,
    stats: &Arc<RwLock<MonitoringStats>>,
    _start_time: &u64,
    last_save_time: &mut u64,
    last_save_incoming_bytes: &mut u64,
    last_save_outgoing_bytes: &mut u64,
    last_save_incoming_packets: &mut u64,
    last_save_outgoing_packets: &mut u64,
    _traffic_history: &Arc<Mutex<Vec<TrafficData>>>,
    last_known_date: &Arc<RwLock<Option<u32>>>,
) {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let today = Local::now().ordinal();

    // Check for day change
    let mut last_date = last_known_date.write();
    if last_date.map_or(true, |d| d != today) {
        if let Some(monitor) = TRAFFIC_MONITORS.get(adapter_name) {
            monitor.reset_daily_stats();
        }
        *last_date = Some(today);
    }
    drop(last_date);

    let current_stats = {
        let stats_guard = stats.read();
        stats_guard.clone()
    };
    
    let incremental_incoming = current_stats.total_incoming_bytes - *last_save_incoming_bytes;
    let incremental_outgoing = current_stats.total_outgoing_bytes - *last_save_outgoing_bytes;
    let incremental_incoming_packets = current_stats.total_incoming_packets - *last_save_incoming_packets;
    let incremental_outgoing_packets = current_stats.total_outgoing_packets - *last_save_outgoing_packets;
    
    // Update persistent state with current cumulative totals
    if let Err(e) = get_persistent_state_manager().update_adapter_state(adapter_name, |state| {
        state.cumulative_incoming_bytes = current_stats.total_incoming_bytes;
        state.cumulative_outgoing_bytes = current_stats.total_outgoing_bytes;
        state.cumulative_incoming_packets = current_stats.total_incoming_packets;
        state.cumulative_outgoing_packets = current_stats.total_outgoing_packets;
        state.lifetime_incoming_bytes = current_stats.total_incoming_bytes;
        state.lifetime_outgoing_bytes = current_stats.total_outgoing_bytes;
    }) {
        // Only log persistent state errors occasionally to avoid spam
        if now % 30 == 0 {  // Log error every 30 seconds max
            eprintln!("⚠️  Persistent state update failed (will retry): {}", e);
        }
    }
    
    // Only save session if there's actual incremental traffic data
    if incremental_incoming == 0 && incremental_outgoing == 0 {
        return;
    }
    
    let session = NetworkSession {
        adapter_name: adapter_name.to_string(),
        start_time: *last_save_time,
        end_time: Some(now),
        total_incoming_bytes: incremental_incoming,
        total_outgoing_bytes: incremental_outgoing,
        total_incoming_packets: incremental_incoming_packets,
        total_outgoing_packets: incremental_outgoing_packets,
        duration: now - *last_save_time,
        traffic_data: current_stats.traffic_rate.clone(),
        top_hosts: current_stats.network_hosts.iter().take(10).cloned().collect(),
        top_services: current_stats.services.iter().take(10).cloned().collect(),
    };

    if let Err(e) = NETWORK_STORAGE.save_session(&session) {
        eprintln!("Failed to save periodic network session: {}", e);
    } else {
        println!("📊 Periodic session saved (8s) - Incremental: ↓{}KB ↑{}KB (Cumulative: ↓{}KB ↑{}KB)", 
            incremental_incoming / 1024, 
            incremental_outgoing / 1024,
            current_stats.total_incoming_bytes / 1024,
            current_stats.total_outgoing_bytes / 1024);
    }
    
    *last_save_time = now;
    *last_save_incoming_bytes = current_stats.total_incoming_bytes;
    *last_save_outgoing_bytes = current_stats.total_outgoing_bytes;
    *last_save_incoming_packets = current_stats.total_incoming_packets;
    *last_save_outgoing_packets = current_stats.total_outgoing_packets;
}

pub fn save_final_session(
    adapter_name: &str,
    start_time: u64,
    stats: &MonitoringStats,
) {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    
    // Save final session if there's any data
    if stats.total_incoming_bytes > 0 || stats.total_outgoing_bytes > 0 {
        let final_session = NetworkSession {
            adapter_name: adapter_name.to_string(),
            start_time,
            end_time: Some(now),
            total_incoming_bytes: stats.total_incoming_bytes,
            total_outgoing_bytes: stats.total_outgoing_bytes,
            total_incoming_packets: stats.total_incoming_packets,
            total_outgoing_packets: stats.total_outgoing_packets,
            duration: now - start_time,
            traffic_data: stats.traffic_rate.clone(),
            top_hosts: stats.network_hosts.iter().take(10).cloned().collect(),
            top_services: stats.services.iter().take(10).cloned().collect(),
        };

        if let Err(e) = NETWORK_STORAGE.save_session(&final_session) {
            eprintln!("Failed to save final network session: {}", e);
        } else {
            println!("Final network session saved on stop - Total: ↓{}KB ↑{}KB", 
                stats.total_incoming_bytes / 1024, 
                stats.total_outgoing_bytes / 1024);
        }
    }

    // Update persistent state to mark as not monitoring and save cumulative totals
    if let Err(e) = get_persistent_state_manager().update_adapter_state(adapter_name, |state| {
        state.cumulative_incoming_bytes = stats.total_incoming_bytes;
        state.cumulative_outgoing_bytes = stats.total_outgoing_bytes;
        state.cumulative_incoming_packets = stats.total_incoming_packets;
        state.cumulative_outgoing_packets = stats.total_outgoing_packets;
        state.last_session_end_time = Some(now);
        state.was_monitoring_on_exit = false;
        state.lifetime_incoming_bytes = stats.total_incoming_bytes;
        state.lifetime_outgoing_bytes = stats.total_outgoing_bytes;
    }) {
        eprintln!("⚠️  Failed to update persistent state on stop: {}", e);
    }
}
