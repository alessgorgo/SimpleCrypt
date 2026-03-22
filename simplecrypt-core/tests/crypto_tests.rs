use simplecrypt_core::{AlgorithmRegistry, Config};
use simplecrypt_core::encryption::{encrypt_data, encrypt_data_v2};
use simplecrypt_core::decryption::decrypt_data;
use tempfile::TempDir;

const TEST_PASSWORD: &str = "test_password_123!";
const TEST_DATA: &str = "Hello, SimpleCrypt! This is test data for encryption.";

#[test]
fn test_aes256gcm_roundtrip() {
    let registry = AlgorithmRegistry::new();
    let algo = registry.get("aes-256-gcm").unwrap();

    let encrypted = encrypt_data(TEST_DATA.as_bytes(), TEST_PASSWORD, algo).unwrap();
    let decrypted = decrypt_data(encrypted.as_bytes(), TEST_PASSWORD, &registry).unwrap();

    assert_eq!(decrypted, TEST_DATA);
}

#[test]
fn test_aes256cbc_roundtrip() {
    let registry = AlgorithmRegistry::new();
    let algo = registry.get("aes-256-cbc").unwrap();

    let encrypted = encrypt_data(TEST_DATA.as_bytes(), TEST_PASSWORD, algo).unwrap();
    let decrypted = decrypt_data(encrypted.as_bytes(), TEST_PASSWORD, &registry).unwrap();

    assert_eq!(decrypted, TEST_DATA);
}

#[test]
fn test_chacha20_poly1305_roundtrip() {
    let registry = AlgorithmRegistry::new();
    let algo = registry.get("chacha20-poly1305").unwrap();

    let encrypted = encrypt_data(TEST_DATA.as_bytes(), TEST_PASSWORD, algo).unwrap();
    let decrypted = decrypt_data(encrypted.as_bytes(), TEST_PASSWORD, &registry).unwrap();

    assert_eq!(decrypted, TEST_DATA);
}

#[test]
fn test_xchacha20_poly1305_roundtrip() {
    let registry = AlgorithmRegistry::new();
    let algo = registry.get("xchacha20-poly1305").unwrap();

    let encrypted = encrypt_data(TEST_DATA.as_bytes(), TEST_PASSWORD, algo).unwrap();
    let decrypted = decrypt_data(encrypted.as_bytes(), TEST_PASSWORD, &registry).unwrap();

    assert_eq!(decrypted, TEST_DATA);
}

#[test]
fn test_v2_backward_compatibility() {
    let registry = AlgorithmRegistry::new();

    // Encrypt using legacy v2.0 format
    let encrypted = encrypt_data_v2(TEST_DATA.as_bytes(), TEST_PASSWORD).unwrap();

    // Verify it's v2.0 format
    let parsed: serde_json::Value = serde_json::from_str(&encrypted).unwrap();
    assert_eq!(parsed["version"], "2.0");

    // Decrypt should still work via v2 path
    let decrypted = decrypt_data(encrypted.as_bytes(), TEST_PASSWORD, &registry).unwrap();
    assert_eq!(decrypted, TEST_DATA);
}

#[test]
fn test_v3_format_has_algorithm_field() {
    let registry = AlgorithmRegistry::new();
    let algo = registry.get("aes-256-gcm").unwrap();

    let encrypted = encrypt_data(TEST_DATA.as_bytes(), TEST_PASSWORD, algo).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&encrypted).unwrap();

    assert_eq!(parsed["version"], "3.0");
    assert_eq!(parsed["algorithm"], "aes-256-gcm");
    assert!(parsed["salt"].is_string());
    assert!(parsed["nonce"].is_string());
    assert!(parsed["data"].is_string());
}

#[test]
fn test_wrong_password_fails() {
    let registry = AlgorithmRegistry::new();
    let algo = registry.get("aes-256-gcm").unwrap();

    let encrypted = encrypt_data(TEST_DATA.as_bytes(), TEST_PASSWORD, algo).unwrap();
    let result = decrypt_data(encrypted.as_bytes(), "wrong_password", &registry);

    assert!(result.is_err());
}

#[test]
fn test_tampered_data_fails() {
    let registry = AlgorithmRegistry::new();
    let algo = registry.get("aes-256-cbc").unwrap();

    let encrypted = encrypt_data(TEST_DATA.as_bytes(), TEST_PASSWORD, algo).unwrap();

    // Tamper with the HMAC field
    let mut parsed: serde_json::Value = serde_json::from_str(&encrypted).unwrap();
    parsed["hmac"] = serde_json::Value::String("AAAA".to_string());
    let tampered = serde_json::to_string(&parsed).unwrap();

    let result = decrypt_data(tampered.as_bytes(), TEST_PASSWORD, &registry);
    assert!(result.is_err());
}

