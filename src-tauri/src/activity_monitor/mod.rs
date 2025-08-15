use std::sync::mpsc::{self, Sender};
use std::sync::Mutex;
use once_cell::sync::Lazy;

pub mod types;
pub mod core;
pub mod file_operations;
pub mod event_loop;
pub mod aggregation;

// Re-export the main functionality
pub use types::TimeUpdateMessage;
pub use core::{get_current_time, initialize_time_tracking};
pub use aggregation::aggregate_log_results;

// Global event queue sender
pub static EVENT_QUEUE_SENDER: Lazy<Mutex<Sender<TimeUpdateMessage>>> = Lazy::new(|| {
    let (sender, receiver) = mpsc::channel::<TimeUpdateMessage>();

    // Spawn the worker thread
    std::thread::spawn(move || {
        event_loop::event_processing_loop(receiver);
    });

    Mutex::new(sender)
});
