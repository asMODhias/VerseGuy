#![allow(clippy::disallowed_methods)]
use axum::body::{self, Body};
use axum::http::{Request, StatusCode};
use base64::engine::general_purpose;
use base64::Engine;
use master_server::build_app;
use master_server::plugins::PluginManifest;
use master_server::state::AppState;
use std::sync::Arc;
use tempfile::tempdir;
use tower::util::ServiceExt;
use verseguy_test_utils::{must, must_opt};

#[tokio::test]
async fn verify_and_revoke_flow() {
    let dir = must(tempdir());
    let state = Arc::new(must(AppState::new(dir.path(), vec![0u8; 32])));
    let app = build_app(state.clone());

    let manifest = PluginManifest {
        id: "org.test.signhttp".to_string(),
        name: "SignHTTPTest".to_string(),
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

    let pub_b64 = general_purpose::STANDARD.encode(
        must_opt(state.keypair.as_ref(), "missing keypair")
            .public
            .to_bytes(),
    );

    // POST /verify/plugin
    // Use published manifest for verification (store_manifest signed the published manifest)
    let manifest_ser = must(serde_json::to_string(&manifest.with_published()));
    let body = format!(
        r#"{{"manifest":{},"public_key_b64":"{}"}}"#,
        manifest_ser, pub_b64
    );
    let req = must(
        Request::builder()
            .method("POST")
            .uri("/verify/plugin")
            .header("content-type", "application/json")
            .body(Body::from(body)),
    );
    let resp = must(app.clone().oneshot(req).await);
    assert_eq!(resp.status(), StatusCode::OK);
    let bytes = must(body::to_bytes(resp.into_body(), 1024 * 1024).await);
    let v: serde_json::Value = must(serde_json::from_slice(&bytes));
    assert!(must_opt(
        v.get("valid").and_then(|b| b.as_bool()),
        "missing valid"
    ));

    // Revoke via admin endpoint
    std::env::set_var("MASTER_ADMIN_TOKEN", "admintoken");
    let revoke_body =
        r#"{"id":"org.test.signhttp","version":"0.1.0","reason":"compromised"}"#.to_string();
    let req2 = must(
        Request::builder()
            .method("POST")
            .uri("/verify/revoke")
            .header("content-type", "application/json")
            .header("x-admin-token", "admintoken")
            .body(Body::from(revoke_body)),
    );
    let resp2 = must(app.clone().oneshot(req2).await);
    assert_eq!(resp2.status(), StatusCode::OK);

    // GET /verify/revocations
    let req3 = must(
        Request::builder()
            .method("GET")
            .uri("/verify/revocations")
            .body(Body::empty()),
    );
    let resp3 = must(app.oneshot(req3).await);
    assert_eq!(resp3.status(), StatusCode::OK);
    let bytes3 = must(body::to_bytes(resp3.into_body(), 1024 * 1024).await);
    let v3: serde_json::Value = must(serde_json::from_slice(&bytes3));
    assert!(!must_opt(
        v3.get("revocations").and_then(|r| r.as_array()),
        "missing revocations"
    )
    .is_empty());
}
