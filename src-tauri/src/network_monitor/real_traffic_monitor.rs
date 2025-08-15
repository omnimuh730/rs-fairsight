use pcap::{Capture, Device, Active, Address};
use std::net::IpAddr;
use etherparse::LaxPacketHeaders;
use dns_lookup::lookup_addr;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::time::{interval, Duration};

pub struct RealTrafficMonitor {
    adapter_name: String,
    capture: Option<Capture<Active>>,
    is_running: Arc<Mutex<bool>>,
}

impl RealTrafficMonitor {
    pub fn new(adapter_name: String) -> Result<Self, String> {
        Ok(Self {
            adapter_name,
            capture: None,
            is_running: Arc::new(Mutex::new(false)),
        })
    }

    pub fn start_capture(&mut self) -> Result<(), String> {
        // Find the specific device
        let device = Device::list()
            .map_err(|e| format!("Failed to list devices: {}", e))?
            .into_iter()
            .find(|d| d.name == self.adapter_name)
            .ok_or(format!("Device {} not found", self.adapter_name))?;

        // Create capture from device (similar to sniffnet)
        let inactive = Capture::from_device(device)
            .map_err(|e| format!("Failed to create capture: {}", e))?;
            
        let cap = inactive
            .promisc(true)                // Enable promiscuous mode
            .buffer_size(2_000_000)      // 2MB buffer like sniffnet
            .snaplen(200)                // Limit packet slice
            .immediate_mode(true)        // Parse packets ASAP
            .timeout(150)                // Timeout for UI updates
            .open()
            .map_err(|e| format!("Failed to open capture: {}", e))?;

        self.capture = Some(cap);
        *self.is_running.lock().unwrap() = true;
        Ok(())
    }

    pub async fn monitor_packets(&mut self) -> Result<(), String> {
        if self.capture.is_none() {
            return Err("Capture not initialized".to_string());
        }

        while *self.is_running.lock().unwrap() {
            let mut cap = self.capture.take().unwrap();
            let result = cap.next_packet();
            self.capture = Some(cap);

            match result {
                Ok(packet) => {
                    // Parse packet headers (similar to sniffnet's approach)
                    if let Ok(headers) = LaxPacketHeaders::from_ethernet(&packet.data) {
                        self.process_packet(headers, packet.data.len()).await;
                    }
                }
                Err(pcap::Error::TimeoutExpired) => {
                    // Continue on timeout (normal for UI updates)
                    continue;
                }
                Err(e) => {
                    eprintln!("Packet capture error: {}", e);
                    break;
                }
            }
        }
        Ok(())
    }

    async fn process_packet(&self, headers: LaxPacketHeaders<'_>, packet_size: usize) {
        // Extract IP addresses from headers
        let (source_ip, dest_ip) = match (&headers.net, &headers.transport) {
            (Some(etherparse::NetHeaders::Ipv4(ipv4, _)), _) => {
                (IpAddr::V4(ipv4.source.into()), IpAddr::V4(ipv4.destination.into()))
            }
            (Some(etherparse::NetHeaders::Ipv6(ipv6, _)), _) => {
                (IpAddr::V6(ipv6.source.into()), IpAddr::V6(ipv6.destination.into()))
            }
            _ => return, // Skip non-IP packets
        };

        // Extract ports if available
        let (source_port, dest_port) = match headers.transport {
            Some(etherparse::TransportHeader::Tcp(tcp)) => {
                (Some(tcp.source_port), Some(tcp.destination_port))
            }
            Some(etherparse::TransportHeader::Udp(udp)) => {
                (Some(udp.source_port), Some(udp.destination_port))
            }
            _ => (None, None),
        };

        // Perform reverse DNS lookup for external addresses
        self.resolve_and_store_host(source_ip, source_port, packet_size).await;
        self.resolve_and_store_host(dest_ip, dest_port, packet_size).await;
    }

    async fn resolve_and_store_host(&self, ip: IpAddr, port: Option<u16>, bytes: usize) {
        // Skip local/loopback addresses
        if ip.is_loopback() || match ip {
            IpAddr::V4(ipv4) => ipv4.is_private(),
            IpAddr::V6(_) => false,
        } {
            return;
        }

        // Perform reverse DNS lookup (like sniffnet)
        let hostname = match lookup_addr(&ip) {
            Ok(name) if !name.is_empty() => Some(name),
            _ => None,
        };

        // TODO: Store in your data structures similar to sniffnet's approach
        println!("Host: {} -> {:?}, Port: {:?}, Bytes: {}", 
            ip, hostname, port, bytes);
    }

    pub fn stop_capture(&self) {
        *self.is_running.lock().unwrap() = false;
    }
}

// Helper function to extract domain from hostname (like sniffnet)
pub fn get_domain_from_hostname(hostname: String) -> String {
    // Simple domain extraction - you might want to use a proper domain parser
    let parts: Vec<&str> = hostname.split('.').collect();
    if parts.len() >= 2 {
        format!("{}.{}", parts[parts.len()-2], parts[parts.len()-1])
    } else {
        hostname
    }
}
