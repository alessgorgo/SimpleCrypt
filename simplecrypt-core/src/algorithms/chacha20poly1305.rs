use crate::algorithm::CryptoAlgorithm;
use crate::errors::{CryptoError, Result};
use chacha20poly1305::{ChaCha20Poly1305 as ChaCha20Cipher, Key, Nonce};
use chacha20poly1305::aead::{Aead, KeyInit};

/// ChaCha20-Poly1305 algorithm
///
/// AEAD cipher — fast on hardware without AES-NI instruction set.
pub struct ChaCha20Poly1305Algorithm;

impl CryptoAlgorithm for ChaCha20Poly1305Algorithm {
    fn id(&self) -> &'static str {
        "chacha20-poly1305"
    }

    fn display_name(&self) -> &'static str {
        "ChaCha20-Poly1305"
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
        let key = Key::from_slice(key);
        let cipher = ChaCha20Cipher::new(key);
        let nonce = Nonce::from_slice(nonce);

        cipher.encrypt(nonce, plaintext)
            .map_err(|_| CryptoError::AeadError)
    }

    fn decrypt(&self, key: &[u8], nonce: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>> {
        let key = Key::from_slice(key);
        let cipher = ChaCha20Cipher::new(key);
        let nonce = Nonce::from_slice(nonce);

        cipher.decrypt(nonce, ciphertext)
            .map_err(|_| CryptoError::AeadError)
    }
}
