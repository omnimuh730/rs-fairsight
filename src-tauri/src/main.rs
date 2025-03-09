#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
use winapi::um::winuser::{
    SetWindowsHookExA,
    CallNextHookEx,
    WH_KEYBOARD_LL,
    WH_MOUSE_LL,
    GetMessageA,
    MSG,
    WM_KEYDOWN,
};
use std::ptr;
use std::thread;
use std::time::{ SystemTime, UNIX_EPOCH };
use std::sync::Mutex;
use lazy_static::lazy_static;
use std::fs::{ self, OpenOptions, File };
use std::io::{ self, Read, Write };
use std::path::Path;
use chrono::{ DateTime, Local, NaiveDate, TimeZone };
use tauri::{
    tray::{ TrayIconBuilder, TrayIconEvent },
    menu::{ MenuBuilder, MenuItem },
    WindowEvent,
};
use std::collections::HashMap;

use std::process;

use tauri::include_image;
use tauri::{ Manager, AppHandle };

static INACTIVE_TIME_PERIOD: u64 = 30;

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
fn sync_time_data(report_date: &str) -> String {
    let log_result = aggregate_log_results(report_date).expect("Failed to aggregate log results");
    log_result
}

#[tauri::command]
fn aggregate_week_activity_logs(data_list: Vec<String>) -> Vec<String> {
    let mut logdb_list = Vec::with_capacity(data_list.len());

    for (_i, s) in data_list.into_iter().enumerate() {
        let styled = format!("rs-fairsight({}).txt", s); // styled is a String
        let result = aggregate_log_results(&styled) // Call aggregate_log_results with &str
            .unwrap_or_else(|e| format!("Error aggregating {}: {}", styled, e)); // Convert Err to String
        logdb_list.push(result); // Push the String (success or error message)
    }
    // Print the entire logdb_list
    //    println!("Debug: logdb_list = {:?}", logdb_list);
    logdb_list
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

// Check if a process with the given PID is running (platform-specific)
fn is_process_running(pid: u32) -> bool {
    #[cfg(target_os = "windows")]
    {
        use std::process::Command;
        let output = Command::new("tasklist")
            .arg("/FI")
            .arg(format!("PID eq {}", pid))
            .output()
            .expect("Failed to execute tasklist");
        output.stdout.windows(pid.to_string().len()).any(|w| w == pid.to_string().as_bytes())
    }
}

// Ensure only one instance is running using a lock file with PID
fn ensure_single_instance(app_handle: &AppHandle) -> Result<std::path::PathBuf, String> {
    let lock_file_path = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data directory: {}", e))?
        .join("app.lock");

    // Ensure the parent directory exists
    if let Some(parent_dir) = lock_file_path.parent() {
        if !parent_dir.exists() {
            std::fs
                ::create_dir_all(parent_dir)
                .map_err(|e| format!("Failed to create directory for lock file: {}", e))?;
        }
    }

    let my_pid = process::id();

    if Path::new(&lock_file_path).exists() {
        let mut file = File::open(&lock_file_path).map_err(|e|
            format!("Failed to open lock file: {}", e)
        )?;
        let mut contents = String::new();
        file.read_to_string(&mut contents).map_err(|e| format!("Failed to read lock file: {}", e))?;
        if let Ok(existing_pid) = contents.trim().parse::<u32>() {
            if is_process_running(existing_pid) {
                return Err(format!("Another instance (PID: {}) is already running.", existing_pid));
            } else {
                println!("Found stale lock file from PID {}, removing it.", existing_pid);
                fs
                    ::remove_file(&lock_file_path)
                    .map_err(|e| format!("Failed to remove stale lock file: {}", e))?;
            }
        }
    }

    let mut file = File::create(&lock_file_path).map_err(|e|
        format!("Failed to create lock file: {}", e)
    )?;
    write!(file, "{}", my_pid).map_err(|e| format!("Failed to write PID to lock file: {}", e))?;

    Ok(lock_file_path)
}
fn main() {
    // Safe to lock mutex here without unsafe
    *LAST_TRACKED_INACTIVE_TIME.lock().unwrap() = get_current_time();
    *LAST_TRACKED_ACTIVE_START_TIME.lock().unwrap() = get_current_time();
    *LAST_TRACKED_ACTIVE_END_TIME.lock().unwrap() = get_current_time();

    // Set up hooks in a background thread
    setup_hooks();

    tauri::Builder
        ::default()
        .setup(|app| {
            let app_handle = app.handle().clone();

            // Ensure single instance; exit if another is running
            let _lock_file_path = match ensure_single_instance(&app_handle) {
                Ok(path) => path,
                Err(e) => {
                    println!("{}", e);
                    std::process::exit(1);
                }
            };

            let quit = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let hide = MenuItem::with_id(app, "hide", "Hide", true, None::<&str>)?;
            let show = MenuItem::with_id(app, "show", "Show", true, None::<&str>)?;

            let menu = MenuBuilder::new(app).item(&show).item(&hide).item(&quit).build()?;

            let _tray = TrayIconBuilder::with_id("main_tray")
                .icon(include_image!("icons/icon.png"))
                .menu(&menu)
                .on_tray_icon_event(|_tray, event| {
                    if let TrayIconEvent::Click { button, .. } = event {
                        if button == tauri::tray::MouseButton::Left {
                        }
                    }
                })
                .on_menu_event(|app, event| {
                    match event.id().as_ref() {
                        "quit" => {
                            std::process::exit(0);
                        }
                        "hide" => {
                            if let Some(window) = app.get_webview_window("main") {
                                window.hide().unwrap();
                            }
                        }
                        "show" => {
                            if let Some(window) = app.get_webview_window("main") {
                                window.show().unwrap();
                                window.set_focus().unwrap();
                            }
                        }
                        _ => {}
                    }
                })
                .build(app)?;
            Ok(())
        })
        .on_window_event(|app, event| {
            match event {
                WindowEvent::CloseRequested { api, .. } => {
                    if let Some(window) = app.get_webview_window("main") {
                        window.hide().unwrap();
                        api.prevent_close();
                    }
                }
                WindowEvent::Destroyed => {
                    // Clean up lock file when the last window is destroyed
                    if let Ok(lock_file_path) = ensure_single_instance(&app.app_handle()) {
                        let _ = fs::remove_file(&lock_file_path);
                    }
                }
                _ => {}
            }
        })
        .plugin(tauri_plugin_opener::init()) // Add any plugins you need
        .invoke_handler(
            tauri::generate_handler![greet, sync_time_data, aggregate_week_activity_logs]
        ) // Register the greet command
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

fn aggregate_log_results(file_name: &str) -> Result<String, Box<dyn std::error::Error>> {
    let log_dir = "fairsight-log";
    let file_path = Path::new(log_dir).join(&file_name);

    if !Path::new(log_dir).exists() {
        println!("No log directory found");
        return Ok("No log files found".to_string());
    }

    if !file_path.exists() {
        return Ok(format!("No log file found for {}", file_name));
    }

    // Extract date from filename (e.g., "rs-fairsight(2025-03-03).txt")
    let date_str = file_name
        .strip_prefix("rs-fairsight(")
        .and_then(|s| s.strip_suffix(").txt"))
        .ok_or("Invalid filename format")?;
    let target_date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d")?;

    // Define the day's boundaries using the date from filename
    let day_start = Local.from_local_datetime(&target_date.and_hms_opt(0, 0, 0).unwrap()).unwrap();
    let day_end = Local.from_local_datetime(&target_date.and_hms_opt(23, 59, 59).unwrap()).unwrap();

    // Rest of the existing logic remains largely the same
    let mut active_groups: HashMap<i64, i64> = HashMap::new();
    let mut inactive_periods: Vec<(DateTime<Local>, DateTime<Local>)> = Vec::new();

    let content = fs::read_to_string(&file_path)?;

    for line in content.lines() {
        let parts: Vec<&str> = line.split(" - ").collect();
        if parts.len() == 2 {
            if line.starts_with("Active time") {
                if
                    let (Ok(period_end), Ok(period_start)) = (
                        parts[0].split_whitespace().last().unwrap().parse::<i64>(),
                        parts[1].parse::<i64>(),
                    )
                {
                    let start = period_start;
                    let end = period_end;
                    active_groups
                        .entry(start)
                        .and_modify(|e| {
                            *e = (*e).max(end);
                        })
                        .or_insert(end);
                }
            } else if line.starts_with("Inactive time") {
                if
                    let (Ok(period_end), Ok(period_start)) = (
                        parts[0].split_whitespace().last().unwrap().parse::<i64>(),
                        parts[1].parse::<i64>(),
                    )
                {
                    let start_time = Local.timestamp_opt(period_start, 0).unwrap();
                    let end_time = Local.timestamp_opt(period_end, 0).unwrap();
                    inactive_periods.push((start_time, end_time));
                }
            }
        }
    }

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

    // Filter events to only include those overlapping with the target date
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
        output.push_str(
            &format!("{}: {} - {}\n", event_type, start.format("%H:%M:%S"), end.format("%H:%M:%S"))
        );
    }
    output.push('\n');

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

    let mut file = OpenOptions::new().write(true).append(true).create(true).open(&filename)?;

    if current_time < *last_tracked_inactive_time {
        let message = "Time Sync error\n";
        file.write_all(message.as_bytes())?;
    } else if current_time - *last_tracked_inactive_time > INACTIVE_TIME_PERIOD {
        let message = format!(
            "Inactive time over 5seconds {} - {}\n",
            current_time,
            *last_tracked_inactive_time
        );
        file.write_all(message.as_bytes())?;
        *last_tracked_active_start_time = current_time;
    } else if *last_tracked_active_end_time != current_time {
        *last_tracked_active_end_time = current_time;
        let message = format!(
            "Active time {} - {}\n",
            *last_tracked_active_end_time,
            *last_tracked_active_start_time
        );
        file.write_all(message.as_bytes())?;
    }

    *last_tracked_inactive_time = current_time;
    Ok(())
}

unsafe extern "system" fn keyboard_hook_callback(
    code: i32,
    w_param: usize,
    l_param: isize
) -> isize {
    if code >= 0 && w_param == (WM_KEYDOWN as usize) {
        let _ = update_track_time(get_current_time());
    }
    unsafe { CallNextHookEx(ptr::null_mut(), code, w_param, l_param) }
}

unsafe extern "system" fn mouse_hook_callback(code: i32, w_param: usize, l_param: isize) -> isize {
    if code >= 0 {
        let _ = update_track_time(get_current_time());
    }
    unsafe { CallNextHookEx(ptr::null_mut(), code, w_param, l_param) }
}
