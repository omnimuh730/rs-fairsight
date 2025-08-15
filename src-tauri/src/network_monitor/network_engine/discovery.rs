use std::collections::{HashMap, HashSet};
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use pcap::Device;
use tokio::sync::RwLock;
use tokio::task;

use super::types::AdapterMonitor;
use super::monitoring::start_adapter_monitoring;
use crate::state_manager::get_state_manager;

pub async fn discover_adapters(
    adapters: &Arc<RwLock<HashMap<String, AdapterMonitor>>>,
    monitoring_tasks: &Arc<RwLock<HashMap<String, task::JoinHandle<()>>>>,
    is_running: &Arc<AtomicBool>,
) -> Result<(), String> {
    let devices = Device::list().map_err(|e| format!("Failed to list devices: {}", e))?;
    let mut current_adapters = adapters.write().await;
    let mut current_tasks = monitoring_tasks.write().await;

    // Track which adapters we've seen
    let mut seen_adapters = HashSet::new();

    for device in devices {
        if device.name.is_empty() || device.name.contains("bluetooth") || device.name.contains("any") {
            continue;
        }

        seen_adapters.insert(device.name.clone());

        if !current_adapters.contains_key(&device.name) {
            // New adapter discovered
            let display_name = device.desc.unwrap_or_else(|| device.name.clone());
            let monitor = AdapterMonitor {
                name: device.name.clone(),
                display_name: display_name.clone(),
                is_active: Arc::new(AtomicBool::new(true)),
                packets_processed: Arc::new(std::sync::atomic::AtomicU64::new(0)),
                bytes_processed: Arc::new(std::sync::atomic::AtomicU64::new(0)),
            };

            println!("🔌 Discovered new adapter: {} ({})", device.name, display_name);
            
            // Update state manager
            let _ = get_state_manager().mark_adapter_active(&device.name, true);
            
            // Start monitoring for this adapter
            let monitoring_task = start_adapter_monitoring(
                device.name.clone(),
                monitor.is_active.clone(),
                monitor.packets_processed.clone(),
                monitor.bytes_processed.clone(),
                is_running.clone(),
            ).await;

            if let Ok(task) = monitoring_task {
                current_tasks.insert(device.name.clone(), task);
                let _ = get_state_manager().start_monitoring(&device.name);
                println!("✅ Started monitoring: {}", device.name);
            }

            current_adapters.insert(device.name.clone(), monitor);
        }
    }

    // Remove adapters that are no longer present
    let mut to_remove = Vec::new();
    for adapter_name in current_adapters.keys() {
        if !seen_adapters.contains(adapter_name) {
            to_remove.push(adapter_name.clone());
        }
    }

    for adapter_name in to_remove {
        println!("🔌 Adapter disconnected: {}", adapter_name);
        
        // Stop monitoring task
        if let Some(task) = current_tasks.remove(&adapter_name) {
            task.abort();
        }
        
        // Update state
        if let Some(monitor) = current_adapters.remove(&adapter_name) {
            monitor.is_active.store(false, Ordering::Relaxed);
            let _ = get_state_manager().mark_adapter_active(&adapter_name, false);
            let _ = get_state_manager().stop_monitoring(&adapter_name);
        }
    }

    Ok(())
}
