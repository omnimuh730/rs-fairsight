use tauri::{
    menu::{MenuBuilder, MenuItem},
    tray::{TrayIconBuilder, TrayIconEvent},
    WindowEvent,
    Manager,
};
use tauri::include_image;
use tauri_plugin_autostart::ManagerExt;

#[cfg(target_os = "macos")]
use crate::macos_utils::{set_activation_policy, activate_app};
#[cfg(target_os = "macos")]
use cocoa::appkit::NSApplicationActivationPolicy;

use crate::app_state::set_app_handle;

pub fn setup_tray_and_window_events(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    set_app_handle(app.handle());
    
    // Enable autostart with better error handling
    if let Err(e) = app.autolaunch().enable() {
        crate::log_warning!("ui_setup", "Failed to enable autostart: {}", e);
        // Continue execution instead of panicking - autostart is not critical for core functionality
    } else {
        crate::log_info!("ui_setup", "Autostart enabled successfully");
    }

    let quit = MenuItem::with_id(app.handle(), "quit", "Quit", true, None::<&str>)?;
    let hide = MenuItem::with_id(app.handle(), "hide", "Hide Window", true, None::<&str>)?;
    let show = MenuItem::with_id(app.handle(), "show", "Show Window", true, None::<&str>)?;

    let menu = MenuBuilder::new(app.handle()).item(&show).item(&hide).item(&quit).build()?;

    let _tray = TrayIconBuilder::with_id("main_tray")
        .icon(include_image!("icons/icon.png"))
        .tooltip("Your App Name")
        .menu(&menu)
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click { button, .. } = event {
                if button == tauri::tray::MouseButton::Left {
                    let app_handle = tray.app_handle();
                    if let Some(window) = app_handle.get_webview_window("main") {
                        match window.is_visible() {
                            Ok(true) => {
                                window.hide().unwrap();
                                #[cfg(target_os = "macos")]
                                {
                                    set_activation_policy(NSApplicationActivationPolicy::NSApplicationActivationPolicyAccessory);
                                }
                            }
                            Ok(false) => {
                                #[cfg(target_os = "macos")]
                                {
                                    set_activation_policy(NSApplicationActivationPolicy::NSApplicationActivationPolicyRegular);
                                    activate_app();
                                }
                                window.show().unwrap();
                                window.set_focus().unwrap();
                            }
                            Err(e) => {
                                eprintln!("Error checking window visibility: {}", e);
                            }
                        }
                    }
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
                        #[cfg(target_os = "macos")]
                        {
                            set_activation_policy(NSApplicationActivationPolicy::NSApplicationActivationPolicyAccessory);
                        }
                    }
                }
                "show" => {
                    if let Some(window) = app.get_webview_window("main") {
                        #[cfg(target_os = "macos")]
                        {
                            set_activation_policy(NSApplicationActivationPolicy::NSApplicationActivationPolicyRegular);
                            activate_app();
                        }
                        window.show().unwrap();
                        window.set_focus().unwrap();
                    }
                }
                _ => {}
            }
        })
        .build(app.handle())?;

    Ok(())
}

pub fn handle_window_event(window: &tauri::Window, event: &WindowEvent) {
    match event {
        WindowEvent::CloseRequested { api, .. } => {
            if let Some(main_window) = window.app_handle().get_webview_window("main") {
                println!("Close requested for main window, hiding.");
                main_window.hide().unwrap_or_else(|e| eprintln!("Error hiding window: {}", e));

                #[cfg(target_os = "macos")]
                {
                    println!("Setting activation policy to Accessory");
                    set_activation_policy(NSApplicationActivationPolicy::NSApplicationActivationPolicyAccessory);
                }

                api.prevent_close();
            } else {
                eprintln!("Close Requested, but 'main' window not found.");
            }
        }
        WindowEvent::Focused(focused) => {
            if *focused {
                if let Some(main_window) = window.app_handle().get_webview_window("main") {
                    if main_window.is_visible().unwrap_or(false) {
                        #[cfg(target_os = "macos")]
                        {
                            println!("Window focused, ensuring Regular activation policy.");
                            set_activation_policy(NSApplicationActivationPolicy::NSApplicationActivationPolicyRegular);
                        }
                    }
                }
            }
        }
        _ => {}
    }
}
