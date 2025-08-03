use crate::time_tracker::aggregate_log_results;
use crate::health_monitor::{HEALTH_MONITOR, get_comprehensive_system_health, SystemHealthStatus};
use crate::logger::{get_logs, get_recent_logs, clear_logs, LogEntry};
use crate::network_monitor::{get_network_adapters, get_monitoring_adapters, NetworkAdapter};
// use crate::network_engine::{get_network_engine, start_network_engine, stop_network_engine};
// use crate::state_manager::{get_state_manager, AdapterMetrics, SystemState};
use crate::traffic_monitor::{get_or_create_monitor, MonitoringStats};
use crate::network_storage::{NETWORK_STORAGE, DailyNetworkSummary};
use crate::persistent_state::{get_persistent_state_manager, AdapterPersistentState};

#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
pub fn sync_time_data(report_date: &str) -> String {
    match aggregate_log_results(report_date) {
        Ok(result) => result,
        Err(e) => format!("Error: {}", e),
    }
}

#[tauri::command]
pub fn aggregate_week_activity_logs(data_list: Vec<String>) -> Vec<String> {
    let mut logdb_list = Vec::with_capacity(data_list.len());

    for (_i, s) in data_list.into_iter().enumerate() {
        let styled = format!("rs-fairsight({}).txt", s);
        let result = aggregate_log_results(&styled)
            .unwrap_or_else(|e| format!("Error aggregating {}: {}", styled, e));
        logdb_list.push(result);
    }

    logdb_list
}

#[tauri::command]
pub fn get_health_status() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    
    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
        
    let last_activity = HEALTH_MONITOR.get_last_activity_time();
    
    if last_activity == 0 {
        "No activity tracked yet".to_string()
    } else {
        let seconds_since_activity = current_time - last_activity;
        if seconds_since_activity < 60 {
            "Time tracking is working normally".to_string()
        } else if seconds_since_activity < 600 {
            format!("Last activity {} seconds ago", seconds_since_activity)
        } else {
            format!("Warning: No activity for {} seconds", seconds_since_activity)
        }
    }
}

#[tauri::command]
pub fn get_comprehensive_health_status() -> SystemHealthStatus {
    get_comprehensive_system_health()
}

#[tauri::command]
pub fn get_all_logs() -> Vec<LogEntry> {
    get_logs()
}

#[tauri::command]
pub fn get_recent_logs_limited(count: usize) -> Vec<LogEntry> {
    get_recent_logs(count)
}

#[tauri::command]
pub fn clear_all_logs() -> String {
    clear_logs();
    "Logs cleared successfully".to_string()
}

#[tauri::command]
pub fn get_network_adapters_command() -> Result<Vec<NetworkAdapter>, String> {
    get_network_adapters()
}

#[tauri::command]
pub fn get_monitoring_adapters_command() -> Result<Vec<String>, String> {
    get_monitoring_adapters()
}

#[tauri::command]
pub async fn start_network_monitoring(adapter_name: String) -> Result<String, String> {
    #[cfg(target_os = "macos")]
    {
        // Check if we have network access permissions on macOS
        if let Err(permission_error) = check_network_permissions().await {
            return Err(format!("Network permissions required: {}", permission_error));
        }
    }
    
    let monitor = get_or_create_monitor(&adapter_name);
    match monitor.start_monitoring().await {
        Ok(_) => Ok(format!("Started monitoring adapter: {}", adapter_name)),
        Err(e) => Err(e),
    }
}

