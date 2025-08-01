use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use dashmap::DashMap;
use parking_lot::RwLock;

use crate::persistent_state::get_persistent_state_manager;
use super::types::{MonitoringConfig, MonitoringStats, TrafficData, NetworkHost, ServiceInfo};
use super::packet_processing::{create_packet_capture, process_real_packet};
use super::simulation::simulate_traffic_tick;
use super::session_manager::{save_periodic_session, save_final_session};

pub struct TrafficMonitor {
    pub config: Arc<RwLock<MonitoringConfig>>,
    pub stats: Arc<RwLock<MonitoringStats>>,
    pub hosts: Arc<DashMap<String, NetworkHost>>,
    pub services: Arc<DashMap<String, ServiceInfo>>,
    pub traffic_history: Arc<Mutex<Vec<TrafficData>>>,
    pub is_running: Arc<RwLock<bool>>,
    pub session_start_time: Arc<RwLock<Option<u64>>>,
}

impl TrafficMonitor {
    pub fn new(adapter_name: String) -> Self {
        // Load persistent state for this adapter
        let persistent_state = match get_persistent_state_manager().get_adapter_state(&adapter_name) {
            Ok(state) => state,
            Err(e) => {
                eprintln!("⚠️  Failed to load persistent state for '{}': {}. Starting fresh.", adapter_name, e);
                None
            }
        };

        let (total_incoming, total_outgoing, total_in_packets, total_out_packets) = 
            if let Some(ref state) = persistent_state {
                (
                    state.cumulative_incoming_bytes,
                    state.cumulative_outgoing_bytes,
                    state.cumulative_incoming_packets,
                    state.cumulative_outgoing_packets,
                )
            } else {
                (0, 0, 0, 0)
            };

        println!("🔄 Initializing TrafficMonitor for '{}' - Restored state: ↓{}KB ↑{}KB", 
            adapter_name, 
            total_incoming / 1024, 
            total_outgoing / 1024);

        Self {
            config: Arc::new(RwLock::new(MonitoringConfig {
                adapter_name,
                is_monitoring: false,
                capture_filter: None,
                max_hosts: 1000,
                max_services: 100,
            })),
            stats: Arc::new(RwLock::new(MonitoringStats {
                total_incoming_bytes: total_incoming,
                total_outgoing_bytes: total_outgoing,
                total_incoming_packets: total_in_packets,
                total_outgoing_packets: total_out_packets,
                monitoring_duration: 0,
                traffic_rate: Vec::new(),
                network_hosts: Vec::new(),
                services: Vec::new(),
            })),
            hosts: Arc::new(DashMap::new()),
            services: Arc::new(DashMap::new()),
            traffic_history: Arc::new(Mutex::new(Vec::new())),
            is_running: Arc::new(RwLock::new(false)),
            session_start_time: Arc::new(RwLock::new(None)),
        }
    }

    pub async fn start_monitoring(&self) -> Result<(), String> {
        let mut is_running = self.is_running.write();
        if *is_running {
            return Err("Monitoring is already running".to_string());
        }
        *is_running = true;

        // Record session start time
        let start_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        *self.session_start_time.write() = Some(start_time);

        let config = self.config.read().clone();
        let adapter_name = config.adapter_name.clone();

        // Update persistent state to mark as monitoring
        if let Err(e) = get_persistent_state_manager().update_adapter_state(&adapter_name, |state| {
            state.session_start_time = Some(start_time);
            state.was_monitoring_on_exit = true;
            if state.first_recorded_time.is_none() {
                state.first_recorded_time = Some(start_time);
            }
        }) {
            eprintln!("⚠️  Failed to update persistent state on start: {}", e);
        }

        println!("🚀 Starting network monitoring for '{}'", adapter_name);
        
        // Clone necessary data for the monitoring task
        let hosts = Arc::clone(&self.hosts);
        let services = Arc::clone(&self.services);
        let traffic_history = Arc::clone(&self.traffic_history);
        let is_running_clone = Arc::clone(&self.is_running);
        let stats = Arc::clone(&self.stats);

        // Start monitoring in a separate task
        tokio::spawn(async move {
            Self::monitor_traffic(
                adapter_name,
                hosts,
                services,
                traffic_history,
                is_running_clone,
                stats,
            ).await;
        });

        Ok(())
    }

    pub fn stop_monitoring(&self) {
        let mut is_running = self.is_running.write();
        if !*is_running {
            return; // Already stopped
        }
        *is_running = false;

        let config = self.config.read();
        let adapter_name = config.adapter_name.clone();

        // Save final session with remaining data before stopping
        if let Some(start_time) = *self.session_start_time.read() {
            let current_stats = self.stats.read();
            save_final_session(&adapter_name, start_time, &current_stats);
        }

        println!("🛑 Stopped monitoring '{}' - final session saved", adapter_name);

        // Reset session start time
        *self.session_start_time.write() = None;
    }

