use std::sync::{Arc, Mutex};
use chrono::{Local, Datelike};
use std::time::Duration;
use dashmap::DashMap;
use parking_lot::RwLock;

use crate::network_monitor::persistent_state::get_persistent_state_manager;
use super::types::{MonitoringConfig, MonitoringStats, TrafficData, NetworkHost, ServiceInfo};
use super::packet_processing::{create_packet_capture, process_real_packet};
use super::session_manager::{save_periodic_session, save_final_session};

pub struct TrafficMonitor {
    pub config: Arc<RwLock<MonitoringConfig>>,
    pub stats: Arc<RwLock<MonitoringStats>>,
    pub hosts: Arc<DashMap<String, NetworkHost>>,
    pub services: Arc<DashMap<String, ServiceInfo>>,
    pub traffic_history: Arc<Mutex<Vec<TrafficData>>>,
    pub is_running: Arc<RwLock<bool>>,
    pub session_start_time: Arc<RwLock<Option<u64>>>,
    last_known_date: Arc<RwLock<Option<u32>>>,
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
            last_known_date: Arc::new(RwLock::new(Some(Local::now().ordinal()))),
        }
    }

    pub fn start_monitoring(&self) -> Result<(), String> {
        {
            let mut is_running = self.is_running.write();
            if *is_running {
                return Err("Monitoring is already running".to_string());
            }
            *is_running = true;
        }

        let start_time = Local::now().timestamp() as u64;
        *self.session_start_time.write() = Some(start_time);

        let config = self.config.read().clone();
        let adapter_name = config.adapter_name.clone();

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

        let hosts = Arc::clone(&self.hosts);
        let services = Arc::clone(&self.services);
        let traffic_history = Arc::clone(&self.traffic_history);
        let is_running_clone = Arc::clone(&self.is_running);
        let stats = Arc::clone(&self.stats);
        let last_known_date = Arc::clone(&self.last_known_date);
        let monitor_clone = Arc::new(self.clone());

        tokio::spawn(async move {
            monitor_clone.monitor_traffic(
                adapter_name,
                hosts,
                services,
                traffic_history,
                is_running_clone,
                stats,
                last_known_date,
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

        if let Some(start_time) = *self.session_start_time.read() {
            let current_stats = self.stats.read();
            save_final_session(&adapter_name, start_time, &current_stats);
        }

        println!("🛑 Stopped monitoring '{}' - final session saved", adapter_name);

        *self.session_start_time.write() = None;
    }

    pub fn get_stats(&self) -> MonitoringStats {
        let mut stats = self.stats.write();

        stats.network_hosts = self.hosts.iter()
            .map(|entry| entry.value().clone())
            .collect::<Vec<_>>();
        
        stats.services = self.services.iter()
            .map(|entry| entry.value().clone())
            .collect::<Vec<_>>();

        stats.network_hosts.sort_by(|a, b| {
            let total_a = a.incoming_bytes + a.outgoing_bytes;
            let total_b = b.incoming_bytes + b.outgoing_bytes;
            total_b.cmp(&total_a)
        });

        stats.services.sort_by(|a, b| b.bytes.cmp(&a.bytes));

        let config = self.config.read();
        stats.network_hosts.truncate(config.max_hosts);
        stats.services.truncate(config.max_services);

        stats.clone()
    }

    pub fn reset_daily_stats(&self) {
        let mut stats = self.stats.write();
        stats.total_incoming_bytes = 0;
        stats.total_outgoing_bytes = 0;
        stats.total_incoming_packets = 0;
        stats.total_outgoing_packets = 0;
        
        self.hosts.clear();
        self.services.clear();

        *self.last_known_date.write() = Some(Local::now().ordinal());
        println!("🌅 Day changed, traffic stats reset for {}", self.config.read().adapter_name);
    }

    async fn monitor_traffic(
        &self,
        adapter_name: String,
        hosts: Arc<DashMap<String, NetworkHost>>,
        services: Arc<DashMap<String, ServiceInfo>>,
        traffic_history: Arc<Mutex<Vec<TrafficData>>>,
        is_running: Arc<RwLock<bool>>,
        stats: Arc<RwLock<MonitoringStats>>,
        last_known_date: Arc<RwLock<Option<u32>>>,
    ) {
        println!("🚀 Starting comprehensive traffic monitoring for adapter: {} (with packet deduplication)", adapter_name);

        let mut capture_opt = create_packet_capture(&adapter_name);

        let mut save_interval = tokio::time::interval(Duration::from_secs(8));
        let start_time = Local::now().timestamp() as u64;
        let mut last_save_time = start_time;
        let mut last_save_incoming_bytes = 0u64;
        let mut last_save_outgoing_bytes = 0u64;
        let mut last_save_incoming_packets = 0u64;
        let mut last_save_outgoing_packets = 0u64;

        loop {
            if !*is_running.read() {
                println!("🛑 Monitoring stopped for {}", adapter_name);
                break;
            }

            if let Some(mut capture) = capture_opt.take() {
                println!("✅ Real packet capture active for {}", adapter_name);
                let mut packet_count = 0u64;
                let mut last_count_report = std::time::Instant::now();

                loop {
                    if !*is_running.read() {
                        println!("🛑 Monitoring loop for {} stopping", adapter_name);
                        capture_opt = Some(capture); 
                        break;
                    }

                    tokio::select! {
                        _ = save_interval.tick() => {
                            if last_count_report.elapsed() >= Duration::from_secs(30) {
                                println!("📊 Adapter {}: captured {} packets in last 30s", adapter_name, packet_count);
                                last_count_report = std::time::Instant::now();
                            }
                            
                            save_periodic_session(
                                &adapter_name, &stats, &start_time, &mut last_save_time,
                                &mut last_save_incoming_bytes, &mut last_save_outgoing_bytes,
                                &mut last_save_incoming_packets, &mut last_save_outgoing_packets,
                                &traffic_history, &last_known_date
                            ).await;
                        }
                        res = async { capture.next_packet() } => {
                            match res {
                                Ok(packet) => {
                                    packet_count += 1;
                                    process_real_packet(
                                        packet, &hosts, &services, &traffic_history, 
                                        &stats, start_time, &adapter_name, &last_known_date
                                    ).await;
                                }
                                Err(pcap::Error::TimeoutExpired) => {
                                    tokio::task::yield_now().await;
                                    continue;
                                }
                                Err(e) => {
                                    eprintln!("❌ Packet capture error on {}: {}. Will attempt to reconnect.", adapter_name, e);
                                    break;
                                }
                            }
                        }
                    }
                }
            } else {
                println!("⚠️  Real packet capture unavailable for adapter: {}", adapter_name);
                println!("🔄 Retrying packet capture every 5 seconds...");
                tokio::time::sleep(Duration::from_secs(5)).await;
                capture_opt = create_packet_capture(&adapter_name);
            }
        }
    }

    pub fn is_monitoring(&self) -> bool {
        *self.is_running.read()
    }
}

impl Clone for TrafficMonitor {
    fn clone(&self) -> Self {
        Self {
            config: Arc::clone(&self.config),
            stats: Arc::clone(&self.stats),
            hosts: Arc::clone(&self.hosts),
            services: Arc::clone(&self.services),
            traffic_history: Arc::clone(&self.traffic_history),
            is_running: Arc::clone(&self.is_running),
            session_start_time: Arc::clone(&self.session_start_time),
            last_known_date: Arc::clone(&self.last_known_date),
        }
    }
}

lazy_static::lazy_static! {
    pub static ref TRAFFIC_MONITORS: Arc<DashMap<String, Arc<TrafficMonitor>>> = Arc::new(DashMap::new());
}

pub fn get_or_create_monitor(adapter_name: &str) -> Arc<TrafficMonitor> {
    TRAFFIC_MONITORS.entry(adapter_name.to_string())
        .or_insert_with(|| Arc::new(TrafficMonitor::new(adapter_name.to_string())))
        .clone()
}

pub fn is_comprehensive_monitoring_running() -> bool {
    TRAFFIC_MONITORS.len() > 1 && TRAFFIC_MONITORS.iter().any(|entry| entry.value().is_monitoring())
}
