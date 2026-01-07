use crate::config::StorageConfig;
use metrics;
use rocksdb::{Options, DB};

use std::sync::Arc;
use std::time::Instant;
use tracing::{debug, error, info};
use crate::prelude::*;

use rand::RngCore;
use base64::Engine;

/// Storage engine wrapping RocksDB
pub struct StorageEngine {
    db: Arc<DB>,
    _config: StorageConfig,
    encryption_key: Option<[u8; 32]>,
}

impl StorageEngine {
    /// Open storage engine
    pub fn open(config: StorageConfig) -> AppResult<Self> {
        // Validate config first
        config.validate().with_context(|| "Invalid storage configuration")?;

        info!(
            path = %config.path.display(),
            encryption = config.encryption_enabled,
            "Opening storage engine"
        );

        // Create directory if it doesn't exist
        if let Some(parent) = config.path.parent() {
            std::fs::create_dir_all(parent).with_context(|| {
                format!("Failed to create database directory: {}", parent.display())
            })?;
        }

        // Configure RocksDB
        let mut opts = Options::default();
        opts.create_if_missing(true);
        opts.set_max_open_files(config.max_open_files);
        opts.set_write_buffer_size(64 * 1024 * 1024); // 64MB
        opts.set_max_write_buffer_number(3);
        opts.set_target_file_size_base(64 * 1024 * 1024); // 64MB

        // Enable compression
        if config.compression_enabled {
            opts.set_compression_type(rocksdb::DBCompressionType::Lz4);
        }

        // WAL configuration
        if config.wal_enabled {
            opts.set_wal_size_limit_mb(100);
        }

        // Open database
        let db = DB::open(&opts, &config.path).map_err(|e| {
            error!(error = %e, "Failed to open database");
            storage_err(format!("Failed to open database: {}", e)).context(format!("path={}", config.path.display()))
        })?;

        // Setup encryption
        let encryption_key = if config.encryption_enabled {
            Some(Self::load_or_generate_key(&config)?)
        } else {
            None
        };

        info!("Storage engine opened successfully");
        metrics::counter!("storage_opened_total", 1);

        Ok(Self {
            db: Arc::new(db),
            _config: config,
            encryption_key,
        })
    }

    /// Get value by key
    pub fn get(&self, key: &[u8]) -> AppResult<Option<Vec<u8>>> {
        let start = Instant::now();

        let result = self.db.get(key).map_err(|e| {
            error!(error = %e, key = ?key, "Failed to get value");
            storage_err(format!("Failed to get value: {}", e))
        })?;

        let duration = start.elapsed();
        metrics::histogram!("storage_get_duration_seconds", duration.as_secs_f64());
        metrics::counter!("storage_get_total", 1);

        // Decrypt if encryption is enabled
        let decrypted = if let Some(data) = result {
            if let Some(key) = &self.encryption_key {
                let encrypted_str = String::from_utf8(data).map_err(|e| storage_err(format!("Invalid UTF-8 in encrypted data: {}", e)))?;

                let decrypted = crate::engine::security_fallback::decrypt_data(&encrypted_str, key)
                    .with_context(|| "Failed to decrypt data")?;

                Some(decrypted)
            } else {
                Some(data)
            }
        } else {
            None
        };

        Ok(decrypted)
    }

    /// Put value by key
    pub fn put(&self, key: &[u8], value: &[u8]) -> AppResult<()> {
        let start = Instant::now();

        // Encrypt if encryption is enabled
        let data_to_store = if let Some(key_bytes) = &self.encryption_key {
            let encrypted = crate::engine::security_fallback::encrypt_data(value, key_bytes)
                .with_context(|| "Failed to encrypt data")?;
            encrypted.into_bytes()
        } else {
            value.to_vec()
        };

        self.db.put(key, &data_to_store).map_err(|e| {
            error!(error = %e, key = ?key, "Failed to put value");
            storage_err(format!("Failed to put value: {}", e))
        })?;

        let duration = start.elapsed();
        metrics::histogram!("storage_put_duration_seconds", duration.as_secs_f64());
        metrics::counter!("storage_put_total", 1);

        Ok(())
    }

    /// Delete value by key
    pub fn delete(&self, key: &[u8]) -> AppResult<()> {
        let start = Instant::now();

        self.db.delete(key).map_err(|e| {
            error!(error = %e, key = ?key, "Failed to delete value");
            storage_err(format!("Failed to delete value: {}", e))
        })?;

        let duration = start.elapsed();
        metrics::histogram!("storage_delete_duration_seconds", duration.as_secs_f64());
        metrics::counter!("storage_delete_total", 1);

        Ok(())
    }

    /// Check if key exists
    pub fn exists(&self, key: &[u8]) -> AppResult<bool> {
        match self.get(key)? {
            Some(_) => Ok(true),
            None => Ok(false),
        }
    }

    /// Scan keys with prefix
    pub fn scan_prefix(&self, prefix: &[u8]) -> AppResult<Vec<(Vec<u8>, Vec<u8>)>> {
        let start = Instant::now();
        let mut results = Vec::new();

        let iter = self.db.prefix_iterator(prefix);

        for item in iter {
            let (key, value) = item.map_err(|e| storage_err(format!("Failed to iterate: {}", e)))?;

            // Decrypt value if needed
            let decrypted_value = if let Some(enc_key) = &self.encryption_key {
                let encrypted_str = String::from_utf8(value.to_vec()).map_err(|e| storage_err(format!("Invalid UTF-8 in encrypted data: {}", e)))?;

                crate::engine::security_fallback::decrypt_data(&encrypted_str, enc_key)
                    .with_context(|| "Failed to decrypt data")?
            } else {
                value.to_vec()
            };

            results.push((key.to_vec(), decrypted_value));
        }

        let duration = start.elapsed();
        metrics::histogram!("storage_scan_duration_seconds", duration.as_secs_f64());
        metrics::counter!("storage_scan_total", 1);

        debug!(prefix = ?prefix, count = results.len(), "Prefix scan completed");

        Ok(results)
    }

