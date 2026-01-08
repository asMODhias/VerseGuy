use master_server::plugins::PluginManifest;
use std::sync::Arc;
use tempfile::TempDir;
use verseguy_test_utils::{must, must_opt};

#[test]
fn isolate_store_manifest() {
    let rt = match tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
    {
        Ok(rt) => rt,
        Err(e) => panic!("failed to build runtime: {}", e),
    };

    rt.block_on(async {
        // create app state with temp DB
        let dir = must(TempDir::new());
        let db_path = must_opt(dir.path().to_str(), "tempdir path not utf8").to_string();
        let state = Arc::new(must(master_server::state::AppState::new(
            db_path,
            b"secret".to_vec(),
        )));

        let manifest = PluginManifest {
            id: "org.isolate.test".to_string(),
            name: "Isolate Test".to_string(),
            version: "0.0.1".to_string(),
            author: Some("Tester".to_string()),
            description: Some("Isolate store_manifest test".to_string()),
            published_at: None,
        };

        // Call store_manifest synchronously
        let res = master_server::plugins::store_manifest(&state.storage, &manifest, None);
        assert!(res.is_ok());
    });
}
