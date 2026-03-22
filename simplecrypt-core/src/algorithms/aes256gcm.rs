use crate::algorithm::CryptoAlgorithm;
use crate::errors::{CryptoError, Result};
use aes_gcm::{Aes256Gcm as AesGcmCipher, Key, Nonce};
use aes_gcm::aead::{Aead, KeyInit};

/// AES-256-GCM algorithm (modern, authenticated encryption)
///
/// AEAD cipher — authentication tag is appended to ciphertext.
pub struct Aes256Gcm;

impl CryptoAlgorithm for Aes256Gcm {
    fn id(&self) -> &'static str {
        "aes-256-gcm"
    }

    fn display_name(&self) -> &'static str {
        "AES-256-GCM"
    }

    fn nonce_size(&self) -> usize {
        12
    }

    fn key_size(&self) -> usize {
        32
    }

    fn is_aead(&self) -> bool {
        true
    }

    fn encrypt(&self, key: &[u8], nonce: &[u8], plaintext: &[u8]) -> Result<Vec<u8>> {
        let key = Key::<AesGcmCipher>::from_slice(key);
        let cipher = AesGcmCipher::new(key);
        let nonce = Nonce::from_slice(nonce);

        cipher.encrypt(nonce, plaintext)
            .map_err(|_| CryptoError::AeadError)
    }

    fn decrypt(&self, key: &[u8], nonce: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>> {
        let key = Key::<AesGcmCipher>::from_slice(key);
        let cipher = AesGcmCipher::new(key);
        let nonce = Nonce::from_slice(nonce);

        cipher.decrypt(nonce, ciphertext)
            .map_err(|_| CryptoError::AeadError)
    }
}
