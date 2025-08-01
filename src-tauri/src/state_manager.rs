// Centralized State Management System for TinkerTicker
// This replaces the distributed state handling across multiple files

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};
use std::fs;
use std::path::PathBuf;
use tokio::sync::broadcast;

#[cfg(not(target_os = "windows"))]
use dirs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdapterMetrics {
    pub name: String,
    pub display_name: String,
    pub is_active: bool,
    pub is_monitoring: bool,
    pub total_bytes_in: u64,
    pub total_bytes_out: u64,
    pub total_packets_in: u64,
    pub total_packets_out: u64,
    pub session_start_time: Option<u64>,
    pub last_seen_time: u64,
    pub connection_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemState {
    pub adapters: HashMap<String, AdapterMetrics>,
    pub global_stats: GlobalStats,
    pub app_metadata: AppMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalStats {
    pub total_bytes_in: u64,
    pub total_bytes_out: u64,
    pub total_packets_in: u64,
    pub total_packets_out: u64,
    pub active_adapters: u32,
    pub monitoring_start_time: Option<u64>,
    pub last_update_time: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppMetadata {
    pub version: String,
    pub last_clean_shutdown: Option<u64>,
    pub startup_time: u64,
    pub is_first_run: bool,
}

#[derive(Debug, Clone)]
pub enum StateEvent {
    AdapterConnected(String),
    AdapterDisconnected(String),
    MonitoringStarted(String),
    MonitoringStopped(String),
    TrafficUpdate { adapter: String, bytes_in: u64, bytes_out: u64, packets_in: u64, packets_out: u64 },
    SystemShutdown,
}

pub struct StateManager {
    state: Arc<RwLock<SystemState>>,
    state_file: PathBuf,
    backup_file: PathBuf,
    event_sender: broadcast::Sender<StateEvent>,
    auto_save_enabled: bool,
}

impl StateManager {
    pub fn new() -> Result<Self, String> {
        let state_dir = Self::get_state_directory()?;
        let state_file = state_dir.join("app_state.json");
        let backup_file = state_dir.join("app_state_backup.json");
        
        let initial_state = Self::load_or_create_state(&state_file, &backup_file)?;
        let (event_sender, _) = broadcast::channel(100);
        
        Ok(Self {
            state: Arc::new(RwLock::new(initial_state)),
            state_file,
            backup_file,
            event_sender,
            auto_save_enabled: true,
        })
    }

    fn get_state_directory() -> Result<PathBuf, String> {
        #[cfg(target_os = "windows")]
        let base_dir = PathBuf::from("C:\\fairsight-state");
        
        #[cfg(target_os = "macos")]
        let base_dir = dirs::home_dir()
            .ok_or("Cannot find home directory")?
            .join("Documents")
            .join("TinkerTicker");
        
        #[cfg(target_os = "linux")]
        let base_dir = dirs::home_dir()
            .ok_or("Cannot find home directory")?
            .join(".local")
            .join("share")
            .join("tinkerticker");

        if !base_dir.exists() {
            fs::create_dir_all(&base_dir)
                .map_err(|e| format!("Failed to create state directory: {}", e))?;
        }
        
        Ok(base_dir)
    }

    fn load_or_create_state(state_file: &PathBuf, backup_file: &PathBuf) -> Result<SystemState, String> {
        // Try loading main state file
        if let Ok(state) = Self::load_state_from_file(state_file) {
            return Ok(state);
        }
        
        // Try loading backup
        if let Ok(state) = Self::load_state_from_file(backup_file) {
            println!("📦 Restored state from backup");
            return Ok(state);
        }
        
        // Create fresh state
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
            
        Ok(SystemState {
            adapters: HashMap::new(),
            global_stats: GlobalStats {
                total_bytes_in: 0,
                total_bytes_out: 0,
                total_packets_in: 0,
                total_packets_out: 0,
                active_adapters: 0,
                monitoring_start_time: None,
                last_update_time: now,
            },
            app_metadata: AppMetadata {
                version: std::env::var("CARGO_PKG_VERSION").unwrap_or_else(|_| "1.0.0".to_string()),
                last_clean_shutdown: Some(now), // Fresh start is clean
                startup_time: now,
                is_first_run: true,
            },
        })
    }

    fn load_state_from_file(file_path: &PathBuf) -> Result<SystemState, String> {
        if !file_path.exists() {
            return Err("State file does not exist".to_string());
        }
        
        let content = fs::read_to_string(file_path)
            .map_err(|e| format!("Failed to read state file: {}", e))?;
        
        serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse state file: {}", e))
    }

    pub fn save_state(&self) -> Result<(), String> {
        if !self.auto_save_enabled {
            return Ok(());
        }

        let state = self.state.read().unwrap();
        
        // Create backup first
        if self.state_file.exists() {
            let _ = fs::copy(&self.state_file, &self.backup_file);
        }
        
        // Save current state
        let content = serde_json::to_string_pretty(&*state)
            .map_err(|e| format!("Failed to serialize state: {}", e))?;
        
        fs::write(&self.state_file, content)
            .map_err(|e| format!("Failed to write state file: {}", e))?;
        
        Ok(())
    }

    pub fn get_state_snapshot(&self) -> SystemState {
        self.state.read().unwrap().clone()
    }

    pub fn update_adapter(&self, adapter_name: &str, updater: impl FnOnce(&mut AdapterMetrics)) -> Result<(), String> {
        let mut state = self.state.write().unwrap();
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        let adapter = state.adapters.entry(adapter_name.to_string()).or_insert_with(|| {
            AdapterMetrics {
                name: adapter_name.to_string(),
                display_name: adapter_name.to_string(),
                is_active: false,
                is_monitoring: false,
                total_bytes_in: 0,
                total_bytes_out: 0,
                total_packets_in: 0,
                total_packets_out: 0,
                session_start_time: None,
                last_seen_time: now,
                connection_count: 0,
            }
        });
        
        updater(adapter);
        adapter.last_seen_time = now;
        state.global_stats.last_update_time = now;
        
        drop(state);
        
        if self.auto_save_enabled {
            self.save_state()?;
        }
        
        Ok(())
    }

    pub fn mark_adapter_active(&self, adapter_name: &str, is_active: bool) -> Result<(), String> {
        self.update_adapter(adapter_name, |adapter| {
            adapter.is_active = is_active;
            if is_active {
                adapter.last_seen_time = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs();
            }
        })?;
        
        if is_active {
            let _ = self.event_sender.send(StateEvent::AdapterConnected(adapter_name.to_string()));
        } else {
            let _ = self.event_sender.send(StateEvent::AdapterDisconnected(adapter_name.to_string()));
        }
        
        Ok(())
    }

    pub fn start_monitoring(&self, adapter_name: &str) -> Result<(), String> {
        self.update_adapter(adapter_name, |adapter| {
            adapter.is_monitoring = true;
            if adapter.session_start_time.is_none() {
                adapter.session_start_time = Some(SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs());
            }
        })?;
        
        let _ = self.event_sender.send(StateEvent::MonitoringStarted(adapter_name.to_string()));
        Ok(())
    }

    pub fn stop_monitoring(&self, adapter_name: &str) -> Result<(), String> {
        self.update_adapter(adapter_name, |adapter| {
            adapter.is_monitoring = false;
        })?;
        
        let _ = self.event_sender.send(StateEvent::MonitoringStopped(adapter_name.to_string()));
        Ok(())
    }

    pub fn update_traffic(&self, adapter_name: &str, bytes_in: u64, bytes_out: u64, packets_in: u64, packets_out: u64) -> Result<(), String> {
        self.update_adapter(adapter_name, |adapter| {
            adapter.total_bytes_in += bytes_in;
            adapter.total_bytes_out += bytes_out;
            adapter.total_packets_in += packets_in;
            adapter.total_packets_out += packets_out;
        })?;
        
        // Update global stats
        {
            let mut state = self.state.write().unwrap();
            state.global_stats.total_bytes_in += bytes_in;
            state.global_stats.total_bytes_out += bytes_out;
            state.global_stats.total_packets_in += packets_in;
            state.global_stats.total_packets_out += packets_out;
            state.global_stats.active_adapters = state.adapters.values()
                .filter(|a| a.is_monitoring)
                .count() as u32;
        }
        
        let _ = self.event_sender.send(StateEvent::TrafficUpdate {
            adapter: adapter_name.to_string(),
            bytes_in,
            bytes_out,
            packets_in,
            packets_out,
        });
        
        Ok(())
    }

    pub fn subscribe_to_events(&self) -> broadcast::Receiver<StateEvent> {
        self.event_sender.subscribe()
    }

    pub fn was_unexpected_shutdown(&self) -> bool {
        let state = self.state.read().unwrap();
        
        // Fresh installs are never unexpected
        if state.app_metadata.is_first_run {
            return false;
        }
        
        // Check if we have a recent clean shutdown
        if let Some(last_shutdown) = state.app_metadata.last_clean_shutdown {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
            
            // If shutdown was within last 10 minutes, consider it clean
            if now - last_shutdown < 600 {
                return false;
            }
        }
        
        // Check if any adapters were monitoring when last saved
        state.adapters.values().any(|adapter| adapter.is_monitoring)
    }

    pub fn mark_clean_shutdown(&self) -> Result<(), String> {
        let mut state = self.state.write().unwrap();
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        state.app_metadata.last_clean_shutdown = Some(now);
        state.app_metadata.is_first_run = false;
        
        // Stop all monitoring
        for adapter in state.adapters.values_mut() {
            adapter.is_monitoring = false;
        }
        
        drop(state);
        
        self.save_state()?;
        let _ = self.event_sender.send(StateEvent::SystemShutdown);
        
        Ok(())
    }

    pub fn cleanup_inactive_adapters(&self, max_age_seconds: u64) -> Result<usize, String> {
        let mut state = self.state.write().unwrap();
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        let initial_count = state.adapters.len();
        
        state.adapters.retain(|_, adapter| {
            adapter.is_active || (now - adapter.last_seen_time) < max_age_seconds
        });
        
        let removed_count = initial_count - state.adapters.len();
        
        if removed_count > 0 {
            println!("🧹 Cleaned up {} inactive adapters", removed_count);
            state.global_stats.last_update_time = now;
            drop(state);
            self.save_state()?;
        }
        
        Ok(removed_count)
    }
}

// Global state manager instance
use std::sync::OnceLock;
static STATE_MANAGER: OnceLock<StateManager> = OnceLock::new();

pub fn get_state_manager() -> &'static StateManager {
    STATE_MANAGER.get_or_init(|| {
        StateManager::new().expect("Failed to initialize state manager")
    })
}
