#![allow(clippy::disallowed_methods)]
use tempfile::TempDir;
use verseguy_auth::local::LocalAuth;
use verseguy_storage::RocksDBStorage;

#[tokio::test]
async fn test_register_duplicate_username() {
    let temp = verseguy_test_utils::must(TempDir::new());
    let storage = verseguy_test_utils::must(RocksDBStorage::open(temp.path()));
    let auth = LocalAuth::new(storage);

    verseguy_test_utils::must(
        auth.register("testuser".to_string(), "password123".to_string())
            .await,
    );

    let res = auth
        .register("testuser".to_string(), "anotherpass".to_string())
        .await;
    if res.is_err() {
    } else {
        panic!("expected Err for duplicate username");
    }
}

#[tokio::test]
async fn test_login_wrong_password() {
    let temp = verseguy_test_utils::must(TempDir::new());
    let storage = verseguy_test_utils::must(RocksDBStorage::open(temp.path()));
    let auth = LocalAuth::new(storage);

    verseguy_test_utils::must(
        auth.register("testuser".to_string(), "password123".to_string())
            .await,
    );

    let res = auth.login("testuser", "wrongpassword").await;
    if res.is_err() {
    } else {
        panic!("expected Err for wrong password");
    }
}

#[tokio::test]
async fn test_change_password_flow() {
    let temp = verseguy_test_utils::must(TempDir::new());
    let storage = verseguy_test_utils::must(RocksDBStorage::open(temp.path()));
    let auth = LocalAuth::new(storage);

    let user = verseguy_test_utils::must(
        auth.register("testuser".to_string(), "oldpassword".to_string())
            .await,
    );

    verseguy_test_utils::must(
        auth.change_password(&user.id, "oldpassword", "newpassword")
            .await,
    );

    // Old password should fail
    if auth.login("testuser", "oldpassword").await.is_err() {
    } else {
        panic!("expected old password to fail");
    }

    // New password works
    let logged = verseguy_test_utils::must(auth.login("testuser", "newpassword").await);
    assert_eq!(logged.id, user.id);
}