    pub fn get_stats(&self) -> MonitoringStats {
        let mut stats = self.stats.write();
        
        // Update hosts and services in stats
        stats.network_hosts = self.hosts.iter()
            .map(|entry| entry.value().clone())
            .collect::<Vec<_>>();
        
        stats.services = self.services.iter()
            .map(|entry| entry.value().clone())
            .collect::<Vec<_>>();

        // Sort hosts by total bytes (descending)
        stats.network_hosts.sort_by(|a, b| {
            let total_a = a.incoming_bytes + a.outgoing_bytes;
            let total_b = b.incoming_bytes + b.outgoing_bytes;
            total_b.cmp(&total_a)
        });

        // Sort services by bytes (descending)
        stats.services.sort_by(|a, b| b.bytes.cmp(&a.bytes));

        // Limit results
        let config = self.config.read();
        stats.network_hosts.truncate(config.max_hosts);
        stats.services.truncate(config.max_services);

        stats.clone()
    }

    async fn monitor_traffic(
        adapter_name: String,
        hosts: Arc<DashMap<String, NetworkHost>>,
        services: Arc<DashMap<String, ServiceInfo>>,
        traffic_history: Arc<Mutex<Vec<TrafficData>>>,
        is_running: Arc<RwLock<bool>>,
        stats: Arc<RwLock<MonitoringStats>>,
    ) {
        println!("🚀 Starting comprehensive traffic monitoring for adapter: {} (with packet deduplication)", adapter_name);

        // Try to create real packet capture
        let capture_opt = create_packet_capture(&adapter_name);

        let mut save_interval = tokio::time::interval(Duration::from_secs(8));
        let start_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let mut last_save_time = start_time;
        let mut last_save_incoming_bytes = 0u64;
        let mut last_save_outgoing_bytes = 0u64;
        let mut last_save_incoming_packets = 0u64;
        let mut last_save_outgoing_packets = 0u64;

        if let Some(mut capture) = capture_opt {
            // Real packet capture mode - continuous capture
            println!("Starting continuous packet capture mode for {}", adapter_name);
            
            loop {
                if !*is_running.read() {
                    break;
                }
                
                tokio::select! {
                    _ = save_interval.tick() => {
                        save_periodic_session(
                            &adapter_name, &stats, &start_time, &mut last_save_time,
                            &mut last_save_incoming_bytes, &mut last_save_outgoing_bytes,
                            &mut last_save_incoming_packets, &mut last_save_outgoing_packets,
                            &traffic_history
                        ).await;
                    }
                    _ = async {
                        // Continuous packet capture loop without delays
                        loop {
                            if !*is_running.read() {
                                break;
                            }
                            
                            match capture.next_packet() {
                                Ok(packet) => {
                                    process_real_packet(
                                        packet, &hosts, &services, &traffic_history, 
                                        &stats, start_time, &adapter_name
                                    ).await;
                                    // Continue immediately to next packet without delay
                                }
                                Err(pcap::Error::TimeoutExpired) => {
                                    // Normal timeout, yield control briefly then continue
                                    tokio::task::yield_now().await;
                                    continue;
                                }
                                Err(e) => {
                                    eprintln!("Packet capture error: {}. Switching to simulation mode.", e);
                                    return; // Exit packet capture loop
                                }
                            }
                        }
                    } => {
                        // Packet capture loop ended, break main loop
                        break;
                    }
                }
            }
        }

        // Fallback to simulation mode if real capture fails or stops
        println!("Using simulation mode for adapter: {}", adapter_name);
        let mut interval = tokio::time::interval(Duration::from_secs(1));
        
        while *is_running.read() {
            tokio::select! {
                _ = interval.tick() => {
                    simulate_traffic_tick(&hosts, &services, &traffic_history, &stats, start_time).await;
                }
                _ = save_interval.tick() => {
                    save_periodic_session(
                        &adapter_name, &stats, &start_time, &mut last_save_time,
                        &mut last_save_incoming_bytes, &mut last_save_outgoing_bytes,
                        &mut last_save_incoming_packets, &mut last_save_outgoing_packets,
                        &traffic_history
                    ).await;
                }
            }
        }

        println!("Stopped traffic monitoring for adapter: {}", adapter_name);
    }

    pub fn is_monitoring(&self) -> bool {
        *self.is_running.read()
    }
}

// Global traffic monitors for each adapter
lazy_static::lazy_static! {
    pub static ref TRAFFIC_MONITORS: Arc<DashMap<String, Arc<TrafficMonitor>>> = Arc::new(DashMap::new());
}

pub fn get_or_create_monitor(adapter_name: &str) -> Arc<TrafficMonitor> {
    TRAFFIC_MONITORS.entry(adapter_name.to_string())
        .or_insert_with(|| Arc::new(TrafficMonitor::new(adapter_name.to_string())))
        .clone()
}
