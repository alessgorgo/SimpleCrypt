# 🛡️ Encryptix: AES-256 Encryption and Decryption Script Documentation

This documentation provides a comprehensive guide to using **Encryptix**, an AES-256 encryption and decryption script. Encryptix enables users to securely encrypt and decrypt files or directories using AES-256 encryption to protect sensitive data.

---

## 📥 Importing the Script from GitHub

1. **Clone the Repository**:

   To get the script from GitHub, open your terminal and run:

   ```bash
   git clone https://github.com/alessgorgo/Encryptix.git
   ```

2. **Navigate to the Directory**:

   After cloning, navigate into the repository folder:

   ```bash
   cd Encryptix
   ```

3. **Make the Script Executable**:

   Ensure the script is executable by running:

   ```bash
   chmod +x Encryptix.sh
   ```

---

## 🛠️ Script Overview

Encryptix provides two main functionalities:

1. **Encrypt**: Encrypts a file or an entire directory using AES-256 encryption.
2. **Decrypt**: Decrypts a previously encrypted file or directory.

The encryption process outputs a JSON file containing the encrypted data and initialization vector (IV) used for encryption.

---

## 📝 How to Use the Script

### General Usage

You can run the script from the terminal using the following syntax:

```bash
./Encryptix.sh {encrypt|decrypt|encrypt_dir|decrypt_dir} input_file_or_path
```

- `{encrypt}`: Encrypt a single file.
- `{decrypt}`: Decrypt a single file.
- `{encrypt_dir}`: Encrypt all files in a directory.
- `{decrypt_dir}`: Decrypt all files in a directory.

### Example Commands

#### **Encrypting a File**

```bash
./Encryptix.sh encrypt sample.txt
```

- **Input**: The `sample.txt` file (or any file type) will be encrypted.
- **Output**: The original `sample.txt` will be replaced with an encrypted version, and the original file will no longer exist.

#### **Decrypting a File**

```bash
./Encryptix.sh decrypt sample.txt
```

- **Input**: The encrypted `sample.txt` file (or any file type) will be decrypted.
- **Output**: The original file content will replace the encrypted file, and the `.txt` file will be restored.

#### **Encrypting a Directory**

```bash
./Encryptix.sh encrypt_dir "my folder"
```

- **Input**: Encrypts all files (of any type) in the directory `my folder`, even if the path contains spaces.
- **Output**: Each file in the directory will be replaced with its encrypted version.

#### **Decrypting a Directory**

```bash
./Encryptix.sh decrypt_dir "my folder"
```

- **Input**: Decrypts all encrypted files (of any type) in the directory `my folder`.
- **Output**: Each file in the directory will be replaced with its original, decrypted content.

---

## ⚙️ Command Structure

1. **Encrypt a File**: Encrypts a single file and replaces it with its encrypted version.

   ```bash
   ./Encryptix.sh encrypt input_file
   ```

2. **Decrypt a File**: Decrypts an encrypted file and replaces it with the original content.

   ```bash
   ./Encryptix.sh decrypt input_file
   ```

3. **Encrypt a Directory**: Recursively encrypts all files (of any type) in a directory and replaces them with encrypted versions.

   ```bash
   ./Encryptix.sh encrypt_dir input_directory
   ```

4. **Decrypt a Directory**: Recursively decrypts all encrypted files (of any type) in a directory and replaces them with the original content.

   ```bash
   ./Encryptix.sh decrypt_dir input_directory
   ```

---

## 💡 Important Notes

- **Input File Types**: The script accepts all kinds of files (e.g., text, images, documents) for encryption and decryption.

- **Password**: For both encryption and decryption operations, you will be prompted to enter a password. Ensure you use a strong password for security.

- **File Path Handling**: The script now supports file paths with spaces or special characters. Always enclose paths with spaces in quotes (`" "`).

- **Dependencies**: The script requires:
  - **Bash**: Make sure you have a Bash shell environment.
  - **OpenSSL**: The script uses OpenSSL for AES-256 encryption/decryption.
  - **jq**: For parsing JSON files.

  Install dependencies if they are missing:

  ```bash
  sudo apt-get install openssl jq  # On Linux systems
  ```

---

## 🛠️ Error Handling

- **Decryption Failure**: If decryption fails (e.g., due to incorrect password or corrupt encrypted file), the script will output an error:

  ```bash
  Error: Decryption failed. Please check your password and the encrypted file.
  ```

- **Invalid Input**: If the provided JSON file lacks an IV or encrypted data, the script will terminate with a validation error.

---

## 📁 File Structure

After running the script, the files will be organized as follows:

- **For File Encryption/Decryption**:
  - The original file will be replaced by the encrypted file or decrypted content.
  - Encrypted files are no longer saved as separate JSON files—they replace the original files directly.

- **For Directory Encryption/Decryption**:
  - Files in the input directory are replaced by their encrypted or decrypted versions.

---

## 🔐 Security Considerations

- **AES-256 Encryption**: This script uses **AES-256-CBC** encryption, which is a highly secure method to protect your files. Always store your passwords securely.

- **Passwords**: Never share your password. If it is lost, decrypting the encrypted files becomes impossible.

- **File Size**: There is no file size limitation, but encryption and decryption times may vary based on the size of files or directories.

---

## 📖 License

This script is open-source and provided for educational purposes. Use responsibly and in compliance with legal regulations regarding encryption and data protection. Always encrypt sensitive information securely!
