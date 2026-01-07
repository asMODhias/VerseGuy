use tempfile::tempdir;
use verseguy_audit::AuditService;
use verseguy_storage::RocksDBStorage;

#[test]
fn log_delete_event_and_export() {
    let dir = tempdir().expect("tempdir");
    let db_path = dir.path().join("db");
    let storage = RocksDBStorage::open(db_path).expect("open storage");

    let svc = AuditService::new(std::sync::Arc::new(storage));

    let entry = svc
        .log_delete_event(Some("actor-1".to_string()), "action:deleted_user:user-1".to_string())
        .expect("log_delete_event");
    assert_eq!(entry.user_id.as_deref(), Some("actor-1"));

    let exports = svc.export_for_user("actor-1").expect("export");
    assert!(exports.iter().any(|e| e.event.contains("deleted_user")));
}