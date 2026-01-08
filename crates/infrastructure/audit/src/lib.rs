//! verseguy-audit: simple audit/event store

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEvent {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub principal_id: String,
    pub action: String,
    pub resource: String,
    pub metadata: serde_json::Value,
    pub version: u64,
}

impl verseguy_storage_infra::repository::Entity for AuditEvent {
    fn entity_type() -> &'static str {
        "audit_event"
    }

    fn id(&self) -> &str {
        &self.id
    }

    fn version(&self) -> u64 {
        self.version
    }

    fn increment_version(&mut self) {
        self.version = self.version.saturating_add(1);
    }
}

pub struct AuditStore {
    repo: verseguy_storage_infra::Repository<AuditEvent>,
}

impl AuditStore {
    pub fn new(engine: std::sync::Arc<verseguy_storage_infra::engine::StorageEngine>) -> Self {
        Self {
            repo: verseguy_storage_infra::Repository::new(engine.clone()),
        }
    }

    pub fn record(&self, e: &mut AuditEvent) -> verseguy_storage_infra::prelude::AppResult<()> {
        self.repo.save(e)?;
        Ok(())
    }

    pub fn list_recent(
        &self,
        limit: usize,
    ) -> verseguy_storage_infra::prelude::AppResult<Vec<AuditEvent>> {
        let mut all = self.repo.list()?;
        all.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        Ok(all.into_iter().take(limit).collect())
    }

    /// Purge events older than `older_than`. Returns number of deleted events.
    pub fn purge_older_than(
        &self,
        older_than: DateTime<Utc>,
    ) -> verseguy_storage_infra::prelude::AppResult<usize> {
        let to_delete = self.repo.find(|e| e.timestamp < older_than)?;
        for e in &to_delete {
            self.repo.delete(&e.id)?;
        }
        Ok(to_delete.len())
    }

    /// Find events older than `older_than` without deleting them (useful for dry-run).
    pub fn find_older_than(
        &self,
        older_than: DateTime<Utc>,
    ) -> verseguy_storage_infra::prelude::AppResult<Vec<AuditEvent>> {
        let found = self.repo.find(|e| e.timestamp < older_than)?;
        Ok(found)
    }

    /// Delete all events for a given principal (GDPR support). Returns number of deleted events.
    pub fn delete_by_principal(
        &self,
        principal_id: &str,
    ) -> verseguy_storage_infra::prelude::AppResult<usize> {
        let to_delete = self.repo.find(|e| e.principal_id == principal_id)?;
        for e in &to_delete {
            self.repo.delete(&e.id)?;
        }
        Ok(to_delete.len())
    }

    /// Run retention by days (helper): deletes events older than `days` days and returns deleted count.
    pub fn run_retention_days(
        &self,
        days: i64,
    ) -> verseguy_storage_infra::prelude::AppResult<usize> {
        let cutoff = Utc::now() - chrono::Duration::days(days);
        self.purge_older_than(cutoff)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use verseguy_storage_infra::config::StorageConfig;
    use verseguy_storage_infra::engine::StorageEngine;

    #[test]
    fn record_and_list_recent() -> verseguy_storage_infra::prelude::AppResult<()> {
        let td = TempDir::new()?;
        let cfg = StorageConfig {
            path: td.path().join("audit_db"),
            encryption_enabled: false,
            ..Default::default()
        };
        let engine = std::sync::Arc::new(StorageEngine::open(cfg)?);
        let store = AuditStore::new(engine.clone());

        let mut e = AuditEvent {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            principal_id: "user-1".to_string(),
            action: "create:org".to_string(),
            resource: "org:123".to_string(),
            metadata: {
                let mut m = serde_json::Map::new();
                m.insert(
                    "foo".to_string(),
                    serde_json::Value::String("bar".to_string()),
                );
                serde_json::Value::Object(m)
            },
            version: 0,
        };

        store.record(&mut e)?;

        let recent = store.list_recent(10)?;
        assert_eq!(recent.len(), 1);
        assert_eq!(recent[0].id, e.id);

        Ok(())
    }

    #[test]
    fn purge_older_and_gdpr_delete() -> verseguy_storage_infra::prelude::AppResult<()> {
        let td = TempDir::new()?;
        let cfg = StorageConfig {
            path: td.path().join("audit_db_retention"),
            encryption_enabled: false,
            ..Default::default()
        };
        let engine = std::sync::Arc::new(StorageEngine::open(cfg)?);
        let store = AuditStore::new(engine.clone());

        // Old event (10 days ago)
        let mut old = AuditEvent {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: Utc::now() - chrono::Duration::days(10),
            principal_id: "user-old".to_string(),
            action: "old_action".to_string(),
            resource: "res:1".to_string(),
            metadata: serde_json::Value::Object(serde_json::Map::new()),
            version: 0,
        };
        store.record(&mut old)?;

        // Recent event
        let mut recent = AuditEvent {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            principal_id: "user-new".to_string(),
            action: "new_action".to_string(),
            resource: "res:2".to_string(),
            metadata: serde_json::Value::Object(serde_json::Map::new()),
            version: 0,
        };
        store.record(&mut recent)?;

        // Purge older than 7 days -> should remove the old event
        let deleted = store.purge_older_than(Utc::now() - chrono::Duration::days(7))?;
        assert_eq!(deleted, 1);
        let all = store.list_recent(10)?;
        assert_eq!(all.len(), 1);
        assert_eq!(all[0].id, recent.id);

        // GDPR delete by principal
        let mut e1 = AuditEvent {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            principal_id: "gdpr-user".to_string(),
            action: "act1".to_string(),
            resource: "r1".to_string(),
            metadata: serde_json::Value::Object(serde_json::Map::new()),
            version: 0,
        };
        let mut e2 = e1.clone();
        e2.id = uuid::Uuid::new_v4().to_string();
        store.record(&mut e1)?;
        store.record(&mut e2)?;

        let deleted_gdpr = store.delete_by_principal("gdpr-user")?;
        assert_eq!(deleted_gdpr, 2);

        Ok(())
    }
}
