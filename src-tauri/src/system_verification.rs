#[cfg(target_os = "macos")]
use std::path::Path;

/// Check if libpcap is available and accessible on the system
/// This helps provide better error messages if bundling failed
#[cfg(target_os = "macos")]
pub fn verify_libpcap_availability() -> Result<String, String> {
    // Check if we can find libpcap in common locations
    let search_paths = vec![
        // Bundled path (should be available if post-build worked)
        "../Frameworks/libpcap.dylib",
        "../Frameworks/libpcap.1.dylib", 
        "../Frameworks/libpcap.1.10.5.dylib",
        "../Frameworks/libpcap.1.10.4.dylib",
        // System paths (fallback)
        "/opt/homebrew/lib/libpcap.dylib",
        "/usr/local/lib/libpcap.dylib",
        "/usr/lib/libpcap.dylib",
    ];
    
    for path in &search_paths {
        if Path::new(path).exists() {
            return Ok(format!("libpcap found at: {}", path));
        }
    }
    
    Err("libpcap not found in any expected location".to_string())
}

/// Check if the app has necessary permissions for packet capture
#[cfg(target_os = "macos")]
pub fn verify_macos_permissions() -> Result<String, String> {
    use std::process::Command;
    
    // Check if we can access /dev/bpf* devices
    let bpf_check = Command::new("ls")
        .arg("-la")
        .arg("/dev/bpf*")
        .output();
        
    match bpf_check {
        Ok(output) => {
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                if stdout.contains("bpf") {
                    Ok("BPF devices accessible".to_string())
                } else {
                    Err("No BPF devices found".to_string())
                }
            } else {
                Err("Cannot access BPF devices - may need elevated permissions".to_string())
            }
        }
        Err(e) => Err(format!("Failed to check BPF devices: {}", e))
    }
}

/// Comprehensive system check for network monitoring capability
#[cfg(target_os = "macos")]
pub fn verify_system_requirements() -> Vec<(String, Result<String, String>)> {
    let mut results = Vec::new();
    
    // Check libpcap availability
    results.push((
        "libpcap Library".to_string(),
        verify_libpcap_availability()
    ));
    
    // Check macOS permissions
    results.push((
        "macOS Permissions".to_string(), 
        verify_macos_permissions()
    ));
    
    // Check if we can create a basic pcap capture (dry run)
    results.push((
        "Packet Capture Test".to_string(),
        test_basic_packet_capture()
    ));
    
    results
}

/// Test if we can initialize pcap without actually capturing
#[cfg(target_os = "macos")]
fn test_basic_packet_capture() -> Result<String, String> {
    use pcap::Device;
    
    match Device::list() {
        Ok(devices) => {
            let count = devices.len();
            if count > 0 {
                Ok(format!("Found {} network devices", count))
            } else {
                Err("No network devices found".to_string())
            }
        }
        Err(e) => Err(format!("Failed to list network devices: {}", e))
    }
}

// Windows and other platforms
#[cfg(target_os = "windows")]
pub fn verify_system_requirements() -> Vec<(String, Result<String, String>)> {
    let mut results = Vec::new();
    
    // Check Npcap DLL availability
    results.push((
        "Npcap Library".to_string(),
        verify_npcap_availability()
    ));
    
    // Check Windows permissions
    results.push((
        "Windows Permissions".to_string(),
        verify_windows_permissions()
    ));
    
    // Check if we can create a basic pcap capture (dry run)
    results.push((
        "Packet Capture Test".to_string(),
        test_basic_packet_capture_windows()
    ));
    
    results
}

/// Check if Npcap DLLs are available on Windows
#[cfg(target_os = "windows")]
fn verify_npcap_availability() -> Result<String, String> {
    use std::path::Path;
    
    // Check for bundled DLLs first (should be available if post-build worked)
    let bundled_paths = vec![
        "./libs/wpcap.dll",
        "./libs/Packet.dll",
        "../libs/wpcap.dll",
        "../libs/Packet.dll",
    ];
    
    let mut bundled_found = 0;
    for path in &bundled_paths {
        if Path::new(path).exists() {
            bundled_found += 1;
        }
    }
    
    if bundled_found >= 1 {
        return Ok(format!("Bundled Npcap DLLs found ({} files)", bundled_found));
    }
    
    // Check system-installed Npcap
    let system_paths = vec![
        "C:\\Windows\\System32\\Npcap\\wpcap.dll",
        "C:\\Windows\\SysWOW64\\Npcap\\wpcap.dll",
        "C:\\Program Files\\Npcap\\wpcap.dll",
        "C:\\Program Files (x86)\\Npcap\\wpcap.dll",
    ];
    
    for path in &system_paths {
        if Path::new(path).exists() {
            return Ok(format!("System Npcap found at: {}", path));
        }
    }
    
    Err("Npcap not found. Please install from https://npcap.com/".to_string())
}

/// Check Windows permissions for packet capture
#[cfg(target_os = "windows")]
fn verify_windows_permissions() -> Result<String, String> {
    // On Windows, admin privileges are typically required for raw packet capture
    // We can't easily check this without attempting capture, so we assume it's fine
    Ok("Windows packet capture permissions (admin privileges may be required)".to_string())
}

/// Test if we can initialize pcap on Windows without actually capturing
#[cfg(target_os = "windows")]
fn test_basic_packet_capture_windows() -> Result<String, String> {
    use pcap::Device;
    
    match Device::list() {
        Ok(devices) => {
            let count = devices.len();
            if count > 0 {
                Ok(format!("Found {} network devices", count))
            } else {
                Err("No network devices found".to_string())
            }
        }
        Err(e) => Err(format!("Failed to list network devices: {}", e))
    }
}

#[cfg(not(any(target_os = "macos", target_os = "windows")))]
pub fn verify_system_requirements() -> Vec<(String, Result<String, String>)> {
    vec![
        ("Platform Check".to_string(), Ok("Linux/Other platform - manual setup may be required".to_string()))
    ]
}
