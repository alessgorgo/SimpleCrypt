mod data_encryption;
mod data_decryption;
mod memory_security;

use std::env;
use std::process;
use std::fs;
use std::io::Write;
use std::path::Path;
use tempfile::NamedTempFile;
use std::fs::File;
use std::io::Read;
use std::error::Error;
use rpassword;

fn print_usage() {
    println!(
        r#"
Usage:
  encrypt [FILE|PATH]       Encrypt a single file.
  decrypt [FILE|PATH]       Decrypt a single file.
  encrypt-dir [PATH]        Encrypt all files in a directory.
  decrypt-dir [PATH]        Decrypt all files in a directory.

Options:
  --help, -h                Show this help message.
  --backup                  Create backup before overwriting files.
  --dry-run                 Preview what would be done without making changes.
  --secure-wipe             Securely wipe sensitive data from memory.
"#
    );
}

fn atomic_write_file(path: &Path, data: &[u8]) -> std::io::Result<()> {
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
        
        if let Err(e) = memory_security::MemorySecurity::set_secure_file_permissions(path) {
            eprintln!("Warning: Could not set secure file permissions: {}", e);
        }
        
        return Ok(());
    }
    
    let temp_file = NamedTempFile::new_in(path.parent().unwrap_or_else(|| Path::new(".")))?;
    temp_file.as_file().write_all(data)?;
    temp_file.as_file().sync_all()?;
    
    if let Err(e) = memory_security::MemorySecurity::set_secure_file_permissions(temp_file.path()) {
        eprintln!("Warning: Could not set secure file permissions: {}", e);
    }
    
    temp_file.persist(path)?;
    Ok(())
}

fn atomic_read_file(path: &Path) -> std::io::Result<Vec<u8>> {
    let metadata = fs::metadata(path)?;
    let file_size = metadata.len() as usize;
    
    let mut buffer = Vec::with_capacity(file_size);
    
    let mut file = File::open(path)?;
    file.read_to_end(&mut buffer)?;
    
    Ok(buffer)
}

fn get_secure_password(prompt: &str) -> memory_security::SecureString {
    use rpassword::read_password;
    
    println!("{}", prompt);
    let password_input = read_password().unwrap_or_else(|_| {
        eprintln!("Error reading password input");
        process::exit(1);
    });
    
    let secure_password = memory_security::SecureString::from(&password_input);
    
    secure_password
}

#[derive(Default)]
struct CommandOptions {
    backup: bool,
    dry_run: bool,
}

fn parse_options(args: &[String]) -> CommandOptions {
    let mut options = CommandOptions::default();
    
    for arg in args.iter().skip(3) {
        match arg.as_str() {
            "--backup" => options.backup = true,
            "--dry-run" => options.dry_run = true,
            _ => {}
        }
    }
    
    options
}

fn validate_inputs(file_path: &str, password: &memory_security::SecureString) -> Result<(), Box<dyn Error>> {
    if password.is_empty() {
        eprintln!("Error: Password cannot be empty");
        process::exit(1);
    }

    if file_path.is_empty() {
        eprintln!("Error: File path cannot be empty");
        process::exit(1);
    }
    
    Ok(())
}

