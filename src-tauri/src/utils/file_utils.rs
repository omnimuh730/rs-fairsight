use std::fs;
use std::path::Path;
use std::io::Write;
use chrono;
use crate::encryption::decrypt_string;

/// Atomically save a backup with timestamp
pub fn save_backup(source_dir: &Path, target_dir: &Path, file_name: &str) -> std::io::Result<()> {
    if !target_dir.exists() {
        fs::create_dir_all(target_dir)?;
    }
    
    let source_file = source_dir.join(file_name);
    if !source_file.exists() {
        return Ok(()); // Nothing to backup
    }
    
    // Create backup with timestamp
    let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
    let backup_name = format!("{}.backup_{}", file_name, timestamp);
    let temp_backup = target_dir.join(format!("{}.tmp", backup_name));
    let final_backup = target_dir.join(backup_name);
    
    // First copy to temporary file
    fs::copy(&source_file, &temp_backup)?;
    
    // Verify the backup by reading it back
    let _ = fs::read(&temp_backup)?;
    
    // Atomically rename to final backup
    fs::rename(&temp_backup, &final_backup)?;
    
    // Keep only the 5 most recent backups
    cleanup_old_backups(target_dir, file_name, 5)?;
    
    Ok(())
}

/// Clean up old backup files, keeping only the most recent ones
fn cleanup_old_backups(backup_dir: &Path, base_file_name: &str, keep_count: usize) -> std::io::Result<()> {
    let mut backups = Vec::new();
    
    if let Ok(entries) = fs::read_dir(backup_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let file_name = entry.file_name();
                let file_name_str = file_name.to_string_lossy();
                
                if file_name_str.starts_with(&format!("{}.backup_", base_file_name)) {
                    if let Ok(metadata) = entry.metadata() {
                        if let Ok(modified) = metadata.modified() {
                            backups.push((entry.path(), modified));
                        }
                    }
                }
            }
        }
    }
    
    // Sort by modification time (newest first)
    backups.sort_by(|a, b| b.1.cmp(&a.1));
    
    // Remove old backups beyond keep_count
    for (path, _) in backups.into_iter().skip(keep_count) {
        let _ = fs::remove_file(path);
    }
    
    Ok(())
}

/// Restore from the most recent valid backup
pub fn load_backup(backup_dir: &Path, restore_dir: &Path, file_name: &str) -> std::io::Result<()> {
    if !restore_dir.exists() {
        fs::create_dir_all(restore_dir)?;
    }
    
    // Find the most recent backup
    let mut backups = Vec::new();
    
    if let Ok(entries) = fs::read_dir(backup_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let entry_file_name = entry.file_name();
                let entry_file_name_str = entry_file_name.to_string_lossy();
                
                if entry_file_name_str.starts_with(&format!("{}.backup_", file_name)) {
                    if let Ok(metadata) = entry.metadata() {
                        if let Ok(modified) = metadata.modified() {
                            backups.push((entry.path(), modified));
                        }
                    }
                }
            }
        }
    }
    
    if backups.is_empty() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "No backup files found"
        ));
    }
    
    // Sort by modification time (newest first)
    backups.sort_by(|a, b| b.1.cmp(&a.1));
    
    // Try to restore from the most recent valid backup
    for (backup_path, _) in backups {
        match fs::copy(&backup_path, restore_dir.join(file_name)) {
            Ok(_) => {
                println!("Successfully restored from backup: {:?}", backup_path);
                return Ok(());
            }
            Err(e) => {
                eprintln!("Failed to restore from backup {:?}: {}", backup_path, e);
                continue;
            }
        }
    }
    
    Err(std::io::Error::new(
        std::io::ErrorKind::Other,
        "All backup restoration attempts failed"
    ))
}

/// Write data atomically to a file with automatic backup
pub fn atomic_write_with_backup(
    file_path: &Path,
    data: &[u8],
    backup_dir: Option<&Path>
) -> std::io::Result<()> {
    // Create backup before writing if backup directory is provided
    if let Some(backup_dir) = backup_dir {
        if file_path.exists() {
            if let (Some(parent), Some(file_name)) = (file_path.parent(), file_path.file_name()) {
                let _ = save_backup(parent, backup_dir, &file_name.to_string_lossy());
            }
        }
    }
    
    // Create parent directory if it doesn't exist
    if let Some(parent) = file_path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent)?;
        }
    }
    
    // Write to temporary file first
    let temp_path = file_path.with_extension(
        format!("{}.tmp", file_path.extension().unwrap_or_default().to_string_lossy())
    );
    
    {
        let mut temp_file = fs::File::create(&temp_path)?;
        temp_file.write_all(data)?;
        temp_file.sync_all()?; // Ensure data is written to disk
    }
    
    // Atomically replace the original file
    fs::rename(&temp_path, file_path)?;
    
    Ok(())
}

pub fn is_log_file_valid(file_path: &Path, key: &[u8; 32]) -> bool {
    let content = match fs::read(file_path) {
        Ok(c) => c,
        Err(_) => return false,
    };
    let mut offset = 0;

    while offset < content.len() {
        // Check for enough bytes for nonce (12) + length (4)
        if content.len() - offset < 12 + 4 {
            return false;
        }

        // Read nonce (12 bytes)
        let nonce_bytes: [u8; 12] = match content[offset..offset + 12].try_into() {
            Ok(n) => n,
            Err(_) => return false,
        };
        offset += 12;

        // Read length (4 bytes)
        let len_bytes: [u8; 4] = match content[offset..offset + 4].try_into() {
            Ok(l) => l,
            Err(_) => return false,
        };
        let encrypted_len = u32::from_le_bytes(len_bytes) as usize;
        offset += 4;

        // Check for enough bytes for encrypted content
        if content.len() - offset < encrypted_len {
            return false;
        }

        let mut encrypted_data = content[offset..offset + encrypted_len].to_vec();
        offset += encrypted_len;

        // Try to decrypt
        if decrypt_string(&mut encrypted_data, key, nonce_bytes).is_err() {
            return false;
        }
    }

    true
}
