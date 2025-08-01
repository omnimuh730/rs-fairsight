use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
use dashmap::DashMap;
use parking_lot::RwLock;
use rand::Rng;

use super::types::{NetworkHost, ServiceInfo, TrafficData, MonitoringStats};
use super::service_analysis::simulate_service;
use super::host_analysis::simulate_network_host;

pub async fn simulate_traffic_tick(
    hosts: &Arc<DashMap<String, NetworkHost>>,
    services: &Arc<DashMap<String, ServiceInfo>>,
    traffic_history: &Arc<Mutex<Vec<TrafficData>>>,
    stats: &Arc<RwLock<MonitoringStats>>,
    start_time: u64,
) {
    let mut rng = rand::rng();
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

    // Simulate random traffic
    let incoming_bytes: u64 = rng.random_range(1000..50000);
    let outgoing_bytes: u64 = rng.random_range(500..25000);
    let incoming_packets: u64 = rng.random_range(10..100);
    let outgoing_packets: u64 = rng.random_range(5..50);

    // Update stats
    {
        let mut stats_guard = stats.write();
        stats_guard.total_incoming_bytes += incoming_bytes;
        stats_guard.total_outgoing_bytes += outgoing_bytes;
        stats_guard.total_incoming_packets += incoming_packets;
        stats_guard.total_outgoing_packets += outgoing_packets;
        stats_guard.monitoring_duration = now - start_time;
    }

    // Add some random hosts
    if rng.random_bool(0.3) {
        simulate_network_host(&hosts, now);
    }

    // Add some random services
    if rng.random_bool(0.2) {
        let total_bytes = incoming_bytes + outgoing_bytes;
        simulate_service(&services, total_bytes);
    }

    // Update traffic history
    let current_data = {
        let stats_guard = stats.read();
        TrafficData {
            timestamp: now,
            incoming_bytes: stats_guard.total_incoming_bytes,
            outgoing_bytes: stats_guard.total_outgoing_bytes,
            incoming_packets: stats_guard.total_incoming_packets,
            outgoing_packets: stats_guard.total_outgoing_packets,
        }
    };

    if let Ok(mut history) = traffic_history.lock() {
        history.push(current_data);
        // Keep only recent history (last 3600 entries)
        if history.len() > 3600 {
            history.remove(0);
        }
    }
}
