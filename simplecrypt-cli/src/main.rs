use clap::{Parser, Subcommand};
use simplecrypt_core::{
    AlgorithmRegistry, Config, MemorySecurity, SecureString,
    encryption, decryption, file_ops,
};
use std::path::Path;
use std::process;

#[derive(Parser)]
#[command(name = "simplecrypt", version = "2.0.0", about = "Multi-algorithm file encryption tool")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Encrypt a single file
    Encrypt {
        /// Path to the file to encrypt
        file: String,
        /// Password (will prompt if not provided)
        password: Option<String>,
        /// Algorithm to use (overrides config default)
        #[arg(long, short)]
        algorithm: Option<String>,
        /// Create a backup before overwriting
        #[arg(long)]
        backup: bool,
        /// Preview what would be done without making changes
        #[arg(long)]
        dry_run: bool,
    },
    /// Decrypt a single file
    Decrypt {
        /// Path to the file to decrypt
        file: String,
        /// Password (will prompt if not provided)
        password: Option<String>,
        /// Create a backup before overwriting
        #[arg(long)]
        backup: bool,
        /// Preview what would be done without making changes
        #[arg(long)]
        dry_run: bool,
    },
    /// Encrypt all files in a directory
    EncryptDir {
        /// Path to the directory
        dir: String,
        /// Password (will prompt if not provided)
        password: Option<String>,
        /// Algorithm to use (overrides config default)
        #[arg(long, short)]
        algorithm: Option<String>,
        /// Create backups before overwriting
        #[arg(long)]
        backup: bool,
        /// Preview what would be done without making changes
        #[arg(long)]
        dry_run: bool,
    },
    /// Decrypt all files in a directory
    DecryptDir {
        /// Path to the directory
        dir: String,
        /// Password (will prompt if not provided)
        password: Option<String>,
        /// Create backups before overwriting
        #[arg(long)]
        backup: bool,
        /// Preview what would be done without making changes
        #[arg(long)]
        dry_run: bool,
    },
    /// Show or modify configuration
    Config {
        /// Show current configuration
        #[arg(long)]
        show: bool,
        /// Set a config value (key=value)
        #[arg(long)]
        set: Option<String>,
    },
    /// List available encryption algorithms
    Algorithms,
}

fn get_password(provided: Option<String>) -> SecureString {
    match provided {
        Some(pwd) => SecureString::from(&pwd),
        None => {
            println!("Enter password: ");
            let pwd = rpassword::read_password().unwrap_or_else(|_| {
                eprintln!("Error reading password input");
                process::exit(1);
            });
            SecureString::from(&pwd)
        }
    }
}