#[tauri::command]
pub async fn start_comprehensive_monitoring() -> Result<String, String> {
    #[cfg(target_os = "macos")]
    {
        // Check if we have network access permissions on macOS
        if let Err(permission_error) = check_network_permissions().await {
            return Err(format!("Network permissions required: {}", permission_error));
        }
    }
    
    let adapters = get_monitoring_adapters()?;
    
    if adapters.is_empty() {
        return Err("No suitable adapters found for comprehensive monitoring".to_string());
    }
    
    println!("🚀 Starting comprehensive monitoring on {} adapters with packet deduplication", adapters.len());
    
    let mut started_adapters = Vec::new();
    let mut failed_adapters = Vec::new();
    
    for adapter_name in adapters {
        let monitor = get_or_create_monitor(&adapter_name);
        match monitor.start_monitoring().await {
            Ok(_) => {
                started_adapters.push(adapter_name);
            }
            Err(e) => {
                failed_adapters.push(format!("{}: {}", adapter_name, e));
            }
        }
    }
    
    if started_adapters.is_empty() {
        Err(format!("Failed to start monitoring on any adapters: {:?}", failed_adapters))
    } else {
        let result = format!(
            "Started comprehensive monitoring on {} adapters: {:?}{}. Packet deduplication ensures accurate traffic counting across all adapters.",
            started_adapters.len(),
            started_adapters,
            if !failed_adapters.is_empty() {
                format!(" (Failed: {:?})", failed_adapters)
            } else {
                String::new()
            }
        );
        Ok(result)
    }
}

#[tauri::command]
pub fn stop_network_monitoring(adapter_name: String) -> Result<String, String> {
    let monitor = get_or_create_monitor(&adapter_name);
    monitor.stop_monitoring();
    Ok(format!("Stopped monitoring adapter: {}", adapter_name))
}

#[tauri::command]
pub fn stop_comprehensive_monitoring() -> Result<String, String> {
    match get_monitoring_adapters() {
        Ok(adapters) => {
            let mut stopped_adapters = Vec::new();
            
            for adapter_name in adapters {
                let monitor = get_or_create_monitor(&adapter_name);
                monitor.stop_monitoring();
                stopped_adapters.push(adapter_name);
            }
            
            Ok(format!("Stopped comprehensive monitoring on {} adapters: {:?}", 
                stopped_adapters.len(), stopped_adapters))
        }
        Err(e) => Err(format!("Failed to get monitoring adapters: {}", e))
    }
}

#[tauri::command]
pub fn get_network_stats(adapter_name: String) -> Result<MonitoringStats, String> {
    let monitor = get_or_create_monitor(&adapter_name);
    Ok(monitor.get_stats())
}

#[tauri::command]
pub fn get_comprehensive_network_stats() -> Result<MonitoringStats, String> {
    let adapters = get_monitoring_adapters()?;
    
    let mut combined_stats = MonitoringStats {
        total_incoming_bytes: 0,
        total_outgoing_bytes: 0,
        total_incoming_packets: 0,
        total_outgoing_packets: 0,
        monitoring_duration: 0,
        traffic_rate: Vec::new(),
        network_hosts: Vec::new(),
        services: Vec::new(),
    };
    
    let mut all_hosts: std::collections::HashMap<String, crate::traffic_monitor::NetworkHost> = std::collections::HashMap::new();
    let mut all_services: std::collections::HashMap<String, crate::traffic_monitor::ServiceInfo> = std::collections::HashMap::new();
    let mut max_duration = 0u64;
    
    for adapter_name in adapters {
        let monitor = get_or_create_monitor(&adapter_name);
        if monitor.is_monitoring() {
            let stats = monitor.get_stats();
            
            // Aggregate totals
            combined_stats.total_incoming_bytes += stats.total_incoming_bytes;
            combined_stats.total_outgoing_bytes += stats.total_outgoing_bytes;
            combined_stats.total_incoming_packets += stats.total_incoming_packets;
            combined_stats.total_outgoing_packets += stats.total_outgoing_packets;
            
            // Track maximum monitoring duration
            if stats.monitoring_duration > max_duration {
                max_duration = stats.monitoring_duration;
            }
            
            // Merge hosts (sum traffic for same IP)
            for host in stats.network_hosts {
                if let Some(existing_host) = all_hosts.get_mut(&host.ip) {
                    existing_host.incoming_bytes += host.incoming_bytes;
                    existing_host.outgoing_bytes += host.outgoing_bytes;
                    existing_host.incoming_packets += host.incoming_packets;
                    existing_host.outgoing_packets += host.outgoing_packets;
                    if host.first_seen < existing_host.first_seen {
                        existing_host.first_seen = host.first_seen;
                    }
                    if host.last_seen > existing_host.last_seen {
                        existing_host.last_seen = host.last_seen;
                    }
                } else {
                    all_hosts.insert(host.ip.clone(), host);
                }
            }
            
            // Merge services (sum traffic for same protocol/port)
            for service in stats.services {
                let service_key = format!("{}:{}", service.protocol, service.port);
                if let Some(existing_service) = all_services.get_mut(&service_key) {
                    existing_service.bytes += service.bytes;
                    existing_service.packets += service.packets;
                } else {
                    all_services.insert(service_key, service);
                }
            }
        }
    }
    
    combined_stats.monitoring_duration = max_duration;
    combined_stats.network_hosts = all_hosts.into_values().collect();
    combined_stats.services = all_services.into_values().collect();
    
    // Sort hosts by total bytes (descending)
    combined_stats.network_hosts.sort_by(|a, b| {
        let total_a = a.incoming_bytes + a.outgoing_bytes;
        let total_b = b.incoming_bytes + b.outgoing_bytes;
        total_b.cmp(&total_a)
    });
    
    // Sort services by bytes (descending)
    combined_stats.services.sort_by(|a, b| b.bytes.cmp(&a.bytes));
    
    // Limit results
    combined_stats.network_hosts.truncate(1000);
    combined_stats.services.truncate(100);
    
    Ok(combined_stats)
}

