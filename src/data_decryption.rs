use std::fs;
use std::io::Read;
use base64::{self, Engine};
use serde_json;
use serde::de::Error;
use openssl::symm::{decrypt, Cipher};
use pbkdf2::pbkdf2_hmac;
use sha2::Sha256;
use hmac::{Hmac, Mac};
use sha2::Sha256 as Sha256Hash;
use zeroize::Zeroize;

use crate::data_encryption::{EncryptionError, Result, adaptive_iterations};

impl From<std::io::Error> for EncryptionError {
    fn from(_err: std::io::Error) -> Self {
        EncryptionError::JsonError(serde_json::Error::custom("IO error"))
    }
}

impl From<std::string::FromUtf8Error> for EncryptionError {
    fn from(_err: std::string::FromUtf8Error) -> Self {
        EncryptionError::JsonError(serde_json::Error::custom("UTF-8 conversion error"))
    }
}

pub fn decrypt_file(file_path: &str, password: &str) -> Result<String> {
    let mut file = fs::File::open(file_path)?;
    let mut encrypted_data = Vec::new();
    
    file.read_to_end(&mut encrypted_data)?;

    let json_string = String::from_utf8(encrypted_data)?;
    let json_map: serde_json::Map<String, serde_json::Value> = serde_json::from_str(&json_string)?;
    
    let version = json_map.get("version")
        .and_then(|v| v.as_str())
        .unwrap_or("1.0");
    
    match version {
        "1.0" => decrypt_v1(&json_map, password),
        "2.0" => decrypt_v2(&json_map, password),
        _ => Err(EncryptionError::JsonError(serde_json::Error::custom("Unsupported version"))),
    }
}

fn decrypt_v1(json_map: &serde_json::Map<String, serde_json::Value>, password: &str) -> Result<String> {
    let salt_b64 = json_map.get("salt").ok_or(EncryptionError::JsonError(serde_json::Error::custom("Missing 'salt' field")))?.as_str().ok_or(EncryptionError::JsonError(serde_json::Error::custom("Invalid 'salt' field")))?;
    let iv_b64 = json_map.get("iv").ok_or(EncryptionError::JsonError(serde_json::Error::custom("Missing 'iv' field")))?.as_str().ok_or(EncryptionError::JsonError(serde_json::Error::custom("Invalid 'iv' field")))?;
    let data_b64 = json_map.get("data").ok_or(EncryptionError::JsonError(serde_json::Error::custom("Missing 'data' field")))?.as_str().ok_or(EncryptionError::JsonError(serde_json::Error::custom("Invalid 'data' field")))?;

    let mut salt = base64::engine::general_purpose::STANDARD.decode(salt_b64)?;
    let mut iv = base64::engine::general_purpose::STANDARD.decode(iv_b64)?;
    let encrypted_payload = base64::engine::general_purpose::STANDARD.decode(data_b64)?;

    if salt.len() != 16 {
        return Err(EncryptionError::SaltLengthMismatch);
    }
    if iv.len() != 16 {
        return Err(EncryptionError::IvLengthMismatch);
    }

    let mut key = vec![0u8; 32];
    pbkdf2_hmac::<Sha256>(
        password.as_bytes(),
        &salt,
        100000,
        &mut key,
    );

    let cipher = Cipher::aes_256_cbc();
    let decrypted_data = decrypt(cipher, &key, Some(&iv), &encrypted_payload)?;
    
    key.zeroize();
    iv.zeroize();
    salt.zeroize();
    
    String::from_utf8(decrypted_data).map_err(|_| EncryptionError::JsonError(serde_json::Error::custom("UTF-8 conversion error")))
}

fn decode_base64_field(map: &serde_json::Map<String, serde_json::Value>, key: &str) -> Result<Vec<u8>> {
    let value = map.get(key)
        .ok_or_else(|| EncryptionError::JsonError(serde_json::Error::custom(format!("Missing '{}' field", key))))?
        .as_str()
        .ok_or_else(|| EncryptionError::JsonError(serde_json::Error::custom(format!("Invalid '{}' field", key))))?;
    
    base64::engine::general_purpose::STANDARD.decode(value).map_err(Into::into)
}

fn decode_base64_field_opt(map: &serde_json::Map<String, serde_json::Value>, key1: &str, key2: &str) -> Result<Vec<u8>> {
    if let Some(_) = map.get(key1) {
        decode_base64_field(map, key1)
    } else if let Some(_) = map.get(key2) {
        decode_base64_field(map, key2)
    } else {
        Err(EncryptionError::JsonError(serde_json::Error::custom(format!("Missing both '{}' and '{}' fields", key1, key2))))
    }
}

fn validate_lengths(salt: &[u8], nonce: &[u8]) -> Result<()> {
    if salt.len() != 16 {
        return Err(EncryptionError::SaltLengthMismatch);
    }
    if nonce.len() != 12 && nonce.len() != 16 {
        return Err(EncryptionError::IvLengthMismatch);
    }
    Ok(())
}

fn verify_hmac(hmac_key: &[u8], nonce: &[u8], encrypted_payload: &[u8], stored_hmac: &[u8]) -> Result<()> {
    let mut hmac = Hmac::<Sha256Hash>::new_from_slice(hmac_key)
        .map_err(|_| EncryptionError::InvalidKeyLength)?;
    hmac.update(nonce);
    hmac.update(encrypted_payload);
    let computed_hmac = hmac.finalize().into_bytes();

    if !constant_time_compare(&computed_hmac, stored_hmac) {
        return Err(EncryptionError::HmacVerificationFailed);
    }
    Ok(())
}

fn decrypt_v2(json_map: &serde_json::Map<String, serde_json::Value>, password: &str) -> Result<String> {
    let salt = decode_base64_field(json_map, "salt")?;
    let nonce = decode_base64_field_opt(json_map, "nonce", "iv")?;
    let encrypted_payload = decode_base64_field(json_map, "data")?;
    let stored_hmac = decode_base64_field(json_map, "hmac")?;

    validate_lengths(&salt, &nonce)?;
    
    let mut master_key = [0u8; 64];
    let iterations = adaptive_iterations(encrypted_payload.len());
    
    pbkdf2_hmac::<Sha256>(password.as_bytes(), &salt, iterations, &mut master_key);
    let (enc_key, hmac_key) = master_key.split_at(32);

    verify_hmac(hmac_key, &nonce, &encrypted_payload, &stored_hmac)?;

    let cipher = Cipher::aes_256_cbc();
    let decrypted_data = decrypt(cipher, enc_key, Some(&nonce), &encrypted_payload)?;
    
    master_key.zeroize();
    
    String::from_utf8(decrypted_data).map_err(|_| EncryptionError::JsonError(serde_json::Error::custom("UTF-8 conversion error")))
}

fn constant_time_compare(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    
    let mut result = 0;
    for (x, y) in a.iter().zip(b.iter()) {
        result |= x ^ y;
    }
    result == 0
}
