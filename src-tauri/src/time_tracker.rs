use chrono::{DateTime, Local, NaiveDate, TimeZone};
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::sync::mpsc::{self, Sender, Receiver};
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};
use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};
use once_cell::sync::Lazy;

#[cfg(target_os = "macos")]
use dirs;

use crate::encryption::{encrypt_string, decrypt_string, KEY};
use crate::file_utils::save_backup;

static INACTIVE_TIME_PERIOD: u64 = 300;
static BACKUP_COUNTER: AtomicUsize = AtomicUsize::new(0);

// Define the type of message to send (just the timestamp)
pub type TimeUpdateMessage = u64;

pub static EVENT_QUEUE_SENDER: Lazy<Mutex<Sender<TimeUpdateMessage>>> = Lazy::new(|| {
    let (sender, receiver) = mpsc::channel::<TimeUpdateMessage>();

    // Spawn the worker thread
    std::thread::spawn(move || {
        event_processing_loop(receiver);
    });

    Mutex::new(sender)
});

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

fn update_track_time(current_time: u64) -> io::Result<()> {
    let mut last_tracked_inactive_time = LAST_TRACKED_INACTIVE_TIME.lock().unwrap();
    let mut last_tracked_active_start_time = LAST_TRACKED_ACTIVE_START_TIME.lock().unwrap();
    let mut last_tracked_active_end_time = LAST_TRACKED_ACTIVE_END_TIME.lock().unwrap();

    // Get the Documents directory path based on OS
    let log_dir;
    #[cfg(target_os = "macos")]
    {
        let home_dir = dirs::home_dir()
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Could not find home directory"))?;
        log_dir = home_dir.join("Documents").join("rs-fairsight");
    }
    #[cfg(target_os = "windows")]
    {
        log_dir = Path::new("C:\\fairsight-log").to_path_buf();
    }

    // Create directory if it doesn't exist
    if !log_dir.exists() {
        fs::create_dir_all(&log_dir)?;
    }

    let current_date = Local::now().format("%Y-%m-%d").to_string();
    let filename = log_dir.join(format!("rs-fairsight({}).txt", current_date));
    let mut file = OpenOptions::new().write(true).append(true).create(true).open(&filename)?;

    if current_time < *last_tracked_inactive_time {
        let message = "Time Sync error\n";
        println!("message: {}", message);
        let (encrypted_data, nonce) = encrypt_string(message, &KEY)
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "Encryption failed"))?;
        file.write_all(&nonce)?; // Write nonce (12 bytes)
        file.write_all(&encrypted_data)?; // Write length + encrypted data
    } else if current_time - *last_tracked_inactive_time > INACTIVE_TIME_PERIOD {
        let message = format!(
            "Inactive time over 5seconds {} - {}\n",
            current_time,
            *last_tracked_inactive_time
        );
        println!("message: {}", message);
        let (encrypted_data, nonce) = encrypt_string(&message, &KEY)
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "Encryption failed"))?;
        file.write_all(&nonce)?;
        file.write_all(&encrypted_data)?;
        *last_tracked_active_start_time = current_time;
    } else if *last_tracked_active_end_time != current_time {
        *last_tracked_active_end_time = current_time;
        let message = format!(
            "Active time {} - {}\n",
            *last_tracked_active_end_time,
            *last_tracked_active_start_time
        );
        println!("message: {}", message);
        let (encrypted_data, nonce) = encrypt_string(&message, &KEY)
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "Encryption failed"))?;
        file.write_all(&nonce)?;
        file.write_all(&encrypted_data)?;
    }

    // After writing to the log file (at the end of the function):
    let count = BACKUP_COUNTER.fetch_add(1, Ordering::SeqCst) + 1;
    if count % 50 == 0 {
        #[cfg(target_os = "windows")]
        {
            let log_dir = Path::new("C:\\fairsight-log");
            let backup_dir = Path::new("C:\\fairsight-backup");
            let current_date = Local::now().format("%Y-%m-%d").to_string();
            let file_name = format!("rs-fairsight({}).txt", current_date);
            let _ = save_backup(log_dir, backup_dir, &file_name);
        }
        #[cfg(target_os = "macos")]
        {
            let home_dir = dirs::home_dir()
                .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Could not find home directory"))?;
            let log_dir = home_dir.join("Documents").join("rs-fairsight");
            let backup_dir = home_dir.join("Documents").join("rs-fairsight-backup");
            let current_date = Local::now().format("%Y-%m-%d").to_string();
            let file_name = format!("rs-fairsight({}).txt", current_date);
            let _ = save_backup(&log_dir, &backup_dir, &file_name);
        }
    }

    *last_tracked_inactive_time = current_time;
    Ok(())
}

