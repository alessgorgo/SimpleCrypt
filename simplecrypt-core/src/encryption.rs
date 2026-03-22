use pbkdf2::pbkdf2_hmac;
use sha2::Sha256;
use hmac::{Hmac, Mac};
use sha2::Sha256 as Sha256Hash;
use zeroize::Zeroize;

use crate::algorithm::CryptoAlgorithm;
use crate::errors::{CryptoError, Result};
use crate::format;

/// Adaptive iterations based on file size for optimal performance
pub fn adaptive_iterations(file_size: usize) -> u32 {
    match file_size {
        0..=1024 => 10000,
        1025..=10240 => 25000,
        10241..=102400 => 50000,
        102401..=1048576 => 100000,
        _ => 600000,
    }
}

/// Encrypt data using the specified algorithm
///
/// Returns a JSON string containing the encrypted envelope (v3.0 format).
pub fn encrypt_data(data: &[u8], password: &str, algorithm: &dyn CryptoAlgorithm) -> Result<String> {
    let nonce_size = algorithm.nonce_size();

    // Generate random salt and nonce
    let mut salt = [0u8; 16];
    let mut nonce = vec![0u8; nonce_size];
    let mut master_key = [0u8; 64];

    openssl::rand::rand_bytes(&mut salt)?;
    openssl::rand::rand_bytes(&mut nonce)?;

    // Derive key using PBKDF2
    let iterations = adaptive_iterations(data.len());
    pbkdf2_hmac::<Sha256>(password.as_bytes(), &salt, iterations, &mut master_key);

    let (enc_key, hmac_key_bytes) = master_key.split_at(32);

    // Encrypt
    let encrypted_data = algorithm.encrypt(&enc_key[..algorithm.key_size()], &nonce, data)?;

    // For non-AEAD algorithms, compute external HMAC
    let hmac_value = if !algorithm.is_aead() {
        let mut hmac = Hmac::<Sha256Hash>::new_from_slice(hmac_key_bytes)
            .map_err(|_| CryptoError::InvalidKeyLength)?;
        hmac.update(&nonce);
        hmac.update(&encrypted_data);
        let digest = hmac.finalize().into_bytes();
        Some(digest.to_vec())
    } else {
        None
    };

    // Securely wipe
    master_key.zeroize();

    // Serialize to JSON envelope
    format::serialize_v3(
        algorithm.id(),
        &salt,
        &nonce,
        &encrypted_data,
        hmac_value.as_deref(),
    )
}

/// Legacy v2.0 encryption (AES-256-CBC only) for backward compatibility
pub fn encrypt_data_v2(data: &[u8], password: &str) -> Result<String> {
    use openssl::symm::{encrypt, Cipher};

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
        .map_err(|_| CryptoError::InvalidKeyLength)?;
    hmac.update(&iv);
    hmac.update(&encrypted_data);
    let hmac_digest = hmac.finalize().into_bytes();

    master_key.zeroize();

    format::serialize_v2(&salt, &iv, &encrypted_data, &hmac_digest)
}
