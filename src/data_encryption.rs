use openssl::symm::{encrypt, Cipher};
use base64::{engine::general_purpose, Engine};
use std::collections::HashMap;
use std::error::Error;
use pbkdf2::pbkdf2_hmac;
use sha2::Sha256;
pub fn encrypt_file(data: &[u8], password: &str) -> Result<String, Box<dyn Error>> {
    // Generate a random salt for PBKDF2
    let mut salt = vec![0u8; 16];
    openssl::rand::rand_bytes(&mut salt)?;

    // Derive key using PBKDF2
    let mut key = vec![0u8; 32]; // AES-256 requires 32 bytes
    pbkdf2_hmac::<Sha256>(
        password.as_bytes(),
        &salt,
        100000, // Number of iterations
        &mut key,
    );

    // Generate random IV for CBC
    let mut iv = vec![0u8; 16];
    openssl::rand::rand_bytes(&mut iv)?;

    // Encrypt the data using AES-256-CBC
    let encrypted_data = encrypt(Cipher::aes_256_cbc(), &key, Some(&iv), data)?;

    // Base64 encoding
    let encoded_salt = general_purpose::STANDARD.encode(&salt);
    let encoded_iv = general_purpose::STANDARD.encode(&iv);
    let encoded_data = general_purpose::STANDARD.encode(&encrypted_data);

    // Securely wipe sensitive data from memory
    key.fill(0);
    iv.fill(0);
    salt.fill(0);

    // Create JSON structure
    let mut json_map = HashMap::new();
    json_map.insert("salt", encoded_salt);
    json_map.insert("iv", encoded_iv);
    json_map.insert("data", encoded_data);

    // Serialize to JSON
    let json_string = serde_json::to_string(&json_map)?;
    Ok(json_string)
}
