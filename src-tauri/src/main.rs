// Prevents additional console window on Windows in release, DO NOT REMOVE!!
use winapi::um::winuser::{
    SetWindowsHookExA, CallNextHookEx, WH_KEYBOARD_LL, WH_MOUSE_LL,
    GetMessageA, MSG, WM_KEYDOWN
};
use std::ptr;
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::Mutex;
use lazy_static::lazy_static;
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::Path;
use chrono::{DateTime, Local, TimeZone};
use std::collections::HashMap;

use tauri::tray::TrayIconBuilder;
use tauri::include_image;

static INACTIVE_TIME_PERIOD: u64 = 5;

lazy_static! {
    static ref LAST_TRACKED_INACTIVE_TIME: Mutex<u64> = Mutex::new(0);
	static ref LAST_TRACKED_ACTIVE_START_TIME: Mutex<u64> = Mutex::new(0);
	static ref LAST_TRACKED_ACTIVE_END_TIME: Mutex<u64> = Mutex::new(0);
}

// Define the greet command directly in main.rs
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn sync_time_data() -> String {
    let log_result = aggregate_log_results()
        .expect("Failed to aggregate log results");
    log_result
}

fn setup_hooks() {
    thread::spawn(|| {
        unsafe {
            // Set up keyboard hook
            let kb_hook_id = SetWindowsHookExA(
                WH_KEYBOARD_LL,
                Some(keyboard_hook_callback),
                ptr::null_mut(),
                0
            );
            if kb_hook_id.is_null() {
                println!("Failed to set keyboard hook");
                return;
            }
    
            // Set up mouse hook
            let mouse_hook_id = SetWindowsHookExA(
                WH_MOUSE_LL,
                Some(mouse_hook_callback),
                ptr::null_mut(),
                0
            );
            if mouse_hook_id.is_null() {
                println!("Failed to set mouse hook");
                return;
            }
    
            // Message loop
            let mut msg: MSG = std::mem::zeroed();
            while GetMessageA(&mut msg, ptr::null_mut(), 0, 0) > 0 {
                // Process messages
            }
        }
    });
}

