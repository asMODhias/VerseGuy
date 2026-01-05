use plugins_adapter_scunpacked::{ScShip, import_from_extracted_dir};
use std::fs;
use tempfile::TempDir;
use verseguy_storage::RocksDBStorage;

#[test]
fn import_from_extracted_dir_parses_xml() {
    let tmp = TempDir::new().unwrap();
    let dir = tmp.path();
    // write sample xml files
    fs::write(
        dir.join("ship1.xml"),
        r#"<Ship><ClassName>origin-100i</ClassName><DisplayName>Origin 100i</DisplayName></Ship>"#,
    )
    .unwrap();
    fs::write(dir.join("ship2.xml"), r#"<Ship><classname>constellation</classname><name>Aegis Constellation</name><role>explorer</role></Ship>"# ).unwrap();

    let storage = RocksDBStorage::open(dir.join("db")).unwrap();

    let n = import_from_extracted_dir(dir, &storage).unwrap();
    assert_eq!(n, 2);

    let s: Option<ScShip> = storage.get(b"scunpacked:ship:origin-100i").unwrap();
    assert!(s.is_some());
    assert_eq!(s.unwrap().name, "Origin 100i");
}
