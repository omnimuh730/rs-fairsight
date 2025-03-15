#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
use chrono::{ DateTime, Local, NaiveDate, TimeZone };
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fs::{ self, OpenOptions };
use std::io::{ self, Write };
use std::path::Path;
use std::sync::Mutex;
use std::time::{ SystemTime, UNIX_EPOCH };
use tauri::{
    menu::{ MenuBuilder, MenuItem },
    tray::{ TrayIconBuilder, TrayIconEvent },
    WindowEvent,
};
use ring::aead::{ Aad, LessSafeKey, Nonce, UnboundKey, AES_256_GCM };
use ring::error::Unspecified;
use ring::rand::{ SecureRandom, SystemRandom };
#[cfg(target_os = "windows")]
use std::ptr;

#[cfg(target_os = "windows")]
use std::thread;

#[cfg(target_os = "windows")]
use winapi::um::winuser::{
    CallNextHookEx,
    GetMessageA,
    SetWindowsHookExA,
    MSG,
    WH_KEYBOARD_LL,
    WH_MOUSE_LL,
    WM_KEYDOWN,
};

#[cfg(target_os = "macos")]
use core_foundation::base::{ kCFAllocatorDefault, TCFType };
#[cfg(target_os = "macos")]
use core_foundation::runloop::{ CFRunLoop, CFRunLoopSource };
#[cfg(target_os = "macos")]
use core_foundation::mach_port::{ CFMachPortRef, CFMachPortCreateRunLoopSource };
#[cfg(target_os = "macos")]
use core_graphics::event::{
    CGEvent,
    CGEventTap,
    CGEventTapProxy,
    CGEventType,
    CGEventTapLocation,
    CGEventTapPlacement,
    CGEventTapOptions,
};

use tauri::include_image;
use tauri::{ Manager };
use tauri_plugin_autostart::{ MacosLauncher, ManagerExt };

static INACTIVE_TIME_PERIOD: u64 = 30;

lazy_static! {
    static ref LAST_TRACKED_INACTIVE_TIME: Mutex<u64> = Mutex::new(0);
    static ref LAST_TRACKED_ACTIVE_START_TIME: Mutex<u64> = Mutex::new(0);
    static ref LAST_TRACKED_ACTIVE_END_TIME: Mutex<u64> = Mutex::new(0);
}

const KEY: [u8; 32] = [0x42; 32]; // Replace with a securely generated key

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
#[cfg(target_os = "windows")]
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

#[cfg(target_os = "macos")]
fn setup_hooks() {
    std::thread::spawn(|| unsafe {
        // Create the event tap
        let event_tap = CGEventTap::new(
            CGEventTapLocation::HID,
            CGEventTapPlacement::HeadInsertEventTap,
            CGEventTapOptions::Default,
            vec![
                CGEventType::KeyDown,
                CGEventType::MouseMoved,
                CGEventType::LeftMouseDown,
                CGEventType::RightMouseDown,
                CGEventType::OtherMouseDown,
                CGEventType::ScrollWheel
            ],
            event_callback
        ).expect("Failed to create event tap");

        // Get the raw mach port pointer using as_concrete_TypeRef
        let mach_port: CFMachPortRef = event_tap.mach_port.as_concrete_TypeRef();

        // Create a run loop source from the mach port
        let raw_source = CFMachPortCreateRunLoopSource(kCFAllocatorDefault, mach_port, 0);
        if raw_source.is_null() {
            panic!("Failed to create run loop source");
        }
        let source = CFRunLoopSource::wrap_under_create_rule(raw_source);

        // Add the source to the current run loop and run it
        let current_runloop = CFRunLoop::get_current();
        current_runloop.add_source(&source, core_foundation::runloop::kCFRunLoopCommonModes);
        CFRunLoop::run_current();
    });
}

