use std::sync::atomic::AtomicUsize;

// Define the type of message to send (just the timestamp)
pub type TimeUpdateMessage = u64;

// Constants
pub static INACTIVE_TIME_PERIOD: u64 = 300;
pub static BACKUP_COUNTER: AtomicUsize = AtomicUsize::new(0);
pub static BACKUP_FREQUENCY: usize = 10; // Backup every 10 operations instead of 50
