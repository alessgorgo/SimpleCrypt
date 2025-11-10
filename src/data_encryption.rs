use openssl::symm::{encrypt, Cipher};
use base64::{engine::general_purpose, Engine};
use std::fmt::Write;
use pbkdf2::pbkdf2_hmac;
use sha2::Sha256;
use hmac::{Hmac, Mac};
use sha2::Sha256 as Sha256Hash;
use zeroize::Zeroize;

#[derive(Debug, thiserror::Error)]
pub enum EncryptionError {
    #[error("OpenSSL error: {0}")]
    OpensslError(#[from] openssl::error::ErrorStack),
    #[error("Base64 decode error: {0}")]
    Base64Error(#[from] base64::DecodeError),
    #[error("JSON serialization error: {0}")]
    JsonError(#[from] serde_json::Error),
    #[error("HMAC verification failed")]
    HmacVerificationFailed,
    #[error("Invalid key length")]
    InvalidKeyLength,
    #[error("IV length mismatch")]
    IvLengthMismatch,
    #[error("Salt length mismatch")]
    SaltLengthMismatch,
}

pub type Result<T> = std::result::Result<T, EncryptionError>;

pub fn adaptive_iterations(file_size: usize) -> u32 {
    match file_size {
        0..=1024 => 10000,
        1025..=10240 => 25000,
        10241..=102400 => 50000,
        102401..=1048576 => 100000,
        _ => 600000,
    }
}

pub fn encrypt_file(data: &[u8], password: &str) -> Result<String> {
    let mut salt = [0u8; 16];
    let mut iv = [0u8; 16];
    let mut master_key = [0u8; 64];
    
    openssl::rand::rand_bytes(&mut salt)?;
    openssl::rand::rand_bytes(&mut iv)?;
    
    let iterations = adaptive_iterations(data.len());
    
    pbkdf2_hmac::<Sha256>(password.as_bytes(), &salt, iterations, &mut master_key);
    
    let (enc_key, hmac_key) = master_key.split_at(32);
    
    let cipher = Cipher::aes_256_cbc();
    let encrypted_data = encrypt(cipher, enc_key, Some(&iv), data)?;

    let mut hmac = Hmac::<Sha256Hash>::new_from_slice(hmac_key)
        .map_err(|_| EncryptionError::InvalidKeyLength)?;
    hmac.update(&iv);
    hmac.update(&encrypted_data);
    let hmac_digest = hmac.finalize().into_bytes();

    master_key.zeroize();

    let json_string = serialize_zero_copy(&salt, &iv, &encrypted_data, &hmac_digest)?;
    Ok(json_string)
}

fn serialize_zero_copy(salt: &[u8; 16], iv: &[u8; 16], data: &[u8], hmac: &[u8]) -> Result<String> {
    let mut output = String::with_capacity(512);
    
    write!(output, r#"{{"version":"2.0","salt":"{}","iv":"{}","data":"{}","hmac":"{}"}}"#,
        general_purpose::STANDARD.encode(salt),
        general_purpose::STANDARD.encode(iv),
        general_purpose::STANDARD.encode(data),
        general_purpose::STANDARD.encode(hmac)
    ).map_err(|_| EncryptionError::JsonError(
        serde_json::Error::io(std::io::Error::new(std::io::ErrorKind::InvalidData, "String formatting error"))
    ))?;
    
    Ok(output)
}