#[tauri::command]
pub async fn is_network_monitoring(adapter_name: String) -> bool {
    let monitor = get_or_create_monitor(&adapter_name);
    monitor.is_monitoring()
}

#[tauri::command]
pub fn is_comprehensive_monitoring_active() -> Result<serde_json::Value, String> {
    let adapters = get_monitoring_adapters()?;
    let mut monitoring_status = std::collections::HashMap::new();
    let mut total_monitoring = 0;
    
    for adapter_name in &adapters {
        let monitor = get_or_create_monitor(adapter_name);
        let is_monitoring = monitor.is_monitoring();
        monitoring_status.insert(adapter_name.clone(), is_monitoring);
        if is_monitoring {
            total_monitoring += 1;
        }
    }
    
    let result = serde_json::json!({
        "total_adapters": adapters.len(),
        "monitoring_adapters": total_monitoring,
        "is_comprehensive": total_monitoring > 1,
        "adapter_status": monitoring_status,
        "adapters": adapters
    });
    
    Ok(result)
}

#[tauri::command]
pub fn get_network_history(start_date: String, end_date: String) -> Result<Vec<DailyNetworkSummary>, String> {
    match NETWORK_STORAGE.get_date_range_data(&start_date, &end_date) {
        Ok(mut data) => {
            println!("📊 Network history requested: {} to {}", start_date, end_date);
            
            // Also try to get today's data if not already included
            let today = chrono::Utc::now().format("%Y-%m-%d").to_string();
            let tomorrow = chrono::Utc::now().checked_add_signed(chrono::Duration::days(1))
                .unwrap_or_else(|| chrono::Utc::now())
                .format("%Y-%m-%d").to_string();
            
            // Check if we need to add today's or tomorrow's data (timezone handling)
            let dates_in_data: std::collections::HashSet<String> = data.iter().map(|d| d.date.clone()).collect();
            
            for check_date in [today.clone(), tomorrow] {
                if !dates_in_data.contains(&check_date) {
                    if let Ok(today_data) = NETWORK_STORAGE.load_daily_summary(&check_date) {
                        println!("📊 Adding missing date data: {}", check_date);
                        data.push(today_data);
                    }
                }
            }
            
            // Sort by date
            data.sort_by(|a, b| a.date.cmp(&b.date));
            
            println!("📊 Returning {} daily summaries for dates: {:?}", 
                data.len(), 
                data.iter().map(|d| &d.date).collect::<Vec<_>>());
            
            // Enhance the data with persistent state information
            let persistent_states = get_persistent_state_manager().get_all_adapter_states().unwrap_or_default();
            
            for summary in &mut data {
                // Add lifetime stats context to each daily summary
                let mut daily_lifetime_incoming = 0u64;
                let mut daily_lifetime_outgoing = 0u64;
                
                for (_adapter_name, state) in &persistent_states {
                    // Check if this adapter had activity on this date
                    if let Some(first_time) = state.first_recorded_time {
                        let summary_timestamp = chrono::NaiveDate::parse_from_str(&summary.date, "%Y-%m-%d")
                            .map(|d| d.and_hms_opt(0, 0, 0).unwrap().and_utc().timestamp() as u64)
                            .unwrap_or(0);
                        
                        if first_time <= summary_timestamp + 86400 { // If adapter was active by end of this day
                            daily_lifetime_incoming += state.lifetime_incoming_bytes;
                            daily_lifetime_outgoing += state.lifetime_outgoing_bytes;
                        }
                    }
                }
                
                println!("📊 Day {}: {} sessions, Session Total: ↓{}KB ↑{}KB, Lifetime Context: ↓{}KB ↑{}KB", 
                    summary.date,
                    summary.sessions.len(),
                    summary.total_incoming_bytes / 1024,
                    summary.total_outgoing_bytes / 1024,
                    daily_lifetime_incoming / 1024,
                    daily_lifetime_outgoing / 1024
                );
            }
            
            Ok(data)
        }
        Err(e) => {
            eprintln!("📊 Failed to get network history: {}", e);
            Err(e)
        }
    }
}

