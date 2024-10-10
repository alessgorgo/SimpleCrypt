# Encryptix - Documentation

## Overview
**Encryptix** is a lightweight terminal-based tool for encrypting and decrypting files and directories using **AES-256** encryption. Packaged as a shell script (`Encryptix.sh`), it provides a simple yet secure method for protecting sensitive data directly from the command line. It supports password-based encryption with **PBKDF2** key derivation for enhanced security and provides user-friendly logging and backup functionalities.

---

## Features

- **File Encryption/Decryption**: Encrypt individual files with AES-256 encryption and securely decrypt them when needed.
- **Directory Encryption/Decryption**: Encrypt or decrypt entire directories, simplifying batch processing of files.
- **Alias Commands**: Simple aliases (`nc`, `dc`, `ncdir`, `dcdir`) to streamline the encryption/decryption process.
- **Backup Creation**: Prompt to create a backup before encrypting a file, safeguarding original content.
- **Secure Logging**: Operations are logged in `$HOME/file_encryption.log` for tracking encryption and decryption events.
- **Password Security**: Utilizes **PBKDF2** for key derivation, improving security over SHA-256.
- **Environment Variable for Password**: Option to use the `$PASSWORD` environment variable to handle passwords more securely.
- **Interactive Password Input**: Secure, masked password input with confirmation for added assurance.

---

## Getting Started

### Prerequisites

Ensure that your system has the following tools installed:

- **OpenSSL**: For encryption and decryption.
- **jq**: For handling JSON data during the encryption process.

To install these dependencies:
```bash
sudo apt install openssl jq
```

### Installation

1. Clone the repository:
    ```bash
    git clone https://github.com/alessgorgo/encryptix.git
    ```

2. Navigate into the project directory:
    ```bash
    cd encryptix
    ```

3. Make the script executable:
    ```bash
    chmod +x Encryptix.sh
    ```

4. Optionally, move it to your `/usr/local/bin` for global access:
    ```bash
    sudo mv Encryptix.sh /usr/local/bin/encryptix
    ```

---

## Usage

Encryptix supports several operations for encryption and decryption of files and directories. Here's a breakdown of the available commands:

### Command Structure

```bash
./Encryptix.sh {operation} {file/directory}
```

### Operations

| Command        | Description                                        |
|----------------|----------------------------------------------------|
| `nc`           | Encrypt a single file (with backup prompt).         |
| `dc`           | Decrypt a single file (with backup).                |
| `ncdir`        | Encrypt all files in a directory.                   |
| `dcdir`        | Decrypt all files in a directory.                   |
| `encrypt`      | Alias for `nc`. Encrypt a single file.              |
| `decrypt`      | Alias for `dc`. Decrypt a single file.              |
| `encrypt-dir`  | Alias for `ncdir`. Encrypt an entire directory.     |
| `decrypt-dir`  | Alias for `dcdir`. Decrypt an entire directory.     |

### Example Usage

#### Encrypt a File

```bash
./Encryptix.sh nc myfile.txt
```

You will be prompted for a passkey and whether you would like to create a backup of the file.

#### Decrypt a File

```bash
./Encryptix.sh dc myfile.txt
```

Provide the same passkey used during encryption.

#### Encrypt All Files in a Directory

```bash
./Encryptix.sh ncdir /path/to/myfolder
```

#### Decrypt All Files in a Directory

```bash
./Encryptix.sh dcdir /path/to/myfolder
```

---

## Detailed Options

### Password Handling

By default, the script will prompt for a password interactively. To securely pass the password via an environment variable, you can set the `$PASSWORD` variable before running the command:

```bash
export PASSWORD="my_secret_password"
./Encryptix.sh nc myfile.txt
```

### Logging

Every action (encrypt/decrypt) is logged in a file located at:

```bash
$HOME/file_encryption.log
```

To view the log directory, you can use the following command:

```bash
./Encryptix.sh --log-dir
```

---

## Backup Creation

When encrypting files, the script will prompt you to create a backup:

```bash
Do you want to create a backup of myfile.txt before encrypting? (y/n):
```

If you select "yes," a backup of the file will be created with a `.bak` extension:

```bash
myfile.txt.bak
```

The original file will be replaced by the encrypted version.

---

## Security Features

1. **AES-256 Encryption**: Encryptix uses AES-256-CBC mode to ensure a high level of data protection.
2. **PBKDF2 Key Derivation**: For better security, passkeys are processed using PBKDF2 (Password-Based Key Derivation Function 2) with 100,000 iterations. This method strengthens the encryption by making brute-force attacks more difficult.
3. **Environment Variable**: Password can be passed via the `$PASSWORD` environment variable for automation or script integration, eliminating the need for interactive input in some scenarios.

---

## Error Handling

### Common Errors and Solutions

- **`openssl: Extra (unknown) options: "kdf_iter" "100000"`**  
  Ensure your OpenSSL version supports PBKDF2. You may need to upgrade your OpenSSL version if it's outdated.

- **`mktemp: mkstemp failed on /dev/shm/...`**  
  Ensure that the `/dev/shm/` directory exists and has appropriate permissions, or modify the `mktemp` command to use a different directory.

- **`Decryption failed. Please check your password.`**  
  This error typically occurs if the wrong password is provided during decryption.

---

## FAQ

### Can I use this script on Windows?

Encryptix is designed for Unix-based systems (Linux and macOS). For Windows, you can use WSL (Windows Subsystem for Linux) to run the script.

### Is my data truly secure?

Encryptix uses AES-256-CBC encryption, which is widely regarded as secure. The use of PBKDF2 for key derivation further enhances the protection by making password guessing significantly harder. However, ensuring the security of your passkey is crucial.

### How can I automate file encryption?

You can automate file encryption/decryption by using the `$PASSWORD` environment variable and integrating Encryptix into your existing shell scripts.

---

## License

Encryptix is licensed under the MIT License. See the [LICENSE](./LICENSE) file for more details.

---

## Changelog

### v1.2-beta.1

+ Added alias support for operations: `nc`, `dc`, `ncdir`, `dcdir`.
+ Implemented logging functionality: logs are saved in `$HOME/file_encryption.log`.
+ Prompted backup creation before encryption.
+ Updated password prompt to "Enter passkey:".
+ Added support for password via `$PASSWORD` environment variable.
+ Improved security by using PBKDF2 for key derivation instead of SHA-256.
/ Internal management of alias operations, removing external alias dependency.
/ Log files are now securely stored in the home directory.
/ `process_files_in_directory` handles both encryption and decryption.
/ Removed outdated use of SHA-256 in favor of PBKDF2 for enhanced security.
- Deprecated old logging methods that did not track encryption/decryption actions.

---

Encryptix is a simple, powerful tool for safeguarding your data. Whether you’re a developer, sysadmin, or privacy-conscious user, Encryptix will help ensure your files remain safe and secure.
