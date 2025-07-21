#[cfg(target_os = "macos")]
use cocoa::{
    base::YES,
    appkit::{NSApp, NSApplication, NSApplicationActivationPolicy},
};

#[cfg(target_os = "macos")]
pub fn set_activation_policy(policy: NSApplicationActivationPolicy) {
    unsafe {
        let ns_app = NSApp();
        ns_app.setActivationPolicy_(policy);
    }
}

#[cfg(target_os = "macos")]
pub fn activate_app() {
    unsafe {
        let ns_app = NSApp();
        ns_app.activateIgnoringOtherApps_(YES);
    }
}