#[tauri::command]
pub fn get_available_network_dates() -> Result<Vec<String>, String> {
    NETWORK_STORAGE.get_available_dates()
}

#[tauri::command]
pub fn cleanup_old_network_data(days_to_keep: u32) -> Result<String, String> {
    NETWORK_STORAGE.cleanup_old_data(days_to_keep)?;
    Ok(format!("Cleaned up network data older than {} days", days_to_keep))
}

#[tauri::command]
pub fn create_network_backup(date: String) -> Result<String, String> {
    NETWORK_STORAGE.create_backup(&date)?;
    Ok(format!("Backup created for network data: {}", date))
}

#[tauri::command]
pub fn restore_network_backup(date: String) -> Result<String, String> {
    NETWORK_STORAGE.restore_from_backup(&date)?;
    Ok(format!("Network data restored from backup: {}", date))
}

#[tauri::command]
pub fn cleanup_network_backups() -> Result<String, String> {
    NETWORK_STORAGE.daily_backup_cleanup()?;
    Ok("Old network backups cleaned up successfully".to_string())
}

#[tauri::command]
pub fn get_adapter_persistent_state(adapter_name: String) -> Result<Option<AdapterPersistentState>, String> {
    get_persistent_state_manager().get_adapter_state(&adapter_name)
}

#[tauri::command]
pub fn get_lifetime_stats() -> Result<std::collections::HashMap<String, AdapterPersistentState>, String> {
    get_persistent_state_manager().get_all_adapter_states()
}

#[tauri::command]
pub fn check_unexpected_shutdown() -> Result<bool, String> {
    get_persistent_state_manager().was_unexpected_shutdown()
}

