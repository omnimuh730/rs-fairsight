// Unified Network Monitoring Engine
// This replaces the complex traffic_monitor.rs with a cleaner architecture

use std::collections::HashMap;
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use dashmap::DashMap;
use pcap::{Device, Capture};
use etherparse::LaxPacketHeaders;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use tokio::sync::RwLock;
use tokio::task;
use serde::{Deserialize, Serialize};
use rand::Rng;

use crate::state_manager::{get_state_manager, StateEvent};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PacketInfo {
    pub source_ip: IpAddr,
    pub dest_ip: IpAddr,
    pub source_port: Option<u16>,
    pub dest_port: Option<u16>,
    pub protocol: String,
    pub size_bytes: u64,
    pub timestamp: u64,
    pub is_outgoing: bool,
}

#[derive(Debug, Clone)]
pub struct AdapterMonitor {
    pub name: String,
    pub display_name: String,
    pub is_active: Arc<AtomicBool>,
    pub packets_processed: Arc<std::sync::atomic::AtomicU64>,
    pub bytes_processed: Arc<std::sync::atomic::AtomicU64>,
}

// Global packet deduplication using microsecond-precision signatures
lazy_static::lazy_static! {
    static ref PACKET_SIGNATURES: Arc<DashMap<String, u64>> = Arc::new(DashMap::new());
}

pub struct NetworkEngine {
    adapters: Arc<RwLock<HashMap<String, AdapterMonitor>>>,
    is_running: Arc<AtomicBool>,
    discovery_task: Option<task::JoinHandle<()>>,
    monitoring_tasks: Arc<RwLock<HashMap<String, task::JoinHandle<()>>>>,
}

