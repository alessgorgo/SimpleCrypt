use base64::{engine::general_purpose, Engine};
use serde::{Deserialize, Serialize};
use serde_json;

use crate::errors::{CryptoError, Result};

/// Encrypted file envelope — serialized as JSON
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedEnvelope {
    pub version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algorithm: Option<String>,
    pub salt: String,
    /// Nonce or IV (base64 encoded)
    #[serde(alias = "iv")]
    pub nonce: String,
    pub data: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hmac: Option<String>,
}

/// Detected version of an encrypted file
#[derive(Debug, Clone, PartialEq)]
pub enum FormatVersion {
    /// v1.0: AES-256-CBC, fixed 100K iterations, no HMAC
    V1,
    /// v2.0: AES-256-CBC, adaptive iterations, HMAC-SHA256
    V2,
    /// v3.0: Multi-algorithm, adaptive iterations, algorithm field
    V3,
}

/// Parse an encrypted JSON file and determine its version
pub fn parse_envelope(json_str: &str) -> Result<(FormatVersion, serde_json::Map<String, serde_json::Value>)> {
    let json_map: serde_json::Map<String, serde_json::Value> = serde_json::from_str(json_str)?;

    let version = json_map.get("version")
        .and_then(|v| v.as_str())
        .unwrap_or("1.0");

    let format_version = match version {
        "1.0" => FormatVersion::V1,
        "2.0" => FormatVersion::V2,
        "3.0" => FormatVersion::V3,
        other => return Err(CryptoError::UnsupportedVersion(other.to_string())),
    };

    Ok((format_version, json_map))
}

/// Serialize a v3.0 encrypted envelope to JSON
pub fn serialize_v3(
    algorithm_id: &str,
    salt: &[u8],
    nonce: &[u8],
    encrypted_data: &[u8],
    hmac: Option<&[u8]>,
) -> Result<String> {
    let envelope = EncryptedEnvelope {
        version: "3.0".to_string(),
        algorithm: Some(algorithm_id.to_string()),
        salt: general_purpose::STANDARD.encode(salt),
        nonce: general_purpose::STANDARD.encode(nonce),
        data: general_purpose::STANDARD.encode(encrypted_data),
        hmac: hmac.map(|h| general_purpose::STANDARD.encode(h)),
    };

    serde_json::to_string(&envelope).map_err(CryptoError::JsonError)
}

/// Serialize a v2.0 encrypted envelope to JSON (for backward compat testing)
pub fn serialize_v2(
    salt: &[u8],
    iv: &[u8],
    encrypted_data: &[u8],
    hmac: &[u8],
) -> Result<String> {
    use std::fmt::Write;

    let mut output = String::with_capacity(512);
    write!(output, r#"{{"version":"2.0","salt":"{}","iv":"{}","data":"{}","hmac":"{}"}}"#,
        general_purpose::STANDARD.encode(salt),
        general_purpose::STANDARD.encode(iv),
        general_purpose::STANDARD.encode(encrypted_data),
        general_purpose::STANDARD.encode(hmac)
    ).map_err(|_| CryptoError::json_custom("String formatting error"))?;

    Ok(output)
}

/// Helper: decode a base64 field from a JSON map
pub fn decode_base64_field(map: &serde_json::Map<String, serde_json::Value>, key: &str) -> Result<Vec<u8>> {
    let value = map.get(key)
        .ok_or_else(|| CryptoError::json_custom(&format!("Missing '{}' field", key)))?
        .as_str()
        .ok_or_else(|| CryptoError::json_custom(&format!("Invalid '{}' field", key)))?;

    general_purpose::STANDARD.decode(value).map_err(CryptoError::Base64Error)
}

/// Helper: decode a base64 field, trying two possible key names
pub fn decode_base64_field_opt(map: &serde_json::Map<String, serde_json::Value>, key1: &str, key2: &str) -> Result<Vec<u8>> {
    if map.get(key1).is_some() {
        decode_base64_field(map, key1)
    } else if map.get(key2).is_some() {
        decode_base64_field(map, key2)
    } else {
        Err(CryptoError::json_custom(&format!("Missing both '{}' and '{}' fields", key1, key2)))
    }
}

/// Validate salt and nonce lengths
pub fn validate_lengths(salt: &[u8], nonce: &[u8]) -> Result<()> {
    if salt.len() != 16 {
        return Err(CryptoError::SaltLengthMismatch);
    }
    // Accept 12-byte (GCM/ChaCha), 16-byte (CBC), or 24-byte (XChaCha) nonces
    if ![12, 16, 24].contains(&nonce.len()) {
        return Err(CryptoError::NonceLengthMismatch);
    }
    Ok(())
}
