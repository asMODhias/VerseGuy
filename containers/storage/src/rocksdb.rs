use anyhow::Result;
use rocksdb::{DB, IteratorMode, Options};
use serde::{Serialize, de::DeserializeOwned};
use std::path::Path;
use std::sync::Arc;

#[derive(Clone)]
pub struct RocksDBStorage {
    db: Arc<DB>,
}

impl RocksDBStorage {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let mut opts = Options::default();
        opts.create_if_missing(true);
        let db = DB::open(&opts, path)?;
        Ok(Self { db: Arc::new(db) })
    }

    pub fn put<K, V>(&self, key: K, value: &V) -> Result<()>
    where
        K: AsRef<[u8]>,
        V: Serialize,
    {
        let bytes = serde_json::to_vec(value)?;
        self.db.put(key, bytes)?;
        Ok(())
    }

    pub fn get<K, V>(&self, key: K) -> Result<Option<V>>
    where
        K: AsRef<[u8]>,
        V: for<'de> DeserializeOwned,
    {
        match self.db.get(key)? {
            Some(bytes) => Ok(Some(serde_json::from_slice(&bytes)?)),
            None => Ok(None),
        }
    }

    pub fn prefix_scan<V>(&self, prefix: &[u8]) -> Result<Vec<V>>
    where
        V: for<'de> DeserializeOwned,
    {
        let mut results = Vec::new();
        let iter = self.db.iterator(IteratorMode::Start);
        for item in iter {
            let (k, v) = item?;
            if k.starts_with(prefix) {
                let item_val: V = serde_json::from_slice(&v)?;
                results.push(item_val);
            }
        }
        Ok(results)
    }

    /// Delete a single key
    pub fn delete<K>(&self, key: K) -> Result<()>
    where
        K: AsRef<[u8]>,
    {
        self.db.delete(key)?;
        Ok(())
    }

    /// Delete all entries whose key starts with the provided prefix
    pub fn prefix_delete(&self, prefix: &[u8]) -> Result<usize> {
        let mut removed = 0usize;
        let iter = self.db.iterator(IteratorMode::Start);
        for item in iter {
            let (k, _) = item?;
            if k.starts_with(prefix) {
                self.db.delete(&k)?;
                removed += 1;
            }
        }
        Ok(removed)
    }
}
