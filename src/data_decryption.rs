use std::fs;
use std::io::{self, Read};
use base64::{self, Engine};
use serde_json;
use openssl::symm::{decrypt_aead, Cipher};
use std::error::Error;
use pbkdf2::pbkdf2_hmac;
use sha2::Sha256;

pub fn decrypt_file(file_path: &str, password: &str) -> Result<String, Box<dyn Error>> {
    let mut file = fs::File::open(file_path)?;
    let mut encrypted_data = Vec::new();
    
    file.read_to_end(&mut encrypted_data)?;

    let json_string = String::from_utf8(encrypted_data)?;
    let json_map: serde_json::Map<String, serde_json::Value> = serde_json::from_str(&json_string)?;
    
    let salt_b64 = json_map.get("salt").ok_or("Missing 'salt' field in JSON")?.as_str().ok_or("Invalid 'salt' field")?;
    let iv_b64 = json_map.get("iv").ok_or("Missing 'iv' field in JSON")?.as_str().ok_or("Invalid 'iv' field")?;
    let data_b64 = json_map.get("data").ok_or("Missing 'data' field in JSON")?.as_str().ok_or("Invalid 'data' field")?;

    let mut salt = base64::engine::general_purpose::STANDARD.decode(salt_b64)?;
    let mut iv = base64::engine::general_purpose::STANDARD.decode(iv_b64)?;
    let encrypted_payload = base64::engine::general_purpose::STANDARD.decode(data_b64)?;

    if salt.len() != 16 {
        return Err("Invalid salt length".into());
    }
    if iv.len() != 16 {
        return Err("Invalid IV length".into());
    }

    let mut key = vec![0u8; 32];
    pbkdf2_hmac::<Sha256>(
        password.as_bytes(),
        &salt,
        100000,
        &mut key,
    );

    let cipher = Cipher::aes_256_cbc();
    let decrypted_data = openssl::symm::decrypt(cipher, &key, Some(&iv), &encrypted_payload)?;
    
    key.fill(0);
    iv.fill(0);
    salt.fill(0);
    
    String::from_utf8(decrypted_data).map_err(|e| e.into())
}
