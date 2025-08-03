fn main() {
    use std::env;
    
    // Set up npcap library path for Windows
    #[cfg(target_os = "windows")]
    {
        // Try to find npcap-sdk in common locations
        let npcap_env = std::env::var("NPCAP_SDK_LIB").unwrap_or_default();
        let possible_paths = [
            "C:\\Users\\Administrator\\Downloads\\npcap-sdk-1.15\\Lib\\x64",
            "C:\\Users\\Administrator\\Downloads\\npcap-sdk-1.15\\Lib",
            "C:\\npcap-sdk\\Lib\\x64",
            "C:\\npcap-sdk\\Lib",
            npcap_env.as_str(),
        ];

        let mut lib_path_found = false;
        for path in &possible_paths {
            if !path.is_empty() && std::path::Path::new(path).exists() {
                println!("cargo:rustc-link-search=native={}", path);
                lib_path_found = true;
                break;
            }
        }

        if !lib_path_found {
            println!("cargo:warning=npcap-sdk not found. Please set NPCAP_SDK_LIB environment variable or place npcap-sdk in a standard location.");
        }
    }

    // Handle macOS-specific libpcap configuration
    #[cfg(target_os = "macos")]
    {
        // Set up libpcap paths for macOS builds
        if let Ok(libpcap_dir) = env::var("LIBPCAP_LIBDIR") {
            println!("cargo:rustc-link-search=native={}", libpcap_dir);
        }
        
        if let Ok(pkg_config_path) = env::var("PKG_CONFIG_PATH") {
            println!("cargo:rustc-env=PKG_CONFIG_PATH={}", pkg_config_path);
        }
        
        // Additional macOS-specific link flags
        println!("cargo:rustc-link-lib=pcap");
        
        // For Homebrew-installed libpcap (Apple Silicon)
        if std::path::Path::new("/opt/homebrew/lib").exists() {
            println!("cargo:rustc-link-search=native=/opt/homebrew/lib");
        }
        // For Homebrew-installed libpcap (Intel)
        if std::path::Path::new("/usr/local/lib").exists() {
            println!("cargo:rustc-link-search=native=/usr/local/lib");
        }
        // System libpcap
        if std::path::Path::new("/usr/lib").exists() {
            println!("cargo:rustc-link-search=native=/usr/lib");
        }
        
        // Check if libpcap is available
        let libpcap_paths = [
            "/opt/homebrew/lib/libpcap.dylib",
            "/usr/local/lib/libpcap.dylib", 
            "/usr/lib/libpcap.dylib",
            "/System/Library/Frameworks/CoreServices.framework/Versions/A/Frameworks/libpcap.dylib"
        ];
        
        let mut pcap_found = false;
        for path in &libpcap_paths {
            if std::path::Path::new(path).exists() {
                pcap_found = true;
                break;
            }
        }
        
        if !pcap_found {
            println!("cargo:warning=libpcap not found. Please install libpcap: brew install libpcap");
        }
    }

    tauri_build::build()
}
