
<img src="public/img/banner.png" alt="simplecrypt">
<a href="https://github.com/alessgorgo/SimpleCrypt/releases"><img src="https://img.shields.io/github/tag-pre/alessgorgo/SimpleCrypt.svg?color=gray" alt="Last Version" /></a>
<a href="https://github.com/alessgorgo/SimpleCrypt/blob/SimpleCrypt-log/LICENSE"><img src="https://img.shields.io/badge/MIT%20License-606060" alt="License" /></a>
<a href="https://github.com/alessgorgo/SimpleCrypt/stargazers"><img src="https://img.shields.io/github/stars/alessgorgo/SimpleCrypt?color=gray&logo=github" alt="Github star" /></a> <br>
  
# Documentation

## Overview

SimpleCrypt v1.5 is a high-performance Rust-based command-line application that provides enterprise-grade file and directory encryption using AES-256-CBC with adaptive PBKDF2 key derivation. It features advanced outlier detection, comprehensive security measures, and optimized performance for both individual files and entire directories with real-time progress feedback.

---

**Important**: Due to security improvements, versions before **SimpleCrypt v1.5** use different encryption formats. Backward compatibility is maintained for v1.0 encrypted files.

---

## Features

- **AES-256-CBC Encryption**: Industry-standard symmetric encryption with HMAC-SHA256 verification
- **Adaptive PBKDF2 Key Derivation**: Intelligent scaling from 10K to 600K iterations based on file size
- **Advanced Outlier Detection**: Enterprise-grade performance monitoring with ≤2% outlier rate
- **File & Directory Support**: Encrypt individual files or entire directories recursively
- **Atomic Operations**: Safe file operations with automatic backup creation
- **Zero-Copy Performance**: Optimized JSON serialization for maximum throughput
- **Secure Memory**: Comprehensive memory wiping using zeroize crate
- **Comprehensive Error Handling**: Detailed error messages with actionable guidance
- **Cross-Platform**: Works on macOS, Linux, and Windows

## Installation

### Prerequisites

- Rust and Cargo (latest stable version)
- OpenSSL development libraries

### System Dependencies

#### macOS
```bash
# Install OpenSSL using Homebrew
brew install openssl

# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

#### Ubuntu/Debian
```bash
# Install OpenSSL development libraries
sudo apt-get update
sudo apt-get install libssl-dev build-essential

# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

#### Fedora/CentOS
```bash
# Install OpenSSL development libraries
sudo dnf install openssl-devel gcc

# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

#### Windows
```powershell
# Install OpenSSL using Chocolatey (as Administrator)
choco install openssl

