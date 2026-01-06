use plugins_adapter_scunpacked::{ScShip, import_from_extracted_dir};
use std::fs;
use tempfile::TempDir;
use verseguy_test_utils::{must, must_opt};
use verseguy_storage::RocksDBStorage;

#[test]
fn import_from_extracted_dir_parses_xml() {
    let tmp = must(TempDir::new());
    let dir = tmp.path();
    // write sample xml files
    must(fs::write(
        dir.join("ship1.xml"),
        r#"<Ship><ClassName>origin-100i</ClassName><DisplayName>Origin 100i</DisplayName></Ship>"#,
    ));
    must(fs::write(dir.join("ship2.xml"), r#"<Ship><classname>constellation</classname><name>Aegis Constellation</name><role>explorer</role></Ship>"#));

    let storage = must(RocksDBStorage::open(dir.join("db")));

    let n = must(import_from_extracted_dir(dir, &storage));
    assert_eq!(n, 2);

    let s: Option<ScShip> = must(storage.get(b"scunpacked:ship:origin-100i"));
    let s = must_opt(s, "missing origin-100i ship");
    assert_eq!(s.name, "Origin 100i");
