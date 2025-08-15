use std::sync::atomic::{AtomicU64, AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::thread;
use serde::{Deserialize, Serialize};

#[cfg(target_os = "windows")]
use crate::utils::app_state::send_message;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemHealthStatus {
    pub activity_monitoring: ActivityMonitoringStatus,
    pub network_monitoring: NetworkMonitoringStatus,
    pub overall_status: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityMonitoringStatus {
    pub is_working: bool,
    pub last_activity_time: u64,
    pub seconds_since_activity: u64,
    pub status_message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkMonitoringStatus {
    pub is_monitoring: bool,
    pub monitoring_mode: String, // "single", "comprehensive", or "none"
    pub active_adapters: Vec<String>,
    pub total_adapters_available: usize,
    pub last_network_activity: u64,
    pub permissions_granted: bool,
    pub status_message: String,
}

pub struct HealthMonitor {
    last_activity_time: Arc<AtomicU64>,
    last_network_activity: Arc<AtomicU64>,
    network_monitoring_active: Arc<AtomicBool>,
}

impl HealthMonitor {
    pub fn new() -> Self {
        Self {
            last_activity_time: Arc::new(AtomicU64::new(0)),
            last_network_activity: Arc::new(AtomicU64::new(0)),
            network_monitoring_active: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn update_activity(&self) {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        self.last_activity_time.store(current_time, Ordering::SeqCst);
    }

    pub fn update_network_activity(&self) {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        self.last_network_activity.store(current_time, Ordering::SeqCst);
    }

    pub fn set_network_monitoring_status(&self, is_active: bool) {
        self.network_monitoring_active.store(is_active, Ordering::SeqCst);
    }

    pub fn get_last_activity_time(&self) -> u64 {
        self.last_activity_time.load(Ordering::SeqCst)
    }

    pub fn get_last_network_activity(&self) -> u64 {
        self.last_network_activity.load(Ordering::SeqCst)
    }

    pub fn is_network_monitoring_active(&self) -> bool {
        self.network_monitoring_active.load(Ordering::SeqCst)
    }

    pub fn get_comprehensive_health_status(&self) -> SystemHealthStatus {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        // Activity monitoring status
        let last_activity = self.get_last_activity_time();
        let activity_status = if last_activity == 0 {
            ActivityMonitoringStatus {
                is_working: false,
                last_activity_time: 0,
                seconds_since_activity: 0,
                status_message: "No activity tracked yet".to_string(),
            }
        } else {
            let seconds_since = current_time.saturating_sub(last_activity);
            let (is_working, message) = if seconds_since < 60 {
                (true, "Time tracking is working normally".to_string())
            } else if seconds_since < 600 {
                (true, format!("Last activity {} seconds ago", seconds_since))
            } else {
                (false, format!("Warning: No activity for {} seconds", seconds_since))
            };

            ActivityMonitoringStatus {
                is_working,
                last_activity_time: last_activity,
                seconds_since_activity: seconds_since,
                status_message: message,
            }
        };

        // Network monitoring status
        let network_status = self.get_network_monitoring_status();

        // Overall system status
        let overall_status = if activity_status.is_working && network_status.is_monitoring {
            "All systems operational".to_string()
        } else if !activity_status.is_working && !network_status.is_monitoring {
            "Both activity and network monitoring have issues".to_string()
        } else if !activity_status.is_working {
            "Activity monitoring needs attention".to_string()
        } else {
            "Network monitoring needs attention".to_string()
        };

        SystemHealthStatus {
            activity_monitoring: activity_status,
            network_monitoring: network_status,
            overall_status,
            timestamp: current_time,
        }
    }

    fn get_network_monitoring_status(&self) -> NetworkMonitoringStatus {
        // Import here to avoid circular dependencies
        use crate::network_monitor::traffic_monitor::{TRAFFIC_MONITORS, is_comprehensive_monitoring_running};
        use crate::network_monitor::network_monitor::get_network_adapters;

        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let is_monitoring = self.is_network_monitoring_active();
        let is_comprehensive = is_comprehensive_monitoring_running();
        let active_adapters: Vec<String> = TRAFFIC_MONITORS.iter()
            .filter(|entry| entry.value().is_monitoring())
            .map(|entry| entry.key().clone())
            .collect();

        let total_adapters = get_network_adapters()
            .map(|adapters| adapters.len())
            .unwrap_or(0);

        let monitoring_mode = if is_comprehensive && active_adapters.len() > 1 {
            "comprehensive".to_string()
        } else if active_adapters.len() == 1 {
            "single".to_string()
        } else {
            "none".to_string()
        };

        let last_network = self.get_last_network_activity();
        
        // Check permissions on macOS
        let permissions_granted = {
            #[cfg(target_os = "macos")]
            {
                use std::process::Command;
                Command::new("tcpdump").arg("-D").output()
                    .map(|output| output.status.success())
                    .unwrap_or(false)
            }
            #[cfg(not(target_os = "macos"))]
            {
                true // Assume permissions are fine on other platforms
            }
        };

        let status_message = if !permissions_granted {
            "Network monitoring requires elevated permissions".to_string()
        } else if !is_monitoring {
            "Network monitoring is not active".to_string()
        } else if active_adapters.is_empty() {
            "Network monitoring active but no adapters captured".to_string()
        } else if last_network == 0 {
            format!("Monitoring {} adapter(s) - waiting for traffic", active_adapters.len())
        } else {
            let seconds_since_network = current_time.saturating_sub(last_network);
            if seconds_since_network < 60 {
                format!("Network monitoring active on {} adapter(s) - traffic flowing", active_adapters.len())
            } else {
                format!("Monitoring {} adapter(s) - last traffic {} seconds ago", active_adapters.len(), seconds_since_network)
            }
        };

        NetworkMonitoringStatus {
            is_monitoring,
            monitoring_mode,
            active_adapters,
            total_adapters_available: total_adapters,
            last_network_activity: last_network,
            permissions_granted,
            status_message,
        }
    }

    pub fn start_monitoring(&self) {
        let last_activity_time = Arc::clone(&self.last_activity_time);
        let last_network_activity = Arc::clone(&self.last_network_activity);
        let network_monitoring_active = Arc::clone(&self.network_monitoring_active);
        
        thread::spawn(move || {
            const CHECK_INTERVAL: Duration = Duration::from_secs(60); // Check every minute
            const ACTIVITY_INACTIVITY_THRESHOLD: u64 = 600; // 10 minutes for activity
            const NETWORK_INACTIVITY_THRESHOLD: u64 = 300; // 5 minutes for network
            
            loop {
                thread::sleep(CHECK_INTERVAL);
                
                let current_time = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs();
                    
                // Check activity monitoring health
                let last_activity = last_activity_time.load(Ordering::SeqCst);
                if last_activity > 0 && current_time - last_activity > ACTIVITY_INACTIVITY_THRESHOLD {
                    crate::log_warning!("health_monitor", "No activity tracked for {} seconds - potential tracking issue", current_time - last_activity);
                    
                    #[cfg(target_os = "windows")]
                    send_message(format!("Warning: Time tracking may have stopped. Last activity: {} seconds ago", current_time - last_activity));
                }

                // Check network monitoring health
                let is_network_active = network_monitoring_active.load(Ordering::SeqCst);
                let last_network = last_network_activity.load(Ordering::SeqCst);
                
                if is_network_active {
                    if last_network > 0 && current_time - last_network > NETWORK_INACTIVITY_THRESHOLD {
                        crate::log_warning!("health_monitor", "Network monitoring active but no traffic captured for {} seconds", current_time - last_network);
                    }
                } else {
                    crate::log_warning!("health_monitor", "Network monitoring is not active - may miss network traffic data");
                }
            }
        });
    }
}

// Global health monitor instance
use once_cell::sync::Lazy;
pub static HEALTH_MONITOR: Lazy<HealthMonitor> = Lazy::new(|| HealthMonitor::new());

pub fn initialize_health_monitoring() {
    HEALTH_MONITOR.start_monitoring();
    println!("Health monitoring initialized");
}

pub fn report_activity() {
    HEALTH_MONITOR.update_activity();
}

pub fn report_network_activity() {
    HEALTH_MONITOR.update_network_activity();
}

pub fn set_network_monitoring_active(is_active: bool) {
    HEALTH_MONITOR.set_network_monitoring_status(is_active);
}

pub fn get_comprehensive_system_health() -> SystemHealthStatus {
    HEALTH_MONITOR.get_comprehensive_health_status()
}
