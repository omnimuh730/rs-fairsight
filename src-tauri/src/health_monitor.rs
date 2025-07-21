use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::thread;

#[cfg(target_os = "windows")]
use crate::app_state::send_message;

pub struct HealthMonitor {
    last_activity_time: Arc<AtomicU64>,
}

impl HealthMonitor {
    pub fn new() -> Self {
        Self {
            last_activity_time: Arc::new(AtomicU64::new(0)),
        }
    }

    pub fn update_activity(&self) {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        self.last_activity_time.store(current_time, Ordering::SeqCst);
    }

    pub fn get_last_activity_time(&self) -> u64 {
        self.last_activity_time.load(Ordering::SeqCst)
    }

    pub fn start_monitoring(&self) {
        let last_activity_time = Arc::clone(&self.last_activity_time);
        
        thread::spawn(move || {
            const CHECK_INTERVAL: Duration = Duration::from_secs(60); // Check every minute
            const INACTIVITY_THRESHOLD: u64 = 600; // 10 minutes
            
            loop {
                thread::sleep(CHECK_INTERVAL);
                
                let current_time = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs();
                    
                let last_activity = last_activity_time.load(Ordering::SeqCst);
                
                if last_activity > 0 && current_time - last_activity > INACTIVITY_THRESHOLD {
                    crate::log_warning!("health_monitor", "No activity tracked for {} seconds - potential tracking issue", current_time - last_activity);
                    
                    #[cfg(target_os = "windows")]
                    send_message(format!("Warning: Time tracking may have stopped. Last activity: {} seconds ago", current_time - last_activity));
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
