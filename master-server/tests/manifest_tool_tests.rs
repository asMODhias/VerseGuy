use std::fs;
use tempfile::tempdir;

#[test]
fn sign_and_verify_roundtrip() {
    let dir = tempdir().unwrap();
    let manifest = dir.path().join("manifest.json");
    let sig = dir.path().join("manifest.sig");
    let key = dir.path().join("manifest.pub");
    fs::write(&manifest, r#"{"id":"org.test","name":"t","version":"1"}"#).unwrap();

    master_server::manifest_tool::sign_manifest(
        manifest.to_str().unwrap(),
        sig.to_str().unwrap(),
        dir.path().join("kp.bin").to_str().unwrap(),
        key.to_str().unwrap(),
    )
    .unwrap();
    let ok = master_server::manifest_tool::verify_manifest(
        manifest.to_str().unwrap(),
        sig.to_str().unwrap(),
        key.to_str().unwrap(),
    )
    .unwrap();
    assert!(ok);
    // tamper
    fs::write(&manifest, r#"{"id":"org.test","name":"t2","version":"1"}"#).unwrap();
    let ok2 = master_server::manifest_tool::verify_manifest(
        manifest.to_str().unwrap(),
        sig.to_str().unwrap(),
        key.to_str().unwrap(),
    )
    .unwrap();
    assert!(!ok2);
}
