use tempfile::tempdir;
use verseguy_auth::local::LocalAuth;
use verseguy_auth::session::SessionService;
use verseguy_auth::License;
use verseguy_storage::RocksDBStorage;

#[tokio::test]
async fn register_login_create_and_validate_session() {
    let dir = tempdir().unwrap();
    let storage = RocksDBStorage::open(dir.path()).unwrap();

    let auth = LocalAuth::new(storage.clone());

    // Register
    let user = auth
        .register("e2euser".to_string(), "strongpassword".to_string())
        .await
        .unwrap();
    assert_eq!(user.username, "e2euser");

    // Login
    let logged = auth.login("e2euser", "strongpassword").await.unwrap();
    assert_eq!(logged.id, user.id);

    // Create session and store it
    let session_service = SessionService::new(b"itest-secret".to_vec());
    let token = session_service
        .create_and_store_session(&user.id, &License::Free, 7, &storage)
        .unwrap();
    assert!(!token.is_empty());

    // Validate token and storage
    let data = session_service
        .validate_token_and_storage(&token, &storage)
        .unwrap();
    assert_eq!(data.claims.sub, user.id);

    // Ensure session record exists in storage
    let sid = data.claims.sid;
    let rec: Option<verseguy_auth::session::SessionRecord> =
        storage.get(format!("session:{}", sid).as_bytes()).unwrap();
    assert!(rec.is_some());
}
