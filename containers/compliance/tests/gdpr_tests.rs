use tempfile::tempdir;
use verseguy_auth::{AuthMethod, License, User};
use verseguy_compliance::{delete_user_data, export_user_data};
use verseguy_storage::RocksDBStorage;

#[test]
fn export_and_delete_user_data() {
    let dir = tempdir().unwrap();
    let db_path = dir.path().to_str().unwrap().to_string();
    let storage = RocksDBStorage::open(&db_path).unwrap();

    // Insert a user (updated struct fields)
    let user = User {
        id: "u1".to_string(),
        username: "tester".to_string(),
        email: None,
        password_hash: Some("h".to_string()),
        auth_method: AuthMethod::Local { username: "tester".to_string() },
        license: License::Free,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };
    storage
        .put(format!("user:id:{}", user.id).as_bytes(), &user)
        .unwrap();
    storage
        .put(format!("user:username:{}", user.username).as_bytes(), &user)
        .unwrap();

    // Insert a session (updated struct)
    let rec = verseguy_auth::Session {
        id: "s1".to_string(),
        user_id: user.id.clone(),
        license: License::Free,
        created_at: chrono::Utc::now(),
        expires_at: chrono::Utc::now() + chrono::Duration::hours(1),
    };
    storage
        .put(format!("session:{}", rec.id).as_bytes(), &rec)
        .unwrap();

    // Export
    let out = export_user_data(&storage, &user.id).unwrap();
    assert!(out.contains("tester"));

    // Delete
    let ok = delete_user_data(&storage, &user.id).unwrap();
    assert!(ok);

    // Ensure deleted
    let u_opt: Option<User> = storage
        .get(format!("user:id:{}", user.id).as_bytes())
        .unwrap();
    assert!(u_opt.is_none());
}