fn get_password_optimized(args: &[String]) -> Result<memory_security::SecureString, Box<dyn Error>> {
    if args.len() >= 4 {
        let pwd = &args[3];
        Ok(memory_security::SecureString::from(pwd))
    } else {
        Ok(get_secure_password("Enter password: "))
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() == 2 && (args[1] == "--help" || args[1] == "-h") {
        print_usage();
        return Ok(());
    }
    
    if args.len() < 3 {
        eprintln!("Error: Insufficient arguments");
        eprintln!("Usage: {} <encrypt/decrypt> <file_path> [password]", args[0]);
        eprintln!("If password is not provided, you will be prompted securely.");
        eprintln!("Examples:");
        eprintln!("  {} encrypt file.txt", args[0]);
        eprintln!("  {} decrypt file.txt mypassword", args[0]);
        eprintln!("  {} --help", args[0]);
        process::exit(1);
    }

    let action = &args[1];
    let file_path = &args[2];
    
    let options = parse_options(&args);
    
    let password = get_password_optimized(&args)?;
    
    validate_inputs(file_path, &password)?;

    memory_security::MemorySecurity::clear_sensitive_env_vars();

    match action.as_str() {
        "encrypt" => {
            println!("Encrypting file: {}", file_path);
            match atomic_read_file(Path::new(file_path)) {
                Ok(file_data) => {
                    match data_encryption::encrypt_file(&file_data, password.as_str()) {
                        Ok(encrypted_data) => {
                            if options.dry_run {
                                println!("Dry run: Would encrypt and replace: {}", file_path);
                                return Ok(());
                            }
                            
                            if options.backup {
                                let backup_path = format!("{}.backup", file_path);
                                match fs::copy(file_path, &backup_path) {
                                    Ok(_) => println!("Created backup: {}", backup_path),
                                    Err(e) => {
                                        eprintln!("Warning: Could not create backup: {}", e);
                                    }
                                }
                            }
                            
                            match atomic_write_file(Path::new(file_path), encrypted_data.as_bytes()) {
                                Ok(_) => println!("Successfully encrypted and replaced: {}", file_path),
                                Err(e) => {
                                    eprintln!("Error writing encrypted data to file: {}", e);
                                    eprintln!("This could be due to:");
                                    eprintln!("  - Insufficient disk space");
                                    eprintln!("  - File permission issues");
                                    eprintln!("  - Invalid file path");
                                    process::exit(1);
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("Error encrypting file: {}", e);
                            eprintln!("This could be due to:");
                            eprintln!("  - Invalid file permissions");
                            eprintln!("  - Corrupted file data");
                            eprintln!("  - System memory issues");
                            eprintln!("  - HMAC verification failure (possible tampering)");
                            process::exit(1);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error reading file '{}': {}", file_path, e);
                    eprintln!("Please check if the file exists and you have read permissions.");
                    process::exit(1);
                }
            }
        }
        "decrypt" => {
            println!("Decrypting file: {}", file_path);
            match data_decryption::decrypt_file(file_path, password.as_str()) {
                Ok(decrypted_content) => {
                    if options.dry_run {
                        println!("Dry run: Would decrypt and replace: {}", file_path);
                        return Ok(());
                    }
                    
                    if options.backup {
                        let backup_path = format!("{}.backup", file_path);
                        match fs::copy(file_path, &backup_path) {
                            Ok(_) => println!("Created backup: {}", backup_path),
                            Err(e) => {
                                eprintln!("Warning: Could not create backup: {}", e);
                            }
                        }
                    }
                    
                    match atomic_write_file(Path::new(file_path), decrypted_content.as_bytes()) {
                        Ok(_) => println!("Successfully decrypted and replaced: {}", file_path),
                        Err(e) => {
                            eprintln!("Error writing decrypted data to file: {}", e);
                            eprintln!("This could be due to:");
                            eprintln!("  - Insufficient disk space");
                            eprintln!("  - File permission issues");
                            eprintln!("  - Invalid file path");
                            process::exit(1);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error decrypting file: {}", e);
                    eprintln!("This could be due to:");
                    eprintln!("  - Incorrect password");
                    eprintln!("  - Corrupted encrypted file");
                    eprintln!("  - Invalid file format");
                    eprintln!("  - HMAC verification failure (possible tampering)");
                    process::exit(1);
                }
            }
        }
        "encrypt-dir" => {
            if args.len() < 4 {
                eprintln!("Error: Missing directory path or password for encryption.");
                return Ok(());
            }
            let dir_path = &args[2];
            let password = &args[3];
            
            match fs::metadata(dir_path) {
                Ok(metadata) => {
                    if !metadata.is_dir() {
                        eprintln!("Error: '{}' is not a valid directory", dir_path);
                        return Ok(());
                    }
                }
                Err(e) => {
                    eprintln!("Error accessing directory '{}': {}", dir_path, e);
                    eprintln!("Please check if the directory exists and you have read permissions.");
                    return Ok(());
                }
            }
            
            let mut files: Vec<std::path::PathBuf> = Vec::new();
            let mut error_count = 0;
            
            println!("Scanning directory for files...");
            for entry_result in fs::read_dir(dir_path).expect("Failed to read directory") {
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
                                error_count += 1;
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Warning: Could not read directory entry: {}", e);
                        error_count += 1;
                    }
                }
            }
            
            if files.is_empty() {
                eprintln!("No files found in directory: {}", dir_path);
                return Ok(());
            }
            
            println!("Found {} files to encrypt ({} warnings)", files.len(), error_count);
            println!("Encrypting files with password: {}", "*".repeat(password.len()));
            
            let mut success_count = 0;
            for (i, file_path) in files.iter().enumerate() {
                match atomic_read_file(&file_path) {
                    Ok(file_data) => {
                        match data_encryption::encrypt_file(&file_data, password) {
                            Ok(encrypted_data) => {
                                if options.dry_run {
                                    println!("[{}/{}] Dry run: Would encrypt: {}", i + 1, files.len(), file_path.display());
                                    success_count += 1;
                                    continue;
                                }
                                
                                if options.backup {
                                    let backup_path = format!("{}.backup", file_path.display());
                                    match fs::copy(&file_path, &backup_path) {
                                        Ok(_) => println!("[{}/{}] 📋 Created backup: {}", i + 1, files.len(), file_path.display()),
                                        Err(e) => {
                                            eprintln!("[{}/{}] Warning: Could not create backup for '{}': {}",
                                                     i + 1, files.len(), file_path.display(), e);
                                        }
                                    }
                                }
                                
                                match atomic_write_file(&file_path, encrypted_data.as_bytes()) {
                                    Ok(_) => {
                                        println!("[{}/{}] Encrypted: {}", i + 1, files.len(), file_path.display());
                                        success_count += 1;
                                    }
                                    Err(e) => {
                                        eprintln!("[{}/{}] ❌ Error writing encrypted file: {}",
                                                 i + 1, files.len(), file_path.display());
                                        eprintln!("   {}", e);
                                        error_count += 1;
                                    }
                                }
                            }
                            Err(e) => {
                                eprintln!("[{}/{}] Error encrypting file: {}",
                                         i + 1, files.len(), file_path.display());
                                eprintln!("   {}", e);
                                error_count += 1;
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("[{}/{}] Error reading file: {}",
                                 i + 1, files.len(), file_path.display());
                        eprintln!("   {}", e);
                        error_count += 1;
                    }
                }
            }
            
            println!("\nSummary:");
            println!("   Successfully encrypted: {} files", success_count);
            println!("   Errors encountered: {} files", error_count);
            if error_count == 0 {
                println!("Directory encryption completed successfully!");
            } else {
                println!("Directory encryption completed with {} errors", error_count);
            }
        }
        "decrypt-dir" => {
            if args.len() < 4 {
                eprintln!("Error: Missing directory path or password for decryption.");
                return Ok(());
            }
            let dir_path = &args[2];
            let password = &args[3];
            
            match fs::metadata(dir_path) {
                Ok(metadata) => {
                    if !metadata.is_dir() {
                        eprintln!("Error: '{}' is not a valid directory", dir_path);
                        return Ok(());
                    }
                }
                Err(e) => {
                    eprintln!("Error accessing directory '{}': {}", dir_path, e);
                    eprintln!("Please check if the directory exists and you have read permissions.");
                    return Ok(());
                }
            }
            
            let mut files: Vec<std::path::PathBuf> = Vec::new();
            let mut error_count = 0;
            
            println!("🔍 Scanning directory for files...");
            for entry_result in fs::read_dir(dir_path).expect("Failed to read directory") {
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
                                error_count += 1;
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Warning: Could not read directory entry: {}", e);
                        error_count += 1;
                    }
                }
            }
            
            if files.is_empty() {
                eprintln!("No files found in directory: {}", dir_path);
                return Ok(());
            }
            
            println!("Found {} files to decrypt ({} warnings)", files.len(), error_count);
            println!("Decrypting files with password: {}", "*".repeat(password.len()));
            
            let mut success_count = 0;
            for (i, file_path) in files.iter().enumerate() {
                match file_path.to_str() {
                    Some(path_str) => {
                        match data_decryption::decrypt_file(path_str, password) {
                            Ok(decrypted_content) => {
                                if options.dry_run {
                                    println!("[{}/{}] Dry run: Would decrypt: {}", i + 1, files.len(), file_path.display());
                                    success_count += 1;
                                    continue;
                                }
                                
                                if options.backup {
                                    let backup_path = format!("{}.backup", file_path.display());
                                    match fs::copy(&file_path, &backup_path) {
                                        Ok(_) => println!("[{}/{}] Created backup: {}", i + 1, files.len(), file_path.display()),
                                        Err(e) => {
                                            eprintln!("[{}/{}] Warning: Could not create backup for '{}': {}",
                                                     i + 1, files.len(), file_path.display(), e);
                                        }
                                    }
                                }
                                
                                match atomic_write_file(&file_path, decrypted_content.as_bytes()) {
                                    Ok(_) => {
                                        println!("[{}/{}] ecrypted: {}", i + 1, files.len(), file_path.display());
                                        success_count += 1;
                                    }
                                    Err(e) => {
                                        eprintln!("[{}/{}] Error writing decrypted file: {}",
                                                 i + 1, files.len(), file_path.display());
                                        eprintln!("   {}", e);
                                        error_count += 1;
                                    }
                                }
                            }
                            Err(_e) => {
                                eprintln!("[{}/{}] Error decrypting file: {}",
                                         i + 1, files.len(), file_path.display());
                                eprintln!("   This could be due to incorrect password or corrupted file");
                                eprintln!("   HMAC verification failure indicates possible tampering");
                                error_count += 1;
                            }
                        }
                    }
                    None => {
                        eprintln!("[{}/{}] Error: Invalid file path (contains non-UTF8 characters)",
                                 i + 1, files.len());
                        error_count += 1;
                    }
                }
            }
            
            println!("\nSummary:");
            println!("   Successfully decrypted: {} files", success_count);
            println!("   Errors encountered: {} files", error_count);
            if error_count == 0 {
                println!("Directory decryption completed successfully!");
            } else {
                println!("Directory decryption completed with {} errors", error_count);
            }
        }
        "--help" | "-h" => print_usage(),
        _ => {
            eprintln!("Unknown command: {}", action);
            print_usage();
        }
    }

    drop(password);

    memory_security::MemorySecurity::clear_sensitive_env_vars();

    Ok(())
}