# Install Rust (if not already installed)
iwr -useb -uri https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe | iex
```

### Build Instructions

1. **Clone or download the project**
   ```bash
   # If using git
   git clone <repository-url>
   cd SimpleCrypt
   
   # Or extract the project files
   ```

2. **Build the application**
   ```bash
   # Build in release mode (optimized)
   cargo build --release
   
   # Or build in debug mode (faster compilation)
   cargo build
   ```

3. **Run the application**
   ```bash
   # From the project directory
   cargo run -- <command> <arguments>
   
   # Or use the compiled binary
   ./target/release/simplecrypt <command> <arguments>
   ```

## Usage

### Basic Commands

#### Encrypt a Single File
```bash
cargo run -- encrypt document.txt mysecurepassword
```

#### Decrypt a Single File
```bash
cargo run -- decrypt document.txt mysecurepassword
```

#### Encrypt All Files in a Directory
```bash
cargo run -- encrypt-dir /path/to/directory mysecurepassword
```

#### Decrypt All Files in a Directory
```bash
cargo run -- decrypt-dir /path/to/directory mysecurepassword
```

#### Show Help
```bash
cargo run -- --help
```

### Advanced Options

#### With Backup Creation
```bash
cargo run -- encrypt document.txt mysecurepassword --backup
```

#### Dry Run (Preview Operations)
```bash
cargo run -- encrypt document.txt mysecurepassword --dry-run
```

### Command Reference

#### `encrypt [FILE] [PASSWORD] [OPTIONS]`
Encrypts a single file using the provided password.

**Arguments:**
- `FILE`: Path to the file to encrypt
- `PASSWORD`: Secure password for encryption

**Options:**
- `--backup`: Create backup before encryption
- `--dry-run`: Preview operation without making changes

**Example:**
```bash
cargo run -- encrypt sensitive_data.txt "MyP@ssw0rd123!" --backup
```

#### `decrypt [FILE] [PASSWORD] [OPTIONS]`
Decrypts a single file using the provided password.

**Arguments:**
- `FILE`: Path to the encrypted file to decrypt
- `PASSWORD`: Password used for encryption

**Options:**
- `--backup`: Create backup before decryption
- `--dry-run`: Preview operation without making changes

**Example:**
```bash
cargo run -- decrypt sensitive_data.txt "MyP@ssw0rd123!"
```

#### `encrypt-dir [DIRECTORY] [PASSWORD]`
Encrypts all files in a directory recursively.

**Arguments:**
- `DIRECTORY`: Path to the directory to encrypt
- `PASSWORD`: Secure password for encryption

**Example:**
```bash
cargo run -- encrypt-dir ./my_documents "MyP@ssw0rd123!"
```

#### `decrypt-dir [DIRECTORY] [PASSWORD]`
Decrypts all files in a directory recursively.

**Arguments:**
- `DIRECTORY`: Path to the directory containing encrypted files
- `PASSWORD`: Password used for encryption

**Example:**
```bash
cargo run -- decrypt-dir ./my_documents "MyP@ssw0rd123!"
```

### Security Best Practices

#### Password Selection
- Use strong passwords (minimum 12 characters)
- Include a mix of uppercase, lowercase, numbers, and special characters
- Avoid common words or predictable patterns
- Use a unique password for each encryption operation

#### Password Management
- Never store passwords in plain text
- Consider using a password manager
- Be careful when typing passwords (watch for shoulder surfing)
- Use environment variables for better security (see advanced usage)

#### File Handling
- Always verify decrypted files before deleting originals
- Keep backups of important files before encryption
- Use version control for important directories
- Test encryption/decryption with non-critical files first

### Advanced Usage

#### Using Environment Variables for Passwords
```bash
# Set password in environment variable
export SIMPLECRYPT_PASSWORD="MyP@ssw0rd123!"

# Use the environment variable
cargo run -- encrypt document.txt $SIMPLECRYPT_PASSWORD
```

#### Scripting and Automation
```bash
#!/bin/bash
# encrypt_script.sh

PASSWORD="SecureScriptPassword2024"
SOURCE_DIR="/path/to/source"
DEST_DIR="/path/to/destination"

# Create destination directory if it doesn't exist
mkdir -p "$DEST_DIR"

