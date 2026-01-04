use master_server::plugins::{verify_manifest, PluginManifest};
use master_server::state::AppState;
use tempfile::tempdir;

#[test]
fn sign_and_verify() {
    let dir = tempdir().unwrap();
    let db_path = dir.path().to_str().unwrap().to_string();
    let state = AppState::new(db_path, b"secret".to_vec()).unwrap();

    let manifest = PluginManifest {
        id: "org.test.sign".to_string(),
        name: "SignTest".to_string(),
        version: "0.1.0".to_string(),
        author: Some("Dev".to_string()),
        description: Some("Test".to_string()),
        published_at: None,
    };

    // store and sign
    master_server::plugins::store_manifest(
        &state.storage,
        &manifest.with_published(),
        state.keypair.as_ref(),
    )
    .unwrap();

    // verify
    let pubkey = state.keypair.as_ref().unwrap().public;
    let ok = verify_manifest(&state.storage, &manifest.with_published(), &pubkey).unwrap();
    assert!(ok);

    // revoke and ensure is_revoked returns true
    master_server::plugins::revoke_manifest(
        &state.storage,
        &manifest.id,
        &manifest.version,
        "test revoke",
    )
    .unwrap();
    let revoked =
        master_server::plugins::is_revoked(&state.storage, &manifest.id, &manifest.version)
            .unwrap();
    assert!(revoked);
}
