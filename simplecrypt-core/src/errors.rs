use serde::de::Error as SerdeError;

/// Unified error types for SimpleCrypt operations
#[derive(Debug, thiserror::Error)]
pub enum CryptoError {
    #[error("OpenSSL error: {0}")]
    OpensslError(#[from] openssl::error::ErrorStack),
    #[error("Base64 decode error: {0}")]
    Base64Error(#[from] base64::DecodeError),
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
    #[error("HMAC verification failed - data may have been tampered with")]
    HmacVerificationFailed,
    #[error("Invalid key length")]
    InvalidKeyLength,
    #[error("IV/nonce length mismatch")]
    NonceLengthMismatch,
    #[error("Salt length mismatch")]
    SaltLengthMismatch,
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("UTF-8 conversion error: {0}")]
    Utf8Error(#[from] std::string::FromUtf8Error),
    #[error("Unsupported format version: {0}")]
    UnsupportedVersion(String),
    #[error("Unknown algorithm: {0}")]
    UnknownAlgorithm(String),
    #[error("AEAD encryption error")]
    AeadError,
    #[error("Config error: {0}")]
    ConfigError(String),
}

impl CryptoError {
    pub fn json_custom(msg: &str) -> Self {
        CryptoError::JsonError(serde_json::Error::custom(msg))
    }
}

pub type Result<T> = std::result::Result<T, CryptoError>;