#[cfg(target_os = "macos")]
fn event_callback(
    _proxy: CGEventTapProxy,
    event_type: CGEventType,
    event: &CGEvent
) -> Option<CGEvent> {
    match event_type {
        CGEventType::KeyDown => {
            activity_handler();
        }
        CGEventType::MouseMoved => {
            activity_handler();
        }
        CGEventType::LeftMouseDown => {
            activity_handler();
        }
        CGEventType::RightMouseDown => {
            activity_handler();
        }
        CGEventType::OtherMouseDown => {
            activity_handler();
        }
        CGEventType::ScrollWheel => {
            activity_handler();
        }
        _ => {}
    }
    Some(event.clone())
}
#[cfg(target_os = "macos")]
fn activity_handler() {
    let _ = update_track_time(get_current_time());
}
fn main() {
    let mut builder = tauri::Builder::default();

    #[cfg(desktop)]
    {
        builder = builder.plugin(
            tauri_plugin_single_instance::init(|app, _args, _cwd| {
                let _ = app.get_webview_window("main").expect("no main window").set_focus();
            })
        );
    }
    // Safe to lock mutex here without unsafe
    *LAST_TRACKED_INACTIVE_TIME.lock().unwrap() = get_current_time();
    *LAST_TRACKED_ACTIVE_START_TIME.lock().unwrap() = get_current_time();
    *LAST_TRACKED_ACTIVE_END_TIME.lock().unwrap() = get_current_time();

    // Set up hooks in a background thread
    setup_hooks();

    builder
        .plugin(
            tauri_plugin_autostart::init(
                MacosLauncher::LaunchAgent, // macOS autostart method
                None // No additional args
            )
        )
        .setup(|app| {
            // Automatically enable autostart on first run (optional)
            app.autolaunch().enable().expect("Failed to enable autostart");

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

    // Read raw bytes since encrypted data isn't plain text
    let content = fs::read(&file_path)?;
    let mut offset = 0;

    while offset < content.len() {
        if content.len() - offset < 12 {
            break; // Not enough bytes for nonce
        }

        // Read nonce (12 bytes)
        let nonce_bytes: [u8; 12] = content[offset..offset + 12].try_into()?;
        offset += 12;

        // Find the next line boundary (assuming encrypted lines are separated somehow, e.g., by length)
        // For simplicity, assume we know encrypted data length or use a delimiter (not implemented here)
        let remaining = &content[offset..];
        let encrypted_len = remaining.len().min(128); // Adjust based on max expected encrypted size
        let mut encrypted_data = remaining[..encrypted_len].to_vec();
        offset += encrypted_len;

        // Decrypt the line
        let decrypted_line = decrypt_string(&mut encrypted_data, &KEY, nonce_bytes).map_err(|e|
            format!("Decryption failed: {:?}", e)
        )?;

        let parts: Vec<&str> = decrypted_line.split(" - ").collect();

        if parts.len() == 2 {
			if decrypted_line.starts_with("Active time") {
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
            } else if decrypted_line.starts_with("Inactive time") {
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
        let (encrypted_data, nonce) = encrypt_string(message, &KEY).map_err(|_| 
            io::Error::new(io::ErrorKind::Other, "Encryption failed")
        )?;
        file.write_all(&nonce)?; // Write nonce first
        file.write_all(&encrypted_data)?; // Then encrypted data
    } else if current_time - *last_tracked_inactive_time > INACTIVE_TIME_PERIOD {
        let message = format!(
            "Inactive time over 5seconds {} - {}\n",
            current_time,
            *last_tracked_inactive_time
        );
        let (encrypted_data, nonce) = encrypt_string(&message, &KEY).map_err(|_| 
            io::Error::new(io::ErrorKind::Other, "Encryption failed")
        )?;
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
        let (encrypted_data, nonce) = encrypt_string(&message, &KEY).map_err(|_| 
            io::Error::new(io::ErrorKind::Other, "Encryption failed")
        )?;
        file.write_all(&nonce)?;
        file.write_all(&encrypted_data)?;
    }

    *last_tracked_inactive_time = current_time;
    Ok(())
}
#[cfg(target_os = "windows")]
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
#[cfg(target_os = "windows")]
unsafe extern "system" fn mouse_hook_callback(code: i32, w_param: usize, l_param: isize) -> isize {
    if code >= 0 {
        let _ = update_track_time(get_current_time());
    }
    unsafe { CallNextHookEx(ptr::null_mut(), code, w_param, l_param) }
}

// Encrypt a string, returning the encrypted bytes and nonce
fn encrypt_string(
    plaintext: &str,
    key_bytes: &[u8; 32]
) -> Result<(Vec<u8>, [u8; 12]), Unspecified> {
    let rng = SystemRandom::new();
    let mut nonce_bytes = [0u8; 12];
    rng.fill(&mut nonce_bytes)?;

    let unbound_key = UnboundKey::new(&AES_256_GCM, key_bytes)?;
    let key = LessSafeKey::new(unbound_key);
    let nonce = Nonce::assume_unique_for_key(nonce_bytes);

    let mut data = plaintext.as_bytes().to_vec();
    key.seal_in_place_append_tag(nonce, Aad::empty(), &mut data)?;

    Ok((data, nonce_bytes))
}

// Decrypt a string, given the encrypted bytes, key, and nonce
// Decrypt a string from encrypted bytes and nonce
fn decrypt_string(
    encrypted_data: &mut Vec<u8>,
    key_bytes: &[u8; 32],
    nonce_bytes: [u8; 12]
) -> Result<String, Unspecified> {
    let unbound_key = UnboundKey::new(&AES_256_GCM, key_bytes)?;
    let key = LessSafeKey::new(unbound_key);
    let nonce = Nonce::assume_unique_for_key(nonce_bytes);

    let decrypted_data = key.open_in_place(nonce, Aad::empty(), encrypted_data)?;
    String::from_utf8(decrypted_data.to_vec()).map_err(|_| Unspecified)
}
