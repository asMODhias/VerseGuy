#![allow(clippy::disallowed_methods)]
use tempfile::tempdir;
use verseguy_auth::License;
use verseguy_auth::local::LocalAuth;
use verseguy_auth::session::SessionService;
use verseguy_storage::RocksDBStorage;
use verseguy_test_utils::must;

#[tokio::test]
async fn register_login_create_and_validate_session() {
    let dir = must(tempdir());
    let storage = must(RocksDBStorage::open(dir.path()));

    let auth = LocalAuth::new(storage.clone());

    // Register
    let user = match auth
        .register("e2euser".to_string(), "strongpassword".to_string())
        .await
    {
        Ok(u) => u,
        Err(e) => panic!("register failed: {}", e),
    };
    assert_eq!(user.username, "e2euser");

    // Login
    let logged = match auth.login("e2euser", "strongpassword").await {
        Ok(l) => l,
        Err(e) => panic!("login failed: {}", e),
    };
    assert_eq!(logged.id, user.id);

    // Create session and store it
    let session_service = SessionService::new(b"itest-secret".to_vec());
    let token =
        match session_service.create_and_store_session(&user.id, &License::Free, 7, &storage) {
            Ok(t) => t,
            Err(e) => panic!("create_and_store_session failed: {}", e),
        };
    assert!(!token.is_empty());

    // Validate token and storage
    let data = match session_service.validate_token_and_storage(&token, &storage) {
        Ok(d) => d,
        Err(e) => panic!("validate_token_and_storage failed: {}", e),
    };
    assert_eq!(data.claims.sub, user.id);

    // Ensure session record exists in storage
    let sid = data.claims.sid;
    let rec: Option<verseguy_auth::session::SessionRecord> =
        match storage.get(format!("session:{}", sid).as_bytes()) {
            Ok(r) => r,
            Err(e) => panic!("storage.get failed: {}", e),
        };
    let _rec = verseguy_test_utils::must_opt(rec, "session not found");
}
