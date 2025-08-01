use pcap::{Capture, Device};
use etherparse::LaxPacketHeaders;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::Arc;
use dashmap::DashMap;
use parking_lot::RwLock;

use super::types::{NetworkHost, ServiceInfo, MonitoringStats, TrafficData};
use super::deduplication::{create_packet_signature, is_duplicate_packet, register_packet};

pub fn create_packet_capture(adapter_name: &str) -> Option<Capture<pcap::Active>> {
    if let Ok(devices) = Device::list() {
        if let Some(device) = devices.into_iter().find(|d| d.name == adapter_name) {
            match Capture::from_device(device) {
                Ok(inactive) => {
                    match inactive
                        .promisc(true)
                        .buffer_size(8_000_000)  // Increase to 8MB buffer for better capture
                        .snaplen(200)            // Limit packet slice but count full packet size
                        .immediate_mode(true)    // Parse packets ASAP
                        .timeout(100)            // Shorter timeout for more responsive capture
                        .open() {
                        Ok(cap) => {
                            println!("Successfully opened packet capture on {}", adapter_name);
                            return Some(cap);
                        }
                        Err(e) => {
                            eprintln!("Failed to open capture on {}: {}. Falling back to simulation.", adapter_name, e);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Failed to create capture from device {}: {}. Falling back to simulation.", adapter_name, e);
                }
            }
        } else {
            eprintln!("Device {} not found. Falling back to simulation.", adapter_name);
        }
    } else {
        eprintln!("Failed to list devices. Falling back to simulation.");
    }
    None
}

pub async fn process_real_packet(
    packet: pcap::Packet<'_>,
    hosts: &Arc<DashMap<String, NetworkHost>>,
    services: &Arc<DashMap<String, ServiceInfo>>,
    traffic_history: &Arc<std::sync::Mutex<Vec<TrafficData>>>,
    stats: &Arc<RwLock<MonitoringStats>>,
    start_time: u64,
    adapter_name: &str,
) {
    if let Ok(headers) = LaxPacketHeaders::from_ethernet(&packet.data) {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

        let (src_ip, dst_ip) = match &headers.net {
            Some(etherparse::NetHeaders::Ipv4(ipv4, _)) => (
                IpAddr::V4(Ipv4Addr::from(ipv4.source)),
                IpAddr::V4(Ipv4Addr::from(ipv4.destination)),
            ),
            Some(etherparse::NetHeaders::Ipv6(ipv6, _)) => (
                IpAddr::V6(Ipv6Addr::from(ipv6.source)),
                IpAddr::V6(Ipv6Addr::from(ipv6.destination)),
            ),
            _ => return, // Skip non-IP packets
        };

        let (src_port, dst_port, protocol) = match &headers.transport {
            Some(etherparse::TransportHeader::Tcp(tcp)) => (
                tcp.source_port,
                tcp.destination_port,
                "TCP".to_string(),
            ),
            Some(etherparse::TransportHeader::Udp(udp)) => (
                udp.source_port,
                udp.destination_port,
                "UDP".to_string(),
            ),
            _ => (0, 0, "Other".to_string()),
        };

        // Create packet signature for deduplication
        let packet_signature = create_packet_signature(
            &src_ip.to_string(),
            &dst_ip.to_string(),
            src_port,
            dst_port,
            &protocol,
            packet.header.ts.tv_usec as u64,
        );

        // Skip if this is a duplicate packet
        if is_duplicate_packet(&packet_signature) {
            return;
        }

        // Register this packet to prevent future duplicates
        register_packet(packet_signature, adapter_name.to_string());

        let packet_size = packet.header.len as u64;
        let is_outgoing = is_outgoing_traffic(&src_ip);

        // Update hosts
        update_host_stats(hosts, &src_ip, &dst_ip, packet_size, is_outgoing, now).await;

        // Update services
        update_service_stats(services, &protocol, dst_port, packet_size).await;

        // Update overall stats
        update_overall_stats(stats, packet_size, is_outgoing).await;

        // Update traffic history
        update_traffic_history(traffic_history, stats, start_time, now).await;
    }
}

async fn update_host_stats(
    hosts: &Arc<DashMap<String, NetworkHost>>,
    src_ip: &IpAddr,
    dst_ip: &IpAddr,
    packet_size: u64,
    is_outgoing: bool,
    now: u64,
) {
    let target_ip = if is_outgoing { dst_ip } else { src_ip };
    let target_ip_str = target_ip.to_string();

    hosts.entry(target_ip_str.clone()).or_insert_with(|| {
        NetworkHost {
            ip: target_ip_str.clone(),
            hostname: None,
            domain: None,
            country: None,
            country_code: None,
            asn: None,
            incoming_bytes: 0,
            outgoing_bytes: 0,
            incoming_packets: 0,
            outgoing_packets: 0,
            first_seen: now,
            last_seen: now,
        }
    }).and_modify(|host| {
        if is_outgoing {
            host.outgoing_bytes += packet_size;
            host.outgoing_packets += 1;
        } else {
            host.incoming_bytes += packet_size;
            host.incoming_packets += 1;
        }
        host.last_seen = now;
    });
}

async fn update_service_stats(
    services: &Arc<DashMap<String, ServiceInfo>>,
    protocol: &str,
    port: u16,
    packet_size: u64,
) {
    if port == 0 {
        return; // Skip unknown ports
    }

    let service_key = format!("{}:{}", protocol, port);
    
    services.entry(service_key.clone()).or_insert_with(|| {
        ServiceInfo {
            protocol: protocol.to_string(),
            port,
            service_name: get_service_name(port, protocol),
            bytes: 0,
            packets: 0,
        }
    }).and_modify(|service| {
        service.bytes += packet_size;
        service.packets += 1;
    });
}

async fn update_overall_stats(
    stats: &Arc<RwLock<MonitoringStats>>,
    packet_size: u64,
    is_outgoing: bool,
) {
    let mut stats_guard = stats.write();
    if is_outgoing {
        stats_guard.total_outgoing_bytes += packet_size;
        stats_guard.total_outgoing_packets += 1;
    } else {
        stats_guard.total_incoming_bytes += packet_size;
        stats_guard.total_incoming_packets += 1;
    }
}

async fn update_traffic_history(
    traffic_history: &Arc<std::sync::Mutex<Vec<TrafficData>>>,
    stats: &Arc<RwLock<MonitoringStats>>,
    start_time: u64,
    now: u64,
) {
    let stats_guard = stats.read();
    let current_data = TrafficData {
        timestamp: now,
        incoming_bytes: stats_guard.total_incoming_bytes,
        outgoing_bytes: stats_guard.total_outgoing_bytes,
        incoming_packets: stats_guard.total_incoming_packets,
        outgoing_packets: stats_guard.total_outgoing_packets,
    };
    drop(stats_guard);

    if let Ok(mut history) = traffic_history.lock() {
        history.push(current_data);
        // Keep only recent history (last 3600 entries)
        if history.len() > 3600 {
            history.remove(0);
        }
    }
}

fn is_outgoing_traffic(ip: &IpAddr) -> bool {
    match ip {
        IpAddr::V4(ipv4) => {
            let octets = ipv4.octets();
            // Local/private address ranges indicate outgoing traffic
            matches!(octets,
                [10, ..] |                    // 10.0.0.0/8
                [172, 16..=31, ..] |          // 172.16.0.0/12
                [192, 168, ..] |              // 192.168.0.0/16
                [169, 254, ..] |              // 169.254.0.0/16 (link-local)
                [127, ..]                     // 127.0.0.0/8 (loopback)
            )
        }
        IpAddr::V6(ipv6) => {
            let segments = ipv6.segments();
            // IPv6 local ranges
            segments[0] == 0xfe80 ||          // Link-local
            segments[0] == 0xfc00 ||          // Unique local
            segments[0] == 0xfd00 ||          // Unique local
            *ipv6 == std::net::Ipv6Addr::LOCALHOST     // Loopback
        }
    }
}

fn get_service_name(port: u16, protocol: &str) -> Option<String> {
    match (port, protocol) {
        (80, "TCP") => Some("HTTP".to_string()),
        (443, "TCP") => Some("HTTPS".to_string()),
        (22, "TCP") => Some("SSH".to_string()),
        (21, "TCP") => Some("FTP".to_string()),
        (25, "TCP") => Some("SMTP".to_string()),
        (53, _) => Some("DNS".to_string()),
        (110, "TCP") => Some("POP3".to_string()),
        (143, "TCP") => Some("IMAP".to_string()),
        (993, "TCP") => Some("IMAPS".to_string()),
        (995, "TCP") => Some("POP3S".to_string()),
        (3389, "TCP") => Some("RDP".to_string()),
        (5432, "TCP") => Some("PostgreSQL".to_string()),
        (3306, "TCP") => Some("MySQL".to_string()),
        (1433, "TCP") => Some("MSSQL".to_string()),
        (6379, "TCP") => Some("Redis".to_string()),
        (27017, "TCP") => Some("MongoDB".to_string()),
        _ => None,
    }
}
