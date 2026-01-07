use chrono::Utc;
use tempfile::TempDir;
use verseguy_audit_infra::{AuditEvent, AuditStore};
use verseguy_storage_infra::config::StorageConfig;
use verseguy_storage_infra::engine::StorageEngine;

#[test]
fn retention_and_gdpr_integration() -> verseguy_storage_infra::prelude::AppResult<()> {
    let td = TempDir::new()?;
    let cfg = StorageConfig {
        path: td.path().join("audit_retention_int"),
        encryption_enabled: false,
        ..Default::default()
    };
    let engine = std::sync::Arc::new(StorageEngine::open(cfg)?);
    let store = AuditStore::new(engine.clone());

    let mut old = AuditEvent {
        id: uuid::Uuid::new_v4().to_string(),
        timestamp: Utc::now() - chrono::Duration::days(10),
        principal_id: "user-old".to_string(),
        action: "old_action".to_string(),
        resource: "res:1".to_string(),
        metadata: serde_json::json!({}),
        version: 0,
    };
    store.record(&mut old)?;

    let mut recent = AuditEvent {
        id: uuid::Uuid::new_v4().to_string(),
        timestamp: Utc::now(),
        principal_id: "user-new".to_string(),
        action: "new_action".to_string(),
        resource: "res:2".to_string(),
        metadata: serde_json::json!({}),
        version: 0,
    };
    store.record(&mut recent)?;

    let deleted = store.purge_older_than(Utc::now() - chrono::Duration::days(7))?;
    assert_eq!(deleted, 1);

    // GDPR delete
    let mut e1 = AuditEvent {
        id: uuid::Uuid::new_v4().to_string(),
        timestamp: Utc::now(),
        principal_id: "gdpr-user".to_string(),
        action: "act1".to_string(),
        resource: "r1".to_string(),
        metadata: serde_json::json!({}),
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
