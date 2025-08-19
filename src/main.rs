mod data_encryption;
mod data_decryption;

use std::env;
use std::process;
use std::fs;
use std::error::Error;
use std::io::Write;

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
"#
    );
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() == 2 && (args[1] == "--help" || args[1] == "-h") {
        print_usage();
        return Ok(());
    }
    
    if args.len() < 4 {
        eprintln!("Error: Insufficient arguments");
        eprintln!("Usage: {} <encrypt/decrypt> <file_path> <password>", args[0]);
        eprintln!("Examples:");
        eprintln!("  {} encrypt file.txt mypassword", args[0]);
        eprintln!("  {} decrypt file.txt mypassword", args[0]);
        eprintln!("  {} --help", args[0]);
        process::exit(1);
    }

    let action = &args[1];
    let file_path = &args[2];
    let password = &args[3];

    if password.is_empty() {
        eprintln!("Error: Password cannot be empty");
        process::exit(1);
    }

    if file_path.is_empty() {
        eprintln!("Error: File path cannot be empty");
        process::exit(1);
    }

    match action.as_str() {
        "encrypt" => {
            println!("Encrypting file: {}", file_path);
            match fs::read(file_path) {
                Ok(file_data) => {
                    match data_encryption::encrypt_file(&file_data, password) {
                        Ok(encrypted_data) => {
                            match fs::write(file_path, encrypted_data) {
                                Ok(_) => println!("✅ Successfully encrypted and replaced: {}", file_path),
                                Err(e) => {
                                    eprintln!("❌ Error writing encrypted data to file: {}", e);
                                    process::exit(1);
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("❌ Error encrypting file: {}", e);
                            eprintln!("This could be due to:");
                            eprintln!("  - Invalid file permissions");
                            eprintln!("  - Corrupted file data");
                            eprintln!("  - System memory issues");
                            process::exit(1);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("❌ Error reading file '{}': {}", file_path, e);
                    eprintln!("Please check if the file exists and you have read permissions.");
                    process::exit(1);
                }
            }
        }
        "decrypt" => {
            println!("Decrypting file: {}", file_path);
            match data_decryption::decrypt_file(file_path, password) {
                Ok(decrypted_content) => {
                    match fs::write(file_path, decrypted_content) {
                        Ok(_) => println!("✅ Successfully decrypted and replaced: {}", file_path),
                        Err(e) => {
                            eprintln!("❌ Error writing decrypted data to file: {}", e);
                            process::exit(1);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("❌ Error decrypting file: {}", e);
                    eprintln!("This could be due to:");
                    eprintln!("  - Incorrect password");
                    eprintln!("  - Corrupted encrypted file");
                    eprintln!("  - Invalid file format");
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
                        eprintln!("❌ Error: '{}' is not a valid directory", dir_path);
                        return Ok(());
                    }
                }
                Err(e) => {
                    eprintln!("❌ Error accessing directory '{}': {}", dir_path, e);
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
                                eprintln!("⚠️  Warning: Could not determine file type for '{}': {}",
                                         entry.path().display(), e);
                                error_count += 1;
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("⚠️  Warning: Could not read directory entry: {}", e);
                        error_count += 1;
                    }
                }
            }
            
            if files.is_empty() {
                eprintln!("❌ No files found in directory: {}", dir_path);
                return Ok(());
            }
            
            println!("📁 Found {} files to encrypt ({} warnings)", files.len(), error_count);
            println!("🔐 Encrypting files with password: {}", "*".repeat(password.len()));
            
            let mut success_count = 0;
            for (i, file_path) in files.iter().enumerate() {
                match fs::read(&file_path) {
                    Ok(file_data) => {
                        match data_encryption::encrypt_file(&file_data, password) {
                            Ok(encrypted_data) => {
                                match fs::write(&file_path, encrypted_data) {
                                    Ok(_) => {
                                        println!("[{}/{}] ✅ Encrypted: {}", i + 1, files.len(), file_path.display());
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
                                eprintln!("[{}/{}] ❌ Error encrypting file: {}",
                                         i + 1, files.len(), file_path.display());
                                eprintln!("   {}", e);
                                error_count += 1;
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("[{}/{}] ❌ Error reading file: {}",
                                 i + 1, files.len(), file_path.display());
                        eprintln!("   {}", e);
                        error_count += 1;
                    }
                }
            }
            
            println!("\n📊 Summary:");
            println!("   Successfully encrypted: {} files", success_count);
            println!("   Errors encountered: {} files", error_count);
            if error_count == 0 {
                println!("🎉 Directory encryption completed successfully!");
            } else {
                println!("⚠️  Directory encryption completed with {} errors", error_count);
            }
        }
        "decrypt-dir" => {
            if args.len() < 4 {
                eprintln!("Error: Missing directory path or password for decryption.");
                return Ok(());
            }
            let dir_path = &args[2];
            let password = &args[3];
            
            // Validate directory exists
            match fs::metadata(dir_path) {
                Ok(metadata) => {
                    if !metadata.is_dir() {
                        eprintln!("❌ Error: '{}' is not a valid directory", dir_path);
                        return Ok(());
                    }
                }
                Err(e) => {
                    eprintln!("❌ Error accessing directory '{}': {}", dir_path, e);
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
                                eprintln!("⚠️  Warning: Could not determine file type for '{}': {}",
                                         entry.path().display(), e);
                                error_count += 1;
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("⚠️  Warning: Could not read directory entry: {}", e);
                        error_count += 1;
                    }
                }
            }
            
            if files.is_empty() {
                eprintln!("❌ No files found in directory: {}", dir_path);
                return Ok(());
            }
            
            println!("📁 Found {} files to decrypt ({} warnings)", files.len(), error_count);
            println!("🔐 Decrypting files with password: {}", "*".repeat(password.len()));
            
            let mut success_count = 0;
            for (i, file_path) in files.iter().enumerate() {
                match file_path.to_str() {
                    Some(path_str) => {
                        match data_decryption::decrypt_file(path_str, password) {
                            Ok(decrypted_content) => {
                                match fs::write(&file_path, decrypted_content) {
                                    Ok(_) => {
                                        println!("[{}/{}] ✅ Decrypted: {}", i + 1, files.len(), file_path.display());
                                        success_count += 1;
                                    }
                                    Err(e) => {
                                        eprintln!("[{}/{}] ❌ Error writing decrypted file: {}",
                                                 i + 1, files.len(), file_path.display());
                                        eprintln!("   {}", e);
                                        error_count += 1;
                                    }
                                }
                            }
                            Err(e) => {
                                eprintln!("[{}/{}] ❌ Error decrypting file: {}",
                                         i + 1, files.len(), file_path.display());
                                eprintln!("   This could be due to incorrect password or corrupted file");
                                error_count += 1;
                            }
                        }
                    }
                    None => {
                        eprintln!("[{}/{}] ❌ Error: Invalid file path (contains non-UTF8 characters)",
                                 i + 1, files.len());
                        error_count += 1;
                    }
                }
            }
            
            println!("\n📊 Summary:");
            println!("   Successfully decrypted: {} files", success_count);
            println!("   Errors encountered: {} files", error_count);
            if error_count == 0 {
                println!("🎉 Directory decryption completed successfully!");
            } else {
                println!("⚠️  Directory decryption completed with {} errors", error_count);
            }
        }
        "--help" | "-h" => print_usage(),
        _ => {
            eprintln!("Unknown command: {}", action);
            print_usage();
        }
    }

    Ok(())
}
