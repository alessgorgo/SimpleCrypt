use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use tempfile::NamedTempFile;

use crate::memory_security::MemorySecurity;

/// Smart atomic write operation that optimizes for small files
pub fn atomic_write_file(path: &Path, data: &[u8]) -> std::io::Result<()> {
    // For very small files (< 1KB), use direct write to avoid atomic operation overhead
    if data.len() < 1024 {
        if path.exists() {
            let backup_path = path.with_extension("backup");
            if let Err(e) = fs::copy(path, &backup_path) {
                eprintln!("Warning: Could not create backup: {}", e);
            }
        }

        let mut file = File::create(path)?;
        file.write_all(data)?;
        file.sync_all()?;

        if let Err(e) = MemorySecurity::set_secure_file_permissions(path) {
            eprintln!("Warning: Could not set secure file permissions: {}", e);
        }

        return Ok(());
    }

    // Use atomic write for larger files
    let temp_file = NamedTempFile::new_in(path.parent().unwrap_or_else(|| Path::new(".")))?;
    temp_file.as_file().write_all(data)?;
    temp_file.as_file().sync_all()?;

    if let Err(e) = MemorySecurity::set_secure_file_permissions(temp_file.path()) {
        eprintln!("Warning: Could not set secure file permissions: {}", e);
    }

    temp_file.persist(path)?;
    Ok(())
}

/// Optimized file read operation with pre-allocated buffer
pub fn atomic_read_file(path: &Path) -> std::io::Result<Vec<u8>> {
    let metadata = fs::metadata(path)?;
    let file_size = metadata.len() as usize;

    let mut buffer = Vec::with_capacity(file_size);
    let mut file = File::open(path)?;
    file.read_to_end(&mut buffer)?;

    Ok(buffer)
}

/// Scan a directory and collect all file paths
pub fn collect_files(dir_path: &Path) -> std::io::Result<Vec<std::path::PathBuf>> {
    let mut files = Vec::new();

    for entry_result in fs::read_dir(dir_path)? {
        match entry_result {
            Ok(entry) => {
                match entry.file_type() {
                    Ok(file_type) => {
                        if file_type.is_file() {
                            files.push(entry.path());
                        }
                    }
                    Err(e) => {
                        eprintln!("Warning: Could not determine file type for '{}': {}",
                                 entry.path().display(), e);
                    }
                }
            }
            Err(e) => {
                eprintln!("Warning: Could not read directory entry: {}", e);
            }
        }
    }

    Ok(files)
}

/// Create a backup of a file
pub fn create_backup(file_path: &Path) -> std::io::Result<std::path::PathBuf> {
    let backup_path = file_path.with_extension("backup");
    fs::copy(file_path, &backup_path)?;
    Ok(backup_path)
}
