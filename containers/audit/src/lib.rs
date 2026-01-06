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
    pub seq: i64,
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
        // Read last hash and seq to maintain strict insertion order even when timestamps collide
        // use audit-meta keys to avoid colliding with audit entries during prefix scans
        let prev_hash: Option<String> = self.db.get(b"audit_meta:last_hash")?;
        let last_seq_opt: Option<i64> = self.db.get(b"audit_meta:last_seq")?;
        let seq = last_seq_opt.map(|s| s + 1).unwrap_or(1);

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
            prev_hash: prev_hash.clone(),
            seq,
            timestamp,
            user_id,
            event,
            hash: hash.clone(),
        };

        let key = format!("audit:{}", id);
        self.db.put(key.as_bytes(), &entry)?;
        // update last pointers
        self.db.put(b"audit_meta:last_hash", &entry.hash)?;
        self.db.put(b"audit_meta:last_seq", &seq)?;

        Ok(entry)
    }

    pub fn verify(&self) -> Result<bool> {
        // Verify chain integrity by recomputing hashes in insertion order (seq)
        let mut items: Vec<AuditEntry> = self.db.prefix_scan(b"audit:")?;
        // deterministic sort by seq
        items.sort_by(|a, b| a.seq.cmp(&b.seq));
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
        // sort deterministically by seq (insertion order)
        items.sort_by(|a, b| a.seq.cmp(&b.seq));
        Ok(items)
    }

    /// Delete all audit entries for a given user and return the number deleted
    pub fn delete_for_user(&self, user_id: &str) -> Result<usize> {
        let items: Vec<AuditEntry> = self.db.prefix_scan(b"audit:")?;
        let mut deleted = 0usize;
        for e in items.into_iter().filter(|e| e.user_id.as_deref() == Some(user_id)) {
            let key = format!("audit:{}", e.id);
            self.db.delete(key.as_bytes())?;
            deleted += 1;
        }
        Ok(deleted)
    }
}