#[test]
fn test_cross_algorithm_decrypt_fails_gracefully() {
    let registry = AlgorithmRegistry::new();
    let gcm = registry.get("aes-256-gcm").unwrap();

    let encrypted = encrypt_data(TEST_DATA.as_bytes(), TEST_PASSWORD, gcm).unwrap();

    // Tamper: change the algorithm field to CBC
    let mut parsed: serde_json::Value = serde_json::from_str(&encrypted).unwrap();
    parsed["algorithm"] = serde_json::Value::String("aes-256-cbc".to_string());
    let tampered = serde_json::to_string(&parsed).unwrap();

    // Should fail because the data was encrypted with GCM
    let result = decrypt_data(tampered.as_bytes(), TEST_PASSWORD, &registry);
    assert!(result.is_err());
}

#[test]
fn test_all_algorithms_listed() {
    let registry = AlgorithmRegistry::new();
    let algorithms = registry.list();

    assert_eq!(algorithms.len(), 4);

    let ids: Vec<&str> = algorithms.iter().map(|a| a.id.as_str()).collect();
    assert!(ids.contains(&"aes-256-cbc"));
    assert!(ids.contains(&"aes-256-gcm"));
    assert!(ids.contains(&"chacha20-poly1305"));
    assert!(ids.contains(&"xchacha20-poly1305"));
}

#[test]
fn test_empty_data_roundtrip() {
    let registry = AlgorithmRegistry::new();
    let algo = registry.get("aes-256-gcm").unwrap();

    let encrypted = encrypt_data(b"", TEST_PASSWORD, algo).unwrap();
    let decrypted = decrypt_data(encrypted.as_bytes(), TEST_PASSWORD, &registry).unwrap();

    assert_eq!(decrypted, "");
}

#[test]
fn test_large_data_roundtrip() {
    let registry = AlgorithmRegistry::new();
    let algo = registry.get("chacha20-poly1305").unwrap();

    let large_data = "A".repeat(100_000);
    let encrypted = encrypt_data(large_data.as_bytes(), TEST_PASSWORD, algo).unwrap();
    let decrypted = decrypt_data(encrypted.as_bytes(), TEST_PASSWORD, &registry).unwrap();

    assert_eq!(decrypted, large_data);
}

#[test]
fn test_unicode_data_roundtrip() {
    let registry = AlgorithmRegistry::new();
    let algo = registry.get("xchacha20-poly1305").unwrap();

    let unicode_data = "Hello! Bonjour! Hallo! Ciao! Ola! Privyet!";
    let encrypted = encrypt_data(unicode_data.as_bytes(), TEST_PASSWORD, algo).unwrap();
    let decrypted = decrypt_data(encrypted.as_bytes(), TEST_PASSWORD, &registry).unwrap();

    assert_eq!(decrypted, unicode_data);
}

#[test]
fn test_config_defaults() {
    let config = Config::default();
    assert_eq!(config.encryption.algorithm, "aes-256-gcm");
    assert_eq!(config.encryption.iterations, "adaptive");
    assert!(config.security.secure_wipe);
    assert!(!config.security.backup_on_encrypt);
    assert_eq!(config.ui.theme, "system");
}

#[test]
fn test_config_save_load_roundtrip() {
    let tmp_dir = TempDir::new().unwrap();
    let config_path = tmp_dir.path().join("test_config.toml");

    let mut config = Config::default();
    config.encryption.algorithm = "chacha20-poly1305".to_string();
    config.security.backup_on_encrypt = true;
    config.ui.theme = "dark".to_string();

    config.save_to(&config_path).unwrap();

    let loaded = Config::load_from(&config_path).unwrap();
    assert_eq!(loaded.encryption.algorithm, "chacha20-poly1305");
    assert!(loaded.security.backup_on_encrypt);
    assert_eq!(loaded.ui.theme, "dark");
}

#[test]
fn test_aead_algorithms_have_no_hmac_field() {
    let registry = AlgorithmRegistry::new();
    let algo = registry.get("aes-256-gcm").unwrap();

    let encrypted = encrypt_data(TEST_DATA.as_bytes(), TEST_PASSWORD, algo).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&encrypted).unwrap();

    // AEAD algorithms should not have an hmac field
    assert!(parsed.get("hmac").is_none());
}

#[test]
fn test_non_aead_algorithms_have_hmac_field() {
    let registry = AlgorithmRegistry::new();
    let algo = registry.get("aes-256-cbc").unwrap();

    let encrypted = encrypt_data(TEST_DATA.as_bytes(), TEST_PASSWORD, algo).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&encrypted).unwrap();

    // Non-AEAD algorithms should have an hmac field
    assert!(parsed.get("hmac").is_some());
}
