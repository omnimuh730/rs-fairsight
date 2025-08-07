use std::collections::HashSet;
use super::types::NetworkSession;

pub fn consolidate_sessions(sessions: Vec<NetworkSession>, adapter_name: &str) -> Result<Vec<NetworkSession>, String> {
    // Group sessions by time windows (30-minute chunks)
    let mut consolidated_sessions = Vec::new();
    let time_window = 1800; // 30 minutes
    
    // Sort sessions by start time
    let mut sessions = sessions;
    let sessions_count = sessions.len();
    sessions.sort_by_key(|s| s.start_time);
    
    let mut current_group = Vec::new();
    let mut last_time_window = 0;
    
    for session in sessions {
        let time_window_start = (session.start_time / time_window) * time_window;
        
        if time_window_start != last_time_window && !current_group.is_empty() {
            // Consolidate the current group
            consolidated_sessions.push(merge_sessions(adapter_name, &current_group)?);
            current_group.clear();
        }
        
        current_group.push(session);
        last_time_window = time_window_start;
    }
    
    if !current_group.is_empty() {
        consolidated_sessions.push(merge_sessions(adapter_name, &current_group)?);
    }
    
    println!("📊 Consolidated sessions: reduced from {} to {} sessions", 
        sessions_count, 
        consolidated_sessions.len());
    
    Ok(consolidated_sessions)
}

pub fn merge_sessions(adapter_name: &str, sessions: &[NetworkSession]) -> Result<NetworkSession, String> {
    if sessions.is_empty() {
        return Err("Cannot merge empty sessions".to_string());
    }
    
    let first_session = &sessions[0];
    let last_session = &sessions[sessions.len() - 1];
    
    let total_incoming: u64 = sessions.iter().map(|s| s.total_incoming_bytes).sum();
    let total_outgoing: u64 = sessions.iter().map(|s| s.total_outgoing_bytes).sum();
    let total_incoming_packets: u64 = sessions.iter().map(|s| s.total_incoming_packets).sum();
    let total_outgoing_packets: u64 = sessions.iter().map(|s| s.total_outgoing_packets).sum();
    
    // Calculate duration based on actual time span, not sum of individual durations
    // This prevents the >24 hour issue when sessions span across day boundaries
    let start_time = first_session.start_time;
    let end_time = last_session.end_time.unwrap_or(start_time);
    let calculated_duration = if end_time > start_time {
        end_time - start_time
    } else {
        0
    };
    
    // Merge hosts and services (keep top entries)
    let mut all_hosts = Vec::new();
    let mut all_services = Vec::new();
    
    for session in sessions {
        all_hosts.extend(session.top_hosts.iter().cloned());
        all_services.extend(session.top_services.iter().cloned());
    }
    
    // Deduplicate and sort hosts
    all_hosts.sort_by(|a, b| {
        let total_a = a.incoming_bytes + a.outgoing_bytes;
        let total_b = b.incoming_bytes + b.outgoing_bytes;
        total_b.cmp(&total_a)
    });
    all_hosts.truncate(10);
    
    // Deduplicate and sort services
    all_services.sort_by(|a, b| b.bytes.cmp(&a.bytes));
    all_services.truncate(10);
    
    Ok(NetworkSession {
        adapter_name: adapter_name.to_string(),
        start_time: first_session.start_time,
        end_time: last_session.end_time,
        total_incoming_bytes: total_incoming,
        total_outgoing_bytes: total_outgoing,
        total_incoming_packets: total_incoming_packets,
        total_outgoing_packets: total_outgoing_packets,
        duration: calculated_duration,
        traffic_data: Vec::new(), // Clear detailed traffic data for consolidated sessions
        top_hosts: all_hosts,
        top_services: all_services,
    })
}

pub fn calculate_unique_counts(sessions: &[NetworkSession]) -> (usize, usize) {
    let mut all_hosts = HashSet::new();
    let mut all_services = HashSet::new();
    
    for session in sessions {
        for host in &session.top_hosts {
            all_hosts.insert(&host.ip);
        }
        for service in &session.top_services {
            all_services.insert(format!("{}:{}", service.protocol, service.port));
        }
    }
    
    (all_hosts.len(), all_services.len())
}

#[cfg(target_os = "macos")]
pub fn calculate_macos_totals(sessions: &[NetworkSession]) -> (u64, u64, u64) {
    // Deduplicate by unique host and service for macOS
    use std::collections::HashMap;
    let mut host_map: HashMap<String, u64> = HashMap::new();
    let mut service_map: HashMap<String, u64> = HashMap::new();
    let mut total_in_bytes = 0u64;
    let mut total_out_bytes = 0u64;
    
    for session in sessions {
        for host in &session.top_hosts {
            let entry = host_map.entry(host.ip.clone()).or_insert(0);
            if *entry == 0 {
                total_in_bytes += host.incoming_bytes;
                total_out_bytes += host.outgoing_bytes;
            }
            *entry += 1;
        }
        for service in &session.top_services {
            let key = format!("{}:{}", service.protocol, service.port);
            let entry = service_map.entry(key).or_insert(0);
            if *entry == 0 {
                // Only count bytes for unique service
                // If you want to sum bytes, you can add service.bytes here
            }
            *entry += 1;
        }
    }
    
    // Calculate duration based on actual time span, not sum of individual durations
    // This prevents the >24 hour issue when sessions span across day boundaries
    let total_duration = if !sessions.is_empty() {
        let first_session = &sessions[0];
        let last_session = &sessions[sessions.len() - 1];
        let start_time = first_session.start_time;
        let end_time = last_session.end_time.unwrap_or(start_time);
        if end_time > start_time {
            end_time - start_time
        } else {
            0
        }
    } else {
        0
    };
    
    (total_in_bytes, total_out_bytes, total_duration)
}
