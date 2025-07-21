#[cfg(target_os = "windows")]
use std::ptr;
#[cfg(target_os = "windows")]
use std::thread;
#[cfg(target_os = "windows")]
use std::time::Duration;
#[cfg(target_os = "windows")]
use winapi::um::winuser::{
    CallNextHookEx,
    GetMessageA,
    SetWindowsHookExA,
    UnhookWindowsHookEx,
    MSG,
    WH_KEYBOARD_LL,
    WH_MOUSE_LL,
    WM_KEYDOWN,
};

#[cfg(target_os = "macos")]
use core_foundation::base::{kCFAllocatorDefault, TCFType};
#[cfg(target_os = "macos")]
use core_foundation::runloop::{CFRunLoop, CFRunLoopSource};
#[cfg(target_os = "macos")]
use core_foundation::mach_port::{CFMachPortRef, CFMachPortCreateRunLoopSource};
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
#[cfg(target_os = "macos")]
use std::sync::atomic::{AtomicBool, Ordering};
#[cfg(target_os = "macos")]
use std::time::Duration;

use crate::time_tracker::{get_current_time, EVENT_QUEUE_SENDER};
use crate::health_monitor::report_activity;

#[cfg(target_os = "windows")]
use crate::app_state::send_message;

#[cfg(target_os = "macos")]
static EVENT_TAP_RUNNING: AtomicBool = AtomicBool::new(false);

#[cfg(target_os = "windows")]
pub fn setup_hooks() {
    thread::spawn(|| {
        let max_retries = 5;
        let mut retry_count = 0;
        
        while retry_count < max_retries {
            match setup_windows_hooks_inner() {
                Ok(_) => {
                    println!("Windows hooks setup successfully");
                    break;
                }
                Err(e) => {
                    retry_count += 1;
                    eprintln!("Failed to setup Windows hooks (attempt {}): {}", retry_count, e);
                    send_message(format!("Hook setup failed (attempt {}): {}", retry_count, e));
                    
                    if retry_count < max_retries {
                        thread::sleep(Duration::from_secs(2));
                    }
                }
            }
        }
        
        if retry_count >= max_retries {
            eprintln!("Failed to setup Windows hooks after {} attempts", max_retries);
            send_message("Critical: Failed to setup Windows hooks after multiple attempts".to_string());
        }
    });
}

#[cfg(target_os = "windows")]
fn setup_windows_hooks_inner() -> Result<(), String> {
    unsafe {
        // Set up keyboard hook
        let kb_hook_id = SetWindowsHookExA(
            WH_KEYBOARD_LL,
            Some(keyboard_hook_callback),
            ptr::null_mut(),
            0
        );
        if kb_hook_id.is_null() {
            return Err("Failed to set keyboard hook".to_string());
        }

        // Set up mouse hook
        let mouse_hook_id = SetWindowsHookExA(
            WH_MOUSE_LL,
            Some(mouse_hook_callback),
            ptr::null_mut(),
            0
        );
        if mouse_hook_id.is_null() {
            UnhookWindowsHookEx(kb_hook_id);
            return Err("Failed to set mouse hook".to_string());
        }

        send_message("Successfully set both keyboard and mouse hooks".to_string());

        // Message loop with error handling
        let mut msg: MSG = std::mem::zeroed();
        loop {
            let result = GetMessageA(&mut msg, ptr::null_mut(), 0, 0);
            if result == -1 {
                // Error occurred
                UnhookWindowsHookEx(kb_hook_id);
                UnhookWindowsHookEx(mouse_hook_id);
                return Err("Message loop error".to_string());
            } else if result == 0 {
                // WM_QUIT received
                break;
            }
            // Continue processing messages
        }
        
        // Cleanup
        UnhookWindowsHookEx(kb_hook_id);
        UnhookWindowsHookEx(mouse_hook_id);
        Ok(())
    }
}

