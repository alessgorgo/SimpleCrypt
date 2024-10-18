
<img src="public/img/banner.png" alt="simplecrypt">

# Documentation

## Overview
**SimpleCrypt** is a lightweight terminal-based tool for encrypting and decrypting files and directories using **AES-256** encryption. Packaged as a shell script (`simplecrypt.sh`), it offers a simple and secure method for protecting sensitive data directly from the command line. With password-based encryption using **Argon2id** for enhanced security, user-friendly logging, backup functionalities, and customizable options, SimpleCrypt simplifies the management of your data security.

---

**Important**: Due to earlier errors, versions before **SimpleCrypt v1.3 Beta 1** are no longer available.

---

## Features

- **File Encryption/Decryption**: Encrypt individual files with AES-256 encryption and decrypt them securely when needed.
- **Directory Encryption/Decryption**: Encrypt or decrypt entire directories, enabling batch processing of multiple files.
- **Alias Commands**: Simple aliases (`nc`, `dc`, `ncdir`, `dcdir`) make encryption and decryption faster and more intuitive.
- **Argon2id Key Derivation**: Utilizes **Argon2id**, a state-of-the-art memory-hard password-hashing algorithm, providing stronger protection compared to PBKDF2.
- **Secure Logging**: Encryption and decryption activities are logged in `$HOME/file_encryption.log`, ensuring traceability.
- **Automatic Config Management**: Creates a config file on the first run, allowing for customizable Argon2 parameters.
- **Password Complexity Checks**: Ensures that passwords meet specified complexity requirements to enhance security.
- **Password via Environment Variable**: Supports the `$PASSWORD` environment variable for secure, non-interactive operations.
- **Silent and Verbose Modes**: New logging flexibility with silent (`-s`) and verbose (`-v`) options.
- **Custom Temporary Directory**: Uses a specified `TMPDIR` for improved management of temporary files.
- **Enhanced Error Handling**: Specific exit codes and improved error messages for better user feedback.
- **Backup Option**: Allows users to create backups of original files during encryption and decryption processes.

---

## Supported Encryption Types (Experimental)

**SimpleCrypt** currently supports the following encryption types:

1. **AES-256-CBC**: The default encryption algorithm used for file and directory encryption. It is widely regarded for its security and efficiency.
  
2. **AES-256-GCM**: An experimental option that provides authenticated encryption with associated data (AEAD). This mode offers both confidentiality and integrity, making it suitable for scenarios where data integrity is crucial.

**Note**: The AES-256-GCM option is still in the experimental stage. Users are encouraged to test its functionality and report any issues.

---

## Supported Key Types and Wrapping Algorithms

### Key Types
- **RSA4K**: A 4096-bit RSA key used for public key encryption and decryption. This is the default key type and provides a high level of security.

### Wrapping Algorithms
- **RSA-OAEP-256**: The default key wrapping algorithm used in SimpleCrypt. It utilizes RSA encryption with Optimal Asymmetric Encryption Padding (OAEP) and SHA-256 for secure key management.

### Signing Algorithms
- **RS512**: An RSA signature algorithm using SHA-512. This is used for signing and verifying data integrity.

---

## Configuration File

**SimpleCrypt** creates a configuration file on the first run, which allows users to customize various parameters for encryption and decryption. Below are the default settings and explanations for each parameter:

### Default File Encryption Configuration

```bash
# Default public key path
PUBLIC_KEY_PATH="/Users/aleks/public_key.pem"

# Default private key path
PRIVATE_KEY_PATH="/Users/aleks/private_key.pem"

# Enable backup by default
ENABLE_BACKUP=true

# Verbose mode to show more details
VERBOSE=true

# Silent mode, no terminal output
SILENT=false

# Key types, algorithms, and operations
KEY=RSA4K
WRAP_UNWRAP=RSA-OAEP-256
SIGN_VERIFY=RS512

# Argon2 parameters (only relevant for AES)
ARGON2_TIME_COST=65536
ARGON2_MEMORY_COST=16
ARGON2_PARALLELISM=4
ARGON2_LENGTH=32
```