impl NetworkEngine {
    pub fn new() -> Self {
        Self {
            adapters: Arc::new(RwLock::new(HashMap::new())),
            is_running: Arc::new(AtomicBool::new(false)),
            discovery_task: None,
            monitoring_tasks: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn start(&mut self) -> Result<(), String> {
        if self.is_running.load(Ordering::Relaxed) {
            return Ok(());
        }

        self.is_running.store(true, Ordering::Relaxed);
        println!("🚀 Starting Network Engine...");

        // Start adapter discovery
        self.start_adapter_discovery().await?;
        
        // Start initial adapter scan and monitoring
        self.scan_and_start_monitoring().await?;
        
        println!("✅ Network Engine started successfully");
        Ok(())
    }

    pub async fn stop(&mut self) -> Result<(), String> {
        if !self.is_running.load(Ordering::Relaxed) {
            return Ok(());
        }

        println!("🛑 Stopping Network Engine...");
        self.is_running.store(false, Ordering::Relaxed);

        // Stop discovery task
        if let Some(task) = self.discovery_task.take() {
            task.abort();
        }

        // Stop all monitoring tasks
        {
            let mut tasks = self.monitoring_tasks.write().await;
            for (adapter_name, task) in tasks.drain() {
                println!("🛑 Stopping monitoring for: {}", adapter_name);
                task.abort();
            }
        }

        // Mark all adapters as inactive
        {
            let adapters = self.adapters.read().await;
            for (adapter_name, monitor) in adapters.iter() {
                monitor.is_active.store(false, Ordering::Relaxed);
                let _ = get_state_manager().mark_adapter_active(adapter_name, false);
                let _ = get_state_manager().stop_monitoring(adapter_name);
            }
        }

        println!("✅ Network Engine stopped");
        Ok(())
    }

    async fn start_adapter_discovery(&mut self) -> Result<(), String> {
        let adapters = self.adapters.clone();
        let monitoring_tasks = self.monitoring_tasks.clone();
        let is_running = self.is_running.clone();

        self.discovery_task = Some(task::spawn(async move {
            let mut discovery_interval = tokio::time::interval(Duration::from_secs(5));
            
            while is_running.load(Ordering::Relaxed) {
                discovery_interval.tick().await;
                
                if let Err(e) = Self::discover_adapters(&adapters, &monitoring_tasks, &is_running).await {
                    eprintln!("⚠️  Adapter discovery error: {}", e);
                }
            }
        }));

        Ok(())
    }

    async fn discover_adapters(
        adapters: &Arc<RwLock<HashMap<String, AdapterMonitor>>>,
        monitoring_tasks: &Arc<RwLock<HashMap<String, task::JoinHandle<()>>>>,
        is_running: &Arc<AtomicBool>,
    ) -> Result<(), String> {
        let devices = Device::list().map_err(|e| format!("Failed to list devices: {}", e))?;
        let mut current_adapters = adapters.write().await;
        let mut current_tasks = monitoring_tasks.write().await;

        // Track which adapters we've seen
        let mut seen_adapters = std::collections::HashSet::new();

        for device in devices {
            if device.name.is_empty() || device.name.contains("bluetooth") || device.name.contains("any") {
                continue;
            }

            seen_adapters.insert(device.name.clone());

            if !current_adapters.contains_key(&device.name) {
                // New adapter discovered
                let display_name = device.desc.unwrap_or_else(|| device.name.clone());
                let monitor = AdapterMonitor {
                    name: device.name.clone(),
                    display_name: display_name.clone(),
                    is_active: Arc::new(AtomicBool::new(true)),
                    packets_processed: Arc::new(std::sync::atomic::AtomicU64::new(0)),
                    bytes_processed: Arc::new(std::sync::atomic::AtomicU64::new(0)),
                };

                println!("🔌 Discovered new adapter: {} ({})", device.name, display_name);
                
                // Update state manager
                let _ = get_state_manager().mark_adapter_active(&device.name, true);
                
                // Start monitoring for this adapter
                let monitoring_task = Self::start_adapter_monitoring(
                    device.name.clone(),
                    monitor.is_active.clone(),
                    monitor.packets_processed.clone(),
                    monitor.bytes_processed.clone(),
                    is_running.clone(),
                ).await;

                if let Ok(task) = monitoring_task {
                    current_tasks.insert(device.name.clone(), task);
                    let _ = get_state_manager().start_monitoring(&device.name);
                    println!("✅ Started monitoring: {}", device.name);
                }

                current_adapters.insert(device.name.clone(), monitor);
            }
        }

        // Remove adapters that are no longer present
        let mut to_remove = Vec::new();
        for adapter_name in current_adapters.keys() {
            if !seen_adapters.contains(adapter_name) {
                to_remove.push(adapter_name.clone());
            }
        }

        for adapter_name in to_remove {
            println!("🔌 Adapter disconnected: {}", adapter_name);
            
            // Stop monitoring task
            if let Some(task) = current_tasks.remove(&adapter_name) {
                task.abort();
            }
            
            // Update state
            if let Some(monitor) = current_adapters.remove(&adapter_name) {
                monitor.is_active.store(false, Ordering::Relaxed);
                let _ = get_state_manager().mark_adapter_active(&adapter_name, false);
                let _ = get_state_manager().stop_monitoring(&adapter_name);
            }
        }

        Ok(())
    }

    async fn start_adapter_monitoring(
        adapter_name: String,
        is_active: Arc<AtomicBool>,
        packets_processed: Arc<std::sync::atomic::AtomicU64>,
        bytes_processed: Arc<std::sync::atomic::AtomicU64>,
        is_running: Arc<AtomicBool>,
    ) -> Result<task::JoinHandle<()>, String> {
        // Open packet capture
        let capture = Self::open_packet_capture(&adapter_name)?;

        let task = task::spawn(async move {
            let mut cap = capture;
            let mut stats_update_interval = tokio::time::interval(Duration::from_secs(1));
            let mut cleanup_counter = 0u64;

            println!("📡 Monitoring packets on: {}", adapter_name);

            loop {
                if !is_running.load(Ordering::Relaxed) || !is_active.load(Ordering::Relaxed) {
                    break;
                }

                tokio::select! {
                    _ = stats_update_interval.tick() => {
                        // Periodic stats update and cleanup
                        cleanup_counter += 1;
                        if cleanup_counter % 30 == 0 { // Every 30 seconds
                            Self::cleanup_packet_signatures();
                        }
                    }
                    
                    packet_result = tokio::task::spawn_blocking({
                        let adapter_name = adapter_name.clone();
                        move || -> Result<Option<PacketInfo>, String> {
                            match cap.next_packet() {
                                Ok(packet) => {
                                    match Self::parse_packet(packet, &adapter_name) {
                                        Ok(Some(packet_info)) => Ok(Some(packet_info)),
                                        Ok(None) => Ok(None), // Duplicate or filtered
                                        Err(e) => {
                                            // Only log parsing errors occasionally
                                            if rand::random::<u8>() % 100 == 0 {
                                                eprintln!("Packet parse error: {}", e);
                                            }
                                            Ok(None)
                                        }
                                    }
                                }
                                Err(pcap::Error::TimeoutExpired) => Ok(None),
                                Err(e) => Err(format!("Capture error: {}", e)),
                            }
                        }
                    }) => {
                        match packet_result {
                            Ok(Ok(Some(packet_info))) => {
                                // Process valid packet
                                packets_processed.fetch_add(1, Ordering::Relaxed);
                                bytes_processed.fetch_add(packet_info.size_bytes, Ordering::Relaxed);
                                
                                // Update state manager
                                let bytes_in = if packet_info.is_outgoing { 0 } else { packet_info.size_bytes };
                                let bytes_out = if packet_info.is_outgoing { packet_info.size_bytes } else { 0 };
                                let packets_in = if packet_info.is_outgoing { 0 } else { 1 };
                                let packets_out = if packet_info.is_outgoing { 1 } else { 0 };
                                
                                let _ = get_state_manager().update_traffic(
                                    &adapter_name,
                                    bytes_in,
                                    bytes_out,
                                    packets_in,
                                    packets_out,
                                );
                            }
                            Ok(Ok(None)) => {
                                // Normal case - timeout or duplicate
                                tokio::task::yield_now().await;
                            }
                            Ok(Err(e)) => {
                                eprintln!("⚠️  Packet capture error on {}: {}", adapter_name, e);
                                tokio::time::sleep(Duration::from_millis(100)).await;
                            }
                            Err(e) => {
                                eprintln!("⚠️  Task error on {}: {}", adapter_name, e);
                                break;
                            }
                        }
                    }
                }
            }

            println!("📡 Stopped monitoring: {}", adapter_name);
        });

        Ok(task)
    }

    fn open_packet_capture(adapter_name: &str) -> Result<Capture<pcap::Active>, String> {
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

    fn parse_packet(packet: pcap::Packet, adapter_name: &str) -> Result<Option<PacketInfo>, String> {
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

        let is_outgoing = Self::is_outgoing_traffic(&source_ip);
        
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
                *ipv6 == Ipv6Addr::LOCALHOST     // Loopback
            }
        }
    }

    fn cleanup_packet_signatures() {
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

    async fn scan_and_start_monitoring(&mut self) -> Result<(), String> {
        // Trigger initial discovery
        let adapters = self.adapters.clone();
        let monitoring_tasks = self.monitoring_tasks.clone();
        let is_running = self.is_running.clone();

        Self::discover_adapters(&adapters, &monitoring_tasks, &is_running).await?;
        Ok(())
    }

    pub async fn get_adapter_stats(&self) -> HashMap<String, (u64, u64, u64, u64)> {
        let adapters = self.adapters.read().await;
        let mut stats = HashMap::new();

        for (name, monitor) in adapters.iter() {
            let packets = monitor.packets_processed.load(Ordering::Relaxed);
            let bytes = monitor.bytes_processed.load(Ordering::Relaxed);
            
            // Get state manager data for in/out breakdown
            let state = get_state_manager().get_state_snapshot();
            if let Some(adapter_metrics) = state.adapters.get(name) {
                stats.insert(
                    name.clone(),
                    (
                        adapter_metrics.total_bytes_in,
                        adapter_metrics.total_bytes_out,
                        adapter_metrics.total_packets_in,
                        adapter_metrics.total_packets_out,
                    ),
                );
            } else {
                stats.insert(name.clone(), (0, 0, 0, 0));
            }
        }

        stats
    }

    pub async fn get_active_adapters(&self) -> Vec<String> {
        let adapters = self.adapters.read().await;
        adapters
            .iter()
            .filter(|(_, monitor)| monitor.is_active.load(Ordering::Relaxed))
            .map(|(name, _)| name.clone())
            .collect()
    }
}

// Global network engine instance
use std::sync::OnceLock;
static NETWORK_ENGINE: OnceLock<tokio::sync::Mutex<NetworkEngine>> = OnceLock::new();

pub async fn get_network_engine() -> &'static tokio::sync::Mutex<NetworkEngine> {
    NETWORK_ENGINE.get_or_init(|| tokio::sync::Mutex::new(NetworkEngine::new()))
}

pub async fn start_network_engine() -> Result<(), String> {
    let engine = get_network_engine().await;
    let mut engine_guard = engine.lock().await;
    engine_guard.start().await
}

pub async fn stop_network_engine() -> Result<(), String> {
    let engine = get_network_engine().await;
    let mut engine_guard = engine.lock().await;
    engine_guard.stop().await
}
