use pcap::{Capture, Device};
use etherparse::LaxPacketHeaders;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::Arc;
use dashmap::DashMap;
use parking_lot::RwLock;

use super::types::{NetworkHost, ServiceInfo, MonitoringStats, TrafficData};
use super::deduplication::{create_packet_signature, is_duplicate_packet, register_packet};
use super::service_analysis::process_service_from_packet;
use super::host_analysis::process_host_from_packet;

pub fn create_packet_capture(adapter_name: &str) -> Option<Capture<pcap::Active>> {
    crate::log_info!("packet_capture", "Attempting to create packet capture for adapter: '{}'", adapter_name);
    
    if let Ok(devices) = Device::list() {
        crate::log_info!("packet_capture", "Successfully listed {} devices for capture setup", devices.len());
        
        if let Some(device) = devices.into_iter().find(|d| d.name == adapter_name) {
            crate::log_info!("packet_capture", "Found target device '{}' - description: {:?}", adapter_name, device.desc);
            
            match Capture::from_device(device) {
                Ok(inactive) => {
                    crate::log_info!("packet_capture", "Created inactive capture for '{}', configuring settings...", adapter_name);
                    
                    match inactive
                        .promisc(true)
                        .buffer_size(8_000_000)  // Increase to 8MB buffer for better capture
                        .snaplen(200)            // Limit packet slice but count full packet size
                        .immediate_mode(true)    // Parse packets ASAP
                        .timeout(100)            // Shorter timeout for more responsive capture
                        .open() {
                        Ok(cap) => {
                            println!("✅ Successfully opened packet capture on {}", adapter_name);
                            crate::log_info!("packet_capture", "✅ Successfully opened packet capture on '{}'", adapter_name);
                            return Some(cap);
                        }
                        Err(e) => {
                            let err_str = e.to_string();
                            if err_str.contains("BIOCPROMISC") {
                                // Suppress error and log for BIOCPROMISC, but still retry silently
                            } else {
                                eprintln!("❌ Failed to open capture on {}: {}. Will retry later.", adapter_name, e);
                                crate::log_error!("packet_capture", "❌ Failed to open capture on '{}': {}", adapter_name, e);
                            }
                        }
                    }
                }
                Err(e) => {
                    crate::log_error!("packet_capture", "❌ Failed to create capture from device '{}': {}. Device may be unavailable or unsupported", adapter_name, e);
                    eprintln!("Failed to create capture from device {}: {}. Will retry later.", adapter_name, e);
                }
            }
        } else {
            crate::log_error!("packet_capture", "❌ Device '{}' not found in available devices list. Device may have been removed or renamed", adapter_name);
            eprintln!("Device {} not found. Will retry later.", adapter_name);
        }
    } else {
        crate::log_error!("packet_capture", "❌ Failed to list devices for capture setup. This may indicate insufficient privileges or pcap library issues");
        eprintln!("Failed to list devices. Will retry later.");
    }
    
    crate::log_warning!("packet_capture", "Packet capture creation failed for '{}' - returning None", adapter_name);
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

        // Report network activity to health monitor
        crate::health_monitor::report_network_activity();

        let packet_size = packet.header.len as u64;
        let is_outgoing = is_outgoing_traffic(&src_ip);

        // Update hosts with enhanced analysis (DNS, GeoIP, domains)
        let target_ip = if is_outgoing { &dst_ip } else { &src_ip };
        process_host_from_packet(target_ip, packet_size, is_outgoing, hosts, now).await;

        // Update services
        if dst_port != 0 {
            process_service_from_packet(&protocol, dst_port, packet_size, services);
        }

        // Update overall stats
        update_overall_stats(stats, packet_size, is_outgoing).await;

        // Update traffic history
        update_traffic_history(traffic_history, stats, start_time, now).await;
    }
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
    _start_time: u64,
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
