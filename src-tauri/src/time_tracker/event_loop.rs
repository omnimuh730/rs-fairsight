use std::sync::mpsc::Receiver;
use super::types::TimeUpdateMessage;
use super::core::update_track_time;

pub fn event_processing_loop(receiver: Receiver<TimeUpdateMessage>) {
    crate::log_info!("time_tracker", "Starting event processing thread...");
    let mut consecutive_errors = 0;
    const MAX_CONSECUTIVE_ERRORS: usize = 10;
    
    while let Ok(current_time) = receiver.recv() {
        match update_track_time(current_time) {
            Ok(_) => {
                consecutive_errors = 0; // Reset error counter on success
            }
            Err(e) => {
                consecutive_errors += 1;
                crate::log_error!("time_tracker", "Error updating track time (error #{} consecutive): {}", consecutive_errors, e);
                
                if consecutive_errors >= MAX_CONSECUTIVE_ERRORS {
                    crate::log_warning!("time_tracker", "Too many consecutive errors ({}), sleeping for 30 seconds before continuing", MAX_CONSECUTIVE_ERRORS);
                    std::thread::sleep(std::time::Duration::from_secs(30));
                    consecutive_errors = 0; // Reset after sleep
                }
                
                // Try to continue processing despite errors
            }
        }
    }
    crate::log_warning!("time_tracker", "Event processing thread shutting down.");
}