fn resolve_algorithm<'a>(
    specified: Option<&str>,
    config: &Config,
    registry: &'a AlgorithmRegistry,
) -> &'a dyn simplecrypt_core::CryptoAlgorithm {
    let algo_id = specified.unwrap_or(&config.encryption.algorithm);
    registry.get(algo_id).unwrap_or_else(|| {
        eprintln!("Unknown algorithm: '{}'. Use 'simplecrypt algorithms' to list available algorithms.", algo_id);
        process::exit(1);
    })
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let config = Config::load();
    let registry = AlgorithmRegistry::new();

    MemorySecurity::clear_sensitive_env_vars();

    match cli.command {
        Commands::Encrypt { file, password, algorithm, backup, dry_run } => {
            let password = get_password(password);
            if password.is_empty() {
                eprintln!("Error: Password cannot be empty");
                process::exit(1);
            }

            let algo = resolve_algorithm(algorithm.as_deref(), &config, &registry);
            let path = Path::new(&file);

            println!("Encrypting file: {} [{}]", file, algo.display_name());

            let file_data = file_ops::atomic_read_file(path)?;

            let encrypted = encryption::encrypt_data(&file_data, password.as_str(), algo)
                .map_err(|e| anyhow::anyhow!("Encryption failed: {}", e))?;

            if dry_run {
                println!("Dry run: Would encrypt and replace: {}", file);
                return Ok(());
            }

            if backup {
                match file_ops::create_backup(path) {
                    Ok(bp) => println!("Created backup: {}", bp.display()),
                    Err(e) => eprintln!("Warning: Could not create backup: {}", e),
                }
            }

            file_ops::atomic_write_file(path, encrypted.as_bytes())?;
            println!("Successfully encrypted: {}", file);
        }

        Commands::Decrypt { file, password, backup, dry_run } => {
            let password = get_password(password);
            if password.is_empty() {
                eprintln!("Error: Password cannot be empty");
                process::exit(1);
            }

            let path = Path::new(&file);
            println!("Decrypting file: {}", file);

            let decrypted = decryption::decrypt_file(&file, password.as_str(), &registry)
                .map_err(|e| anyhow::anyhow!("Decryption failed: {}", e))?;

            if dry_run {
                println!("Dry run: Would decrypt and replace: {}", file);
                return Ok(());
            }

            if backup {
                match file_ops::create_backup(path) {
                    Ok(bp) => println!("Created backup: {}", bp.display()),
                    Err(e) => eprintln!("Warning: Could not create backup: {}", e),
                }
            }

            file_ops::atomic_write_file(path, decrypted.as_bytes())?;
            println!("Successfully decrypted: {}", file);
        }

        Commands::EncryptDir { dir, password, algorithm, backup, dry_run } => {
            let password = get_password(password);
            if password.is_empty() {
                eprintln!("Error: Password cannot be empty");
                process::exit(1);
            }

            let algo = resolve_algorithm(algorithm.as_deref(), &config, &registry);
            let dir_path = Path::new(&dir);

            if !dir_path.is_dir() {
                eprintln!("Error: '{}' is not a valid directory", dir);
                process::exit(1);
            }

            let files = file_ops::collect_files(dir_path)?;
            if files.is_empty() {
                eprintln!("No files found in directory: {}", dir);
                return Ok(());
            }

            println!("Found {} files to encrypt [{}]", files.len(), algo.display_name());

            let mut success_count = 0;
            let mut error_count = 0;

            for (i, file_path) in files.iter().enumerate() {
                match file_ops::atomic_read_file(file_path) {
                    Ok(file_data) => {
                        match encryption::encrypt_data(&file_data, password.as_str(), algo) {
                            Ok(encrypted) => {
                                if dry_run {
                                    println!("[{}/{}] Dry run: Would encrypt: {}", i + 1, files.len(), file_path.display());
                                    success_count += 1;
                                    continue;
                                }

                                if backup {
                                    if let Err(e) = file_ops::create_backup(file_path) {
                                        eprintln!("[{}/{}] Warning: Could not create backup for '{}': {}",
                                                 i + 1, files.len(), file_path.display(), e);
                                    }
                                }

                                match file_ops::atomic_write_file(file_path, encrypted.as_bytes()) {
                                    Ok(_) => {
                                        println!("[{}/{}] Encrypted: {}", i + 1, files.len(), file_path.display());
                                        success_count += 1;
                                    }
                                    Err(e) => {
                                        eprintln!("[{}/{}] Error writing: {} - {}", i + 1, files.len(), file_path.display(), e);
                                        error_count += 1;
                                    }
                                }
                            }
                            Err(e) => {
                                eprintln!("[{}/{}] Error encrypting: {} - {}", i + 1, files.len(), file_path.display(), e);
                                error_count += 1;
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("[{}/{}] Error reading: {} - {}", i + 1, files.len(), file_path.display(), e);
                        error_count += 1;
                    }
                }
            }

            println!("\nSummary: {} encrypted, {} errors", success_count, error_count);
        }

        Commands::DecryptDir { dir, password, backup, dry_run } => {
            let password = get_password(password);
            if password.is_empty() {
                eprintln!("Error: Password cannot be empty");
                process::exit(1);
            }

            let dir_path = Path::new(&dir);

            if !dir_path.is_dir() {
                eprintln!("Error: '{}' is not a valid directory", dir);
                process::exit(1);
            }

            let files = file_ops::collect_files(dir_path)?;
            if files.is_empty() {
                eprintln!("No files found in directory: {}", dir);
                return Ok(());
            }

            println!("Found {} files to decrypt", files.len());

            let mut success_count = 0;
            let mut error_count = 0;

            for (i, file_path) in files.iter().enumerate() {
                match file_path.to_str() {
                    Some(path_str) => {
                        match decryption::decrypt_file(path_str, password.as_str(), &registry) {
                            Ok(decrypted) => {
                                if dry_run {
                                    println!("[{}/{}] Dry run: Would decrypt: {}", i + 1, files.len(), file_path.display());
                                    success_count += 1;
                                    continue;
                                }

                                if backup {
                                    if let Err(e) = file_ops::create_backup(file_path) {
                                        eprintln!("[{}/{}] Warning: Could not create backup for '{}': {}",
                                                 i + 1, files.len(), file_path.display(), e);
                                    }
                                }

                                match file_ops::atomic_write_file(file_path, decrypted.as_bytes()) {
                                    Ok(_) => {
                                        println!("[{}/{}] Decrypted: {}", i + 1, files.len(), file_path.display());
                                        success_count += 1;
                                    }
                                    Err(e) => {
                                        eprintln!("[{}/{}] Error writing: {} - {}", i + 1, files.len(), file_path.display(), e);
                                        error_count += 1;
                                    }
                                }
                            }
                            Err(_e) => {
                                eprintln!("[{}/{}] Error decrypting: {} - incorrect password or corrupted file",
                                         i + 1, files.len(), file_path.display());
                                error_count += 1;
                            }
                        }
                    }
                    None => {
                        eprintln!("[{}/{}] Error: Invalid file path (non-UTF8)", i + 1, files.len());
                        error_count += 1;
                    }
                }
            }

            println!("\nSummary: {} decrypted, {} errors", success_count, error_count);
        }

        Commands::Config { show, set } => {
            if show || set.is_none() {
                let config = Config::load();
                let toml_str = toml::to_string_pretty(&config)?;
                println!("Configuration (from {:?}):\n", Config::config_file_path());
                println!("{}", toml_str);
            }

            if let Some(kv) = set {
                let parts: Vec<&str> = kv.splitn(2, '=').collect();
                if parts.len() != 2 {
                    eprintln!("Invalid format. Use: --set key=value");
                    eprintln!("Example: --set encryption.algorithm=aes-256-gcm");
                    process::exit(1);
                }

                let mut config = Config::load();
                match parts[0] {
                    "encryption.algorithm" => config.encryption.algorithm = parts[1].to_string(),
                    "encryption.iterations" => config.encryption.iterations = parts[1].to_string(),
                    "security.secure_wipe" => config.security.secure_wipe = parts[1].parse().unwrap_or(true),
                    "security.backup_on_encrypt" => config.security.backup_on_encrypt = parts[1].parse().unwrap_or(false),
                    "ui.theme" => config.ui.theme = parts[1].to_string(),
                    other => {
                        eprintln!("Unknown config key: {}", other);
                        process::exit(1);
                    }
                }

                config.save().map_err(|e| anyhow::anyhow!("Failed to save config: {}", e))?;
                println!("Config updated: {} = {}", parts[0], parts[1]);
            }
        }

        Commands::Algorithms => {
            println!("Available encryption algorithms:\n");
            for info in registry.list() {
                let aead_tag = if info.is_aead { "AEAD" } else { "requires HMAC" };
                println!("  {:<22} {} (key: {}B, nonce: {}B, {})",
                    info.id, info.display_name, info.key_size, info.nonce_size, aead_tag);
            }
            println!("\nDefault: {}", config.encryption.algorithm);
            println!("Change with: simplecrypt config --set encryption.algorithm=<id>");
        }
    }

    MemorySecurity::clear_sensitive_env_vars();
    Ok(())
}
