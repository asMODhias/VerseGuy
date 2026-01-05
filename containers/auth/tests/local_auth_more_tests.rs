use verseguy_auth::local::LocalAuth;
use tempfile::TempDir;
use verseguy_storage::RocksDBStorage;

#[tokio::test]
async fn test_register_duplicate_username() {
    let temp = TempDir::new().unwrap();
    let storage = RocksDBStorage::open(temp.path()).unwrap();
    let auth = LocalAuth::new(storage);

    auth.register("testuser".to_string(), "password123".to_string())
        .await
        .expect("first register failed");

    let res = auth
        .register("testuser".to_string(), "anotherpass".to_string())
        .await;
    assert!(res.is_err());
}

#[tokio::test]
async fn test_login_wrong_password() {
    let temp = TempDir::new().unwrap();
    let storage = RocksDBStorage::open(temp.path()).unwrap();
    let auth = LocalAuth::new(storage);

    auth.register("testuser".to_string(), "password123".to_string())
        .await
        .expect("register failed");

    let res = auth.login("testuser", "wrongpassword").await;
    assert!(res.is_err());
}

#[tokio::test]
async fn test_change_password_flow() {
    let temp = TempDir::new().unwrap();
    let storage = RocksDBStorage::open(temp.path()).unwrap();
    let auth = LocalAuth::new(storage);

    let user = auth
        .register("testuser".to_string(), "oldpassword".to_string())
        .await
        .expect("register failed");

    auth.change_password(&user.id, "oldpassword", "newpassword")
        .await
        .expect("change password failed");

    // Old password should fail
    assert!(auth.login("testuser", "oldpassword").await.is_err());

    // New password works
    let logged = auth.login("testuser", "newpassword").await.expect("login new pass");
    assert_eq!(logged.id, user.id);
}
