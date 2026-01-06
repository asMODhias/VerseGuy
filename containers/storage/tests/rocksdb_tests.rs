use serde::{Deserialize, Serialize};
use tempfile::TempDir;
use verseguy_storage::RocksDBStorage;
use verseguy_test_utils::{must, must_opt};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
struct TestItem {
    id: String,
    name: String,
}

#[test]
fn test_open_database() {
    let temp_dir = must(TempDir::new());
    let storage = must(RocksDBStorage::open(temp_dir.path()));
    assert!(storage.path().is_some());
}

#[test]
fn test_put_and_get() {
    let temp_dir = must(TempDir::new());
    let db = must(RocksDBStorage::open(temp_dir.path()));

    let item = TestItem {
        id: "1".to_string(),
        name: "Alice".to_string(),
    };
    must(db.put(b"user:1", &item));

    let got: Option<TestItem> = must(db.get(b"user:1"));
    let got = must_opt(got, "missing item");
    assert_eq!(got, item);
}

#[test]
fn test_get_nonexistent() {
    let temp_dir = must(TempDir::new());
    let db = must(RocksDBStorage::open(temp_dir.path()));

    let got: Option<TestItem> = must(db.get(b"nonexistent"));
    assert!(got.is_none());
}

#[test]
fn test_delete_and_prefix() {
    let temp_dir = must(TempDir::new());
    let db = must(RocksDBStorage::open(temp_dir.path()));

    let a = TestItem { id: "1".into(), name: "A".into() };
    let b = TestItem { id: "2".into(), name: "B".into() };
    let c = TestItem { id: "3".into(), name: "C".into() };

    must(db.put(b"user:1", &a));
    must(db.put(b"user:2", &b));
    must(db.put(b"user:3", &c));
    must(db.put(b"post:1", &"Post 1"));

    must(db.delete(b"user:2"));

    let users: Vec<TestItem> = must(db.prefix_scan(b"user:"));
    assert_eq!(users.len(), 2);
}