fn main() {
    // Safe to lock mutex here without unsafe
    *LAST_TRACKED_INACTIVE_TIME.lock().unwrap() = get_current_time();
	*LAST_TRACKED_ACTIVE_START_TIME.lock().unwrap() = get_current_time();
	*LAST_TRACKED_ACTIVE_END_TIME.lock().unwrap() = get_current_time();

    // Set up hooks in a background thread
    setup_hooks();

    tauri::Builder::default()
        .setup(|app| {
            let image = include_image!("icons/icon.png");
            let _tray = TrayIconBuilder::new()
                .icon(image)
                .build(app)?;
            Ok(())
        })
        .plugin(tauri_plugin_opener::init()) // Add any plugins you need
        .invoke_handler(tauri::generate_handler![greet, sync_time_data]) // Register the greet command
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn get_current_time() -> u64 {
    let now = SystemTime::now();
    
    match now.duration_since(UNIX_EPOCH) {
        Ok(duration) => duration.as_secs(),
        Err(_e) => 0,
    }
}

fn aggregate_log_results() -> Result<String, Box<dyn std::error::Error>> {
    let log_dir = "fairsight-log";
    if !Path::new(log_dir).exists() {
        println!("No log directory found");
        return Ok("No log files found".to_string());
    }

    // Use HashMap to group active periods by start time
    let mut active_groups: HashMap<i64, i64> = HashMap::new();
    // Vector to store inactive periods
    let mut inactive_periods: Vec<(DateTime<Local>, DateTime<Local>)> = Vec::new();

    // Process all log files
    for entry in fs::read_dir(log_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("txt") {
            let content = fs::read_to_string(&path)?;
            for line in content.lines() {
                let parts: Vec<&str> = line.split(" - ").collect();
                if parts.len() == 2 {
                    if line.starts_with("Active time") {
                        if let (Ok(period_end), Ok(period_start)) = (
                            parts[0].split_whitespace().last().unwrap().parse::<i64>(),
                            parts[1].parse::<i64>()
                        ) {
                            let start = period_start;
                            let end = period_end;
                            // Group by start time, update with max end time
                            active_groups.entry(start)
                                .and_modify(|e| *e = (*e).max(end))
                                .or_insert(end);
                        }
                    } else if line.starts_with("Inactive time") {
                        if let (Ok(period_end), Ok(period_start)) = (
                            parts[0].split_whitespace().last().unwrap().parse::<i64>(),
                            parts[1].parse::<i64>()
                        ) {
                            let start_time = Local.timestamp_opt(period_start, 0).unwrap();
                            let end_time = Local.timestamp_opt(period_end, 0).unwrap();
                            inactive_periods.push((start_time, end_time));
                        }
                    }
                }
            }
        }
    }

    // Combine all events into a single vector
    let mut all_events: Vec<(DateTime<Local>, DateTime<Local>, &str)> = Vec::new();

    // Add active periods from groups
    for (start, max_end) in active_groups {
        let start_time = Local.timestamp_opt(start, 0).unwrap();
        let end_time = Local.timestamp_opt(max_end, 0).unwrap();
        all_events.push((start_time, end_time, "Active"));
    }

    // Add inactive periods
    for (start, end) in inactive_periods {
        all_events.push((start, end, "Inactive"));
    }

    // Sort all events by start time
    all_events.sort_by(|a, b| a.0.cmp(&b.0));

    // Define the current day's boundaries
    let now = Local::now();
    let today = now.date_naive();
    let day_start = Local.from_local_datetime(&today.and_hms_opt(0, 0, 0).unwrap()).unwrap();
    let day_end = Local.from_local_datetime(&today.and_hms_opt(23, 59, 59).unwrap()).unwrap();

    // Filter events to only include those overlapping with today
    let mut today_events: Vec<(DateTime<Local>, DateTime<Local>, &str)> = all_events
        .into_iter()
        .filter(|(start, end, _)| {
            *start <= day_end && *end >= day_start
        })
        .map(|(start, end, event_type)| {
            // Clip events to fit within today's boundaries
            let clipped_start = start.max(day_start);
            let clipped_end = end.min(day_end);
            (clipped_start, clipped_end, event_type)
        })
        .collect();

    // Sort again after clipping (though usually unnecessary due to prior sort)
    today_events.sort_by(|a, b| a.0.cmp(&b.0));

    // Build the complete list with "Not run" periods
    let mut final_events = Vec::new();
    if !today_events.is_empty() {
        // Check from day_start to first event
        if day_start < today_events[0].0 {
            final_events.push((day_start, today_events[0].0, "Not run"));
        }
        // Add the first event
        final_events.push(today_events[0]);
        // Check gaps between events
        for i in 1..today_events.len() {
            if today_events[i - 1].1 < today_events[i].0 {
                final_events.push((today_events[i - 1].1, today_events[i].0, "Not run"));
            }
            final_events.push(today_events[i]);
        }
        // Check from last event to day_end
        if today_events.last().unwrap().1 < day_end {
            final_events.push((today_events.last().unwrap().1, day_end, "Not run"));
        }
    } else {
        // If no events today, entire day is "Not run"
        final_events.push((day_start, day_end, "Not run"));
    }

    // Print the aggregated report for today
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
    /*
    println!("==={} activity log===", now.format("%Y-%m-%d"));
    for (start, end, event_type) in &final_events {
        println!(
            "{}: {} - {}",
            event_type,
            start.format("%H:%M:%S"),
            end.format("%H:%M:%S")
        );
    }
    println!();
*/
    Ok(output)
}

fn update_track_time(current_time: u64) -> io::Result<()> {
    let mut last_tracked_inactive_time = LAST_TRACKED_INACTIVE_TIME.lock().unwrap();
    let mut last_tracked_active_start_time = LAST_TRACKED_ACTIVE_START_TIME.lock().unwrap();
    let mut last_tracked_active_end_time = LAST_TRACKED_ACTIVE_END_TIME.lock().unwrap();

    let log_dir = "fairsight-log";
    if !Path::new(log_dir).exists() {
        fs::create_dir(log_dir)?;
    }

    let current_date = Local::now().format("%Y-%m-%d").to_string();
    let filename = format!("{}/rs-fairsight({}).txt", log_dir, current_date);

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(&filename)?;

    if current_time < *last_tracked_inactive_time {
        let message = "Time Sync error\n";
        file.write_all(message.as_bytes())?;
    } 
    else if current_time - *last_tracked_inactive_time > INACTIVE_TIME_PERIOD {
        let message = format!(
            "Inactive time over 5seconds {} - {}\n", 
            current_time, 
            *last_tracked_inactive_time
        );
        file.write_all(message.as_bytes())?;
        *last_tracked_active_start_time = current_time;
        /* 
        if let Err(e) = aggregate_log_results() {
            println!("Failed to print aggregate results: {}", e);
        }
        */
    } 
    else if *last_tracked_active_end_time != current_time {
        *last_tracked_active_end_time = current_time;
        let message = format!(
            "Active time {} - {}\n", 
            *last_tracked_active_end_time, 
            *last_tracked_active_start_time
        );
        file.write_all(message.as_bytes())?;
        /*
        if let Err(e) = aggregate_log_results() {
            println!("Failed to print aggregate results: {}", e);
        }
        */
    }
    
    *last_tracked_inactive_time = current_time;
    Ok(())
}

unsafe extern "system" fn keyboard_hook_callback(code: i32, w_param: usize, l_param: isize) -> isize {
    if code >= 0 && w_param == WM_KEYDOWN as usize {
        let _ = update_track_time(get_current_time());
    }
	unsafe {
    	CallNextHookEx(ptr::null_mut(), code, w_param, l_param)
	}
}

unsafe extern "system" fn mouse_hook_callback(code: i32, w_param: usize, l_param: isize) -> isize {
    if code >= 0 {
        let _ = update_track_time(get_current_time());
    }
    unsafe {
		CallNextHookEx(ptr::null_mut(), code, w_param, l_param)
	}
}