use std::fs;
use std::process::Command;
use tempfile::tempdir;

#[test]
fn test_single_file_encryption_decryption() {
    // Create a temporary directory for our test
    let temp_dir = tempdir().expect("Failed to create temp directory");
    let test_file = temp_dir.path().join("test.txt");
    
    // Create a test file with known content
    let test_content = "Hello, World! This is a test file for encryption.";
    fs::write(&test_file, test_content).expect("Failed to write test file");
    
    // Test encryption
    let output = Command::new(env!("CARGO_BIN_EXE_v1_3-rust"))
        .args(&["encrypt", test_file.to_str().unwrap(), "testpassword"])
        .output()
        .expect("Failed to execute encryption");
    
    assert!(output.status.success(), "Encryption failed: {}", String::from_utf8_lossy(&output.stderr));
    
    // Test decryption
    let output = Command::new(env!("CARGO_BIN_EXE_v1_3-rust"))
        .args(&["decrypt", test_file.to_str().unwrap(), "testpassword"])
        .output()
        .expect("Failed to execute decryption");
    
    assert!(output.status.success(), "Decryption failed: {}", String::from_utf8_lossy(&output.stderr));
    
    // Verify the content is the same
    let decrypted_content = fs::read_to_string(&test_file).expect("Failed to read decrypted file");
    assert_eq!(test_content, decrypted_content, "Decrypted content doesn't match original");
}

#[test]
fn test_directory_encryption_decryption() {
    // Create a temporary directory for our test
    let temp_dir = tempdir().expect("Failed to create temp directory");
    let test_dir = temp_dir.path().join("test_subdir");
    fs::create_dir(&test_dir).expect("Failed to create test subdirectory");
    
    // Create multiple test files
    let files = [
        ("file1.txt", "Content of file 1"),
        ("file2.txt", "Content of file 2 with special chars: áéíóú"),
        ("file3.txt", "Another file with numbers: 1234567890"),
    ];
    
    for (filename, content) in &files {
        let file_path = test_dir.join(filename);
        fs::write(&file_path, content).expect("Failed to write test file");
    }
    
    // Test directory encryption
    let output = Command::new(env!("CARGO_BIN_EXE_v1_3-rust"))
        .args(&["encrypt-dir", test_dir.to_str().unwrap(), "testpassword"])
        .output()
        .expect("Failed to execute directory encryption");
    
    assert!(output.status.success(), "Directory encryption failed: {}", String::from_utf8_lossy(&output.stderr));
    
    // Test directory decryption
    let output = Command::new(env!("CARGO_BIN_EXE_v1_3-rust"))
        .args(&["decrypt-dir", test_dir.to_str().unwrap(), "testpassword"])
        .output()
        .expect("Failed to execute directory decryption");
    
    assert!(output.status.success(), "Directory decryption failed: {}", String::from_utf8_lossy(&output.stderr));
    
    // Verify all files are decrypted correctly
    for (filename, expected_content) in &files {
        let file_path = test_dir.join(filename);
        let decrypted_content = fs::read_to_string(&file_path).expect("Failed to read decrypted file");
        assert_eq!(*expected_content, decrypted_content, "Decrypted content doesn't match original for {}", filename);
    }
}

#[test]
fn test_wrong_password_fails() {
    // Create a temporary directory for our test
    let temp_dir = tempdir().expect("Failed to create temp directory");
    let test_file = temp_dir.path().join("test.txt");
    
    // Create a test file
    let test_content = "This should not be decryptable with wrong password";
    fs::write(&test_file, test_content).expect("Failed to write test file");
    
    // Encrypt with correct password
    let output = Command::new(env!("CARGO_BIN_EXE_v1_3-rust"))
        .args(&["encrypt", test_file.to_str().unwrap(), "correctpassword"])
        .output()
        .expect("Failed to execute encryption");
    
    assert!(output.status.success(), "Encryption failed: {}", String::from_utf8_lossy(&output.stderr));
    
    // Try to decrypt with wrong password
    let output = Command::new(env!("CARGO_BIN_EXE_v1_3-rust"))
        .args(&["decrypt", test_file.to_str().unwrap(), "wrongpassword"])
        .output()
        .expect("Failed to execute decryption");
    
    // Should fail
    assert!(!output.status.success(), "Decryption should have failed with wrong password");
}

#[test]
fn test_help_command() {
    let output = Command::new(env!("CARGO_BIN_EXE_v1_3-rust"))
        .args(&["--help"])
        .output()
        .expect("Failed to execute help command");
    
    assert!(output.status.success(), "Help command failed");
    let output_str = String::from_utf8_lossy(&output.stdout);
    assert!(output_str.contains("Usage:"), "Help output should contain Usage:");
    assert!(output_str.contains("encrypt"), "Help output should contain encrypt");
    assert!(output_str.contains("decrypt"), "Help output should contain decrypt");
}

#[test]
fn test_empty_password_validation() {
    // Create a temporary directory for our test
    let temp_dir = tempdir().expect("Failed to create temp directory");
    let test_file = temp_dir.path().join("test.txt");
    
    // Create a test file
    let test_content = "Test content";
    fs::write(&test_file, test_content).expect("Failed to write test file");
    
    // Try to encrypt with empty password
    let output = Command::new(env!("CARGO_BIN_EXE_v1_3-rust"))
        .args(&["encrypt", test_file.to_str().unwrap(), ""])
        .output()
        .expect("Failed to execute encryption with empty password");
    
    // Should fail
    assert!(!output.status.success(), "Encryption should have failed with empty password");
    
    // Try to decrypt with empty password
    let output = Command::new(env!("CARGO_BIN_EXE_v1_3-rust"))
        .args(&["decrypt", test_file.to_str().unwrap(), ""])
        .output()
        .expect("Failed to execute decryption with empty password");
    
    // Should fail
    assert!(!output.status.success(), "Decryption should have failed with empty password");
}