use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use dashmap::DashMap;
use parking_lot::RwLock;
use rand::Rng;

use super::types::{NetworkHost, ServiceInfo, TrafficData, MonitoringStats};

pub async fn simulate_traffic_tick(
    hosts: &Arc<DashMap<String, NetworkHost>>,
    services: &Arc<DashMap<String, ServiceInfo>>,
    traffic_history: &Arc<Mutex<Vec<TrafficData>>>,
    stats: &Arc<RwLock<MonitoringStats>>,
    start_time: u64,
) {
    let mut rng = rand::thread_rng();
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

    // Simulate random traffic
    let incoming_bytes: u64 = rng.gen_range(1000..50000);
    let outgoing_bytes: u64 = rng.gen_range(500..25000);
    let incoming_packets: u64 = rng.gen_range(10..100);
    let outgoing_packets: u64 = rng.gen_range(5..50);

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
    if rng.gen_bool(0.3) {
        let random_ip = format!("192.168.1.{}", rng.gen_range(1..255));
        hosts.entry(random_ip.clone()).or_insert_with(|| {
            NetworkHost {
                ip: random_ip,
                hostname: Some(format!("host-{}", rng.gen_range(1..1000))),
                domain: Some("local.lan".to_string()),
                country: Some("Unknown".to_string()),
                country_code: Some("XX".to_string()),
                asn: None,
                incoming_bytes: 0,
                outgoing_bytes: 0,
                incoming_packets: 0,
                outgoing_packets: 0,
                first_seen: now,
                last_seen: now,
            }
        }).and_modify(|host| {
            host.incoming_bytes += rng.gen_range(100..5000);
            host.outgoing_bytes += rng.gen_range(50..2500);
            host.incoming_packets += rng.gen_range(1..20);
            host.outgoing_packets += rng.gen_range(1..10);
            host.last_seen = now;
        });
    }

    // Add some random services
    if rng.gen_bool(0.2) {
        let common_ports = [80, 443, 22, 21, 25, 53, 110, 143, 993, 995];
        let port = common_ports[rng.gen_range(0..common_ports.len())];
        let protocol = if rng.gen_bool(0.8) { "TCP" } else { "UDP" };
        let service_key = format!("{}:{}", protocol, port);
        
        services.entry(service_key.clone()).or_insert_with(|| {
            ServiceInfo {
                protocol: protocol.to_string(),
                port,
                service_name: get_service_name(port),
                bytes: 0,
                packets: 0,
            }
        }).and_modify(|service| {
            service.bytes += rng.gen_range(100..10000);
            service.packets += rng.gen_range(1..50);
        });
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

fn get_service_name(port: u16) -> Option<String> {
    match port {
        80 => Some("HTTP".to_string()),
        443 => Some("HTTPS".to_string()),
        22 => Some("SSH".to_string()),
        21 => Some("FTP".to_string()),
        25 => Some("SMTP".to_string()),
        53 => Some("DNS".to_string()),
        110 => Some("POP3".to_string()),
        143 => Some("IMAP".to_string()),
        993 => Some("IMAPS".to_string()),
        995 => Some("POP3S".to_string()),
        3389 => Some("RDP".to_string()),
        5432 => Some("PostgreSQL".to_string()),
        3306 => Some("MySQL".to_string()),
        1433 => Some("MSSQL".to_string()),
        6379 => Some("Redis".to_string()),
        27017 => Some("MongoDB".to_string()),
        _ => None,
    }
}
