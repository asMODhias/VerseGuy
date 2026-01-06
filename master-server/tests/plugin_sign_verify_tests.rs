use master_server::plugins::{verify_manifest, PluginManifest};
use master_server::state::AppState;
use tempfile::tempdir;
use verseguy_test_utils::{must, must_opt};

#[test]
fn sign_and_verify() {
    let dir = must(tempdir());
    let db_path = must_opt(dir.path().to_str(), "tempdir path not utf8").to_string();
    let state = must(AppState::new(db_path, b"secret".to_vec()));

    let manifest = PluginManifest {
        id: "org.test.sign".to_string(),
        name: "SignTest".to_string(),
        version: "0.1.0".to_string(),
        author: Some("Dev".to_string()),
        description: Some("Test".to_string()),
        published_at: None,
    };

    // store and sign
    must(master_server::plugins::store_manifest(
        &state.storage,
        &manifest.with_published(),
        state.keypair.as_ref(),
    ));

    // verify
    let pubkey = must_opt(state.keypair.as_ref(), "missing keypair").public;
    let ok = must(verify_manifest(&state.storage, &manifest.with_published(), &pubkey));
    assert!(ok);

    // revoke and ensure is_revoked returns true
    must(master_server::plugins::revoke_manifest(
        &state.storage,
        &manifest.id,
        &manifest.version,
        "test revoke",
    ));
    let revoked = must(master_server::plugins::is_revoked(
        &state.storage,
        &manifest.id,
        &manifest.version,
    ));
    assert!(revoked);
}
