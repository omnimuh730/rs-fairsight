use std::fs;
use std::path::PathBuf;
use chrono::{Local, NaiveDateTime, TimeZone};

pub fn create_backup(storage_dir: &PathBuf, backup_dir: &PathBuf, date: &str) -> Result<(), String> {
    let source_file = storage_dir.join(format!("network-{}.json", date));
    
    if !source_file.exists() {
        return Ok(()); // Nothing to backup
    }
    
    // Create backup with timestamp
    let timestamp = Local::now().format("%Y%m%d_%H%M%S");
    let backup_name = format!("network-{}.json.backup_{}", date, timestamp);
    let temp_backup = backup_dir.join(format!("{}.tmp", backup_name));
    let final_backup = backup_dir.join(backup_name);
    
    // First copy to temporary file
    fs::copy(&source_file, &temp_backup)
        .map_err(|e| format!("Failed to create temporary backup: {}", e))?;
    
    // Verify the backup by reading it back
    let _ = fs::read(&temp_backup)
        .map_err(|e| format!("Failed to verify backup: {}", e))?;
    
    // Atomically rename to final backup
    fs::rename(&temp_backup, &final_backup)
        .map_err(|e| format!("Failed to finalize backup: {}", e))?;
    
    // Keep only the 5 most recent backups for this date
    cleanup_old_backups(backup_dir, date, 5)?;
    
    println!("Network data backup created for date: {}", date);
    Ok(())
}

pub fn cleanup_old_backups(backup_dir: &PathBuf, date: &str, keep_count: usize) -> Result<(), String> {
    let mut backups = Vec::new();
    
    if let Ok(entries) = fs::read_dir(backup_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let file_name = entry.file_name();
                let file_name_str = file_name.to_string_lossy();
                
                if file_name_str.starts_with(&format!("network-{}.json.backup_", date)) {
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

pub fn restore_from_backup(storage_dir: &PathBuf, backup_dir: &PathBuf, date: &str) -> Result<(), String> {
    // Find the most recent backup for this date
    let mut backups = Vec::new();
    
    if let Ok(entries) = fs::read_dir(backup_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let entry_file_name = entry.file_name();
                let entry_file_name_str = entry_file_name.to_string_lossy();
                
                if entry_file_name_str.starts_with(&format!("network-{}.json.backup_", date)) {
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
        return Err(format!("No backup files found for date: {}", date));
    }
    
    // Sort by modification time (newest first)
    backups.sort_by(|a, b| b.1.cmp(&a.1));
    
    // Restore from the most recent backup
    let (backup_path, _) = &backups[0];
    let restore_path = storage_dir.join(format!("network-{}.json", date));
    let temp_restore = storage_dir.join(format!("network-{}.json.tmp", date));
    
    // Copy backup to temporary file
    fs::copy(backup_path, &temp_restore)
        .map_err(|e| format!("Failed to copy backup: {}", e))?;
    
    // Verify the restore file
    let _ = fs::read(&temp_restore)
        .map_err(|e| format!("Failed to verify restore file: {}", e))?;
    
    // Atomically replace the original
    fs::rename(&temp_restore, &restore_path)
        .map_err(|e| format!("Failed to finalize restore: {}", e))?;
    
    println!("Network data restored from backup for date: {}", date);
    Ok(())
}

pub fn daily_backup_cleanup(backup_dir: &PathBuf) -> Result<(), String> {
    let today = Local::now().format("%Y-%m-%d").to_string();
    let cutoff_date = Local::now() - chrono::Duration::days(7); // Keep backups for 7 days
    
    if let Ok(entries) = fs::read_dir(backup_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let file_name = entry.file_name();
                let file_name_str = file_name.to_string_lossy();
                
                // Skip today's backups
                if file_name_str.contains(&today) {
                    continue;
                }
                
                // Extract timestamp from backup filename
                if file_name_str.contains(".backup_") {
                    if let Some(timestamp_part) = file_name_str.split(".backup_").nth(1) {
                        if let Ok(backup_date) = NaiveDateTime::parse_from_str(timestamp_part, "%Y%m%d_%H%M%S") {
                            let backup_datetime = Local.from_local_datetime(&backup_date);
                            if let Some(backup_datetime) = backup_datetime.single() {
                                if backup_datetime < cutoff_date {
                                    if let Err(e) = fs::remove_file(entry.path()) {
                                        eprintln!("Failed to remove old network backup: {}", e);
                                    } else {
                                        println!("Removed old network backup: {}", file_name_str);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    
    Ok(())
}
