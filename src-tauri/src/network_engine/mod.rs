use std::sync::OnceLock;

pub mod types;
pub mod packet_capture;
pub mod discovery;
pub mod monitoring;
pub mod engine;

// Re-export main functionality
pub use types::{PacketInfo, AdapterMonitor};
pub use engine::NetworkEngine;

// Global network engine instance
static NETWORK_ENGINE: OnceLock<tokio::sync::Mutex<NetworkEngine>> = OnceLock::new();

pub async fn get_network_engine() -> &'static tokio::sync::Mutex<NetworkEngine> {
    NETWORK_ENGINE.get_or_init(|| tokio::sync::Mutex::new(NetworkEngine::new()))
}

pub async fn start_network_engine() -> Result<(), String> {
    let engine = get_network_engine().await;
    let mut engine_guard = engine.lock().await;
    engine_guard.start().await
}

pub async fn stop_network_engine() -> Result<(), String> {
    let engine = get_network_engine().await;
    let mut engine_guard = engine.lock().await;
    engine_guard.stop().await
}
