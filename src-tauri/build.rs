fn main() {
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

    tauri_build::build()
}
