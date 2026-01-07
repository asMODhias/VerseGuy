use tempfile::tempdir;
use verseguy_auth::{AuthMethod, License, User};
use verseguy_compliance::{delete_user_data, export_user_data};
use verseguy_storage::RocksDBStorage;
use verseguy_test_utils::{must, must_opt};

#[test]
fn export_and_delete_user_data() {
    let dir = must(tempdir());
    let db_path = must_opt(dir.path().to_str(), "tempdir path not utf8").to_string();
    let storage = must(RocksDBStorage::open(&db_path));

    // Insert a user (updated struct fields)
    let user = User {
        id: "u1".to_string(),
        username: "tester".to_string(),
        email: None,
        password_hash: Some("h".to_string()),
        auth_method: AuthMethod::Local {
            username: "tester".to_string(),
            password_hash: "h".to_string(),
        },
        license: License::Free,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };
    must(storage.put(format!("user:id:{}", user.id).as_bytes(), &user));
    must(storage.put(format!("user:username:{}", user.username).as_bytes(), &user));

    // Insert a session (updated struct)
    let rec = verseguy_auth::Session {
        id: "s1".to_string(),
        user_id: user.id.clone(),
        license: License::Free,
        created_at: chrono::Utc::now(),
        expires_at: chrono::Utc::now() + chrono::Duration::hours(1),
    };
    must(storage.put(format!("session:{}", rec.id).as_bytes(), &rec));

    // Export
    let out = must(export_user_data(&storage, &user.id));
    assert!(out.contains("tester"));

    // Delete
    let ok = must(delete_user_data(&storage, &user.id));
    assert!(ok);

    // Ensure deleted
    let u_opt: Option<User> = must(storage.get(format!("user:id:{}", user.id).as_bytes()));
    assert!(u_opt.is_none());
}
