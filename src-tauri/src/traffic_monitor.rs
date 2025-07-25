use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use dashmap::DashMap;
use parking_lot::RwLock;
use rand::Rng;
use crate::network_storage::{NETWORK_STORAGE, NetworkSession};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrafficData {
    pub timestamp: u64,
    pub incoming_bytes: u64,
    pub outgoing_bytes: u64,
    pub incoming_packets: u64,
    pub outgoing_packets: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkHost {
    pub ip: String,
    pub hostname: Option<String>,
    pub country: Option<String>,
    pub country_code: Option<String>,
    pub incoming_bytes: u64,
    pub outgoing_bytes: u64,
    pub incoming_packets: u64,
    pub outgoing_packets: u64,
    pub first_seen: u64,
    pub last_seen: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceInfo {
    pub protocol: String,
    pub port: u16,
    pub service_name: Option<String>,
    pub bytes: u64,
    pub packets: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringStats {
    pub total_incoming_bytes: u64,
    pub total_outgoing_bytes: u64,
    pub total_incoming_packets: u64,
    pub total_outgoing_packets: u64,
    pub monitoring_duration: u64,
    pub traffic_rate: Vec<TrafficData>,
    pub network_hosts: Vec<NetworkHost>,
    pub services: Vec<ServiceInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringConfig {
    pub adapter_name: String,
    pub is_monitoring: bool,
    pub capture_filter: Option<String>,
    pub max_hosts: usize,
    pub max_services: usize,
}

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
        Self {
            config: Arc::new(RwLock::new(MonitoringConfig {
                adapter_name,
                is_monitoring: false,
                capture_filter: None,
                max_hosts: 1000,
                max_services: 100,
            })),
            stats: Arc::new(RwLock::new(MonitoringStats {
                total_incoming_bytes: 0,
                total_outgoing_bytes: 0,
                total_incoming_packets: 0,
                total_outgoing_packets: 0,
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
        
        // Clone necessary data for the monitoring task
        let adapter_name = config.adapter_name.clone();
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

        // Save session data to local storage
        if let Some(start_time) = *self.session_start_time.read() {
            let end_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
            let stats = self.get_stats();
            
            let session = NetworkSession {
                adapter_name: self.config.read().adapter_name.clone(),
                start_time,
                end_time: Some(end_time),
                total_incoming_bytes: stats.total_incoming_bytes,
                total_outgoing_bytes: stats.total_outgoing_bytes,
                total_incoming_packets: stats.total_incoming_packets,
                total_outgoing_packets: stats.total_outgoing_packets,
                duration: end_time - start_time,
                traffic_data: stats.traffic_rate.clone(),
                top_hosts: stats.network_hosts.iter().take(20).cloned().collect(), // Top 20 hosts
                top_services: stats.services.iter().take(20).cloned().collect(), // Top 20 services
            };

            if let Err(e) = NETWORK_STORAGE.save_session(&session) {
                eprintln!("Failed to save network session: {}", e);
            } else {
                println!("Network session saved successfully");
            }

            // Reset session start time
            *self.session_start_time.write() = None;
        }
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
        println!("Starting traffic monitoring for adapter: {}", adapter_name);

        // Simulate network monitoring (in a real implementation, this would use pcap)
        let mut interval = tokio::time::interval(Duration::from_secs(1));
        let mut save_interval = tokio::time::interval(Duration::from_secs(8)); // Save every 8 seconds
        let start_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let mut last_save_time = start_time;

        while *is_running.read() {
            tokio::select! {
                _ = interval.tick() => {
                    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
                    
                    // Simulate traffic data (replace with real packet capture)
                    let mut rng = rand::rng();
                    let incoming_bytes = rng.random_range(1024..102400) as u64; // 1KB to 100KB per second
                    let outgoing_bytes = rng.random_range(512..51200) as u64;   // 0.5KB to 50KB per second
                    let incoming_packets = incoming_bytes / 1024 + 1;
                    let outgoing_packets = outgoing_bytes / 1024 + 1;

            // Update traffic history
            {
                let mut history = traffic_history.lock().unwrap();
                history.push(TrafficData {
                    timestamp: now,
                    incoming_bytes,
                    outgoing_bytes,
                    incoming_packets,
                    outgoing_packets,
                });

                // Keep only last 300 data points (5 minutes at 1-second intervals)
                if history.len() > 300 {
                    history.remove(0);
                }
            }

            // Update total stats
            {
                let mut stats_guard = stats.write();
                stats_guard.total_incoming_bytes += incoming_bytes;
                stats_guard.total_outgoing_bytes += outgoing_bytes;
                stats_guard.total_incoming_packets += incoming_packets;
                stats_guard.total_outgoing_packets += outgoing_packets;
                stats_guard.monitoring_duration = now - start_time;
                
                // Update traffic rate history
                let history = traffic_history.lock().unwrap();
                stats_guard.traffic_rate = history.clone();
            }

            // Simulate some network hosts
            let mut rng = rand::rng();
            if rng.gen::<f32>() < 0.3 {
                Self::simulate_network_host(&hosts, now);
            }

            // Simulate some services
            if rng.gen::<f32>() < 0.2 {
                Self::simulate_service(&services, incoming_bytes + outgoing_bytes);
            }
                }
                
                _ = save_interval.tick() => {
                    // Save session data every 8 seconds
                    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
                    let current_stats = {
                        let stats_guard = stats.read();
                        stats_guard.clone()
                    };
                    
                    let session = NetworkSession {
                        adapter_name: adapter_name.clone(),
                        start_time: last_save_time,
                        end_time: Some(now),
                        total_incoming_bytes: current_stats.total_incoming_bytes,
                        total_outgoing_bytes: current_stats.total_outgoing_bytes,
                        total_incoming_packets: current_stats.total_incoming_packets,
                        total_outgoing_packets: current_stats.total_outgoing_packets,
                        duration: now - last_save_time,
                        traffic_data: current_stats.traffic_rate.clone(),
                        top_hosts: current_stats.network_hosts.iter().take(10).cloned().collect(),
                        top_services: current_stats.services.iter().take(10).cloned().collect(),
                    };

                    if let Err(e) = NETWORK_STORAGE.save_session(&session) {
                        eprintln!("Failed to save periodic network session: {}", e);
                    } else {
                        println!("Periodic network session saved (8 seconds)");
                    }
                    
                    last_save_time = now;
                }
            }
        }

        println!("Stopped traffic monitoring for adapter: {}", adapter_name);
    }

    fn simulate_network_host(hosts: &Arc<DashMap<String, NetworkHost>>, now: u64) {
        let ips = [
            "192.168.1.1", "8.8.8.8", "1.1.1.1", "172.16.0.1",
            "10.0.0.1", "208.67.222.222", "4.4.4.4", "192.168.0.1"
        ];
        
        let countries = [
            ("United States", "US"), ("Canada", "CA"), ("Germany", "DE"),
            ("United Kingdom", "GB"), ("Japan", "JP"), ("Australia", "AU")
        ];

        let mut rng = rand::rng();
        let ip = ips[rng.random_range(0..ips.len())].to_string();
        let (country, country_code) = countries[rng.random_range(0..countries.len())];
        
        let incoming = rng.random_range(1024..20480) as u64; // 1KB to 20KB per host
        let outgoing = rng.random_range(512..10240) as u64;  // 0.5KB to 10KB per host

        hosts.entry(ip.clone()).and_modify(|host| {
            host.incoming_bytes += incoming;
            host.outgoing_bytes += outgoing;
            host.incoming_packets += incoming / 1024 + 1;
            host.outgoing_packets += outgoing / 1024 + 1;
            host.last_seen = now;
        }).or_insert(NetworkHost {
            ip: ip.clone(),
            hostname: None,
            country: Some(country.to_string()),
            country_code: Some(country_code.to_string()),
            incoming_bytes: incoming,
            outgoing_bytes: outgoing,
            incoming_packets: incoming / 1024 + 1,
            outgoing_packets: outgoing / 1024 + 1,
            first_seen: now,
            last_seen: now,
        });
    }

    fn simulate_service(services: &Arc<DashMap<String, ServiceInfo>>, bytes: u64) {
        let service_data = [
            ("TCP", 80, "HTTP"),
            ("TCP", 443, "HTTPS"),
            ("TCP", 53, "DNS"),
            ("UDP", 53, "DNS"),
            ("TCP", 22, "SSH"),
            ("TCP", 21, "FTP"),
            ("TCP", 25, "SMTP"),
            ("TCP", 993, "IMAPS"),
            ("TCP", 995, "POP3S"),
            ("UDP", 123, "NTP"),
        ];

        let mut rng = rand::rng();
        let (protocol, port, service_name) = service_data[rng.random_range(0..service_data.len())];
        let key = format!("{}:{}", protocol, port);
        
        services.entry(key.clone()).and_modify(|service| {
            service.bytes += bytes;
            service.packets += bytes / 1024 + 1;
        }).or_insert(ServiceInfo {
            protocol: protocol.to_string(),
            port,
            service_name: Some(service_name.to_string()),
            bytes,
            packets: bytes / 1024 + 1,
        });
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
