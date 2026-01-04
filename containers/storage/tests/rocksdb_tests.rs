use serde::{Deserialize, Serialize};
use tempfile::tempdir;
use verseguy_storage::RocksDBStorage;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
struct TestItem {
    id: String,
    name: String,
}

#[test]
fn put_get_and_prefix_scan() {
    let dir = tempdir().unwrap();
    let db = RocksDBStorage::open(dir.path()).unwrap();

    let item = TestItem {
        id: "1".to_string(),
        name: "Alice".to_string(),
    };
    db.put(b"user:1", &item).unwrap();

    let got: Option<TestItem> = db.get(b"user:1").unwrap();
    assert_eq!(got.unwrap(), item);

    let items: Vec<TestItem> = db.prefix_scan(b"user:").unwrap();
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].id, "1");
}