#[tauri::command]
pub fn get_current_network_totals() -> Result<std::collections::HashMap<String, serde_json::Value>, String> {
    let mut totals = std::collections::HashMap::new();
    
    // Get persistent state totals (lifetime/cumulative)
    let persistent_states = get_persistent_state_manager().get_all_adapter_states()?;
    
    // Get today's session data
    let today = chrono::Utc::now().format("%Y-%m-%d").to_string();
    let today_sessions = NETWORK_STORAGE.load_daily_summary(&today).unwrap_or_else(|_| {
        crate::network_storage::DailyNetworkSummary {
            date: today.clone(),
            sessions: Vec::new(),
            total_incoming_bytes: 0,
            total_outgoing_bytes: 0,
            total_duration: 0,
            unique_hosts: 0,
            unique_services: 0,
        }
    });
    
    // Structure the persistent state to match frontend expectations
    let persistent_state_structure = serde_json::json!({
        "persistent_state": persistent_states,
        "last_shutdown_time": get_persistent_state_manager().get_last_shutdown_time().unwrap_or(0),
        "app_version": "1.0.0"
    });
    
    // Combine the data
    totals.insert("persistent_state".to_string(), persistent_state_structure);
    totals.insert("today_sessions".to_string(), serde_json::to_value(&today_sessions).unwrap());
    
    // Calculate combined totals
    let mut combined_incoming = 0u64;
    let mut combined_outgoing = 0u64;
    for state in persistent_states.values() {
        combined_incoming += state.cumulative_incoming_bytes;
        combined_outgoing += state.cumulative_outgoing_bytes;
    }
    
    let combined = serde_json::json!({
        "total_incoming_bytes": combined_incoming,
        "total_outgoing_bytes": combined_outgoing,
        "session_incoming_bytes": today_sessions.total_incoming_bytes,
        "session_outgoing_bytes": today_sessions.total_outgoing_bytes,
        "active_adapters": persistent_states.len(),
        "today_sessions_count": today_sessions.sessions.len()
    });
    
    totals.insert("combined_totals".to_string(), combined);
    
    println!("📊 Current totals - Persistent: ↓{}KB ↑{}KB, Sessions: ↓{}KB ↑{}KB", 
        combined_incoming / 1024, combined_outgoing / 1024,
        today_sessions.total_incoming_bytes / 1024, today_sessions.total_outgoing_bytes / 1024);
    
    Ok(totals)
}

// macOS Network Permission Functions
#[cfg(target_os = "macos")]
async fn check_network_permissions() -> Result<(), String> {
    
    crate::log_info!("macos_permissions", "Checking macOS network monitoring permissions via tcpdump...");
    
    // Try to create a test pcap handle to check permissions
    // This will trigger the system permission dialog if needed
    use std::process::Command;

    match Command::new("tcpdump")
        .arg("-D")
        .output()
    {
        Ok(output) => {
            crate::log_info!("macos_permissions", "tcpdump command executed - status: {}", output.status);
            
            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                let stdout = String::from_utf8_lossy(&output.stdout);
                
                crate::log_warning!("macos_permissions", "tcpdump failed - stdout: '{}', stderr: '{}'", stdout, stderr);
                
                if stderr.contains("permission") || stderr.contains("Operation not permitted") {
                    let error_msg = "Network monitoring requires administrator privileges. Please allow network access in System Preferences → Security & Privacy → Privacy → Developer Tools or run with sudo.";
                    crate::log_error!("macos_permissions", "{}", error_msg);
                    return Err(error_msg.to_string());
                } else {
                    crate::log_warning!("macos_permissions", "tcpdump failed for unknown reason, but may not be permission-related");
                }
            } else {
                crate::log_info!("macos_permissions", "✅ tcpdump permissions check passed");
            }
            Ok(())
        },
        Err(e) => {
            let error_msg = format!("Failed to check network permissions: {}. Please install tcpdump or enable network monitoring permissions.", e);
            crate::log_error!("macos_permissions", "{}", error_msg);
            Err(error_msg)
        }
    }
}

#[tauri::command]
pub async fn request_network_permissions() -> Result<String, String> {
    #[cfg(target_os = "macos")]
    {
        use crate::macos_utils::{check_bpf_permissions, request_network_permissions, get_permission_instructions};
        
        match check_bpf_permissions() {
            Ok(_) => Ok("Network permissions are already granted.".to_string()),
            Err(_) => {
                // Try to open System Preferences automatically
                match request_network_permissions() {
                    Ok(_) => Ok("Please grant network permissions in System Preferences, then restart the application.".to_string()),
                    Err(_) => {
                        // Fallback to manual instructions
                        Ok(get_permission_instructions())
                    }
                }
            }
        }
    }
    
    #[cfg(not(target_os = "macos"))]
    {
        Ok("Network permissions not required on this platform.".to_string())
    }
}

#[tauri::command]
pub async fn check_network_permissions_status() -> bool {
    #[cfg(target_os = "macos")]
    {
        check_network_permissions().await.is_ok()
    }
    
    #[cfg(not(target_os = "macos"))]
    {
        true // Always allow on other platforms
    }
}
