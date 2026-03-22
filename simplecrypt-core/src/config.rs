use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::errors::{CryptoError, Result};

/// Application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub encryption: EncryptionConfig,
    #[serde(default)]
    pub security: SecurityConfig,
    #[serde(default)]
    pub ui: UiConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionConfig {
    /// Default algorithm for new encryptions
    #[serde(default = "default_algorithm")]
    pub algorithm: String,
    /// Iteration mode: "adaptive" or a fixed number
    #[serde(default = "default_iterations")]
    pub iterations: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    /// Wipe sensitive memory on drop
    #[serde(default = "default_true")]
    pub secure_wipe: bool,
    /// Auto-create .backup files before encryption
    #[serde(default)]
    pub backup_on_encrypt: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiConfig {
    /// Theme: "light", "dark", or "system"
    #[serde(default = "default_theme")]
    pub theme: String,
}

fn default_algorithm() -> String { "aes-256-gcm".to_string() }
fn default_iterations() -> String { "adaptive".to_string() }
fn default_true() -> bool { true }
fn default_theme() -> String { "system".to_string() }

impl Default for Config {
    fn default() -> Self {
        Self {
            encryption: EncryptionConfig::default(),
            security: SecurityConfig::default(),
            ui: UiConfig::default(),
        }
    }
}

impl Default for EncryptionConfig {
    fn default() -> Self {
        Self {
            algorithm: default_algorithm(),
            iterations: default_iterations(),
        }
    }
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            secure_wipe: true,
            backup_on_encrypt: false,
        }
    }
}

impl Default for UiConfig {
    fn default() -> Self {
        Self {
            theme: default_theme(),
        }
    }
}

impl Config {
    /// Get the platform-specific config directory path
    pub fn config_dir() -> Option<PathBuf> {
        dirs::config_dir().map(|d| d.join("SimpleCrypt"))
    }

    /// Get the full path to the config file
    pub fn config_file_path() -> Option<PathBuf> {
        Self::config_dir().map(|d| d.join("config.toml"))
    }

    /// Load config from the default platform path, falling back to defaults
    pub fn load() -> Self {
        Self::config_file_path()
            .and_then(|path| {
                if path.exists() {
                    std::fs::read_to_string(&path).ok()
                } else {
                    None
                }
            })
            .and_then(|contents| toml::from_str(&contents).ok())
            .unwrap_or_default()
    }

    /// Load config from a specific path
    pub fn load_from(path: &std::path::Path) -> Result<Self> {
        let contents = std::fs::read_to_string(path)?;
        toml::from_str(&contents).map_err(|e| CryptoError::ConfigError(e.to_string()))
    }

    /// Save config to the default platform path
    pub fn save(&self) -> Result<()> {
        let config_dir = Self::config_dir()
            .ok_or_else(|| CryptoError::ConfigError("Cannot determine config directory".to_string()))?;

        std::fs::create_dir_all(&config_dir)?;

        let path = config_dir.join("config.toml");
        let contents = toml::to_string_pretty(self)
            .map_err(|e| CryptoError::ConfigError(e.to_string()))?;

        std::fs::write(&path, contents)?;
        Ok(())
    }

    /// Save config to a specific path
    pub fn save_to(&self, path: &std::path::Path) -> Result<()> {
        let contents = toml::to_string_pretty(self)
            .map_err(|e| CryptoError::ConfigError(e.to_string()))?;
        std::fs::write(path, contents)?;
        Ok(())
    }
}
