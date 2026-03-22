use crate::algorithm::CryptoAlgorithm;
use crate::errors::{CryptoError, Result};
use chacha20poly1305::{XChaCha20Poly1305 as XChaCha20Cipher, Key, XNonce};
use chacha20poly1305::aead::{Aead, KeyInit};

/// XChaCha20-Poly1305 algorithm
///
/// AEAD cipher with extended 24-byte nonce — safest against nonce reuse.
pub struct XChaCha20Poly1305Algorithm;

impl CryptoAlgorithm for XChaCha20Poly1305Algorithm {
    fn id(&self) -> &'static str {
        "xchacha20-poly1305"
    }

    fn display_name(&self) -> &'static str {
        "XChaCha20-Poly1305"
    }

    fn nonce_size(&self) -> usize {
        24
    }

    fn key_size(&self) -> usize {
        32
    }

    fn is_aead(&self) -> bool {
        true
    }

    fn encrypt(&self, key: &[u8], nonce: &[u8], plaintext: &[u8]) -> Result<Vec<u8>> {
        let key = Key::from_slice(key);
        let cipher = XChaCha20Cipher::new(key);
        let nonce = XNonce::from_slice(nonce);

        cipher.encrypt(nonce, plaintext)
            .map_err(|_| CryptoError::AeadError)
    }

    fn decrypt(&self, key: &[u8], nonce: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>> {
        let key = Key::from_slice(key);
        let cipher = XChaCha20Cipher::new(key);
        let nonce = XNonce::from_slice(nonce);

        cipher.decrypt(nonce, ciphertext)
            .map_err(|_| CryptoError::AeadError)
    }
}
