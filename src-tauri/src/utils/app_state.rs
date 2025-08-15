use tauri::AppHandle;
use std::sync::Mutex;
use once_cell::sync::Lazy;

#[cfg(target_os = "windows")]
use tauri::Emitter;

static APP_HANDLE: Lazy<Mutex<Option<AppHandle>>> = Lazy::new(|| Mutex::new(None));

pub fn set_app_handle(handle: &AppHandle) {
    let mut app_handle = APP_HANDLE.lock().unwrap();
    *app_handle = Some(handle.clone());
}

#[cfg(target_os = "windows")]
pub fn get_app_handle() -> Option<AppHandle> {
    let app_handle = APP_HANDLE.lock().unwrap();
    app_handle.clone()
}

#[cfg(target_os = "windows")]
pub fn send_message(msg: String) {
    if let Some(handle) = get_app_handle() {
        handle.emit("my-event", msg).unwrap();
    }
}
