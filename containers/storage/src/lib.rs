use anyhow::{Context, Result};
use rocksdb::{DB, Options, IteratorMode};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync::Arc;
use tracing::{debug, info};

/// Storage container using RocksDB
#[derive(Clone)]
pub struct Storage {
    db: Arc<DB>,
}

impl Storage {
    /// Open database at specified path
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path_ref = path.as_ref();
        info!("Opening storage at: {:?}", path_ref);
        
        let mut opts = Options::default();
        opts.create_if_missing(true);
        opts.create_missing_column_families(true);
        opts.set_max_open_files(512);
        opts.set_keep_log_file_num(10);
        opts.set_max_background_jobs(4);
        
        let db = DB::open(&opts, path_ref)
            .context(format!("Failed to open RocksDB at {:?}", path_ref))?;
        
        info!("Storage opened successfully");
        
        Ok(Self {
            db: Arc::new(db),
        })
    }
    
    /// Put value with key
    pub fn put<K, V>(&self, key: K, value: &V) -> Result<()>
    where
        K: AsRef<[u8]>,
        V: Serialize,
    {
        let key_ref = key.as_ref();
        let key_str = std::str::from_utf8(key_ref).unwrap_or("<binary>");
        debug!("PUT: {}", key_str);
        
        let value_bytes = serde_json::to_vec(value)
            .context("Failed to serialize value")?;
        
        self.db.put(key_ref, value_bytes)
            .context(format!("Failed to write key: {}", key_str))?;
        
        Ok(())
    }
    
    /// Get value by key
    pub fn get<K, V>(&self, key: K) -> Result<Option<V>>
    where
        K: AsRef<[u8]>,
        V: for<'de> Deserialize<'de>,
    {
        let key_ref = key.as_ref();
        let key_str = std::str::from_utf8(key_ref).unwrap_or("<binary>");
        debug!("GET: {}", key_str);
        
        let value_bytes = self.db.get(key_ref)
            .context(format!("Failed to read key: {}", key_str))?;
        
        match value_bytes {
            Some(bytes) => {
                let value = serde_json::from_slice(&bytes)
                    .context("Failed to deserialize value")?;
                Ok(Some(value))
            }
            None => Ok(None),
        }
    }
    
    /// Delete value by key
    pub fn delete<K>(&self, key: K) -> Result<()>
    where
        K: AsRef<[u8]>,
    {
        let key_ref = key.as_ref();
        let key_str = std::str::from_utf8(key_ref).unwrap_or("<binary>");
        debug!("DELETE: {}", key_str);
        
        self.db.delete(key_ref)
            .context(format!("Failed to delete key: {}", key_str))?;
        
        Ok(())
    }
    
    /// Scan with prefix
    pub fn prefix_scan<K, V>(&self, prefix: K) -> Result<Vec<V>>
    where
        K: AsRef<[u8]>,
        V: for<'de> Deserialize<'de>,
    {
        let prefix_bytes = prefix.as_ref();
        debug!("PREFIX_SCAN: {:?}", std::str::from_utf8(prefix_bytes).unwrap_or("<binary>"));
        
        let iter = self.db.iterator(IteratorMode::From(prefix_bytes, rocksdb::Direction::Forward));
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

    /// Get underlying DB path if exposed
    pub fn path(&self) -> Option<&Path> {
        Some(self.db.path())
    }
}

// Backwards-compatible alias used by existing tests and other crates
// Some crates referenced `RocksDBStorage` previously; provide an alias
pub type RocksDBStorage = Storage;

// Database schema utilities
pub mod schema;
