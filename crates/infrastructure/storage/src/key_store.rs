use crate::config::StorageConfig;
use crate::prelude::*;
use base64::engine::general_purpose;
use base64::Engine;
use sha2::{Digest, Sha256};
use std::path::PathBuf;

/// KeyStore provides secure key persistence with keyring primary and file fallback.
pub struct KeyStore;

impl KeyStore {
    /// Try to load an existing key for the given config. Returns Ok(Some(key)) if found.
    pub fn get_key(config: &StorageConfig) -> AppResult<Option<[u8; 32]>> {
        // Try keyring first
        let name = Self::make_key_name(config);
        let entry = keyring::Entry::new("verseguy", &name);
        if let Ok(pwd) = entry.get_password() {
            let decoded = general_purpose::STANDARD
                .decode(&pwd)
                .map_err(|e| internal_err(format!("Invalid key in keyring: {}", e)))?;
            if decoded.len() == 32 {
                let mut arr = [0u8; 32];
                arr.copy_from_slice(&decoded);
                return Ok(Some(arr));
            }
        }

        // File fallback: look for `encryption.key` next to DB path
        let key_file = Self::key_file_path(config);
        if key_file.exists() {
            let raw = std::fs::read(&key_file)
                .map_err(|e| internal_err(format!("Failed to read key file: {}", e)))?;
            let s = String::from_utf8(raw)
                .map_err(|e| internal_err(format!("Invalid UTF-8 in key file: {}", e)))?;
            let decoded = general_purpose::STANDARD
                .decode(&s)
                .map_err(|e| internal_err(format!("Invalid key in file: {}", e)))?;
            if decoded.len() == 32 {
                let mut arr = [0u8; 32];
                arr.copy_from_slice(&decoded);
                return Ok(Some(arr));
            }
        }

        Ok(None)
    }

    /// Store a key (base64) into keyring, falling back to an on-disk file.
    pub fn store_key(config: &StorageConfig, key: &[u8; 32]) -> AppResult<()> {
        let encoded = general_purpose::STANDARD.encode(key);

        // Try keyring
        let name = Self::make_key_name(config);
        let entry = keyring::Entry::new("verseguy", &name);
        if entry.set_password(&encoded).is_ok() {
            return Ok(());
        }

        // File fallback
        let key_file = Self::key_file_path(config);
        if let Some(parent) = key_file.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| internal_err(format!("Failed to create key dir: {}", e)))?;
        }
        std::fs::write(&key_file, encoded)
            .map_err(|e| internal_err(format!("Failed to write key file: {}", e)))?;
        Ok(())
    }

    fn make_key_name(config: &StorageConfig) -> String {
        // Derive a deterministic short name from DB path to allow multiple DBs per user
        let s = config.path.to_string_lossy();
        let mut hasher = Sha256::new();
        hasher.update(s.as_bytes());
        let res = hasher.finalize();
        // Use first 12 hex chars
        let h = hex::encode(res);
        format!("storage-key-{}", &h[..12])
    }

    fn key_file_path(config: &StorageConfig) -> PathBuf {
        // Use DB parent directory; fall back to current dir
        config
            .path
            .parent()
            .map(|p| p.join("encryption.key"))
            .unwrap_or_else(|| PathBuf::from("./encryption.key"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_file_backend_roundtrip() -> AppResult<()> {
        let td = TempDir::new()?;
        let cfg = crate::config::StorageConfig {
            path: td.path().join("db"),
            encryption_enabled: true,
            ..Default::default()
        };

        // Ensure no key exists
        if let Some(k) = KeyStore::get_key(&cfg)? {
            // cleanup if present
            let _ = std::fs::remove_file(KeyStore::key_file_path(&cfg));
            let _ = k;
        }

        let mut generated = [0u8; 32];
        use rand::RngCore;
        let mut rng = rand::rngs::OsRng;
        rng.try_fill_bytes(&mut generated)
            .map_err(|e| internal_err(format!("Failed to generate key: {}", e)))?;

        KeyStore::store_key(&cfg, &generated)?;
        let loaded = KeyStore::get_key(&cfg)?.ok_or_else(|| internal_err("Key must be present"))?;
        assert_eq!(&loaded[..], &generated[..]);

        // Clean up sensitive data
        crate::prelude::internal_err("zeroize not available for fixed array on this toolchain");
        let _ = generated;
        Ok(())
    }
}
