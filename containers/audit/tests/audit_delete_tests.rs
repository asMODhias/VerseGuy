use verseguy_audit::AuditService;
use verseguy_storage::RocksDBStorage;
use anyhow::Result;
use tempfile::TempDir;

#[test]
fn log_delete_event_and_export() -> Result<()> {
    let dir = TempDir::new()?;
    let db_path = dir.path().join("db");
    let storage = RocksDBStorage::open(db_path)?;

    let svc = AuditService::new(std::sync::Arc::new(storage));

    let entry = svc.log_delete_event(Some("actor-1".to_string()), "action:deleted_user:user-1".to_string())?;
    assert_eq!(entry.user_id.as_deref(), Some("actor-1"));

    let exports = svc.export_for_user("actor-1")?;
    assert!(exports.iter().any(|e| e.event.contains("deleted_user")));

    Ok(())
}