# Encrypt all files in source directory
for file in "$SOURCE_DIR"/*; do
    if [ -f "$file" ]; then
        filename=$(basename "$file")
        echo "Encrypting $filename..."
        cargo run -- encrypt "$file" "$PASSWORD" --backup
        mv "$file" "$DEST_DIR/"
    fi
done

echo "Encryption completed successfully!"
```

#### Batch Processing with Error Handling
```bash
#!/bin/bash
# batch_decrypt.sh

PASSWORD="MySecretPassword"
ENCRYPTED_DIR="./encrypted_files"
DECRYPTED_DIR="./decrypted_files"

# Create output directory
mkdir -p "$DECRYPTED_DIR"

# Process files with error handling
for file in "$ENCRYPTED_DIR"/*; do
    if [ -f "$file" ]; then
        filename=$(basename "$file")
        echo "Processing $filename..."
        
        if cargo run -- decrypt "$file" "$PASSWORD" > /dev/null 2>&1; then
            echo "✅ Successfully decrypted $filename"
            mv "$file" "$DECRYPTED_DIR/"
        else
            echo "❌ Failed to decrypt $filename - check password or file integrity"
        fi
    fi
done
```

## Technical Details

### Encryption Specifications

- **Algorithm**: AES-256-CBC (Cipher Block Chaining)
- **Key Derivation**: PBKDF2 with SHA-256 (adaptive iterations)
- **Iterations**: 10K-600K based on file size for optimal performance
- **Salt Length**: 16 bytes (cryptographically secure random)
- **IV Length**: 16 bytes (cryptographically secure random)
- **Authentication**: HMAC-SHA256 for integrity verification
- **Output Format**: JSON-encoded structure with version support

### Adaptive Iteration Scaling

| File Size | PBKDF2 Iterations | Security Level | Performance |
|-----------|-------------------|----------------|-------------|
| ≤1KB | 10,000 | Good | Excellent |
| ≤10KB | 25,000 | Strong | Very Good |
| ≤100KB | 50,000 | Very Strong | Good |
| ≤1MB | 100,000 | Excellent | Fair |
| >1MB | 600,000 | Maximum | Acceptable |

### Output Format

Encrypted files contain a JSON structure with the following fields:

```json
{
  "version": "2.0",
  "salt": "base64-encoded-salt",
  "iv": "base64-encoded-iv", 
  "data": "base64-encoded-encrypted-data",
  "hmac": "base64-encoded-hmac-signature"
}
```

### Security Considerations

1. **Memory Security**: Sensitive data (keys, salts, IVs) is zeroed from memory after use
2. **No Key Storage**: Keys are derived from passwords and never stored permanently
3. **Random Values**: Salt and IV are generated using cryptographically secure random number generation
4. **HMAC Verification**: Integrity protection prevents tampering and corruption
5. **Constant-Time Comparisons**: Prevents timing attacks in password verification
6. **Atomic Operations**: Prevents file corruption during encryption/decryption

### Performance Optimizations

- **Zero-Copy Serialization**: Eliminates intermediate allocations
- **Smart I/O**: Direct writes for small files, atomic operations for large files
- **Pre-allocated Buffers**: Reduces memory reallocations
- **Stack Allocation**: Fixed-size data uses stack instead of heap
- **Adaptive Security**: Optimizes iterations based on file size

## Troubleshooting

### Common Issues

#### "Error: Password cannot be empty"
**Cause**: Empty password provided
**Solution**: Use a non-empty password with minimum 12 characters

#### "Error: File not found"
**Cause**: Specified file or directory doesn't exist
**Solution**: Verify the path is correct and file exists

#### "Error decrypting file: HMAC verification failed"
**Cause**: File tampering, corruption, or incorrect password
**Solution**: Verify password and file integrity, check for unauthorized modifications

#### "Permission denied"
**Cause**: Insufficient permissions to read/write files
**Solution**: Check file permissions and run with appropriate privileges

#### "Failed to read directory"
**Cause**: Directory doesn't exist or insufficient permissions
**Solution**: Verify directory path and permissions

### Debug Information

For detailed debugging, you can enable verbose output:

```bash
# Enable Rust backtrace on errors
RUST_BACKTRACE=1 cargo run -- encrypt file.txt password

# Check file permissions
ls -la file.txt

# Verify OpenSSL installation
openssl version

# Performance benchmarking
cargo bench
```

## Testing

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with verbose output
cargo test -- --nocapture

# Run specific test
cargo test test_single_file_encryption_decryption

# Run integration tests
cargo test --test integration_tests
```

### Test Coverage

The test suite includes:
- Single file encryption/decryption (15 tests total)
- Directory encryption/decryption with progress tracking
- Wrong password validation and HMAC verification
- Empty password validation
- Help command functionality
- Backward compatibility with v1.0 format
- Atomic operations and backup creation
- Dry run mode validation

## Performance

### Benchmarks

| Operation | Average Time | Memory Usage | Notes |
|-----------|--------------|--------------|-------|
| 1KB Encryption | ~5ms | 4.6MB | 10K iterations |
| 10KB Encryption | ~12ms | 4.8MB | 25K iterations |
| 100KB Encryption | ~25ms | 5.2MB | 50K iterations |
| 1MB Encryption | ~95ms | 6.8MB | 100K iterations |
| Outlier Detection | <10ms | Minimal | 500 measurements |

### Optimization Features

- **1.3% performance improvement** through zero-copy serialization
- **60-80% faster** small file operations with adaptive iterations
- **Zero compilation warnings** for clean, maintainable code
- **Sub-10ms outlier detection** for real-time monitoring

## Contributing

### Development Setup

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for new functionality
5. Ensure all tests pass
6. Submit a pull request

### Code Style

- Follow Rust standard formatting: `cargo fmt`
- Run clippy lints: `cargo clippy`
- Ensure comprehensive test coverage
- Update documentation for new features
- Maintain zero compilation warnings

## License

This project is licensed under the MIT License. See the LICENSE file for details.

## Support

For issues, questions, or contributions:
1. Check the troubleshooting section
2. Review existing issues
3. Create a new issue with detailed information
4. Include system information and error messages

## Version History

- **v1.0.0**: Initial release with basic encryption/decryption functionality
- **v1.1.0**: Added directory operations and progress feedback
- **v1.2.0**: Implemented PBKDF2 key derivation and comprehensive error handling
- **v1.3.0**: Added secure memory practices and integration tests
- **v1.4.0**: Performance optimizations and outlier detection system
- **v1.5.0**: Adaptive security scaling, atomic operations, and enterprise-grade reliability

---

**Disclaimer**: This software is provided "as is" without warranty. Always backup important files before encryption operations. The authors are not responsible for any data loss or security breaches resulting from the use of this software.

---

## **Complete Installation Process:**

### **System Dependencies:**
- **macOS**: OpenSSL via Homebrew + Rust
- **Ubuntu/Debian**: `libssl-dev` + Rust
- **Fedora/CentOS**: `openssl-devel` + Rust  
- **Windows**: OpenSSL via Chocolatey + Rust

### **Build Instructions:**
1. Install prerequisites (OpenSSL + Rust)
2. Clone/download project
3. Build with `cargo build --release`
4. Run with `cargo run -- <command> <arguments>`

## **Detailed Usage Guide:**

### **Basic Commands:**
- `encrypt [FILE] [PASSWORD]` - Single file encryption
- `decrypt [FILE] [PASSWORD]` - Single file decryption
- `encrypt-dir [DIRECTORY] [PASSWORD]` - Directory encryption
- `decrypt-dir [DIRECTORY] [PASSWORD]` - Directory decryption
- `--backup` - Create backup before operation
- `--dry-run` - Preview operation without changes
- `--help` - Show help

### **Security Best Practices:**
- Strong password requirements (12+ chars, mixed characters)
- Password management recommendations
- File handling best practices
- Environment variable usage for security

### **Advanced Usage:**
- Scripting and automation examples
- Batch processing with error handling
- Environment variable integration
- Performance considerations

### **Technical Specifications:**
- **Algorithm**: AES-256-CBC with PBKDF2-SHA256
- **Key Derivation**: 10K-600K adaptive iterations
- **Authentication**: HMAC-SHA256 integrity verification
- **Output Format**: JSON-encoded structure with version support
- **Memory Security**: Secure data wiping with zeroize

### **Performance Features:**
- **Adaptive Security**: Optimized iterations based on file size
- **Zero-Copy Serialization**: Eliminates intermediate allocations
- **Smart I/O**: Conditional atomic operations
- **Outlier Detection**: Enterprise-grade performance monitoring

### **Troubleshooting:**
- Common error messages and solutions
- Debug information and environment setup
- Permission issues and file handling
- HMAC verification and integrity checking

### **Testing:**
- Test suite coverage (15 comprehensive tests)
- Running instructions and benchmarking
- Integration tests and backward compatibility
- Performance validation and optimization verification

