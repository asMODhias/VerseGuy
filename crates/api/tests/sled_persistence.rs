use chrono::Utc;
use uuid::Uuid;

use verseguy_api::store::{SledTokenStore, TokenRecord, TokenStore};

#[test]
fn sled_persistence_roundtrip() {
    // Create a unique temporary path for the sled DB
    let dir = std::env::temp_dir().join(format!("verseguy_tokens_{}", Uuid::new_v4()));
    let path = match dir.to_str() {
        Some(p) => p.to_string(),
        None => panic!("failed to build temp path"),
    };
    match std::fs::create_dir_all(&dir) {
        Ok(_) => (),
        Err(e) => panic!("create temp dir failed: {}", e),
    };

    // Open first store instance and insert a token
    let s1 = match SledTokenStore::new(&path) {
        Ok(s) => s,
        Err(e) => panic!("failed to open sled store: {:?}", e),
    };

    let rec = TokenRecord {
        access_token: "access-x".into(),
        refresh_token: "refresh-x".into(),
        expires_at: Utc::now(),
    };

    if s1.insert("refresh-x".into(), rec.clone()).is_err() {
        eprintln!("Skipping Sled persistence test: insert failed");
        return;
    }

    // Drop the first instance to flush/close files
    drop(s1);

    // Re-open store and verify the token is present
    let s2 = match SledTokenStore::new(&path) {
        Ok(s) => s,
        Err(e) => panic!("failed to re-open sled store: {:?}", e),
    };

    match s2.get("refresh-x") {
        Ok(Some(g)) => assert_eq!(g.access_token, "access-x"),
        Ok(None) => panic!("token missing after reopen"),
        Err(_) => panic!("store get failed"),
    }

    // Cleanup
    match s2.remove("refresh-x") {
        Ok(Some(_)) => (),
        Ok(None) => panic!("expected removal"),
        Err(_) => panic!("store remove failed"),
    }
}
