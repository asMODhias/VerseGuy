use verseguy_audit::AuditService;
use verseguy_storage::RocksDBStorage;
use tempfile::tempdir;
use std::sync::Arc;

#[test]
fn log_and_verify_audit_chain() {
    let dir = tempdir().unwrap();
    let db_path = dir.path().to_str().unwrap().to_string();
    let storage = RocksDBStorage::open(&db_path).unwrap();
    let storage = Arc::new(storage);
    let svc = AuditService::new(storage.clone());

    let e1 = svc.log_event(Some("user1".to_string()), "login".to_string()).unwrap();
    let e2 = svc.log_event(Some("user1".to_string()), "publish_plugin".to_string()).unwrap();

    // Debug: list all entries and computed expected hashes
    let exported_all = svc.export_for_user("user1").unwrap();
    for it in &exported_all {
        println!("entry: id={} ts={} user={:?} event={} hash={}", it.id, it.timestamp, it.user_id, it.event, it.hash);
    }

    assert!(svc.verify().unwrap(), "audit chain verification failed");

    let exported = svc.export_for_user("user1").unwrap();
    assert!(exported.len() >= 2);
}
