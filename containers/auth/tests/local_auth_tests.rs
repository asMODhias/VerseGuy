#![allow(clippy::disallowed_methods)]
use tempfile::tempdir;
use verseguy_auth::local::LocalAuth;
use verseguy_storage::RocksDBStorage;

#[tokio::test]
async fn register_and_login() {
    let dir = verseguy_test_utils::must(tempdir());
    let storage = verseguy_test_utils::must(RocksDBStorage::open(dir.path()));
    let auth = LocalAuth::new(storage);

    let user = match auth
        .register("alice".to_string(), "password123".to_string())
        .await
    {
        Ok(u) => u,
        Err(e) => panic!("register failed: {}", e),
    };
    assert_eq!(user.username, "alice");

    let logged = match auth.login("alice", "password123").await {
        Ok(l) => l,
        Err(e) => panic!("login failed: {}", e),
    };
    assert_eq!(logged.id, user.id);
}