pub fn aggregate_log_results(file_name: &str) -> Result<String, Box<dyn std::error::Error>> {
    let log_dir;
    #[cfg(target_os = "macos")]
    {
        let home_dir = dirs::home_dir()
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Could not find home directory"))?;
        log_dir = home_dir.join("Documents").join("rs-fairsight");
    }
    #[cfg(target_os = "windows")]
    {
        log_dir = Path::new("C:\\fairsight-log").to_path_buf();
    }

    let file_path = log_dir.join(&file_name);

    if !log_dir.exists() {
        println!("No log directory found");
        return Ok("No log files found".to_string());
    }

    if !file_path.exists() {
        return Ok(format!("No log file found for {}", file_name));
    }

    let date_str = file_name
        .strip_prefix("rs-fairsight(")
        .and_then(|s| s.strip_suffix(").txt"))
        .ok_or("Invalid filename format")?;
    let target_date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d")?;

    let day_start = Local.from_local_datetime(&target_date.and_hms_opt(0, 0, 0).unwrap()).unwrap();
    let day_end = Local.from_local_datetime(&target_date.and_hms_opt(23, 59, 59).unwrap()).unwrap();

    let mut active_groups: HashMap<i64, i64> = HashMap::new();
    let mut inactive_periods: Vec<(DateTime<Local>, DateTime<Local>)> = Vec::new();

    let content = fs::read(&file_path)?;
    let mut offset = 0;

    while offset < content.len() {
        if content.len() - offset < 12 + 4 {
            break; // Not enough bytes for nonce (12) + length (4)
        }

        // Read nonce (12 bytes)
        let nonce_bytes: [u8; 12] = content[offset..offset + 12].try_into()?;
        offset += 12;

        // Read length (4 bytes)
        let len_bytes: [u8; 4] = content[offset..offset + 4].try_into()?;
        let encrypted_len = u32::from_le_bytes(len_bytes) as usize;
        offset += 4;

        if content.len() - offset < encrypted_len {
            break; // Not enough data for encrypted content
        }

        let mut encrypted_data = content[offset..offset + encrypted_len].to_vec();
        offset += encrypted_len;

        // Decrypt the line
        let decrypted_line = decrypt_string(&mut encrypted_data, &KEY, nonce_bytes)
            .map_err(|e| format!("Decryption failed: {:?}", e))?;

        let parts: Vec<&str> = decrypted_line.split(" - ").collect();

        if parts.len() == 2 {
            if decrypted_line.starts_with("Active time") {
                let end_str = parts[0].split_whitespace().last().unwrap();
                let start_str = parts[1].trim();

                let end_result = end_str.parse::<i64>();
                let start_result = start_str.parse::<i64>();

                match (&end_result, &start_result) {
                    (Ok(period_end), Ok(period_start)) => {
                        let start = *period_start;
                        let end = *period_end;
                        active_groups
                            .entry(start)
                            .and_modify(|e| {
                                *e = (*e).max(end);
                            })
                            .or_insert(end);
                    }
                    _ => {
                        println!("Failed to parse: end={:?}, start={:?}", end_result, start_result);
                    }
                }
            } else if decrypted_line.starts_with("Inactive time") {
                let end_str = parts[0].split_whitespace().last().unwrap();
                let start_str = parts[1].trim();

                match (end_str.parse::<i64>(), start_str.parse::<i64>()) {
                    (Ok(period_end), Ok(period_start)) => {
                        let start_time = Local.timestamp_opt(period_start, 0).unwrap();
                        let end_time = Local.timestamp_opt(period_end, 0).unwrap();
                        inactive_periods.push((start_time, end_time));
                    }
                    _ => {
                        println!(
                            "Failed to parse inactive time: end='{}', start='{}'",
                            end_str,
                            start_str
                        );
                    }
                }
            }
        }
    }

    // Process events and generate output
    let mut all_events: Vec<(DateTime<Local>, DateTime<Local>, &str)> = Vec::new();
    for (start, max_end) in active_groups {
        let start_time = Local.timestamp_opt(start, 0).unwrap();
        let end_time = Local.timestamp_opt(max_end, 0).unwrap();
        all_events.push((start_time, end_time, "Active"));
    }
    for (start, end) in inactive_periods {
        all_events.push((start, end, "Inactive"));
    }
    all_events.sort_by(|a, b| a.0.cmp(&b.0));

    let mut target_events: Vec<(DateTime<Local>, DateTime<Local>, &str)> = all_events
        .into_iter()
        .filter(|(start, end, _)| *start <= day_end && *end >= day_start)
        .map(|(start, end, event_type)| {
            let clipped_start = start.max(day_start);
            let clipped_end = end.min(day_end);
            (clipped_start, clipped_end, event_type)
        })
        .collect();

    target_events.sort_by(|a, b| a.0.cmp(&b.0));

    let mut final_events = Vec::new();
    if !target_events.is_empty() {
        if day_start < target_events[0].0 {
            final_events.push((day_start, target_events[0].0, "Not run"));
        }
        final_events.push(target_events[0]);
        for i in 1..target_events.len() {
            if target_events[i - 1].1 < target_events[i].0 {
                final_events.push((target_events[i - 1].1, target_events[i].0, "Not run"));
            }
            final_events.push(target_events[i]);
        }
        if target_events.last().unwrap().1 < day_end {
            final_events.push((target_events.last().unwrap().1, day_end, "Not run"));
        }
    } else {
        final_events.push((day_start, day_end, "Not run"));
    }

    let mut output = String::new();
    for (start, end, event_type) in &final_events {
        output.push_str(&format!(
            "{}: {} - {}\n",
            event_type,
            start.format("%H:%M:%S"),
            end.format("%H:%M:%S")
        ));
    }
    output.push('\n');

    Ok(output)
}

fn event_processing_loop(receiver: Receiver<TimeUpdateMessage>) {
    println!("Starting event processing thread...");
    while let Ok(current_time) = receiver.recv() {
        if let Err(e) = update_track_time(current_time) {
            eprintln!("Error updating track time: {}", e);
        }
    }
    println!("Event processing thread shutting down.");
}
