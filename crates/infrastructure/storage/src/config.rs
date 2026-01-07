use crate::prelude::*;
use base64::Engine;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Storage configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    /// Database path
    pub path: PathBuf,

    /// Enable encryption at rest
    pub encryption_enabled: bool,

    /// Encryption key (32 bytes, base64 encoded)
    pub encryption_key: Option<String>,

    /// Enable write-ahead log (WAL)
    pub wal_enabled: bool,

    /// Cache size in MB
    pub cache_size_mb: usize,

    /// Max open files
    pub max_open_files: i32,

    /// Enable compression
    pub compression_enabled: bool,

    /// Backup directory
    pub backup_dir: Option<PathBuf>,

    /// Auto-backup interval in hours (0 = disabled)
    pub auto_backup_hours: u64,

    /// Number of backups to keep
    pub backup_retention: usize,
}

impl Default for StorageConfig {
    fn default() -> Self {
        Self {
            path: PathBuf::from("./data/db"),
            encryption_enabled: true,
            encryption_key: None,
            wal_enabled: true,
            cache_size_mb: 256,
            max_open_files: 1000,
            compression_enabled: true,
            backup_dir: Some(PathBuf::from("./data/backups")),
            auto_backup_hours: 24,
            backup_retention: 7,
        }
    }
}

impl StorageConfig {
    /// Validate configuration
    pub fn validate(&self) -> AppResult<()> {
        // Path validation
        if self.path.to_str().is_some_and(|s| s.is_empty()) {
            return Err(configuration_err("Database path cannot be empty"))
                .with_context(|| "field=path");
        }

        // Cache size validation
        if self.cache_size_mb == 0 {
            return Err(configuration_err("Cache size must be > 0"))
                .with_context(|| "field=cache_size_mb");
        }

        if self.cache_size_mb > 8192 {
            return Err(configuration_err("Cache size too large (max 8GB)"))
                .with_context(|| "field=cache_size_mb")
                .with_context(|| "max=8192");
        }

        // Encryption key validation
        if self.encryption_enabled {
            if let Some(key) = &self.encryption_key {
                // Decode base64
                let decoded = base64::engine::general_purpose::STANDARD
                    .decode(key)
                    .map_err(|e| configuration_err(format!("Invalid encryption key: {}", e)))
                    .with_context(|| "field=encryption_key")?;

                if decoded.len() != 32 {
                    return Err(configuration_err("Encryption key must be 32 bytes"))
                        .with_context(|| "field=encryption_key")
                        .with_context(|| format!("actual_length={}", decoded.len()));
                }
            }
        }

        // Backup validation
        if self.backup_retention == 0 {
            return Err(configuration_err("Backup retention must be > 0"))
                .with_context(|| "field=backup_retention");
        }

        Ok(())
    }

    /// Development configuration
    pub fn development() -> Self {
        Self {
            path: PathBuf::from("./data/dev.db"),
            encryption_enabled: false,
            cache_size_mb: 64,
            auto_backup_hours: 0, // Disabled
            ..Default::default()
        }
    }

    /// Production configuration
    pub fn production() -> Self {
        Self {
            path: PathBuf::from("/var/lib/verseguy/db"),
            encryption_enabled: true,
            cache_size_mb: 512,
            auto_backup_hours: 6,
            backup_retention: 30,
            ..Default::default()
        }
    }
}
