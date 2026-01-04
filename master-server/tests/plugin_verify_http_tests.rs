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

#[tokio::test]
async fn verify_and_revoke_flow() {
    let dir = tempdir().unwrap();
    let state = Arc::new(AppState::new(dir.path(), vec![0u8; 32]).unwrap());
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
    master_server::plugins::store_manifest(
        &state.storage,
        &manifest.with_published(),
        state.keypair.as_ref(),
    )
    .unwrap();

    let pub_b64 = general_purpose::STANDARD.encode(state.keypair.as_ref().unwrap().public.to_bytes());

    // POST /verify/plugin
    // Use published manifest for verification (store_manifest signed the published manifest)
    let body = serde_json::json!({"manifest": manifest.with_published(), "public_key_b64": pub_b64}).to_string();
    let req = Request::builder()
        .method("POST")
        .uri("/verify/plugin")
        .header("content-type", "application/json")
        .body(Body::from(body))
        .unwrap();
    let resp = app.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let bytes = body::to_bytes(resp.into_body(), 1024 * 1024).await.unwrap();
    let v: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(v.get("valid").unwrap().as_bool().unwrap(), true);

    // Revoke via admin endpoint
    std::env::set_var("MASTER_ADMIN_TOKEN", "admintoken");
    let revoke_body = serde_json::json!({"id": "org.test.signhttp", "version": "0.1.0", "reason": "compromised"}).to_string();
    let req2 = Request::builder()
        .method("POST")
        .uri("/verify/revoke")
        .header("content-type", "application/json")
        .header("x-admin-token", "admintoken")
        .body(Body::from(revoke_body))
        .unwrap();
    let resp2 = app.clone().oneshot(req2).await.unwrap();
    assert_eq!(resp2.status(), StatusCode::OK);

    // GET /verify/revocations
    let req3 = Request::builder()
        .method("GET")
        .uri("/verify/revocations")
        .body(Body::empty())
        .unwrap();
    let resp3 = app.oneshot(req3).await.unwrap();
    assert_eq!(resp3.status(), StatusCode::OK);
    let bytes3 = body::to_bytes(resp3.into_body(), 1024 * 1024).await.unwrap();
    let v3: serde_json::Value = serde_json::from_slice(&bytes3).unwrap();
    assert!(v3.get("revocations").unwrap().as_array().unwrap().len() >= 1);
}
