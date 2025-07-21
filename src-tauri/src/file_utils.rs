use std::fs;
use std::path::Path;
use crate::encryption::decrypt_string;

/// Copy a file from `source_dir/file_name` to `target_dir/file_name`
pub fn save_backup(source_dir: &Path, target_dir: &Path, file_name: &str) -> std::io::Result<()> {
    if !target_dir.exists() {
        fs::create_dir_all(target_dir)?;
    }
    let source_file = source_dir.join(file_name);
    let target_file = target_dir.join(file_name);
    fs::copy(&source_file, &target_file)?;
    Ok(())
}

/// Copy a file from `backup_dir/file_name` to `restore_dir/file_name`
pub fn load_backup(backup_dir: &Path, restore_dir: &Path, file_name: &str) -> std::io::Result<()> {
    if !restore_dir.exists() {
        fs::create_dir_all(restore_dir)?;
    }
    let backup_file = backup_dir.join(file_name);
    let restore_file = restore_dir.join(file_name);
    fs::copy(&backup_file, &restore_file)?;
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
