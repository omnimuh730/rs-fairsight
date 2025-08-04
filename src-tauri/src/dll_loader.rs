use std::ffi::OsString;
use std::os::windows::ffi::OsStrExt;
use std::path::PathBuf;
use winapi::um::libloaderapi::LoadLibraryW;
use winapi::shared::minwindef::HMODULE;

pub fn ensure_npcap_dlls_loaded() -> Result<(), String> {
    // Try to load the DLLs that should be bundled with the app
    let dll_names = ["wpcap.dll", "Packet.dll"];
    
    // First try to load from the bundled location (app's resource directory)
    let exe_dir = std::env::current_exe()
        .ok()
        .and_then(|path| path.parent().map(|p| p.to_path_buf()));
    
    for dll_name in &dll_names {
        let mut loaded = false;
        
        // Try bundled DLL first
        if let Some(ref exe_dir) = exe_dir {
            let bundled_path = exe_dir.join(dll_name);
            if bundled_path.exists() {
                match load_dll_from_path(&bundled_path) {
                    Ok(handle) => {
                        println!("✅ Successfully loaded bundled {}", dll_name);
                        let _ = handle; // Keep handle alive
                        loaded = true;
                    }
                    Err(e) => {
                        println!("⚠️  Could not load bundled {}: {}", dll_name, e);
                    }
                }
            }
        }
        
        // If bundled DLL failed, try system DLL
        if !loaded {
            match load_dll(dll_name) {
                Ok(handle) => {
                    println!("✅ Successfully loaded system {}", dll_name);
                    let _ = handle; // Keep handle alive
                }
                Err(e) => {
                    println!("⚠️  Could not load system {}: {}", dll_name, e);
                    // Continue anyway - pcap crate might still work
                }
            }
        }
    }
    
    Ok(())
}

fn load_dll_from_path(path: &PathBuf) -> Result<HMODULE, String> {
    let wide_path: Vec<u16> = path.as_os_str()
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();
    
    unsafe {
        let handle = LoadLibraryW(wide_path.as_ptr());
        if handle.is_null() {
            return Err(format!("Failed to load from path: {}", path.display()));
        }
        Ok(handle)
    }
}

fn load_dll(dll_name: &str) -> Result<HMODULE, String> {
    let wide_name: Vec<u16> = dll_name.encode_utf16()
        .chain(std::iter::once(0))
        .collect();
    
    unsafe {
        let handle = LoadLibraryW(wide_name.as_ptr());
        if handle.is_null() {
            return Err(format!("Failed to load {}", dll_name));
        }
        Ok(handle)
    }
}
