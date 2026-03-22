pub mod algorithm;
pub mod algorithms;
pub mod config;
pub mod decryption;
pub mod encryption;
pub mod errors;
pub mod file_ops;
pub mod format;
pub mod memory_security;

// Re-export key types for convenience
pub use algorithm::{AlgorithmInfo, AlgorithmRegistry, CryptoAlgorithm};
pub use config::Config;
pub use errors::{CryptoError, Result};
pub use memory_security::{MemorySecurity, SecureMemoryGuard, SecureString};
