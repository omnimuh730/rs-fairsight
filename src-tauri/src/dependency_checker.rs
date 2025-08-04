use std::path::Path;
use std::fs;
use serde_json::{json, Value};

/// Check if required dependencies are available at runtime
pub fn check_runtime_dependencies() -> Result<Value, String> {
    let mut results = json!({
        "dependencies_available": false,
        "libpcap": {
            "found": false,
            "path": null,
            "version": null
        },
        "recommended_action": null
    });
    
    // Define search paths for libpcap (in order of preference)
    let libpcap_search_paths = [
        // Standardized InnoMonitor dependency location (highest priority)
        "/usr/local/lib/innomonitor/libpcap.dylib",
        // Homebrew keg-only locations (Apple Silicon)
        "/opt/homebrew/opt/libpcap/lib/libpcap.dylib",
        // Homebrew keg-only locations (Intel)
        "/usr/local/opt/libpcap/lib/libpcap.dylib",
        // Standard Homebrew paths (fallback)
        "/opt/homebrew/lib/libpcap.dylib",
        "/usr/local/lib/libpcap.dylib",
        // System paths
        "/usr/lib/libpcap.dylib",
    ];
    
    // Try to find libpcap
    for path in &libpcap_search_paths {
        if Path::new(path).exists() {
            results["libpcap"]["found"] = json!(true);
            results["libpcap"]["path"] = json!(path);
            
            // Try to get version info if available
            if let Some(version) = get_libpcap_version(path) {
                results["libpcap"]["version"] = json!(version);
            }
            
            results["dependencies_available"] = json!(true);
            break;
        }
    }
    
    // Set recommended action based on findings
    if !results["libpcap"]["found"].as_bool().unwrap_or(false) {
        results["recommended_action"] = json!({
            "action": "install_dependencies",
            "message": "libpcap not found. Please run the dependency installer.",
            "installer_script": "install-macos-deps.sh",
            "manual_install": "brew install libpcap"
        });
    } else {
        results["recommended_action"] = json!({
            "action": "none",
            "message": "All dependencies are available."
        });
    }
    
    Ok(results)
}

/// Try to get libpcap version information
fn get_libpcap_version(libpcap_path: &str) -> Option<String> {
    // Try to read version from dependency info file first
    if libpcap_path.contains("/usr/local/lib/innomonitor/") {
        let info_file = "/usr/local/lib/innomonitor/dependency-info.json";
        if let Ok(content) = fs::read_to_string(info_file) {
            if let Ok(info) = serde_json::from_str::<Value>(&content) {
                if let Some(version) = info["libpcap_version"].as_str() {
                    return Some(version.to_string());
                }
            }
        }
    }
    
    // Fallback: try to use pkg-config if available
    if let Ok(output) = std::process::Command::new("pkg-config")
        .args(&["--modversion", "libpcap"])
        .output() {
        if output.status.success() {
            let version = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !version.is_empty() {
                return Some(version);
            }
        }
    }
    
    None
}

/// Get a user-friendly dependency status message
pub fn get_dependency_status_message() -> String {
    match check_runtime_dependencies() {
        Ok(status) => {
            if status["dependencies_available"].as_bool().unwrap_or(false) {
                if let Some(path) = status["libpcap"]["path"].as_str() {
                    if let Some(version) = status["libpcap"]["version"].as_str() {
                        format!("✅ Dependencies OK - libpcap {} found at {}", version, path)
                    } else {
                        format!("✅ Dependencies OK - libpcap found at {}", path)
                    }
                } else {
                    "✅ Dependencies OK".to_string()
                }
            } else {
                "❌ Dependencies missing - run install-macos-deps.sh".to_string()
            }
        }
        Err(e) => {
            format!("⚠️ Dependency check failed: {}", e)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_dependency_check() {
        let result = check_runtime_dependencies();
        assert!(result.is_ok());
        
        let status = result.unwrap();
        assert!(status.is_object());
        assert!(status.get("dependencies_available").is_some());
        assert!(status.get("libpcap").is_some());
    }
}