    /// Flush WAL to disk
    pub fn flush(&self) -> AppResult<()> {
        self.db.flush().map_err(|e| {
            error!(error = %e, "Failed to flush database");
            storage_err(format!("Failed to flush: {}", e))
        })?;

        debug!("Database flushed");
        Ok(())
    }

    /// Get database statistics
    pub fn stats(&self) -> AppResult<String> {
        self.db
            .property_value("rocksdb.stats")
            .map_err(|e| storage_err(format!("Failed to get stats: {}", e)))?
            .ok_or_else(|| storage_err("Stats not available".to_string()))
    }

    /// Load or generate encryption key
    fn load_or_generate_key(config: &StorageConfig) -> AppResult<[u8; 32]> {
        // 1) Prefer explicit config-provided key
        if let Some(key_str) = &config.encryption_key {
            let decoded = base64::engine::general_purpose::STANDARD
                .decode(key_str)
                .map_err(|e| configuration_err(format!("Invalid encryption key: {}", e)))?;

            if decoded.len() != 32 {
                return Err(configuration_err("Encryption key must be 32 bytes"));
            }

            let mut key = [0u8; 32];
            key.copy_from_slice(&decoded);
            return Ok(key);
        }

        // 2) Try KeyStore (keyring primary, file fallback)
        if let Some(existing) = crate::key_store::KeyStore::get_key(config)? {
            return Ok(existing);
        }

        // 3) Generate a new key and persist it
        let mut key = [0u8; 32];
        let mut rng = rand::rngs::OsRng;
        let _ = rng.try_fill_bytes(&mut key);

        if let Err(e) = crate::key_store::KeyStore::store_key(config, &key) {
            tracing::warn!(error = %e, "Failed to persist encryption key (keyring/file fallback)");
        }

        Ok(key)
    }
}

mod security_fallback {
    use crate::prelude::*;
    use rand::RngCore;
    use base64::Engine;

    /// Simple XOR-based fallback encryption + base64 encode
    pub fn encrypt_data(value: &[u8], key: &[u8; 32]) -> AppResult<String> {
        let mut out = value.to_vec();
        for (i, b) in out.iter_mut().enumerate() {
            *b ^= key[i % key.len()];
        }
        Ok(base64::engine::general_purpose::STANDARD.encode(&out))
    }

    pub fn decrypt_data(encrypted: &str, key: &[u8; 32]) -> AppResult<Vec<u8>> {
        let mut decoded = base64::engine::general_purpose::STANDARD.decode(encrypted).map_err(|e| internal_err(format!("Failed to base64-decode: {}", e)))?;
        for (i, b) in decoded.iter_mut().enumerate() {
            *b ^= key[i % key.len()];
        }
        Ok(decoded)
    }

    #[allow(dead_code)]
    fn generate_encryption_key() -> [u8; 32] {
        let mut key = [0u8; 32];
        // Use OS RNG to generate key
        let mut rng = rand::rngs::OsRng;
        let _ = rng.try_fill_bytes(&mut key);
        key
    }
}

// Ensure proper cleanup
impl Drop for StorageEngine {
    fn drop(&mut self) {
        info!("Closing storage engine");

        // Flush before closing
        if let Err(e) = self.flush() {
            error!(error = %e, "Failed to flush on close");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn test_config(temp_dir: &TempDir) -> StorageConfig {
        StorageConfig {
            path: temp_dir.path().join("test.db"),
            encryption_enabled: false,
            ..Default::default()
        }
    }

    #[test]
    fn test_open_storage() -> AppResult<()> {
        let temp_dir = TempDir::new()?;
        let config = test_config(&temp_dir);

        let _storage = StorageEngine::open(config)?;
        Ok(())
    }

    #[test]
    fn test_put_get() -> AppResult<()> {
        let temp_dir = TempDir::new()?;
        let config = test_config(&temp_dir);
        let storage = StorageEngine::open(config)?;

        let key = b"test_key";
        let value = b"test_value";

        storage.put(key, value)?;
        let retrieved = storage.get(key)?;

        assert_eq!(retrieved, Some(value.to_vec()));
        Ok(())
    }

    #[test]
    fn test_delete() -> AppResult<()> {
        let temp_dir = TempDir::new()?;
        let config = test_config(&temp_dir);
        let storage = StorageEngine::open(config)?;

        let key = b"test_key";
        let value = b"test_value";

        storage.put(key, value)?;
        assert!(storage.exists(key)?);

        storage.delete(key)?;
        assert!(!storage.exists(key)?);
        Ok(())
    }

    #[test]
    fn test_scan_prefix() -> AppResult<()> {
        let temp_dir = TempDir::new()?;
        let config = test_config(&temp_dir);
        let storage = StorageEngine::open(config)?;

        storage.put(b"user:1", b"alice")?;
        storage.put(b"user:2", b"bob")?;
        storage.put(b"org:1", b"acme")?;

        let results = storage.scan_prefix(b"user:")?;

        assert_eq!(results.len(), 2);
        Ok(())
    }

    #[test]
    fn test_encryption() -> AppResult<()> {
        let temp_dir = TempDir::new()?;
        let mut config = test_config(&temp_dir);
        config.encryption_enabled = true;

        let storage = StorageEngine::open(config)?;

        let key = b"secret_key";
        let value = b"secret_value";

        storage.put(key, value)?;
        let retrieved = storage.get(key)?;

        assert_eq!(retrieved, Some(value.to_vec()));
        Ok(())
    }
}
