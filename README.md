# 🛡️ File Obfuscation Script Documentation

This documentation provides a simple guide on how to use the File Obfuscation Script. This script allows you to obfuscate and deobfuscate files and directories using AES-256 encryption.

  

## 📥 Importing the Script from GitHub

1. **Clone the Repository**:

Open your terminal and run the following command to clone the repository from GitHub:

```bash

git clone https://github.com/alessgorgo/aes-obfuscator.git

```

  

2. **Navigate to the Directory**:

Change to the directory containing the script:

```bash

cd aes-obfuscator

```

  

3. **Make the Script Executable**:

Ensure the script has executable permissions by running:

```bash

chmod +x AesObfTool.sh

```

  

## ⚙️ Compiling the Script

This script is written in Bash, so there is no need for compilation. You can run it directly if you have the necessary permissions.

  

## 📝 How to Use the Script


### Running the Script

To execute the script, use the following command in your terminal:

```bash

./AesObfTool.sh

```

  

### Menu Options

Upon running the script, you will see a menu with the following options:


1. **Obfuscate Application**: Protect an application by obfuscating its files.

2. **Deobfuscate Application**: Restore an obfuscated application back to its original form.

3. **Obfuscate Text File**: Encrypt a text file to protect its content.

4. **Deobfuscate Text File**: Decrypt an obfuscated text file back to its original content.

  

### Example Usage

- **Obfuscate an Application**:

1. Choose option 1.

2. Enter the name of the application (without `.app`).

3. The obfuscated files will be saved in a new directory prefixed with `obfuscated-`.

  

- **Deobfuscate an Application**:

1. Choose option 2.

2. Provide the name of the obfuscated directory.

3. The original files will be restored to the specified original directory.

  

- **Obfuscate a Text File**:

1. Choose option 3.

2. Enter the name of the text file to obfuscate.

3. Specify a name for the output obfuscated file.

  

- **Deobfuscate a Text File**:

1. Choose option 4.

2. Provide the name of the obfuscated text file.

3. Specify a name for the restored original text file.

  

### 💡 Notes

- The script uses OpenSSL for encryption and decryption. Ensure OpenSSL is installed on your system.

- The obfuscated version of the application or text file will be created in the same directory where you run the script.

  

### 🛠️ Dependencies

- Bash shell

- OpenSSL

  

## 📁 File Structure

- `key.bin`: This file contains the generated encryption key used for obfuscation.

- The output files will be saved in the specified directories as per the obfuscation or deobfuscation operations.

  

## 📖 License

This script is provided for educational purposes. Please ensure to use it responsibly and comply with all relevant laws and regulations regarding file obfuscation and encryption.
