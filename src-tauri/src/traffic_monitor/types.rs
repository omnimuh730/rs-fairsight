use serde::{Deserialize, Serialize};

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
    pub domain: Option<String>,      // Add domain field like sniffnet
    pub country: Option<String>,
    pub country_code: Option<String>,
    pub asn: Option<String>,         // Add ASN field like sniffnet
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
