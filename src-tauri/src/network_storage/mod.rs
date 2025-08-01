pub mod types;
pub mod utils;
pub mod backup;
pub mod consolidation;
pub mod file_ops;
pub mod manager;

// Re-export main functionality
pub use types::{NetworkSession, DailyNetworkSummary};
pub use manager::NetworkStorageManager;

// Global storage manager instance
lazy_static::lazy_static! {
    pub static ref NETWORK_STORAGE: NetworkStorageManager = {
        NetworkStorageManager::new().expect("Failed to initialize network storage")
    };
}
