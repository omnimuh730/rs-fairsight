use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdapterPersistentState {
    pub adapter_name: String,
    pub cumulative_incoming_bytes: u64,
    pub cumulative_outgoing_bytes: u64,
    pub cumulative_incoming_packets: u64,
    pub cumulative_outgoing_packets: u64,
    pub session_start_time: Option<u64>,
    pub last_session_end_time: Option<u64>,
    pub was_monitoring_on_exit: bool,
    pub lifetime_incoming_bytes: u64,
    pub lifetime_outgoing_bytes: u64,
    pub first_recorded_time: Option<u64>,
    pub last_update_time: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppPersistentState {
    pub adapters: HashMap<String, AdapterPersistentState>,
    pub last_shutdown_time: Option<u64>,
    pub app_version: String,
    pub created_at: u64,
    pub updated_at: u64,
}

pub struct PersistentStateManager {
    state_file_path: PathBuf,
    backup_file_path: PathBuf,
}

impl PersistentStateManager {
    pub fn new() -> Result<Self, String> {
        let state_dir = std::path::Path::new("C:\\fairsight-network-log");
        if !state_dir.exists() {
            fs::create_dir_all(&state_dir)
                .map_err(|e| format!("Failed to create state directory: {}", e))?;
        }
        
        let state_file_path = state_dir.join("persistent_state.json");
        let backup_file_path = state_dir.join("persistent_state_backup.json");
        
        Ok(Self {
            state_file_path,
            backup_file_path,
        })
    }

    pub fn load_state(&self) -> Result<AppPersistentState, String> {
        // Try to load from main file first
        if let Ok(state) = self.load_from_file(&self.state_file_path) {
            return Ok(state);
        }
        
        // If main file fails, try backup
        if let Ok(state) = self.load_from_file(&self.backup_file_path) {
            println!("⚠️  Loaded from backup state file");
            return Ok(state);
        }
        
        // If both fail, create new state with clean shutdown marked
        let current_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
            
        let new_state = AppPersistentState {
            adapters: HashMap::new(),
            last_shutdown_time: Some(current_time), // Mark as clean start
            app_version: env!("CARGO_PKG_VERSION").to_string(),
            created_at: current_time,
            updated_at: current_time,
        };
        
        // Save the new state immediately to establish clean state
        let _ = self.save_state(&new_state);
        
        Ok(new_state)
    }

    fn load_from_file(&self, file_path: &PathBuf) -> Result<AppPersistentState, String> {
        if !file_path.exists() {
            return Err("State file does not exist".to_string());
        }
        
        let content = fs::read_to_string(file_path)
            .map_err(|e| format!("Failed to read state file: {}", e))?;
        
        let state: AppPersistentState = serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse state file: {}", e))?;
        
        Ok(state)
    }

    pub fn save_state(&self, state: &AppPersistentState) -> Result<(), String> {
        // Ensure the directory exists
        if let Some(parent_dir) = self.state_file_path.parent() {
            if !parent_dir.exists() {
                fs::create_dir_all(parent_dir)
                    .map_err(|e| format!("Failed to create state directory: {}", e))?;
            }
        }

        // Create backup of current state if it exists
        if self.state_file_path.exists() {
            if let Err(e) = fs::copy(&self.state_file_path, &self.backup_file_path) {
                eprintln!("Warning: Failed to create backup: {}", e);
            }
        }
        
        let content = serde_json::to_string_pretty(state)
            .map_err(|e| format!("Failed to serialize state: {}", e))?;
        
        // Try direct write first (simpler approach for Windows)
        match fs::write(&self.state_file_path, &content) {
            Ok(_) => {
                // Verify the file was written correctly by reading it back
                match fs::read_to_string(&self.state_file_path) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(format!("Failed to verify written state file: {}", e))
                }
            }
            Err(_) => {
                // If direct write fails, try atomic write using temporary file
                let temp_path = self.state_file_path.with_extension("tmp");
                
                fs::write(&temp_path, &content)
                    .map_err(|e| format!("Failed to write temporary state file: {}", e))?;
                
                // Verify the temporary file was written correctly
                let _ = fs::read_to_string(&temp_path)
                    .map_err(|e| format!("Failed to verify temporary state file: {}", e))?;
                
                // Atomically replace the original file
                fs::rename(&temp_path, &self.state_file_path)
                    .map_err(|e| format!("Failed to finalize state save: {}", e))?;
                
                Ok(())
            }
        }
    }

    pub fn update_adapter_state(&self, adapter_name: &str, updater: impl FnOnce(&mut AdapterPersistentState)) -> Result<(), String> {
        let mut state = self.load_state()?;
        
        let current_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        // Get or create adapter state
        let adapter_state = state.adapters.entry(adapter_name.to_string()).or_insert_with(|| {
            AdapterPersistentState {
                adapter_name: adapter_name.to_string(),
                cumulative_incoming_bytes: 0,
                cumulative_outgoing_bytes: 0,
                cumulative_incoming_packets: 0,
                cumulative_outgoing_packets: 0,
                session_start_time: None,
                last_session_end_time: None,
                was_monitoring_on_exit: false,
                lifetime_incoming_bytes: 0,
                lifetime_outgoing_bytes: 0,
                first_recorded_time: None,
                last_update_time: current_time,
            }
        });
        
        // Apply the update
        updater(adapter_state);
        adapter_state.last_update_time = current_time;
        
        // Update app state
        state.updated_at = current_time;
        
        self.save_state(&state)
    }

    pub fn get_adapter_state(&self, adapter_name: &str) -> Result<Option<AdapterPersistentState>, String> {
        let state = self.load_state()?;
        Ok(state.adapters.get(adapter_name).cloned())
    }

    pub fn mark_clean_shutdown(&self) -> Result<(), String> {
        let mut state = self.load_state()?;
        
        let current_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        state.last_shutdown_time = Some(current_time);
        state.updated_at = current_time;
        
        // Mark all adapters as not monitoring
        for adapter_state in state.adapters.values_mut() {
            adapter_state.was_monitoring_on_exit = false;
            adapter_state.last_session_end_time = Some(current_time);
        }
        
        self.save_state(&state)
    }

    pub fn was_unexpected_shutdown(&self) -> Result<bool, String> {
        let state = self.load_state()?;
        
        // If this is a fresh install (no adapters recorded), it's not unexpected
        if state.adapters.is_empty() {
            return Ok(false);
        }
        
        // If we have a recent clean shutdown timestamp (within last 5 minutes), it's not unexpected
        if let Some(last_shutdown) = state.last_shutdown_time {
            let current_time = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
            
            // If shutdown was recent (within 5 minutes), consider it clean
            if current_time - last_shutdown < 300 {
                return Ok(false);
            }
        }
        
        // If any adapter was marked as monitoring on exit, it was unexpected
        let was_monitoring = state.adapters.values()
            .any(|adapter| adapter.was_monitoring_on_exit);
        
        Ok(was_monitoring)
    }

    pub fn get_all_adapter_states(&self) -> Result<HashMap<String, AdapterPersistentState>, String> {
        let state = self.load_state()?;
        Ok(state.adapters)
    }

    pub fn get_last_shutdown_time(&self) -> Result<u64, String> {
        let state = self.load_state()?;
        Ok(state.last_shutdown_time.unwrap_or(0))
    }
}

// Global instance
use std::sync::OnceLock;
static PERSISTENT_STATE: OnceLock<PersistentStateManager> = OnceLock::new();

pub fn get_persistent_state_manager() -> &'static PersistentStateManager {
    PERSISTENT_STATE.get_or_init(|| {
        PersistentStateManager::new().expect("Failed to initialize persistent state manager")
    })
}
