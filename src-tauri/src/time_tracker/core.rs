use chrono::Local;
use lazy_static::lazy_static;
use std::fs;
use std::io;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::file_utils::save_backup;
use super::types::{INACTIVE_TIME_PERIOD};
use super::file_operations::{get_platform_directories, write_encrypted_message_to_file, should_create_backup, get_current_backup_count};

lazy_static! {
    static ref LAST_TRACKED_INACTIVE_TIME: Mutex<u64> = Mutex::new(0);
    static ref LAST_TRACKED_ACTIVE_START_TIME: Mutex<u64> = Mutex::new(0);
    static ref LAST_TRACKED_ACTIVE_END_TIME: Mutex<u64> = Mutex::new(0);
}

pub fn get_current_time() -> u64 {
    let now = SystemTime::now();

    match now.duration_since(UNIX_EPOCH) {
        Ok(duration) => duration.as_secs(),
        Err(_e) => 0,
    }
}

pub fn initialize_time_tracking() {
    *LAST_TRACKED_INACTIVE_TIME.lock().unwrap() = get_current_time();
    *LAST_TRACKED_ACTIVE_START_TIME.lock().unwrap() = get_current_time();
    *LAST_TRACKED_ACTIVE_END_TIME.lock().unwrap() = get_current_time();
}

pub fn update_track_time(current_time: u64) -> io::Result<()> {
    let mut last_tracked_inactive_time = LAST_TRACKED_INACTIVE_TIME.lock().unwrap();
    let mut last_tracked_active_start_time = LAST_TRACKED_ACTIVE_START_TIME.lock().unwrap();
    let mut last_tracked_active_end_time = LAST_TRACKED_ACTIVE_END_TIME.lock().unwrap();

    // Get platform-specific directories
    let (log_dir, backup_dir) = get_platform_directories()?;

    // Create directory if it doesn't exist
    if !log_dir.exists() {
        fs::create_dir_all(&log_dir)?;
    }

    let current_date = Local::now().format("%Y-%m-%d").to_string();
    let filename = log_dir.join(format!("rs-fairsight({}).txt", current_date));

    let message = if current_time < *last_tracked_inactive_time {
        let msg = "Time Sync error\n".to_string();
        crate::log_warning!("time_tracker", "Time sync error detected");
        msg
    } else if current_time - *last_tracked_inactive_time > INACTIVE_TIME_PERIOD {
        *last_tracked_active_start_time = current_time;
        let msg = format!(
            "Inactive time over 5seconds {} - {}\n",
            current_time,
            *last_tracked_inactive_time
        );
        crate::log_info!("time_tracker", "User became active after {} seconds of inactivity", current_time - *last_tracked_inactive_time);
        msg
    } else if *last_tracked_active_end_time != current_time {
        *last_tracked_active_end_time = current_time;
        format!(
            "Active time {} - {}\n",
            *last_tracked_active_end_time,
            *last_tracked_active_start_time
        )
    } else {
        // No message to write
        *last_tracked_inactive_time = current_time;
        return Ok(());
    };

    // Write the message using our improved atomic write function
    write_encrypted_message_to_file(&filename, &message, Some(&backup_dir))?;

    // Periodic backup (reduced frequency)
    if should_create_backup() {
        let file_name = format!("rs-fairsight({}).txt", current_date);
        let count = get_current_backup_count();
        if let Err(e) = save_backup(&log_dir, &backup_dir, &file_name) {
            crate::log_error!("time_tracker", "Backup failed: {}", e);
        } else {
            crate::log_info!("time_tracker", "Backup created successfully (operation #{})", count);
        }
    }

    *last_tracked_inactive_time = current_time;
    Ok(())
}