### Parameter Descriptions

- **PUBLIC_KEY_PATH**: Specifies the file path to the public key used for encryption. Modify this to point to your own public key.
  
- **PRIVATE_KEY_PATH**: Specifies the file path to the private key used for decryption. Modify this to point to your own private key.

- **ENABLE_BACKUP**: Determines whether to enable backups of original files during encryption and decryption. Set to `true` to create backups.

- **VERBOSE**: When set to `true`, the script will output more detailed information during operations. Set to `false` for minimal output.

- **SILENT**: If set to `true`, the script suppresses all terminal output. This is useful for automated scripts where output is not needed.

- **KEY**: Specifies the type of key used for encryption. In this case, it is set to `RSA4K`, which refers to a 4096-bit RSA key.

- **WRAP_UNWRAP**: Indicates the key wrapping/unwrapping algorithm used. Set to `RSA-OAEP-256` for secure key management.

- **SIGN_VERIFY**: Specifies the algorithm used for signing and verifying data. Set to `RS512`, which uses RSA with SHA-512.

- **ARGON2_TIME_COST**: Defines the time cost parameter for Argon2 key derivation, determining how long the hashing should take.

- **ARGON2_MEMORY_COST**: Specifies the amount of memory (in megabytes) allocated for Argon2 during key derivation.

- **ARGON2_PARALLELISM**: Sets the level of parallelism for Argon2 key derivation, indicating how many threads can be used.

- **ARGON2_LENGTH**: Specifies the desired length of the derived key in bytes.

---

## Getting Started

### Prerequisites

Ensure your system has the following tools installed:

- **OpenSSL**: For encryption and decryption.
- **jq**: For handling JSON data during encryption.
- **argon2**: For secure key derivation.

To install these dependencies:
```bash
sudo apt install openssl jq argon2
```

### Installation

1. Clone the repository:
    ```bash
    git clone https://github.com/alessgorgo/SimpleCrypt.git
    ```

2. Navigate into the project directory:
    ```bash
    cd SimpleCrypt
    ```

3. Make the script executable:
    ```bash
    chmod +x simplecrypt.sh
    ```

4. Optionally, move it to your `/usr/local/bin` for global access:
    ```bash
    sudo mv simplecrypt.sh /usr/local/bin/SimpleCrypt
    ```

---

## Usage

SimpleCrypt provides various operations for file and directory encryption/decryption. Here's how to use them:

### Command Structure

```bash
./SimpleCrypt.sh {operation} {file/directory} [options]
```

### Operations

| Command        | Description                                        |
|----------------|----------------------------------------------------|
| `nc`           | Encrypt a file.                                    |
| `dc`           | Decrypt a file.                                    |
| `ncdir`        | Encrypt all files in a directory.                 |
| `dcdir`        | Decrypt all files in a directory.                 |
| `encrypt`      | Alias for `nc`. Encrypt a file.                   |
| `decrypt`      | Alias for `dc`. Decrypt a file.                   |
| `encrypt-dir`  | Alias for `ncdir`. Encrypt all files in a directory.|
| `decrypt-dir`  | Alias for `dcdir`. Decrypt all files in a directory.|

### Example Usage

#### Encrypt a File

```bash
./SimpleCrypt.sh nc myfile.txt
```

You will be prompted for a passkey to encrypt the file.

#### Decrypt a File

```bash
./SimpleCrypt.sh dc myfile.txt
```

Provide the same passkey used during encryption.

#### Encrypt All Files in a Directory

```bash
./SimpleCrypt.sh ncdir /path/to/myfolder
```

#### Decrypt All Files in a Directory

```bash
./SimpleCrypt.sh dcdir /path/to/myfolder
```

---

## Options

### Password Handling

