fn main() {
    use std::env;
    
    // Set up npcap library path for Windows with bundling support
    #[cfg(target_os = "windows")]
    {
        use std::path::Path;
        use std::fs;
        
        println!("cargo:rerun-if-env-changed=LIBPCAP_LIBDIR");
        println!("cargo:rerun-if-env-changed=PCAP_LIBDIR");
        println!("cargo:rerun-if-env-changed=NPCAP_SDK_LIB");
        
        // Try to find npcap-sdk in various locations, prioritizing environment variables and x64 paths
        let possible_paths = [
            // GitHub Actions / CI environment variables (x64 priority)
            std::env::var("LIBPCAP_LIBDIR").unwrap_or_default(),
            std::env::var("PCAP_LIBDIR").unwrap_or_default(),
            std::env::var("NPCAP_SDK_LIB").unwrap_or_default(),
            // User's installed Npcap SDK path (x64 priority)
            "C:\\npcap-sdk\\Lib\\x64".to_string(),
            "C:\\npcap-sdk\\Lib".to_string(),
            // Temporary directory for GitHub Actions (x64 priority)
            format!("{}\\npcap-sdk\\Lib\\x64", std::env::var("TEMP").unwrap_or_default()),
            // Local development paths (x64 priority)
            "C:\\Users\\Administrator\\Downloads\\npcap-sdk-1.15\\Lib\\x64".to_string(),
            // x64 system paths
            "C:\\Windows\\System32\\Npcap".to_string(),
            "C:\\Program Files\\Npcap".to_string(),
            // Fallback to non-x64 paths only if x64 not found
            format!("{}\\npcap-sdk\\Lib", std::env::var("TEMP").unwrap_or_default()),
            "C:\\Users\\Administrator\\Downloads\\npcap-sdk-1.15\\Lib".to_string(),
            "C:\\npcap-sdk\\Lib".to_string(),
            "C:\\Windows\\SysWOW64\\Npcap".to_string(),
            "C:\\Program Files (x86)\\Npcap".to_string(),
        ];

        let mut lib_path_found = false;
//        let mut npcap_lib_path = None;
        let mut wpcap_dll_path = None;
        
        // Find the library path - prioritize x64 libraries
        for path in &possible_paths {
            if !path.is_empty() && Path::new(path).exists() {
                // Check if this path contains x64 libraries (prefer x64 for 64-bit builds)
                let wpcap_lib = Path::new(path).join("wpcap.lib");
                if wpcap_lib.exists() {
                    // For 64-bit builds, skip non-x64 paths if we haven't checked x64 paths yet
                    if cfg!(target_arch = "x86_64") && !path.contains("x64") {
                        // Check if there's an x64 version available
                        let potential_x64_path = if path.ends_with("\\Lib") {
                            format!("{}\\x64", path)
                        } else {
                            format!("{}\\x64", path)
                        };
                        
                        let x64_wpcap_lib = Path::new(&potential_x64_path).join("wpcap.lib");
                        if x64_wpcap_lib.exists() {
                            // Use the x64 version instead
                            println!("cargo:rustc-link-search=native={}", potential_x64_path);
                            println!("cargo:warning=Found npcap library at: {} (using x64 version)", potential_x64_path);
                            lib_path_found = true;
//                            npcap_lib_path = Some(potential_x64_path);
                            break;
                        } else {
                            println!("cargo:warning=WARNING: Using non-x64 library path for 64-bit build: {}", path);
                            println!("cargo:warning=This may cause architecture mismatch errors!");
                        }
                    }
                    
                    println!("cargo:rustc-link-search=native={}", path);
                    println!("cargo:warning=Found npcap library at: {}", path);
                    
                    // Set include path for pcap headers
                    let include_path = if path.contains("\\Lib\\x64") {
                        path.replace("\\Lib\\x64", "\\Include")
                    } else if path.contains("\\Lib") {
                        path.replace("\\Lib", "\\Include")
                    } else {
                        format!("{}\\..\\Include", path)
                    };
                    
                    if Path::new(&include_path).exists() {
                        println!("cargo:rustc-env=PCAP_INCLUDE_DIR={}", include_path);
                        println!("cargo:warning=Found npcap headers at: {}", include_path);
                    }
                    
                    lib_path_found = true;
                    break;
                }
            }
        }

        // Look for wpcap.dll in runtime locations for bundling
        let dll_search_paths = [
            // User's Npcap SDK installation (may contain runtime DLLs)
            "C:\\npcap-sdk\\Lib\\x64\\wpcap.dll",
            "C:\\npcap-sdk\\Lib\\wpcap.dll",
            // System installed Npcap runtime
            "C:\\Windows\\System32\\Npcap\\wpcap.dll",
            "C:\\Windows\\SysWOW64\\Npcap\\wpcap.dll", 
            "C:\\Program Files\\Npcap\\wpcap.dll",
            "C:\\Program Files (x86)\\Npcap\\wpcap.dll",
        ];
        
        for dll_path in &dll_search_paths {
            if Path::new(dll_path).exists() {
                wpcap_dll_path = Some(dll_path.to_string());
                println!("cargo:warning=Found wpcap.dll at: {}", dll_path);
                break;
            }
        }

        if !lib_path_found {
            println!("cargo:warning=npcap-sdk not found in any of the following locations:");
            for path in &possible_paths {
                if !path.is_empty() {
                    println!("cargo:warning=  - {}", path);
                }
            }
            println!("cargo:warning=Please install Npcap SDK from: https://npcap.com/#download");
            println!("cargo:warning=Or set NPCAP_SDK_LIB environment variable");
        }
        
        // Prepare for Windows app bundle copying (only during release builds)
        if let Ok(profile) = env::var("PROFILE") {
            if profile == "release" {
                if let Some(dll_path) = wpcap_dll_path {
                    // Copy wpcap.dll to a temporary location for bundling
                    let out_dir = env::var("OUT_DIR").unwrap();
                    let temp_npcap_dir = format!("{}\\npcap_bundle", out_dir);
                    
                    if let Err(e) = fs::create_dir_all(&temp_npcap_dir) {
                        println!("cargo:warning=Failed to create temp npcap directory: {}", e);
                    } else {
                        let dest_path = format!("{}\\wpcap.dll", temp_npcap_dir);
                        if let Err(e) = fs::copy(&dll_path, &dest_path) {
                            println!("cargo:warning=Failed to copy wpcap.dll for bundling: {}", e);
                        } else {
                            println!("cargo:warning=Prepared wpcap.dll for app bundle: {} -> {}", dll_path, dest_path);
                            // Store the paths for post-build processing
                            println!("cargo:rustc-env=WPCAP_SOURCE_PATH={}", dll_path);
                            println!("cargo:rustc-env=WPCAP_BUNDLE_PATH={}", dest_path);
                        }
                    }
                    
                    // Also look for Packet.dll (required by wpcap.dll)
                    let packet_dll_path = dll_path.replace("wpcap.dll", "Packet.dll");
                    if Path::new(&packet_dll_path).exists() {
                        let dest_packet_path = format!("{}\\Packet.dll", temp_npcap_dir);
                        if let Err(e) = fs::copy(&packet_dll_path, &dest_packet_path) {
                            println!("cargo:warning=Failed to copy Packet.dll: {}", e);
                        } else {
                            println!("cargo:warning=Prepared Packet.dll for app bundle: {} -> {}", packet_dll_path, dest_packet_path);
                        }
                    }
                }
            }
        }
        
        // Link the required libraries
        println!("cargo:rustc-link-lib=wpcap");
        println!("cargo:rustc-link-lib=ws2_32");
        println!("cargo:rustc-link-lib=iphlpapi");
    }

    // Handle macOS-specific libpcap configuration with bundling support
    #[cfg(target_os = "macos")]
    {
        use std::path::Path;
        use std::fs;
        
        println!("cargo:rerun-if-env-changed=LIBPCAP_LIBDIR");
        println!("cargo:rerun-if-env-changed=PKG_CONFIG_PATH");
        
        // Set up libpcap paths for macOS builds
        if let Ok(libpcap_dir) = env::var("LIBPCAP_LIBDIR") {
            println!("cargo:rustc-link-search=native={}", libpcap_dir);
        }
        
        if let Ok(pkg_config_path) = env::var("PKG_CONFIG_PATH") {
            println!("cargo:rustc-env=PKG_CONFIG_PATH={}", pkg_config_path);
        }
        
        // Find libpcap and prepare for bundling
        let libpcap_search_paths = [
            // Homebrew paths (Apple Silicon)
            "/opt/homebrew/lib/libpcap.dylib",
            "/opt/homebrew/Cellar/libpcap/1.10.5/lib/libpcap.1.10.5.dylib",
            "/opt/homebrew/Cellar/libpcap/1.10.4/lib/libpcap.1.10.4.dylib",
            // Homebrew paths (Intel)
            "/usr/local/lib/libpcap.dylib", 
            "/usr/local/Cellar/libpcap/1.10.5/lib/libpcap.1.10.5.dylib",
            "/usr/local/Cellar/libpcap/1.10.4/lib/libpcap.1.10.4.dylib",
            // System paths
            "/usr/lib/libpcap.dylib",
            "/System/Library/Frameworks/CoreServices.framework/Versions/A/Frameworks/libpcap.dylib"
        ];
        
        let mut libpcap_source_path = None;
        let mut libpcap_lib_dir = None;
        
        // Find the best available libpcap
        for path in &libpcap_search_paths {
            if Path::new(path).exists() {
                libpcap_source_path = Some(path.to_string());
                libpcap_lib_dir = Some(Path::new(path).parent().unwrap().to_string_lossy().to_string());
                println!("cargo:warning=Found libpcap at: {}", path);
                break;
            }
        }
        
        match (libpcap_source_path, libpcap_lib_dir) {
            (Some(source_path), Some(lib_dir)) => {
                // Add the library directory to the link search path
                println!("cargo:rustc-link-search=native={}", lib_dir);
                println!("cargo:rustc-link-lib=pcap");
                
                // Prepare for app bundle copying (only during release builds)
                if let Ok(profile) = env::var("PROFILE") {
                    if profile == "release" {
                        // Copy libpcap to a temporary location that we can reference later
                        let out_dir = env::var("OUT_DIR").unwrap();
                        let temp_libpcap_dir = format!("{}/libpcap_bundle", out_dir);
                        
                        if let Err(e) = fs::create_dir_all(&temp_libpcap_dir) {
                            println!("cargo:warning=Failed to create temp libpcap directory: {}", e);
                        } else {
                            let dest_path = format!("{}/libpcap.dylib", temp_libpcap_dir);
                            if let Err(e) = fs::copy(&source_path, &dest_path) {
                                println!("cargo:warning=Failed to copy libpcap for bundling: {}", e);
                            } else {
                                println!("cargo:warning=Prepared libpcap for app bundle: {} -> {}", source_path, dest_path);
                                // Store the paths for post-build processing
                                println!("cargo:rustc-env=LIBPCAP_SOURCE_PATH={}", source_path);
                                println!("cargo:rustc-env=LIBPCAP_BUNDLE_PATH={}", dest_path);
                            }
                        }
                    }
                }
            }
            (None, None) => {
                println!("cargo:warning=libpcap not found in any standard location!");
                println!("cargo:warning=Please install libpcap: brew install libpcap");
                println!("cargo:warning=Or set LIBPCAP_LIBDIR environment variable");
                
                // Try to continue with system linking
                println!("cargo:rustc-link-lib=pcap");
            }
            _ => unreachable!()
        }
        
        // Additional macOS framework dependencies for network monitoring
        println!("cargo:rustc-link-lib=framework=CoreFoundation");
        println!("cargo:rustc-link-lib=framework=SystemConfiguration");
    }

    tauri_build::build()
}
