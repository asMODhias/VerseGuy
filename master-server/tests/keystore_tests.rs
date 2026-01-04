use master_server::keystore;
use tempfile::tempdir;

#[test]
fn key_persistence_roundtrip() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("master.key");
    // Ensure load_or_generate creates the file
    let kp1 = keystore::load_or_generate(&path).unwrap();
    assert!(path.exists());

    // Ensure load_keypair reads back the same public key
    let kp2 = keystore::load_keypair(&path).unwrap();
    assert_eq!(kp1.public, kp2.public);
}
