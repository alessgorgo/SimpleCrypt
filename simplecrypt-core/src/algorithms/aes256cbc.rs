use crate::algorithm::CryptoAlgorithm;
use crate::errors::Result;
use openssl::symm::{encrypt, decrypt, Cipher};

/// AES-256-CBC algorithm (legacy, backward compatible)
///
/// Not an AEAD cipher — requires external HMAC for authentication.
pub struct Aes256Cbc;

impl CryptoAlgorithm for Aes256Cbc {
    fn id(&self) -> &'static str {
        "aes-256-cbc"
    }

    fn display_name(&self) -> &'static str {
        "AES-256-CBC (Legacy)"
    }

    fn nonce_size(&self) -> usize {
        16
    }

    fn key_size(&self) -> usize {
        32
    }

    fn is_aead(&self) -> bool {
        false
    }

    fn encrypt(&self, key: &[u8], nonce: &[u8], plaintext: &[u8]) -> Result<Vec<u8>> {
        let cipher = Cipher::aes_256_cbc();
        let ciphertext = encrypt(cipher, key, Some(nonce), plaintext)?;
        Ok(ciphertext)
    }

    fn decrypt(&self, key: &[u8], nonce: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>> {
        let cipher = Cipher::aes_256_cbc();
        let plaintext = decrypt(cipher, key, Some(nonce), ciphertext)?;
        Ok(plaintext)
    }
}
