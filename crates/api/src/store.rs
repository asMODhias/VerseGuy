use chrono::{DateTime, Utc};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[allow(dead_code)]
#[derive(Debug)]
pub enum StoreError {
    Backend(String),
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct TokenRecord {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_at: DateTime<Utc>,
}

pub trait TokenStore: Send + Sync + 'static {
    fn insert(&self, refresh_token: String, record: TokenRecord) -> Result<(), StoreError>;
    fn get(&self, refresh_token: &str) -> Result<Option<TokenRecord>, StoreError>;
    #[allow(dead_code)]
    fn remove(&self, refresh_token: &str) -> Result<Option<TokenRecord>, StoreError>;
}

/// Simple in-memory store; used by default and by tests
pub struct InMemoryTokenStore {
    inner: Mutex<HashMap<String, TokenRecord>>,
}

impl InMemoryTokenStore {
    pub fn new() -> Self {
        InMemoryTokenStore {
            inner: Mutex::new(HashMap::new()),
        }
    }
}

impl TokenStore for InMemoryTokenStore {
    fn insert(&self, refresh_token: String, record: TokenRecord) -> Result<(), StoreError> {
        match self.inner.lock() {
            Ok(mut m) => {
                m.insert(refresh_token, record);
                Ok(())
            }
            Err(_) => Err(StoreError::Backend("lock_error".into())),
        }
    }

    fn get(&self, refresh_token: &str) -> Result<Option<TokenRecord>, StoreError> {
        match self.inner.lock() {
            Ok(m) => Ok(m.get(refresh_token).cloned()),
            Err(_) => Err(StoreError::Backend("lock_error".into())),
        }
    }

    fn remove(&self, refresh_token: &str) -> Result<Option<TokenRecord>, StoreError> {
        match self.inner.lock() {
            Ok(mut m) => Ok(m.remove(refresh_token)),
            Err(_) => Err(StoreError::Backend("lock_error".into())),
        }
    }
}

/// Sled-backed store for persistence
pub struct SledTokenStore {
    db: sled::Db,
}

impl SledTokenStore {
    pub fn new(path: &str) -> Result<Self, StoreError> {
        match sled::open(path) {
            Ok(db) => Ok(SledTokenStore { db }),
            Err(e) => Err(StoreError::Backend(format!("sled open: {}", e))),
        }
    }
}

impl TokenStore for SledTokenStore {
    fn insert(&self, refresh_token: String, record: TokenRecord) -> Result<(), StoreError> {
        match serde_json::to_vec(&record) {
            Ok(bytes) => match self.db.insert(refresh_token.as_bytes(), bytes) {
                Ok(_) => {
                    let _ = self.db.flush();
                    Ok(())
                }
                Err(e) => Err(StoreError::Backend(format!("sled insert: {}", e))),
            },
            Err(e) => Err(StoreError::Backend(format!("serialize: {}", e))),
        }
    }

    fn get(&self, refresh_token: &str) -> Result<Option<TokenRecord>, StoreError> {
        match self.db.get(refresh_token.as_bytes()) {
            Ok(Some(iv)) => match serde_json::from_slice(&iv) {
                Ok(rec) => Ok(Some(rec)),
                Err(e) => Err(StoreError::Backend(format!("deserialize: {}", e))),
            },
            Ok(None) => Ok(None),
            Err(e) => Err(StoreError::Backend(format!("sled get: {}", e))),
        }
    }

    fn remove(&self, refresh_token: &str) -> Result<Option<TokenRecord>, StoreError> {
        match self.db.remove(refresh_token.as_bytes()) {
            Ok(Some(iv)) => match serde_json::from_slice(&iv) {
                Ok(rec) => {
                    let _ = self.db.flush();
                    Ok(Some(rec))
                }
                Err(e) => Err(StoreError::Backend(format!("deserialize: {}", e))),
            },
            Ok(None) => Ok(None),
            Err(e) => Err(StoreError::Backend(format!("sled remove: {}", e))),
        }
    }
}

use std::env;

/// Global store selection: default = in-memory; set VERSEGUY_API_TOKEN_STORE=sled to use sled persistence
pub static TOKEN_STORE: Lazy<Arc<dyn TokenStore>> = Lazy::new(|| {
    let backend = env::var("VERSEGUY_API_TOKEN_STORE").unwrap_or_default();
    if backend == "sled" {
        match SledTokenStore::new("data/verseguy_tokens") {
            Ok(s) => Arc::new(s),
            Err(_) => Arc::new(InMemoryTokenStore::new()),
        }
    } else {
        Arc::new(InMemoryTokenStore::new())
    }
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn in_memory_store_insert_get_remove() {
        let s = InMemoryTokenStore::new();
        let rec = TokenRecord {
            access_token: "a".into(),
            refresh_token: "r1".into(),
            expires_at: Utc::now(),
        };
        assert!(s.insert("r1".into(), rec.clone()).is_ok());
        match s.get("r1") {
            Ok(Some(g)) => assert_eq!(g.access_token, "a"),
            Ok(None) => panic!("missing record"),
            Err(_) => panic!("store error"),
        }
        match s.remove("r1") {
            Ok(Some(_)) => (),
            Ok(None) => panic!("expected removal"),
            Err(_) => panic!("store error"),
        }
        match s.get("r1") {
            Ok(None) => (),
            Ok(Some(_)) => panic!("should be gone"),
            Err(_) => panic!("store error"),
        }
    }
}
