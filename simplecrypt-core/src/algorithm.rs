use crate::errors::Result;

/// Trait defining a cryptographic algorithm for encryption/decryption
pub trait CryptoAlgorithm: Send + Sync {
    /// Unique string identifier stored in the encrypted file envelope
    fn id(&self) -> &'static str;

    /// Human-readable name for display in UI
    fn display_name(&self) -> &'static str;

    /// Required nonce/IV size in bytes
    fn nonce_size(&self) -> usize;

    /// Required key size in bytes
    fn key_size(&self) -> usize;

    /// Whether this algorithm provides built-in authentication (AEAD).
    /// If false, the framework adds HMAC-SHA256 externally.
    fn is_aead(&self) -> bool;

    /// Encrypt plaintext with the given key and nonce. Returns ciphertext.
    /// For AEAD algorithms, the authentication tag is appended to ciphertext.
    fn encrypt(&self, key: &[u8], nonce: &[u8], plaintext: &[u8]) -> Result<Vec<u8>>;

    /// Decrypt ciphertext with the given key and nonce. Returns plaintext.
    /// For AEAD algorithms, verifies the authentication tag.
    fn decrypt(&self, key: &[u8], nonce: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>>;
}

/// Information about an available algorithm (for UI display)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AlgorithmInfo {
    pub id: String,
    pub display_name: String,
    pub is_aead: bool,
    pub key_size: usize,
    pub nonce_size: usize,
}

impl AlgorithmInfo {
    pub fn from_algorithm(algo: &dyn CryptoAlgorithm) -> Self {
        Self {
            id: algo.id().to_string(),
            display_name: algo.display_name().to_string(),
            is_aead: algo.is_aead(),
            key_size: algo.key_size(),
            nonce_size: algo.nonce_size(),
        }
    }
}

/// Algorithm registry — maps algorithm IDs to implementations
pub struct AlgorithmRegistry {
    algorithms: Vec<Box<dyn CryptoAlgorithm>>,
}

impl AlgorithmRegistry {
    /// Create a new registry with all built-in algorithms
    pub fn new() -> Self {
        let algorithms: Vec<Box<dyn CryptoAlgorithm>> = vec![
            Box::new(super::algorithms::aes256cbc::Aes256Cbc),
            Box::new(super::algorithms::aes256gcm::Aes256Gcm),
            Box::new(super::algorithms::chacha20poly1305::ChaCha20Poly1305Algorithm),
            Box::new(super::algorithms::xchacha20poly1305::XChaCha20Poly1305Algorithm),
        ];

        Self { algorithms }
    }

    /// Look up an algorithm by its string ID
    pub fn get(&self, id: &str) -> Option<&dyn CryptoAlgorithm> {
        self.algorithms.iter()
            .find(|a| a.id() == id)
            .map(|a| a.as_ref())
    }

    /// List all available algorithms
    pub fn list(&self) -> Vec<AlgorithmInfo> {
        self.algorithms.iter()
            .map(|a| AlgorithmInfo::from_algorithm(a.as_ref()))
            .collect()
    }

    /// Get the default algorithm (AES-256-GCM)
    pub fn default_algorithm(&self) -> &dyn CryptoAlgorithm {
        self.get("aes-256-gcm").expect("default algorithm must exist")
    }
}

impl Default for AlgorithmRegistry {
    fn default() -> Self {
        Self::new()
    }
}
