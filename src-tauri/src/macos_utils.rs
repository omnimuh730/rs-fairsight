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

#[cfg(target_os = "macos")]
pub fn check_bpf_permissions() -> Result<(), String> {
    use std::process::Command;
    use std::fs;
    
    // First check if we can access BPF devices directly
    for i in 0..=255 {
        let bpf_path = format!("/dev/bpf{}", i);
        if fs::metadata(&bpf_path).is_ok() {
            // Try to open the BPF device
            match fs::File::open(&bpf_path) {
                Ok(_) => {
                    println!("✅ Successfully accessed {}", bpf_path);
                    return Ok(());
                }
                Err(e) => {
                    if e.kind() == std::io::ErrorKind::PermissionDenied {
                        continue; // Try next BPF device
                    }
                }
            }
        }
    }
    
    // If direct BPF access fails, check via tcpdump
    match Command::new("tcpdump").arg("-D").output() {
        Ok(output) => {
            if output.status.success() {
                println!("✅ tcpdump has network permissions");
                Ok(())
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                if stderr.contains("permission") || stderr.contains("Operation not permitted") {
                    Err("Network monitoring requires elevated privileges or Developer Tools permission.".to_string())
                } else {
                    Err(format!("tcpdump failed: {}", stderr))
                }
            }
        }
        Err(e) => Err(format!("Failed to run tcpdump: {}. Please ensure tcpdump is installed.", e))
    }
}

#[cfg(target_os = "macos")]
pub fn request_network_permissions() -> Result<(), String> {
    use std::process::Command;
    
    println!("🔐 Requesting network monitoring permissions...");
    
    // Try to open System Preferences to the Privacy & Security > Developer Tools section
    match Command::new("open")
        .arg("x-apple.systempreferences:com.apple.preference.security?Privacy_DeveloperTools")
        .spawn()
    {
        Ok(_) => {
            println!("📱 Opening System Preferences...");
            println!("💡 Please enable Developer Tools access for this application in:");
            println!("   System Preferences → Security & Privacy → Privacy → Developer Tools");
            Ok(())
        }
        Err(e) => {
            println!("⚠️  Could not open System Preferences automatically: {}", e);
            println!("📖 Manual steps:");
            println!("   1. Open System Preferences");
            println!("   2. Go to Security & Privacy → Privacy");
            println!("   3. Select 'Developer Tools' from the list");
            println!("   4. Enable access for this application");
            println!("   5. Restart the application");
            Err("Manual permission setup required".to_string())
        }
    }
}

#[cfg(target_os = "macos")]
pub fn fix_bpf_permissions() -> Result<(), String> {
    use std::process::Command;
    
    println!("🔧 Attempting to fix BPF permissions...");
    
    // Try to fix BPF permissions using sudo (if available)
    match Command::new("sudo")
        .args(&["chmod", "+r", "/dev/bpf*"])
        .output()
    {
        Ok(output) => {
            if output.status.success() {
                println!("✅ BPF permissions updated successfully");
                Ok(())
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                Err(format!("Failed to update BPF permissions: {}", stderr))
            }
        }
        Err(_) => {
            println!("⚠️  sudo not available or permission denied");
            println!("📖 Alternative solutions:");
            println!("   1. Run the application with 'sudo':");
            println!("      sudo ./InnoMonitor");
            println!("   2. Enable Developer Tools access in System Preferences");
            println!("   3. Add your user to the 'admin' group");
            Err("Manual permission fix required".to_string())
        }
    }
}

#[cfg(target_os = "macos")]
pub fn get_permission_instructions() -> String {
    format!(
        "🚨 macOS Network Permission Required\n\
        \n\
        Your application needs permission to monitor network traffic.\n\
        \n\
        💡 Solution Options:\n\
        \n\
        Option 1 - Developer Tools Permission (Recommended):\n\
        1. Open System Preferences → Security & Privacy\n\
        2. Click the Privacy tab\n\
        3. Select 'Developer Tools' from the left sidebar\n\
        4. Check the box next to your application\n\
        5. Restart the application\n\
        \n\
        Option 2 - Run with elevated privileges:\n\
        sudo ./InnoMonitor\n\
        \n\
        Option 3 - Fix BPF device permissions:\n\
        sudo chmod +r /dev/bpf*\n\
        \n\
        ℹ️  This permission is required because network packet capture\n\
        requires low-level system access to network interfaces.\n\
        \n\
        🔒 Privacy Note: InnoMonitor only reads packet headers,\n\
        never the content of your communications."
    )
}
