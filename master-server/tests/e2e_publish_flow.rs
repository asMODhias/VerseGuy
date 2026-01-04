use axum::http::{Method, Request, Response};
use master_server::build_app;
use master_server::plugins::verify_manifest;
use master_server::state::AppState;
use serde_json::json;
use std::sync::Arc;
use tower::util::ServiceExt;

#[tokio::test]
async fn register_login_publish_verify() {
    let dir = tempfile::tempdir().unwrap();
    let db_path = dir.path().to_str().unwrap().to_string();
    // create appstate with keypair (MASTER_KEY_FILE not necessary; AppState::new generates keypair internally)
    let state = Arc::new(AppState::new(db_path, b"secret".to_vec()).unwrap());

    let app = build_app(state.clone());

    // Register
    let reg_body = json!({ "username": "tester", "password": "s3cretpass" });
    let req: Request<axum::body::Body> = Request::builder()
        .method(Method::POST)
        .uri("/auth/register")
        .header("content-type", "application/json")
        .body(axum::body::Body::from(reg_body.to_string()))
        .unwrap();
    let resp: Response<axum::body::Body> = app.clone().oneshot(req).await.unwrap();
    assert!(resp.status().is_success());

    // Login
    let login_body = json!({ "username": "tester", "password": "s3cretpass" });
    let req: Request<axum::body::Body> = Request::builder()
        .method(Method::POST)
        .uri("/auth/login")
        .header("content-type", "application/json")
        .body(axum::body::Body::from(login_body.to_string()))
        .unwrap();
    let resp: Response<axum::body::Body> = app.clone().oneshot(req).await.unwrap();
    assert!(resp.status().is_success());
    let body_bytes = axum::body::to_bytes(resp.into_body(), 1024 * 1024)
        .await
        .unwrap();
    let v: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
    let token = v.get("token").and_then(|t| t.as_str()).unwrap();
    assert!(!token.is_empty());

    // Publish plugin
    let manifest = json!({
        "manifest": {
            "id": "org.e2e.test",
            "name": "E2E Test Plugin",
            "version": "0.0.1",
            "author": "Tester",
            "description": "E2E",
            "published_at": null
        }
    });

    let req: Request<axum::body::Body> = Request::builder()
        .method(Method::POST)
        .uri("/plugins/publish")
        .header("content-type", "application/json")
        .body(axum::body::Body::from(manifest.to_string()))
        .unwrap();

    let resp: Response<axum::body::Body> = app.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status().as_u16(), 201);

    // Search and verify
    let req: Request<axum::body::Body> = Request::builder()
        .method(Method::GET)
        .uri("/plugins/search?q=E2E")
        .body(axum::body::Body::empty())
        .unwrap();
    let resp: Response<axum::body::Body> = app.oneshot(req).await.unwrap();
    let body_bytes = axum::body::to_bytes(resp.into_body(), 1024 * 1024)
        .await
        .unwrap();
    let v: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
    let results = v.get("results").and_then(|r| r.as_array()).unwrap();
    assert_eq!(results.len(), 1);
    let manifest_json = &results[0];
    let manifest: master_server::plugins::PluginManifest =
        serde_json::from_value(manifest_json.clone()).unwrap();

    // verify signature using server public key
    let pubkey = state.keypair.as_ref().unwrap().public;
    let ok = verify_manifest(&state.storage, &manifest, &pubkey).unwrap();
    assert!(ok);
}
