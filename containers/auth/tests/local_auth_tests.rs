use tempfile::tempdir;
use verseguy_auth::local::LocalAuth;
use verseguy_storage::RocksDBStorage;

#[tokio::test]
async fn register_and_login() {
    let dir = tempdir().unwrap();
    let storage = RocksDBStorage::open(dir.path()).unwrap();
    let auth = LocalAuth::new(storage);

    let user = auth
        .register("alice".to_string(), "password123".to_string())
        .await
        .unwrap();
    assert_eq!(user.username, "alice");

    let logged = auth.login("alice", "password123").await.unwrap();
    assert_eq!(logged.id, user.id);
}