By default, SimpleCrypt will prompt for a password interactively. To securely pass a password using the `$PASSWORD` environment variable, you can do so like this:

```bash
export PASSWORD="my_secret_password"
./SimpleCrypt.sh nc myfile.txt
```

### Password Complexity Checks

To enhance security, passwords must meet specific complexity criteria, including a minimum length and inclusion of numbers, special characters, and uppercase/lowercase letters.

### Silent and Verbose Modes

- Use `-s` for silent mode to suppress output.
- Use `-v` for verbose mode for detailed logging.

Example:

```bash
./SimpleCrypt.sh nc myfile.txt -v
```

### Logging

Encryption and decryption actions are logged in:

```bash
$HOME/file_encryption.log
```

To locate the log directory:

```bash
./SimpleCrypt.sh --log-dir
```

---

## Security Features

1. **AES-256 Encryption**: SimpleCrypt uses AES-256-CBC to secure data

, a widely regarded and secure encryption method.
2. **Argon2id Key Derivation**: Passkeys are derived using **Argon2id**, a memory-hard algorithm that offers better resistance against brute-force attacks compared to PBKDF2.
3. **Automatic Config File Creation**: If the config file is not found, it is automatically created with default Argon2 parameters.
4. **Password Complexity Checks**: Passwords are validated for strength before processing.
5. **Password via Environment Variable**: The `$PASSWORD` environment variable can be used to securely pass passwords for automated operations.
6. **Encrypted Data in JSON**: Encrypted files now store their initialization vector (IV) within a JSON format for better portability.
7. **Backup Option**: Enables users to create backups of original files during encryption, ensuring data safety.

---

## Error Handling

### Common Errors and Solutions

- **`openssl: Extra (unknown) options: "kdf_iter" "100000"`**  
  Ensure your OpenSSL version supports Argon2id. If it’s outdated, you may need to upgrade OpenSSL.

- **`mktemp: mkstemp failed on /dev/shm/...`**  
  Verify that the `/dev/shm/` directory exists and has appropriate permissions, or modify the `mktemp` command to use another directory.

- **`Decryption failed. Please check your password.`**  
  This error usually occurs if the incorrect password is provided.

---

## FAQ

### Can I use this script on Windows?

SimpleCrypt is designed for Unix-based systems (Linux and macOS). On Windows, you can use WSL (Windows Subsystem for Linux) to run SimpleCrypt.

### How secure is my data?

SimpleCrypt uses AES-256-CBC encryption, recognized as highly secure. The Argon2id key derivation function further strengthens the protection by making password guessing significantly harder. The security of your passkey is critical to ensure full protection.

### Can I automate encryption tasks?

Yes, by using the `$PASSWORD` environment variable, you can automate file encryption and decryption in scripts without needing interactive password input.

---

## License

SimpleCrypt is licensed under the MIT License. See the [LICENSE](./LICENSE) file for more details.

---

## Changelog

### Release v1.3-beta.3

+ Fixed terminal freeze issue during password input by improving the `derive_key` function.
+ Corrected handling of Argon2id command for key derivation with the appropriate password and salt.
+ Added robust error handling and validation for encryption and decryption processes.
+ Implemented automatic config file creation if missing, with defaults for Argon2 parameters.
+ Improved debug logging for enhanced traceability of encryption and decryption operations.
+ Refined backup creation logic to ensure original files are preserved during encryption.
+ Enforced better cleanup of sensitive data in memory, enhancing security.
+ Introduced logging flexibility with silent (`-s`) and verbose (`-v`) options.
+ Enhanced password complexity checks to ensure strong security measures.

### Key Updates in Documentation:
- Added **Backup Option** to Features.
- Updated **Supported Encryption Types** section for experimental AES-256-GCM.
- Updated **Supported Key Types and Wrapping Algorithms** section with details.
- Updated **Security Features** to include backup functionalities.
- Added **Configuration File** section with detailed explanations of each parameter.
- Revised the **Changelog** to reflect the latest improvements.
