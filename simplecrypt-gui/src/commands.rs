use serde::{Deserialize, Serialize};
use simplecrypt_core::{
    AlgorithmRegistry, Config,
    algorithm::AlgorithmInfo,
    decryption, encryption, file_ops,
};
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationResult {
    pub success: bool,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DirectoryResult {
    pub success_count: usize,
    pub error_count: usize,
    pub errors: Vec<String>,
}

#[tauri::command]
pub fn encrypt_file(
    path: String,
    password: String,
    algorithm: Option<String>,
) -> Result<OperationResult, String> {
    let config = Config::load();
    let registry = AlgorithmRegistry::new();

    let algo_id = algorithm.as_deref().unwrap_or(&config.encryption.algorithm);
    let algo = registry.get(algo_id)
        .ok_or_else(|| format!("Unknown algorithm: {}", algo_id))?;

    let file_path = Path::new(&path);
    let file_data = file_ops::atomic_read_file(file_path)
        .map_err(|e| format!("Failed to read file: {}", e))?;

    let encrypted = encryption::encrypt_data(&file_data, &password, algo)
        .map_err(|e| format!("Encryption failed: {}", e))?;

    if config.security.backup_on_encrypt {
        let _ = file_ops::create_backup(file_path);
    }

    file_ops::atomic_write_file(file_path, encrypted.as_bytes())
        .map_err(|e| format!("Failed to write encrypted file: {}", e))?;

    Ok(OperationResult {
        success: true,
        message: format!("Encrypted: {} [{}]", path, algo_id),
    })
}

#[tauri::command]
pub fn decrypt_file(
    path: String,
    password: String,
) -> Result<OperationResult, String> {
    let registry = AlgorithmRegistry::new();

    let decrypted = decryption::decrypt_file(&path, &password, &registry)
        .map_err(|e| format!("Decryption failed: {}", e))?;

    let file_path = Path::new(&path);
    file_ops::atomic_write_file(file_path, decrypted.as_bytes())
        .map_err(|e| format!("Failed to write decrypted file: {}", e))?;

    Ok(OperationResult {
        success: true,
        message: format!("Decrypted: {}", path),
    })
}

#[tauri::command]
pub fn encrypt_directory(
    path: String,
    password: String,
    algorithm: Option<String>,
) -> Result<DirectoryResult, String> {
    let config = Config::load();
    let registry = AlgorithmRegistry::new();

    let algo_id = algorithm.as_deref().unwrap_or(&config.encryption.algorithm);
    let algo = registry.get(algo_id)
        .ok_or_else(|| format!("Unknown algorithm: {}", algo_id))?;

    let dir_path = Path::new(&path);
    let files = file_ops::collect_files(dir_path)
        .map_err(|e| format!("Failed to scan directory: {}", e))?;

    let mut success_count = 0;
    let mut error_count = 0;
    let mut errors = Vec::new();

    for file_path in &files {
        match file_ops::atomic_read_file(file_path) {
            Ok(file_data) => {
                match encryption::encrypt_data(&file_data, &password, algo) {
                    Ok(encrypted) => {
                        match file_ops::atomic_write_file(file_path, encrypted.as_bytes()) {
                            Ok(_) => success_count += 1,
                            Err(e) => {
                                error_count += 1;
                                errors.push(format!("{}: {}", file_path.display(), e));
                            }
                        }
                    }
                    Err(e) => {
                        error_count += 1;
                        errors.push(format!("{}: {}", file_path.display(), e));
                    }
                }
            }
            Err(e) => {
                error_count += 1;
                errors.push(format!("{}: {}", file_path.display(), e));
            }
        }
    }

    Ok(DirectoryResult { success_count, error_count, errors })
}

#[tauri::command]
pub fn decrypt_directory(
    path: String,
    password: String,
) -> Result<DirectoryResult, String> {
    let registry = AlgorithmRegistry::new();
    let dir_path = Path::new(&path);
    let files = file_ops::collect_files(dir_path)
        .map_err(|e| format!("Failed to scan directory: {}", e))?;

    let mut success_count = 0;
    let mut error_count = 0;
    let mut errors = Vec::new();

    for file_path in &files {
        match file_path.to_str() {
            Some(path_str) => {
                match decryption::decrypt_file(path_str, &password, &registry) {
                    Ok(decrypted) => {
                        match file_ops::atomic_write_file(file_path, decrypted.as_bytes()) {
                            Ok(_) => success_count += 1,
                            Err(e) => {
                                error_count += 1;
                                errors.push(format!("{}: {}", file_path.display(), e));
                            }
                        }
                    }
                    Err(e) => {
                        error_count += 1;
                        errors.push(format!("{}: {}", file_path.display(), e));
                    }
                }
            }
            None => {
                error_count += 1;
                errors.push(format!("{}: Invalid file path (non-UTF8)", file_path.display()));
            }
        }
    }

    Ok(DirectoryResult { success_count, error_count, errors })
}

#[tauri::command]
pub fn list_algorithms() -> Vec<AlgorithmInfo> {
    let registry = AlgorithmRegistry::new();
    registry.list()
}

#[tauri::command]
pub fn get_config() -> Result<Config, String> {
    Ok(Config::load())
}

#[tauri::command]
pub fn save_config(config: Config) -> Result<(), String> {
    config.save().map_err(|e| format!("Failed to save config: {}", e))
}
