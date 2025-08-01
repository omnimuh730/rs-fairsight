use serde::{Deserialize, Serialize};
use crate::traffic_monitor::{TrafficData, NetworkHost, ServiceInfo};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkSession {
    pub adapter_name: String,
    pub start_time: u64,
    pub end_time: Option<u64>,
    pub total_incoming_bytes: u64,
    pub total_outgoing_bytes: u64,
    pub total_incoming_packets: u64,
    pub total_outgoing_packets: u64,
    pub duration: u64,
    pub traffic_data: Vec<TrafficData>,
    pub top_hosts: Vec<NetworkHost>,
    pub top_services: Vec<ServiceInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyNetworkSummary {
    pub date: String, // YYYY-MM-DD format
    pub sessions: Vec<NetworkSession>,
    pub total_incoming_bytes: u64,
    pub total_outgoing_bytes: u64,
    pub total_duration: u64,
    pub unique_hosts: usize,
    pub unique_services: usize,
}
