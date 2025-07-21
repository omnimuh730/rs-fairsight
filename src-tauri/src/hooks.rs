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

use crate::time_tracker::{get_current_time, EVENT_QUEUE_SENDER};

#[cfg(target_os = "windows")]
use crate::app_state::send_message;

#[cfg(target_os = "windows")]
pub fn setup_hooks() {
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
                send_message("Failed to set keyboard hook".to_string());
                return;
            } else {
                send_message("Successfully set keyboard hook".to_string());
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
                send_message("Failed to set mouse hook".to_string());
                return;
            } else {
                send_message("Successfully set mouse hook".to_string());
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
pub fn setup_hooks() {
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
    let current_time = get_current_time();
    let _ = EVENT_QUEUE_SENDER.lock().unwrap().send(current_time);
}

#[cfg(target_os = "windows")]
unsafe extern "system" fn keyboard_hook_callback(
    code: i32,
    w_param: usize,
    l_param: isize
) -> isize {
    if code >= 0 && w_param == (WM_KEYDOWN as usize) {
        let current_time = get_current_time();
        let _ = EVENT_QUEUE_SENDER.lock().unwrap().send(current_time);
    }
    unsafe { CallNextHookEx(ptr::null_mut(), code, w_param, l_param) }
}

#[cfg(target_os = "windows")]
unsafe extern "system" fn mouse_hook_callback(code: i32, w_param: usize, l_param: isize) -> isize {
    if code >= 0 {
        let current_time = get_current_time();
        let _ = EVENT_QUEUE_SENDER.lock().unwrap().send(current_time);
    }
    unsafe { CallNextHookEx(ptr::null_mut(), code, w_param, l_param) }
}
