use master_server::keystore;
use tempfile::tempdir;
use verseguy_test_utils::must;

#[test]
fn key_persistence_roundtrip() {
    let dir = must(tempdir());
    let path = dir.path().join("master.key");
    // Ensure load_or_generate creates the file
    let kp1 = must(keystore::load_or_generate(&path));
    assert!(path.exists());

    // Ensure load_keypair reads back the same public key
    let kp2 = must(keystore::load_keypair(&path));
    assert_eq!(kp1.public, kp2.public);
}
