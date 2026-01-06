use std::fs;
use tempfile::tempdir;
use verseguy_test_utils::{must, must_opt};

#[test]
fn sign_and_verify_roundtrip() {
    let dir = must(tempdir());
    let manifest = dir.path().join("manifest.json");
    let sig = dir.path().join("manifest.sig");
    let key = dir.path().join("manifest.pub");
    must(fs::write(&manifest, r#"{"id":"org.test","name":"t","version":"1"}"#));

    must(master_server::manifest_tool::sign_manifest(
        must_opt(manifest.to_str(), "manifest path not utf8"),
        must_opt(sig.to_str(), "sig path not utf8"),
        must_opt(dir.path().join("kp.bin").to_str(), "kp path not utf8"),
        must_opt(key.to_str(), "key path not utf8"),
    ));
    let ok = must(master_server::manifest_tool::verify_manifest(
        must_opt(manifest.to_str(), "manifest path not utf8"),
        must_opt(sig.to_str(), "sig path not utf8"),
        must_opt(key.to_str(), "key path not utf8"),
    ));
    assert!(ok);
    // tamper
    must(fs::write(&manifest, r#"{"id":"org.test","name":"t2","version":"1"}"#));
    let ok2 = must(master_server::manifest_tool::verify_manifest(
        must_opt(manifest.to_str(), "manifest path not utf8"),
        must_opt(sig.to_str(), "sig path not utf8"),
        must_opt(key.to_str(), "key path not utf8"),
    ));
    assert!(!ok2);
}
