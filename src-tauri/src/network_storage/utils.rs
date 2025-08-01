use std::fs;
use std::path::PathBuf;

pub fn get_platform_directories() -> Result<(PathBuf, PathBuf), String> {
    #[cfg(target_os = "macos")]
    {
        use dirs;
        let home_dir = dirs::home_dir().ok_or("Failed to get home directory")?;
        let storage_dir = home_dir.join("Documents").join("rs-fairsight-network-log");
        let backup_dir = home_dir.join("Documents").join("rs-fairsight-network-backup");
        create_directories(&storage_dir, &backup_dir)?;
        Ok((storage_dir, backup_dir))
    }
    #[cfg(not(target_os = "macos"))]
    {
        let storage_dir = std::path::Path::new("C:\\fairsight-network-log").to_path_buf();
        let backup_dir = std::path::Path::new("C:\\fairsight-network-backup").to_path_buf();
        create_directories(&storage_dir, &backup_dir)?;
        Ok((storage_dir, backup_dir))
    }
}

fn create_directories(storage_dir: &PathBuf, backup_dir: &PathBuf) -> Result<(), String> {
    if !storage_dir.exists() {
        fs::create_dir_all(&storage_dir)
            .map_err(|e| format!("Failed to create network storage directory: {}", e))?;
    }
    if !backup_dir.exists() {
        fs::create_dir_all(&backup_dir)
            .map_err(|e| format!("Failed to create network backup directory: {}", e))?;
    }
    Ok(())
}
