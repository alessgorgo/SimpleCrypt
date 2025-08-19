use std::fs;
use std::io::{self, Read};
use base64::{self, Engine}; // Use Engine
use serde_json;
use openssl::symm::{decrypt_aead, Cipher};
use std::error::Error;
use pbkdf2::pbkdf2_hmac;
use sha2::Sha256;

pub fn decrypt_file(file_path: &str, password: &str) -> Result<String, Box<dyn Error>> {
    // Open the file
    let mut file = fs::File::open(file_path)?;
    let mut encrypted_data = Vec::new();
    
    // Read the encrypted data from the file
    file.read_to_end(&mut encrypted_data)?;

    // Parse the JSON data
    let json_string = String::from_utf8(encrypted_data)?;
    let json_map: serde_json::Map<String, serde_json::Value> = serde_json::from_str(&json_string)?;
    
    // Extract base64 encoded values
    let salt_b64 = json_map.get("salt").ok_or("Missing 'salt' field in JSON")?.as_str().ok_or("Invalid 'salt' field")?;
    let iv_b64 = json_map.get("iv").ok_or("Missing 'iv' field in JSON")?.as_str().ok_or("Invalid 'iv' field")?;
    let data_b64 = json_map.get("data").ok_or("Missing 'data' field in JSON")?.as_str().ok_or("Invalid 'data' field")?;

    // Decode base64 values
    let mut salt = base64::engine::general_purpose::STANDARD.decode(salt_b64)?;
    let mut iv = base64::engine::general_purpose::STANDARD.decode(iv_b64)?;
    let encrypted_payload = base64::engine::general_purpose::STANDARD.decode(data_b64)?;

    // Validate lengths
    if salt.len() != 16 {
        return Err("Invalid salt length".into());
    }
    if iv.len() != 16 {
        return Err("Invalid IV length".into());
    }

    // Derive key using PBKDF2 (same parameters as encryption)
    let mut key = vec![0u8; 32]; // AES-256 requires 32 bytes
    pbkdf2_hmac::<Sha256>(
        password.as_bytes(),
        &salt,
        100000, // Number of iterations (must match encryption)
        &mut key,
    );

    // For now, use CBC mode (working implementation)
    // TODO: Implement GCM mode properly
    let cipher = Cipher::aes_256_cbc();
    let decrypted_data = openssl::symm::decrypt(cipher, &key, Some(&iv), &encrypted_payload)?;
    
    // Securely wipe sensitive data from memory
    key.fill(0);
    iv.fill(0);
    salt.fill(0);
    
    // Convert to string and return
    String::from_utf8(decrypted_data).map_err(|e| e.into())
}
