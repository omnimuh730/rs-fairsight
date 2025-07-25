fn main() {
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

    tauri_build::build()
}
