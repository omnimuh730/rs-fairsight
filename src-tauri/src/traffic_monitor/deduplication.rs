use dashmap::DashMap;
use std::sync::Arc;

// Global packet deduplication store to prevent counting same packets across adapters
lazy_static::lazy_static! {
    pub static ref PACKET_DEDUP: Arc<DashMap<String, (u64, String)>> = Arc::new(DashMap::new());
}

pub fn create_packet_signature(
    src_ip: &str,
    dst_ip: &str,
    src_port: u16,
    dst_port: u16,
    protocol: &str,
    timestamp_ms: u64,
) -> String {
    format!("{}:{}->{}:{}:{}:{}", src_ip, src_port, dst_ip, dst_port, protocol, timestamp_ms)
}

pub fn is_duplicate_packet(signature: &str) -> bool {
    PACKET_DEDUP.contains_key(signature)
}

pub fn register_packet(signature: String, adapter_name: String) {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    
    PACKET_DEDUP.insert(signature, (timestamp, adapter_name));
}

pub fn cleanup_old_signatures() {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    
    let cutoff = now - 10; // Remove signatures older than 10 seconds
    
    PACKET_DEDUP.retain(|_k, v| v.0 > cutoff);
}
