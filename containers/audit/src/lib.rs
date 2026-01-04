use anyhow::Result;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::sync::Arc;
use uuid::Uuid;
use verseguy_storage::RocksDBStorage;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct AuditEntry {
    pub id: String,
    pub prev_hash: Option<String>,
    pub timestamp: i64,
    pub user_id: Option<String>,
    pub event: String,
    pub hash: String,
}

pub struct AuditService {
    db: Arc<RocksDBStorage>,
}

impl AuditService {
    pub fn new(db: Arc<RocksDBStorage>) -> Self {
        Self { db }
    }

    pub fn log_event(&self, user_id: Option<String>, event: String) -> Result<AuditEntry> {
        // scan for last audit entry using deterministic ordering (timestamp, id)
        let mut prev_hash: Option<String> = None;
        let mut items: Vec<AuditEntry> = self.db.prefix_scan(b"audit:")?;
        // sort by timestamp then id to have deterministic order and pick the last as previous
        items.sort_by(|a, b| (a.timestamp, a.id.as_str()).cmp(&(b.timestamp, b.id.as_str())));
        if let Some(last) = items.last() {
            prev_hash = Some(last.hash.clone());
        }

        let id = Uuid::new_v4().to_string();
        // use millisecond resolution to reduce chance of identical timestamps
        let timestamp = Utc::now().timestamp_millis();

        // compute hash = sha256(prev_hash || timestamp || user_id || event)
        let mut hasher = Sha256::new();
        if let Some(ref p) = prev_hash {
            hasher.update(p.as_bytes());
        }
        hasher.update(timestamp.to_string().as_bytes());
        if let Some(ref u) = user_id {
            hasher.update(u.as_bytes());
        }
        hasher.update(event.as_bytes());
        let hash = hex::encode(hasher.finalize());

        let entry = AuditEntry {
            id: id.clone(),
            prev_hash,
            timestamp,
            user_id,
            event,
            hash: hash.clone(),
        };

        let key = format!("audit:{}", id);
        self.db.put(key.as_bytes(), &entry)?;

        Ok(entry)
    }

    pub fn verify(&self) -> Result<bool> {
        // Verify chain integrity by recomputing hashes in chronological order
        let mut items: Vec<AuditEntry> = self.db.prefix_scan(b"audit:")?;
        // deterministic sort by timestamp then id
        items.sort_by(|a, b| (a.timestamp, a.id.as_str()).cmp(&(b.timestamp, b.id.as_str())));
        let mut prev_hash: Option<String> = None;
        for it in items {
            // recompute hash
            let mut hasher = Sha256::new();
            if let Some(ref p) = prev_hash {
                hasher.update(p.as_bytes());
            }
            hasher.update(it.timestamp.to_string().as_bytes());
            if let Some(ref u) = it.user_id {
                hasher.update(u.as_bytes());
            }
            hasher.update(it.event.as_bytes());
            let expected = hex::encode(hasher.finalize());
            if expected != it.hash {
                return Ok(false);
            }
            prev_hash = Some(it.hash);
        }
        Ok(true)
    }

    pub fn export_for_user(&self, user_id: &str) -> Result<Vec<AuditEntry>> {
        let mut items: Vec<AuditEntry> = self.db.prefix_scan(b"audit:")?;
        items.retain(|e| e.user_id.as_deref() == Some(user_id));
        // sort deterministically by timestamp then id
        items.sort_by(|a, b| (a.timestamp, a.id.as_str()).cmp(&(b.timestamp, b.id.as_str())));
        Ok(items)
    }
}
