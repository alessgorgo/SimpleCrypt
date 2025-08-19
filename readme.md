
<img src="public/img/banner.png" alt="simplecrypt">
<a href="https://github.com/alessgorgo/SimpleCrypt/releases"><img src="https://img.shields.io/github/tag-pre/alessgorgo/SimpleCrypt.svg?color=gray" alt="Last Version" /></a>
<a href="https://github.com/alessgorgo/SimpleCrypt/blob/SimpleCrypt-log/LICENSE"><img src="https://img.shields.io/badge/MIT%20License-606060" alt="License" /></a>
<a href="https://github.com/alessgorgo/SimpleCrypt/stargazers"><img src="https://img.shields.io/github/stars/alessgorgo/SimpleCrypt?color=gray&logo=github" alt="Github star" /></a> <br>
  
# Documentation

## Overview

SimpleCrypt is a robust Rust-based command-line application that provides secure file and directory encryption using AES-256-CBC with PBKDF2 key derivation. It offers comprehensive features for both individual files and entire directories with progress feedback and secure memory practices.

---

**Important**: Due to earlier errors, versions before **SimpleCrypt v1.3 Beta 1** are no longer available.

---

## Features

- **AES-256-CBC Encryption**: Industry-standard symmetric encryption
- **PBKDF2 Key Derivation**: Secure password-based key derivation with 100,000 iterations
- **File & Directory Support**: Encrypt individual files or entire directories recursively
- **Progress Feedback**: Real-time progress indicators and completion summaries
- **Secure Memory**: Sensitive data is securely wiped from memory after operations
- **Comprehensive Error Handling**: Detailed error messages and validation
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

### Command Reference

#### `encrypt [FILE] [PASSWORD]`
Encrypts a single file using the provided password.

**Arguments:**
- `FILE`: Path to the file to encrypt
- `PASSWORD`: Secure password for encryption

**Example:**
```bash
cargo run -- encrypt sensitive_data.txt "MyP@ssw0rd123!"
```

#### `decrypt [FILE] [PASSWORD]`
Decrypts a single file using the provided password.

**Arguments:**
- `FILE`: Path to the encrypted file to decrypt
- `PASSWORD`: Password used for encryption

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
        cargo run -- encrypt "$file" "$PASSWORD"
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
- **Key Derivation**: PBKDF2 with SHA-256
- **Iterations**: 100,000 (for brute-force resistance)
- **Salt Length**: 16 bytes (randomly generated)
- **IV Length**: 16 bytes (randomly generated)
- **Output Format**: JSON-encoded structure

### Output Format

Encrypted files contain a JSON structure with the following fields:

```json
{
  "salt": "base64-encoded-salt",
  "iv": "base64-encoded-iv", 
  "data": "base64-encoded-encrypted-data"
}
```

### Security Considerations

1. **Memory Security**: Sensitive data (keys, salts, IVs) is zeroed from memory after use
2. **No Key Storage**: Keys are derived from passwords and never stored permanently
3. **Random Values**: Salt and IV are generated using cryptographically secure random number generation
4. **Error Handling**: Comprehensive validation prevents security vulnerabilities from malformed input

### Performance Considerations

- **PBKDF2 Iterations**: Higher iterations increase security but slow down operations
- **Large Files**: Memory usage scales with file size (entire file loaded into memory)
- **Directory Operations**: Progress indicators help monitor long-running operations

## Troubleshooting

### Common Issues

#### "Error: Password cannot be empty"
**Cause**: Empty password provided
**Solution**: Use a non-empty password

#### "Error: File not found"
**Cause**: Specified file or directory doesn't exist
**Solution**: Verify the path is correct and file exists

#### "Error decrypting file: This could be due to incorrect password or corrupted file"
**Cause**: Wrong password or corrupted encrypted file
**Solution**: Verify password and file integrity

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
```

### Test Coverage

The test suite includes:
- Single file encryption/decryption
- Directory encryption/decryption
- Wrong password validation
- Empty password validation
- Help command functionality

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
- **Key Derivation**: 100,000 iterations
- **Output Format**: JSON-encoded structure
- **Memory Security**: Secure data wiping practices

### **Troubleshooting:**
- Common error messages and solutions
- Debug information
- Permission issues
- File corruption handling

### **Testing:**
- Test suite coverage (5 comprehensive tests)
- Running instructions
- Test scenarios covered
