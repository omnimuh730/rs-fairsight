use pcap::{Capture, Device};
use etherparse::LaxPacketHeaders;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::time::{SystemTime, UNIX_EPOCH};
use dashmap::DashMap;
use std::sync::{Arc, Mutex};
use rand::Rng;

use super::types::{PacketInfo, EthHeader, IpHeader, TcpHeader, UdpHeader, IcmpHeader, PacketHeaders};

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
        .snaplen(65535) // Capture full packets
        .buffer_size(8_000_000) // 8MB buffer
        .timeout(100) // 100ms timeout
        .immediate_mode(true)
        .open()
        .map_err(|e| format!("Failed to open capture: {}", e))?;

    Ok(capture)
}

pub fn parse_packet(packet: pcap::Packet, _adapter_name: &str) -> Result<PacketHeaders, String> {
    let packet_headers = LaxPacketHeaders::from_ethernet(&packet.data)
        .map_err(|e| format!("Failed to parse packet headers: {}", e))?;

    let eth_header = packet_headers.ethernet.map(|eth| EthHeader {
        source: eth.source,
        destination: eth.destination,
        ether_type: eth.ether_type,
    });

    let ip_header = match &packet_headers.net {
        Some(etherparse::NetHeaders::Ipv4(ipv4, _)) => Some(IpHeader::IPv4 {
            source: Ipv4Addr::from(ipv4.source),
            destination: Ipv4Addr::from(ipv4.destination),
            protocol: ipv4.protocol,
            ttl: ipv4.time_to_live,
            ihl: ipv4.ihl(),
            dscp: ipv4.dscp,
            ecn: ipv4.ecn,
            packet_id: ipv4.identification,
            dont_fragment: ipv4.dont_fragment,
            more_fragments: ipv4.more_fragments,
            fragment_offset: ipv4.fragment_offset,
            header_checksum: ipv4.header_checksum,
        }),
        Some(etherparse::NetHeaders::Ipv6(ipv6, _)) => Some(IpHeader::IPv6 {
            source: Ipv6Addr::from(ipv6.source),
            destination: Ipv6Addr::from(ipv6.destination),
            next_header: ipv6.next_header,
            hop_limit: ipv6.hop_limit,
            traffic_class: ipv6.traffic_class,
            flow_label: ipv6.flow_label,
            payload_length: ipv6.payload_length,
        }),
        None => None,
    };

    let transport_header = match &packet_headers.transport {
        Some(etherparse::TransportHeader::Tcp(tcp)) => Some(PacketHeaders::TransportHeader::Tcp(TcpHeader {
            source_port: tcp.source_port,
            destination_port: tcp.destination_port,
            sequence_number: tcp.sequence_number,
            acknowledgment_number: tcp.acknowledgment_number,
            data_offset: tcp.data_offset(),
            fin: tcp.fin,
            syn: tcp.syn,
            rst: tcp.rst,
            psh: tcp.psh,
            ack: tcp.ack,
            urg: tcp.urg,
            ece: tcp.ece,
            cwr: tcp.cwr,
            ns: tcp.ns,
            window_size: tcp.window_size,
            checksum: tcp.checksum,
            urgent_pointer: tcp.urgent_pointer,
        })),
        Some(etherparse::TransportHeader::Udp(udp)) => Some(PacketHeaders::TransportHeader::Udp(UdpHeader {
            source_port: udp.source_port,
            destination_port: udp.destination_port,
            length: udp.length,
            checksum: udp.checksum,
        })),
        Some(etherparse::TransportHeader::Icmpv4(icmpv4)) => Some(PacketHeaders::TransportHeader::Icmpv4(IcmpHeader {
            icmp_type: icmpv4.icmp_type(),
            icmp_code: icmpv4.icmp_code(),
            checksum: icmpv4.checksum(),
        })),
        Some(etherparse::TransportHeader::Icmpv6(icmpv6)) => Some(PacketHeaders::TransportHeader::Icmpv6(IcmpHeader {
            icmp_type: icmpv6.icmp_type(),
            icmp_code: icmpv6.icmp_code(),
            checksum: icmpv6.checksum(),
        })),
        None => None,
    };

    let app_layer_size = packet_headers.payload.len() as u64;
    let packet_size = packet.header.len as u64; // Use actual packet size from header
    let timestamp = packet.header.ts.tv_sec as u64; // Use seconds for timestamp

    // Deduplication based on a more robust signature including relevant header fields
    let packet_signature = create_packet_signature(&packet_headers, timestamp, &packet.data);

    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs();
    if is_duplicate_packet(&packet_signature, now) {
        // This is a duplicate packet - skip it for processing downstream
        // However, we return the parsed headers anyway, so subsequent steps
        // can decide how to handle duplicates if needed (e.g., for UI counts)
    } else {
        // Store signature with expiry time (e.g., 5 seconds from now)
        register_packet_signature(packet_signature.clone(), now + 5);
    }

    Ok(PacketHeaders {
        timestamp: timestamp,
        eth: eth_header,
        ip: ip_header,
        transport: transport_header,
        payload_size: app_layer_size,
        packet_size: packet_size, // Store the actual captured packet size
    })
}

