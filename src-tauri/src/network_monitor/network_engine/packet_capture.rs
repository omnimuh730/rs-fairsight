use pcap::{Capture, Device};
use etherparse::LaxPacketHeaders;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::time::{SystemTime, UNIX_EPOCH};
use dashmap::DashMap;
use std::sync::Arc;
use rand::Rng;

use super::types::PacketInfo;

// Global packet deduplication using microsecond-precision signatures
lazy_static::lazy_static! {
    static ref PACKET_SIGNATURES: Arc<DashMap<String, u64>> = Arc::new(DashMap::new());
}

pub fn open_packet_capture(adapter_name: &str) -> Result<Capture<pcap::Active>, String> {
    let devices = Device::list().map_err(|e| format!("Failed to list devices: {}", e))?;
    
    let device = devices
        .into_iter()
        .find(|d| d.name == adapter_name)
        .ok_or_else(|| format!("Adapter '{}' not found", adapter_name))?;

    let capture = Capture::from_device(device)
        .map_err(|e| format!("Failed to create capture: {}", e))?
        .promisc(true)
        .snaplen(200) // Capture headers only
        .buffer_size(8_000_000) // 8MB buffer
        .timeout(100) // 100ms timeout
        .immediate_mode(true)
        .open()
        .map_err(|e| format!("Failed to open capture: {}", e))?;

    Ok(capture)
}

pub fn parse_packet(packet: pcap::Packet, _adapter_name: &str) -> Result<Option<PacketInfo>, String> {
    let headers = LaxPacketHeaders::from_ethernet(&packet.data)
        .map_err(|e| format!("Failed to parse packet headers: {}", e))?;

    let (source_ip, dest_ip) = match &headers.net {
        Some(etherparse::NetHeaders::Ipv4(ipv4, _)) => {
            (
                IpAddr::V4(Ipv4Addr::from(ipv4.source)),
                IpAddr::V4(Ipv4Addr::from(ipv4.destination)),
            )
        }
        Some(etherparse::NetHeaders::Ipv6(ipv6, _)) => {
            (
                IpAddr::V6(Ipv6Addr::from(ipv6.source)),
                IpAddr::V6(Ipv6Addr::from(ipv6.destination)),
            )
        }
        _ => return Ok(None), // Skip non-IP packets
    };

    let (source_port, dest_port, protocol) = match &headers.transport {
        Some(etherparse::TransportHeader::Tcp(tcp)) => {
            (Some(tcp.source_port), Some(tcp.destination_port), "TCP".to_string())
        }
        Some(etherparse::TransportHeader::Udp(udp)) => {
            (Some(udp.source_port), Some(udp.destination_port), "UDP".to_string())
        }
        Some(etherparse::TransportHeader::Icmpv4(_)) => {
            (None, None, "ICMP".to_string())
        }
        Some(etherparse::TransportHeader::Icmpv6(_)) => {
            (None, None, "ICMPv6".to_string())
        }
        _ => (None, None, "Other".to_string()),
    };

    // Create packet signature for deduplication
    let packet_signature = format!(
        "{}->{}:{}:{}:{}",
        source_ip,
        dest_ip,
        source_port.unwrap_or(0),
        dest_port.unwrap_or(0),
        packet.header.ts.tv_usec
    );

    // Check for duplicates
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    if PACKET_SIGNATURES.contains_key(&packet_signature) {
        // This is a duplicate packet - skip it
        return Ok(None);
    }

    // Store signature with expiry time (5 seconds from now)
    PACKET_SIGNATURES.insert(packet_signature, now + 5);

    let is_outgoing = is_outgoing_traffic(&source_ip);
    
    Ok(Some(PacketInfo {
        source_ip,
        dest_ip,
        source_port,
        dest_port,
        protocol,
        size_bytes: packet.header.len as u64,
        timestamp: now,
        is_outgoing,
    }))
}

pub fn is_outgoing_traffic(ip: &IpAddr) -> bool {
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
            *ipv6 == Ipv6Addr::LOCALHOST     // Loopback
        }
    }
}

pub fn cleanup_packet_signatures() {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    let initial_count = PACKET_SIGNATURES.len();
    PACKET_SIGNATURES.retain(|_, &mut expiry_time| expiry_time > now);
    let removed_count = initial_count - PACKET_SIGNATURES.len();

    if removed_count > 0 {
        println!("🧹 Cleaned up {} expired packet signatures", removed_count);
    }
}
