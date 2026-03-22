use std::fs;
use std::io::Read;
use pbkdf2::pbkdf2_hmac;
use sha2::Sha256;
use hmac::{Hmac, Mac};
use sha2::Sha256 as Sha256Hash;
use openssl::symm::{decrypt as openssl_decrypt, Cipher};
use zeroize::Zeroize;

use crate::algorithm::AlgorithmRegistry;
use crate::encryption::adaptive_iterations;
use crate::errors::{CryptoError, Result};
use crate::format::{self, FormatVersion};
use crate::memory_security::MemorySecurity;

/// Decrypt an encrypted file, auto-detecting the format version
pub fn decrypt_file(file_path: &str, password: &str, registry: &AlgorithmRegistry) -> Result<String> {
    let mut file = fs::File::open(file_path)?;
    let mut encrypted_data = Vec::new();
    file.read_to_end(&mut encrypted_data)?;

    decrypt_data(&encrypted_data, password, registry)
}

/// Decrypt encrypted data (bytes), auto-detecting the format version
pub fn decrypt_data(encrypted_bytes: &[u8], password: &str, registry: &AlgorithmRegistry) -> Result<String> {
    let json_string = String::from_utf8(encrypted_bytes.to_vec())?;
    let (version, json_map) = format::parse_envelope(&json_string)?;

    match version {
        FormatVersion::V1 => decrypt_v1(&json_map, password),
        FormatVersion::V2 => decrypt_v2(&json_map, password),
        FormatVersion::V3 => decrypt_v3(&json_map, password, registry),
    }
}

/// Decrypt v1.0 format (AES-256-CBC, fixed 100K iterations, no HMAC)
fn decrypt_v1(json_map: &serde_json::Map<String, serde_json::Value>, password: &str) -> Result<String> {
    let salt = format::decode_base64_field(json_map, "salt")?;
    let iv = format::decode_base64_field(json_map, "iv")?;
    let encrypted_payload = format::decode_base64_field(json_map, "data")?;

    if salt.len() != 16 {
        return Err(CryptoError::SaltLengthMismatch);
    }
    if iv.len() != 16 {
        return Err(CryptoError::NonceLengthMismatch);
    }

    let mut key = vec![0u8; 32];
    pbkdf2_hmac::<Sha256>(password.as_bytes(), &salt, 100000, &mut key);

    let cipher = Cipher::aes_256_cbc();
    let decrypted_data = openssl_decrypt(cipher, &key, Some(&iv), &encrypted_payload)?;

    key.zeroize();

    String::from_utf8(decrypted_data).map_err(CryptoError::Utf8Error)
}

/// Decrypt v2.0 format (AES-256-CBC, adaptive iterations, HMAC-SHA256)
fn decrypt_v2(json_map: &serde_json::Map<String, serde_json::Value>, password: &str) -> Result<String> {
    let salt = format::decode_base64_field(json_map, "salt")?;
    let nonce = format::decode_base64_field_opt(json_map, "nonce", "iv")?;
    let encrypted_payload = format::decode_base64_field(json_map, "data")?;
    let stored_hmac = format::decode_base64_field(json_map, "hmac")?;

    format::validate_lengths(&salt, &nonce)?;

    let mut master_key = [0u8; 64];
    let iterations = adaptive_iterations(encrypted_payload.len());
    pbkdf2_hmac::<Sha256>(password.as_bytes(), &salt, iterations, &mut master_key);

    let (enc_key, hmac_key) = master_key.split_at(32);

    // Verify HMAC before decryption
    verify_hmac(hmac_key, &nonce, &encrypted_payload, &stored_hmac)?;

    let cipher = Cipher::aes_256_cbc();
    let decrypted_data = openssl_decrypt(cipher, enc_key, Some(&nonce), &encrypted_payload)?;

    master_key.zeroize();

    String::from_utf8(decrypted_data).map_err(CryptoError::Utf8Error)
}

/// Decrypt v3.0 format (multi-algorithm, adaptive iterations)
fn decrypt_v3(
    json_map: &serde_json::Map<String, serde_json::Value>,
    password: &str,
    registry: &AlgorithmRegistry,
) -> Result<String> {
    // Read algorithm from envelope
    let algorithm_id = json_map.get("algorithm")
        .and_then(|v| v.as_str())
        .ok_or_else(|| CryptoError::json_custom("Missing 'algorithm' field in v3.0 envelope"))?;

    let algorithm = registry.get(algorithm_id)
        .ok_or_else(|| CryptoError::UnknownAlgorithm(algorithm_id.to_string()))?;

    let salt = format::decode_base64_field(json_map, "salt")?;
    let nonce = format::decode_base64_field_opt(json_map, "nonce", "iv")?;
    let encrypted_payload = format::decode_base64_field(json_map, "data")?;

    format::validate_lengths(&salt, &nonce)?;

    // Derive key
    let mut master_key = [0u8; 64];
    let iterations = adaptive_iterations(encrypted_payload.len());
    pbkdf2_hmac::<Sha256>(password.as_bytes(), &salt, iterations, &mut master_key);

    let (enc_key, hmac_key) = master_key.split_at(32);

    // For non-AEAD algorithms, verify external HMAC first
    if !algorithm.is_aead() {
        let stored_hmac = format::decode_base64_field(json_map, "hmac")?;
        verify_hmac(hmac_key, &nonce, &encrypted_payload, &stored_hmac)?;
    }

    // Decrypt
    let decrypted_data = algorithm.decrypt(&enc_key[..algorithm.key_size()], &nonce, &encrypted_payload)?;

    master_key.zeroize();

    String::from_utf8(decrypted_data).map_err(CryptoError::Utf8Error)
}

/// Verify HMAC-SHA256 with constant-time comparison
fn verify_hmac(hmac_key: &[u8], nonce: &[u8], encrypted_payload: &[u8], stored_hmac: &[u8]) -> Result<()> {
    let mut hmac = Hmac::<Sha256Hash>::new_from_slice(hmac_key)
        .map_err(|_| CryptoError::InvalidKeyLength)?;
    hmac.update(nonce);
    hmac.update(encrypted_payload);
    let computed_hmac = hmac.finalize().into_bytes();

    if !MemorySecurity::constant_time_compare(&computed_hmac, stored_hmac) {
        return Err(CryptoError::HmacVerificationFailed);
    }
    Ok(())
}
