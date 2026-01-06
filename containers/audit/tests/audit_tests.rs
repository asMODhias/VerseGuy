use std::sync::Arc;
use tempfile::tempdir;
use verseguy_audit::AuditService;
use verseguy_storage::RocksDBStorage;

#[test]
fn log_and_verify_audit_chain() {
    let dir = match tempdir() {
        Ok(d) => d,
        Err(e) => panic!("failed to create tempdir: {}", e),
    };
    let db_path = match dir.path().to_str() {
        Some(s) => s.to_string(),
        None => panic!("tempdir path not valid UTF-8"),
    };
    let storage = match RocksDBStorage::open(&db_path) {
        Ok(s) => s,
        Err(e) => panic!("failed to open RocksDBStorage: {}", e),
    };
    let storage = Arc::new(storage);
    let svc = AuditService::new(storage.clone());

    let _e1 = match svc.log_event(Some("user1".to_string()), "login".to_string()) {
        Ok(e) => e,
        Err(e) => panic!("log_event failed: {}", e),
    };
    let _e2 = match svc.log_event(Some("user1".to_string()), "publish_plugin".to_string()) {
        Ok(e) => e,
        Err(e) => panic!("log_event failed: {}", e),
    };

    // Debug: list all entries and computed expected hashes
    let exported_all = match svc.export_for_user("user1") {
        Ok(v) => v,
        Err(e) => panic!("export_for_user failed: {}", e),
    };
    for it in &exported_all {
        println!(
            "entry: id={} ts={} user={:?} event={} hash={}",
            it.id, it.timestamp, it.user_id, it.event, it.hash
        );
    }

    let verified = match svc.verify() {
        Ok(v) => v,
        Err(e) => panic!("verify failed: {}", e),
    };
    assert!(verified, "audit chain verification failed");

    let exported = match svc.export_for_user("user1") {
        Ok(v) => v,
        Err(e) => panic!("export_for_user failed: {}", e),
    };
    assert!(exported.len() >= 2);
}
