use std::sync::{Arc, atomic::{AtomicBool, AtomicU64}};
use serde::{Deserialize, Serialize};
use std::net::IpAddr;

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
    pub packets_processed: Arc<AtomicU64>,
    pub bytes_processed: Arc<AtomicU64>,
}