#[cfg(target_os = "macos")]
pub fn setup_hooks() {
    std::thread::spawn(|| {
        let max_retries = 3;
        let mut retry_count = 0;
        
        while retry_count < max_retries {
            EVENT_TAP_RUNNING.store(true, Ordering::SeqCst);
            
            match setup_macos_hooks_inner() {
                Ok(_) => {
                    println!("macOS event tap completed normally");
                    break;
                }
                Err(e) => {
                    retry_count += 1;
                    eprintln!("macOS event tap failed (attempt {}): {}", retry_count, e);
                    
                    if retry_count < max_retries {
                        println!("Retrying event tap setup in 3 seconds...");
                        std::thread::sleep(Duration::from_secs(3));
                    }
                }
            }
            
            EVENT_TAP_RUNNING.store(false, Ordering::SeqCst);
        }
        
        if retry_count >= max_retries {
            eprintln!("Failed to setup macOS event tap after {} attempts", max_retries);
        }
    });
    
    // Start a monitoring thread to restart the event tap if it stops
    std::thread::spawn(|| {
        loop {
            std::thread::sleep(Duration::from_secs(30)); // Check every 30 seconds
            
            if !EVENT_TAP_RUNNING.load(Ordering::SeqCst) {
                println!("Event tap not running, attempting restart...");
                setup_hooks(); // Recursive call to restart
                break; // Exit this monitoring thread as a new one will be started
            }
        }
    });
}

#[cfg(target_os = "macos")]
fn setup_macos_hooks_inner() -> Result<(), String> {
    unsafe {
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
        ).map_err(|e| format!("Failed to create event tap: {:?}", e))?;

        // Check if the event tap is enabled
        if !event_tap.is_enabled() {
            return Err("Event tap is not enabled (accessibility permissions may be required)".to_string());
        }

        // Get the raw mach port pointer using as_concrete_TypeRef
        let mach_port: CFMachPortRef = event_tap.mach_port.as_concrete_TypeRef();

        // Create a run loop source from the mach port
        let raw_source = CFMachPortCreateRunLoopSource(kCFAllocatorDefault, mach_port, 0);
        if raw_source.is_null() {
            return Err("Failed to create run loop source".to_string());
        }
        let source = CFRunLoopSource::wrap_under_create_rule(raw_source);

        // Add the source to the current run loop and run it
        let current_runloop = CFRunLoop::get_current();
        current_runloop.add_source(&source, core_foundation::runloop::kCFRunLoopCommonModes);
        
        println!("Starting macOS event tap run loop...");
        CFRunLoop::run_current();
        
        println!("macOS event tap run loop stopped");
        Ok(())
    }
}

#[cfg(target_os = "macos")]
fn event_callback(
    _proxy: CGEventTapProxy,
    event_type: CGEventType,
    event: &CGEvent
) -> Option<CGEvent> {
    // Reset the running flag to indicate the event tap is active
    EVENT_TAP_RUNNING.store(true, Ordering::SeqCst);
    
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
    let current_time = get_current_time();
    report_activity(); // Report to health monitor
    if let Err(e) = EVENT_QUEUE_SENDER.lock().unwrap().send(current_time) {
        eprintln!("Failed to send event to queue: {}", e);
    }
}

#[cfg(target_os = "windows")]
unsafe extern "system" fn keyboard_hook_callback(
    code: i32,
    w_param: usize,
    l_param: isize
) -> isize {
    if code >= 0 && w_param == (WM_KEYDOWN as usize) {
        let current_time = get_current_time();
        report_activity(); // Report to health monitor
        if let Err(e) = EVENT_QUEUE_SENDER.lock().unwrap().send(current_time) {
            eprintln!("Failed to send keyboard event to queue: {}", e);
        }
    }
    CallNextHookEx(ptr::null_mut(), code, w_param, l_param)
}

#[cfg(target_os = "windows")]
unsafe extern "system" fn mouse_hook_callback(code: i32, w_param: usize, l_param: isize) -> isize {
    if code >= 0 {
        let current_time = get_current_time();
        report_activity(); // Report to health monitor
        if let Err(e) = EVENT_QUEUE_SENDER.lock().unwrap().send(current_time) {
            eprintln!("Failed to send mouse event to queue: {}", e);
        }
    }
    CallNextHookEx(ptr::null_mut(), code, w_param, l_param)
}