// --- Deduplication Logic ---

// Note: This is a basic deduplication based on a signature and expiry time.
// For more robust deduplication, consider a sliding window or other algorithms.

// Function to create a packet signature (example - needs refinement based on required granularity)
fn create_packet_signature(headers: &LaxPacketHeaders, timestamp_secs: u64, packet_data: &[u8]) -> String {
    // Example signature: src_ip-dst_ip-protocol-src_port-dst_port-timestamp_sec
    let packet_signature = format!(
        "{}->{}:{}:{}:{}",
        if let Some(ref net) = headers.net {
            match net {
                etherparse::NetHeaders::Ipv4(ipv4, _) => Ipv4Addr::from(ipv4.source).to_string(),
                etherparse::NetHeaders::Ipv6(ipv6, _) => Ipv6Addr::from(ipv6.source).to_string(),
            }
        } else { "".to_string() },
        if let Some(ref net) = headers.net {
            match net {
                etherparse::NetHeaders::Ipv4(ipv4, _) => Ipv4Addr::from(ipv4.destination).to_string(),
                etherparse::NetHeaders::Ipv6(ipv6, _) => Ipv6Addr::from(ipv6.destination).to_string(),
            }
        } else { "".to_string() },
        if let Some(ref transport) = headers.transport {
            match transport {
                etherparse::TransportHeader::Tcp(_) => "TCP",
                etherparse::TransportHeader::Udp(_) => "UDP",
                etherparse::TransportHeader::Icmpv4(_) => "ICMPv4",
                etherparse::TransportHeader::Icmpv6(_) => "ICMPv6",
            }.to_string()
        } else { "Other".to_string() },
        if let Some(ref transport) = headers.transport {
            match transport {
                etherparse::TransportHeader::Tcp(tcp) => tcp.source_port.to_string(),
                etherparse::TransportHeader::Udp(udp) => udp.source_port.to_string(),
                _ => "0".to_string(),
            }
        } else { "0".to_string() },
        if let Some(ref transport) = headers.transport {
            match transport {
                etherparse::TransportHeader::Tcp(tcp) => tcp.destination_port.to_string(),
                etherparse::TransportHeader::Udp(udp) => udp.destination_port.to_string(),
                _ => "0".to_string(),
            }
        } else { "0".to_string() },
        // Using seconds for timestamp in signature is less granular but suitable for cleanup
        // Consider including a hash of the payload for more accurate deduplication
        // std::collections::hash_map::DefaultHasher::new().write(&packet_data).finish().to_string()
        // timestamp_secs
    );

    packet_signature
}

// Function to check if a packet is a duplicate
fn is_duplicate_packet(signature: &str, now_secs: u64) -> bool {
    if let Some(&expiry_time) = PACKET_SIGNATURES.get(signature) {
        // Check if the signature is still considered a duplicate
        expiry_time > now_secs
    } else {
        false
    }
}

// Function to register a packet signature
fn register_packet_signature(signature: String, expiry_time_secs: u64) {
    PACKET_SIGNATURES.insert(signature, expiry_time_secs);
}


// Add other helper functions for parsing specific protocols (e.g., HTTP, DNS) here or in a new module

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
