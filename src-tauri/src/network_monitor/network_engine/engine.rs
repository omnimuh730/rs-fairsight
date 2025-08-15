use std::collections::HashMap;
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use std::time::Duration;
use tokio::sync::RwLock;
use tokio::task;

use super::types::AdapterMonitor;
use super::discovery::discover_adapters;
use crate::network_monitor::state_manager::get_state_manager;

pub struct NetworkEngine {
    adapters: Arc<RwLock<HashMap<String, AdapterMonitor>>>,
    is_running: Arc<AtomicBool>,
    discovery_task: Option<task::JoinHandle<()>>,
    monitoring_tasks: Arc<RwLock<HashMap<String, task::JoinHandle<()>>>>,
}

impl NetworkEngine {
    pub fn new() -> Self {
        Self {
            adapters: Arc::new(RwLock::new(HashMap::new())),
            is_running: Arc::new(AtomicBool::new(false)),
            discovery_task: None,
            monitoring_tasks: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn start(&mut self) -> Result<(), String> {
        if self.is_running.load(Ordering::Relaxed) {
            return Ok(());
        }

        self.is_running.store(true, Ordering::Relaxed);
        println!("🚀 Starting Network Engine...");

        // Start adapter discovery
        self.start_adapter_discovery().await?;
        
        // Start initial adapter scan and monitoring
        self.scan_and_start_monitoring().await?;
        
        println!("✅ Network Engine started successfully");
        Ok(())
    }

    pub async fn stop(&mut self) -> Result<(), String> {
        if !self.is_running.load(Ordering::Relaxed) {
            return Ok(());
        }

        println!("🛑 Stopping Network Engine...");
        self.is_running.store(false, Ordering::Relaxed);

        // Stop discovery task
        if let Some(task) = self.discovery_task.take() {
            task.abort();
        }

        // Stop all monitoring tasks
        {
            let mut tasks = self.monitoring_tasks.write().await;
            for (adapter_name, task) in tasks.drain() {
                println!("🛑 Stopping monitoring for: {}", adapter_name);
                task.abort();
            }
        }

        // Mark all adapters as inactive
        {
            let adapters = self.adapters.read().await;
            for (adapter_name, monitor) in adapters.iter() {
                monitor.is_active.store(false, Ordering::Relaxed);
                let _ = get_state_manager().mark_adapter_active(adapter_name, false);
                let _ = get_state_manager().stop_monitoring(adapter_name);
            }
        }

        println!("✅ Network Engine stopped");
        Ok(())
    }

    async fn start_adapter_discovery(&mut self) -> Result<(), String> {
        let adapters = self.adapters.clone();
        let monitoring_tasks = self.monitoring_tasks.clone();
        let is_running = self.is_running.clone();

        self.discovery_task = Some(task::spawn(async move {
            let mut discovery_interval = tokio::time::interval(Duration::from_secs(5));
            
            while is_running.load(Ordering::Relaxed) {
                discovery_interval.tick().await;
                
                if let Err(e) = discover_adapters(&adapters, &monitoring_tasks, &is_running).await {
                    eprintln!("⚠️  Adapter discovery error: {}", e);
                }
            }
        }));

        Ok(())
    }

    async fn scan_and_start_monitoring(&mut self) -> Result<(), String> {
        // Trigger initial discovery
        let adapters = self.adapters.clone();
        let monitoring_tasks = self.monitoring_tasks.clone();
        let is_running = self.is_running.clone();

        discover_adapters(&adapters, &monitoring_tasks, &is_running).await?;
        Ok(())
    }

    pub async fn get_adapter_stats(&self) -> HashMap<String, (u64, u64, u64, u64)> {
        let adapters = self.adapters.read().await;
        let mut stats = HashMap::new();

        for (name, _monitor) in adapters.iter() {
            // Get state manager data for in/out breakdown
            let state = get_state_manager().get_state_snapshot();
            if let Some(adapter_metrics) = state.adapters.get(name) {
                stats.insert(
                    name.clone(),
                    (
                        adapter_metrics.total_bytes_in,
                        adapter_metrics.total_bytes_out,
                        adapter_metrics.total_packets_in,
                        adapter_metrics.total_packets_out,
                    ),
                );
            } else {
                stats.insert(name.clone(), (0, 0, 0, 0));
            }
        }

        stats
    }

    pub async fn get_active_adapters(&self) -> Vec<String> {
        let adapters = self.adapters.read().await;
        adapters
            .iter()
            .filter(|(_, monitor)| monitor.is_active.load(Ordering::Relaxed))
            .map(|(name, _)| name.clone())
            .collect()
    }
}
