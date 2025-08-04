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
        println!("cargo:rerun-if-changed=npcap-sdk/Lib/x64");
        
        // Try to find npcap-sdk in various locations, prioritizing bundled SDK
        let possible_paths = [
            // Bundled Npcap SDK (highest priority)
            "npcap-sdk/Lib/x64".to_string(),
            "npcap-sdk/Lib".to_string(),
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
                println!("cargo:warning=🔧 Release build detected - preparing dependency bundling");
                
                // Create bundling directory structure
                let out_dir = env::var("OUT_DIR").unwrap();
                let bundle_base = format!("{}\\..\\..\\..\\bundle_dependencies", out_dir);
                let bundle_libs_dir = format!("{}\\libs", bundle_base);
                
                if let Err(e) = fs::create_dir_all(&bundle_libs_dir) {
                    println!("cargo:warning=Failed to create bundle directory: {}", e);
                } else {
                    println!("cargo:warning=📁 Created bundle directory: {}", bundle_libs_dir);
                    
                    // Bundle runtime DLLs if found
                    let dll_bundled = if let Some(ref dll_path) = wpcap_dll_path {
                        let wpcap_dest = format!("{}\\wpcap.dll", bundle_libs_dir);
                        if let Err(e) = fs::copy(dll_path, &wpcap_dest) {
                            println!("cargo:warning=⚠️  Failed to copy wpcap.dll for bundling: {}", e);
                            false
                        } else {
                            println!("cargo:warning=✅ Bundled wpcap.dll: {} -> {}", dll_path, wpcap_dest);
                            
                            // Also bundle Packet.dll (required dependency)
                            let packet_dll_path = dll_path.replace("wpcap.dll", "Packet.dll");
                            if Path::new(&packet_dll_path).exists() {
                                let packet_dest = format!("{}\\Packet.dll", bundle_libs_dir);
                                if let Err(e) = fs::copy(&packet_dll_path, &packet_dest) {
                                    println!("cargo:warning=⚠️  Failed to copy Packet.dll: {}", e);
                                } else {
                                    println!("cargo:warning=✅ Bundled Packet.dll: {} -> {}", packet_dll_path, packet_dest);
                                }
                            }
                            
                            // Bundle NPF service driver if available
                            let npf_sys_path = dll_path.replace("wpcap.dll", "NPF.sys");
                            if Path::new(&npf_sys_path).exists() {
                                let npf_dest = format!("{}\\NPF.sys", bundle_libs_dir);
                                if let Err(e) = fs::copy(&npf_sys_path, &npf_dest) {
                                    println!("cargo:warning=⚠️  Failed to copy NPF.sys: {}", e);
                                } else {
                                    println!("cargo:warning=✅ Bundled NPF.sys: {} -> {}", npf_sys_path, npf_dest);
                                }
                            }
                            true
                        }
                    } else {
                        println!("cargo:warning=⚠️  No Npcap runtime DLLs found for bundling");
                        println!("cargo:warning=💡 Users will need to install Npcap separately");
                        false
                    };
                    
                    // Create bundling metadata file
                    let metadata_file = format!("{}\\bundle_info.txt", bundle_base);
                    let metadata_content = format!(
                        "InnoMonitor Windows Bundle\n\
                        Build Time: {}\n\
                        Npcap DLLs: {}\n\
                        Bundle Directory: {}\n",
                        chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC"),
                        if dll_bundled { "Included" } else { "Not Found" },
                        bundle_libs_dir
                    );
                    
                    if let Err(e) = fs::write(&metadata_file, metadata_content) {
                        println!("cargo:warning=Failed to write bundle metadata: {}", e);
                    } else {
                        println!("cargo:warning=📝 Created bundle metadata: {}", metadata_file);
                    }
                }
            }
        }
        
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
            println!("cargo:warning=Run install-macos-deps.sh to install dependencies");
        }
        
        // Always try to link against pcap
        println!("cargo:rustc-link-lib=pcap");
        
        // Additional macOS framework dependencies for network monitoring
        println!("cargo:rustc-link-lib=framework=CoreFoundation");
        println!("cargo:rustc-link-lib=framework=SystemConfiguration");
    }

    tauri_build::build()
}
