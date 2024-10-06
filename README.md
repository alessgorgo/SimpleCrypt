
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
./Encryptix.sh {encrypt|decrypt|encrypt_dir|decrypt_dir} input_path output_path
```

- `{encrypt}`: Encrypt a single file.
- `{decrypt}`: Decrypt a single file.
- `{encrypt_dir}`: Encrypt all files in a directory.
- `{decrypt_dir}`: Decrypt all files in a directory.

### Example Commands

#### **Encrypting a File**

```bash
./Encryptix.sh encrypt sample.txt encrypted_sample.json
```

- **Input**: The `sample.txt` file will be encrypted.
- **Output**: The encrypted data is stored in `encrypted_sample.json`.

#### **Decrypting a File**

```bash
./Encryptix.sh decrypt encrypted_sample.json decrypted_sample.txt
```

- **Input**: The `encrypted_sample.json` file will be decrypted.
- **Output**: The decrypted content is saved in `decrypted_sample.txt`.

#### **Encrypting a Directory**

```bash
./Encryptix.sh encrypt_dir my_directory encrypted_directory
```

- **Input**: Encrypts all files in the `my_directory`.
- **Output**: The encrypted files are saved as `.json` files in `encrypted_directory`.

#### **Decrypting a Directory**

```bash
./Encryptix.sh decrypt_dir encrypted_directory decrypted_directory
```

- **Input**: Decrypts all `.json` files from `encrypted_directory`.
- **Output**: Decrypted files are saved in `decrypted_directory`.

---

## ⚙️ Command Structure

1. **Encrypt a File**: Encrypts a single file and outputs a JSON file with encrypted data and IV.

   ```bash
   ./Encryptix.sh encrypt input_file output_file
   ```

2. **Decrypt a File**: Decrypts an encrypted JSON file and restores the original content.

   ```bash
   ./Encryptix.sh decrypt input_file output_file
   ```

3. **Encrypt a Directory**: Recursively encrypts all files in a directory and outputs each file as an encrypted JSON.

   ```bash
   ./Encryptix.sh encrypt_dir input_directory output_directory
   ```

4. **Decrypt a Directory**: Recursively decrypts all JSON files in a directory and restores the original files.

   ```bash
   ./Encryptix.sh decrypt_dir input_directory output_directory
   ```

---

## 💡 Important Notes

- **Password**: For both encryption and decryption operations, you will be prompted to enter a password. Ensure you use a strong password for security.

- **JSON Output**: Encrypted files are saved in JSON format, containing both the encrypted data and IV used for decryption.

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
  - The original file remains unchanged.
  - Encrypted content is saved in a JSON file (`output_file.json`).
  - Decrypted content is saved with a specified file extension (e.g., `.txt`, `.doc`).

- **For Directory Encryption/Decryption**:
  - The encrypted files will be saved in the specified output directory with a `.json` extension.
  - Decrypted files will be restored to their original format and saved in the output directory.

---

## 🔐 Security Considerations

- **AES-256 Encryption**: This script uses **AES-256-CBC** encryption, which is a highly secure method to protect your files. Always store your passwords securely.

- **Passwords**: Never share your password. If it is lost, decrypting the encrypted files becomes impossible.

- **File Size**: There is no file size limitation, but encryption and decryption times may vary based on the size of files or directories.

---

## 📖 License

This script is open-source and provided for educational purposes. Use responsibly and in compliance with legal regulations regarding encryption and data protection. Always encrypt sensitive information securely!

---

This documentation provides a detailed overview of the script. You can follow these steps to encrypt or decrypt your files and directories efficiently and securely.
