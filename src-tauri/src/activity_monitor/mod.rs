pub use self::{
    aggregation::aggregate_log_results,
    core::{get_current_time, initialize_time_tracking},
    types::TimeUpdateMessage,
};
use once_cell::sync::Lazy;
use std::sync::{
    mpsc::{self, Sender},
    Mutex,
};

pub mod aggregation;
pub mod core;
pub mod event_loop;
pub mod file_operations;
pub mod types;

// Global event queue sender
pub static EVENT_QUEUE_SENDER: Lazy<Mutex<Sender<TimeUpdateMessage>>> = Lazy::new(|| {
    let (tx, rx) = mpsc::channel::<TimeUpdateMessage>();

    // Spawn the worker thread
    std::thread::spawn(move || {
        event_loop::event_processing_loop(rx);
    });

    Mutex::new(tx)
});
