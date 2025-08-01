use std::fs;
use std::io;
use std::path::Path;
use std::sync::atomic::Ordering;

use crate::encryption::{encrypt_string, decrypt_string, KEY};
use crate::file_utils::atomic_write_with_backup;
use super::types::{BACKUP_COUNTER, BACKUP_FREQUENCY};

#[cfg(target_os = "macos")]
use dirs;

pub fn get_platform_directories() -> io::Result<(std::path::PathBuf, std::path::PathBuf)> {
    #[cfg(target_os = "macos")]
    {
        let home_dir = dirs::home_dir()
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Could not find home directory"))?;
        let log_dir = home_dir.join("Documents").join("rs-fairsight");
        let backup_dir = home_dir.join("Documents").join("rs-fairsight-backup");
        Ok((log_dir, backup_dir))
    }
    #[cfg(target_os = "windows")]
    {
        let log_dir = Path::new("C:\\fairsight-log").to_path_buf();
        let backup_dir = Path::new("C:\\fairsight-backup").to_path_buf();
        Ok((log_dir, backup_dir))
    }
}

pub fn write_encrypted_message_to_file(
    file_path: &Path,
    message: &str,
    backup_dir: Option<&Path>
) -> io::Result<()> {
    let (encrypted_data, nonce) = encrypt_string(message, &KEY)
        .map_err(|_| io::Error::new(io::ErrorKind::Other, "Encryption failed"))?;
    
    // Prepare the data to write: nonce (12 bytes) + length (4 bytes) + encrypted data
    let mut data = Vec::new();
    data.extend_from_slice(&nonce);
    data.extend_from_slice(&encrypted_data);
    
    // Check if file exists and read current content
    let mut existing_content = if file_path.exists() {
        fs::read(file_path).unwrap_or_default()
    } else {
        Vec::new()
    };
    
    // Append new data
    existing_content.extend_from_slice(&data);
    
    // Use atomic write with backup
    atomic_write_with_backup(file_path, &existing_content, backup_dir)?;
    
    Ok(())
}

pub fn should_create_backup() -> bool {
    let count = BACKUP_COUNTER.fetch_add(1, Ordering::SeqCst) + 1;
    count % BACKUP_FREQUENCY == 0
}

pub fn get_current_backup_count() -> usize {
    BACKUP_COUNTER.load(Ordering::SeqCst)
}
