fn main() {
    use std::env;
    
    // Set up npcap library path for Windows
    #[cfg(target_os = "windows")]
    {
        println!("cargo:rerun-if-env-changed=LIBPCAP_LIBDIR");
        println!("cargo:rerun-if-env-changed=PCAP_LIBDIR");
        println!("cargo:rerun-if-env-changed=NPCAP_SDK_LIB");
        
        // Try to find npcap-sdk in various locations, prioritizing environment variables
        let possible_paths = [
            // GitHub Actions / CI environment variables
            std::env::var("LIBPCAP_LIBDIR").unwrap_or_default(),
            std::env::var("PCAP_LIBDIR").unwrap_or_default(),
            std::env::var("NPCAP_SDK_LIB").unwrap_or_default(),
            // Temporary directory for GitHub Actions
            format!("{}\\npcap-sdk\\Lib\\x64", std::env::var("TEMP").unwrap_or_default()),
            format!("{}\\npcap-sdk\\Lib", std::env::var("TEMP").unwrap_or_default()),
            // Local development paths
            "C:\\Users\\Administrator\\Downloads\\npcap-sdk-1.15\\Lib\\x64".to_string(),
            "C:\\Users\\Administrator\\Downloads\\npcap-sdk-1.15\\Lib".to_string(),
            "C:\\npcap-sdk\\Lib\\x64".to_string(),
            "C:\\npcap-sdk\\Lib".to_string(),
        ];

        let mut lib_path_found = false;
        for path in &possible_paths {
            if !path.is_empty() && std::path::Path::new(path).exists() {
                println!("cargo:rustc-link-search=native={}", path);
                println!("cargo:warning=Found npcap library at: {}", path);
                lib_path_found = true;
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
            println!("cargo:warning=Please set LIBPCAP_LIBDIR or NPCAP_SDK_LIB environment variable or install npcap-sdk.");
        }
        
        // Also try to link the wpcap library explicitly
        println!("cargo:rustc-link-lib=wpcap");
        println!("cargo:rustc-link-lib=ws2_32");
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
