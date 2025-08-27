fn main() {
    use std::env;
    
    // Set up npcap library path for Windows with bundling support
    #[cfg(target_os = "windows")]
    {
        use std::path::Path;

        let npcap_lib_path = "C:\\Program Files\\Npcap\\Lib";
        let wpcap_lib = Path::new(npcap_lib_path).join("wpcap.lib");

        if wpcap_lib.exists() {
            println!("cargo:rustc-link-search=native={}", npcap_lib_path);
            println!("cargo:warning=Found npcap library at: {}", npcap_lib_path);
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

            let include_path = npcap_lib_path.replace("\\Lib", "\\Include");
            if Path::new(&include_path).exists() {
                println!("cargo:rustc-env=PCAP_INCLUDE_DIR={}", include_path);
                println!("cargo:warning=Found npcap headers at: {}", include_path);
            } else {
                println!("cargo:warning=Npcap Include directory not found at: {}", include_path);
            }
        } else {
            println!("cargo:warning=wpcap.lib not found at the expected location: {}", npcap_lib_path);
            println!("cargo:warning=Please ensure Npcap SDK is installed to C:\\Program Files\\Npcap");
            println!("cargo:warning=or adjust the build.rs script if installed elsewhere.");
        }

        // Assume Npcap DLLs are available in system paths for runtime
        // No bundling logic here based on the request.

        // Link the required libraries
        println!("cargo:rustc-link-lib=wpcap");
        println!("cargo:rustc-link-lib=ws2_32");
        println!("cargo:rustc-link-lib=iphlpapi");
    }

    // Handle macOS-specific libpcap configuration - NO bundling approach
    #[cfg(target_os = "macos")]
    {
        use std::path::Path;

        println!("cargo:rerun-if-env-changed=LIBPCAP_LIBDIR");
        println!("cargo:rerun-if-env-changed=PKG_CONFIG_PATH");

        // Set up libpcap paths for macOS builds
        if let Ok(libpcap_dir) = env::var("LIBPCAP_LIBDIR") {
            println!("cargo:rustc-link-search=native={}", libpcap_dir);
        }

        if let Ok(pkg_config_path) = env::var("PKG_CONFIG_PATH") {
            println!("cargo:rustc-env=PKG_CONFIG_PATH={}", pkg_config_path);
        }

        // Define search paths for libpcap at runtime (no bundling)
        let libpcap_search_paths = [
            // Standardized InnoMonitor dependency location (preferred)
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

        let mut libpcap_found = false;

        // Find any available libpcap for build-time linking
        for path in &libpcap_search_paths {
            if Path::new(path).exists() {
                let lib_dir = Path::new(path).parent().unwrap().to_string_lossy();
                println!("cargo:rustc-link-search=native={}", lib_dir);
                println!("cargo:warning=Found libpcap for build at: {}", path);
                libpcap_found = true;
                break;
            }
        }

        if !libpcap_found {
            println!("cargo:warning=No libpcap found during build - app will look for it at runtime");
            println!("cargo:warning=Expected runtime locations:");
            for path in &libpcap_search_paths {
                println!("cargo:warning=  {}", path);
            }
        }
        
        // Always try to link against pcap
        println!("cargo:rustc-link-lib=pcap");

        // Additional macOS framework dependencies for network monitoring
        println!("cargo:rustc-link-lib=framework=CoreFoundation");
        println!("cargo:rustc-link-lib=framework=SystemConfiguration");
    }

    tauri_build::build()
}
