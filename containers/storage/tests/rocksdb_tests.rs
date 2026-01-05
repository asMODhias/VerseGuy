use serde::{Deserialize, Serialize};
use tempfile::TempDir;
use verseguy_storage::RocksDBStorage;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
struct TestItem {
    id: String,
    name: String,
}

#[test]
fn test_open_database() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let storage = RocksDBStorage::open(temp_dir.path()).expect("Failed to open database");
    assert!(storage.path().is_some());
}

#[test]
fn test_put_and_get() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let db = RocksDBStorage::open(temp_dir.path()).expect("Failed to open database");

    let item = TestItem {
        id: "1".to_string(),
        name: "Alice".to_string(),
    };
    db.put(b"user:1", &item).expect("Failed to put");

    let got: Option<TestItem> = db.get(b"user:1").expect("Failed to get");
    assert_eq!(got.unwrap(), item);
}

#[test]
fn test_get_nonexistent() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let db = RocksDBStorage::open(temp_dir.path()).expect("Failed to open database");

    let got: Option<TestItem> = db.get(b"nonexistent").expect("Failed to get");
    assert!(got.is_none());
}

#[test]
fn test_delete_and_prefix() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let db = RocksDBStorage::open(temp_dir.path()).expect("Failed to open database");

    let a = TestItem { id: "1".into(), name: "A".into() };
    let b = TestItem { id: "2".into(), name: "B".into() };
    let c = TestItem { id: "3".into(), name: "C".into() };

    db.put(b"user:1", &a).expect("put a");
    db.put(b"user:2", &b).expect("put b");
    db.put(b"user:3", &c).expect("put c");
    db.put(b"post:1", &"Post 1").expect("put post");

    db.delete(b"user:2").expect("delete user 2");

    let users: Vec<TestItem> = db.prefix_scan(b"user:").expect("scan users");
    assert_eq!(users.len(), 2);
}
