use anyhow::{Context, Result};
use rocksdb::{DB, Options, IteratorMode};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync::Arc;
use tracing::{debug, error, info};

/// RocksDB storage wrapper
#[derive(Clone)]
pub struct RocksDBStorage {
    db: Arc<DB>,
}

impl RocksDBStorage {
    /// Open database at path
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        info!("Opening database at: {:?}", path.as_ref());

        let mut opts = Options::default();
        opts.create_if_missing(true);
        opts.create_missing_column_families(true);

        let db = DB::open(&opts, path.as_ref())
            .context("Failed to open RocksDB database")?;

        info!("Database opened successfully");

        Ok(Self { db: Arc::new(db) })
    }

    /// Put value with key
    pub fn put<K, V>(&self, key: K, value: &V) -> Result<()>
    where
        K: AsRef<[u8]>,
        V: Serialize,
    {
        let key_ref = key.as_ref();
        debug!("PUT key: {:?}", std::str::from_utf8(key_ref).unwrap_or("<binary>"));

        let value_bytes = serde_json::to_vec(value).context("Failed to serialize value")?;

        self.db
            .put(key_ref, value_bytes)
            .context("Failed to write to database")?;

        Ok(())
    }

    /// Get value by key
    pub fn get<K, V>(&self, key: K) -> Result<Option<V>>
    where
        K: AsRef<[u8]>,
        V: for<'de> Deserialize<'de>,
    {
        let key_ref = key.as_ref();
        debug!("GET key: {:?}", std::str::from_utf8(key_ref).unwrap_or("<binary>"));

        let value_bytes = self
            .db
            .get(key_ref)
            .context("Failed to read from database")?;

        match value_bytes {
            Some(bytes) => {
                let value = serde_json::from_slice(&bytes).context("Failed to deserialize value")?;
                Ok(Some(value))
            }
            None => {
                debug!("Key not found");
                Ok(None)
            }
        }
    }

    /// Delete value by key
    pub fn delete<K>(&self, key: K) -> Result<()>
    where
        K: AsRef<[u8]>,
    {
        let key_ref = key.as_ref();
        debug!("DELETE key: {:?}", std::str::from_utf8(key_ref).unwrap_or("<binary>"));

        self.db
            .delete(key_ref)
            .context("Failed to delete from database")?;

        Ok(())
    }

    /// Scan with prefix, returning deserialized values
    pub fn prefix_scan<K, V>(&self, prefix: K) -> Result<Vec<V>>
    where
        K: AsRef<[u8]>,
        V: for<'de> Deserialize<'de>,
    {
        let prefix_bytes = prefix.as_ref();
        debug!("PREFIX_SCAN: {:?}", std::str::from_utf8(prefix_bytes).unwrap_or("<binary>"));

        let iter = self
            .db
            .iterator(IteratorMode::From(prefix_bytes, rocksdb::Direction::Forward));

        let mut results = Vec::new();
        for item in iter {
            let (key, value) = item.context("Iterator error")?;

            if !key.starts_with(prefix_bytes) {
                break;
            }

            let v = serde_json::from_slice(&value).context("Failed to deserialize scanned value")?;
            results.push(v);
        }

        debug!("Found {} items with prefix", results.len());
        Ok(results)
    }

    /// Get database path when available
    pub fn path(&self) -> Option<&Path> {
        // rocksdb::DB has path() method returning &Path
        Some(self.db.path())
    }
}

