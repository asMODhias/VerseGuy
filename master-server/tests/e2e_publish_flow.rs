#![allow(clippy::disallowed_methods)]
use axum::http::{Method, Request, Response, StatusCode};
use master_server::build_app;
use master_server::plugins::verify_manifest;
use master_server::state::AppState;
use serde_json::Value;
use std::sync::Arc;
use tower::util::ServiceExt;
use verseguy_test_utils::{must, must_opt};

#[tokio::test]
async fn register_login_publish_verify() {
    let dir = must(tempfile::tempdir());
    let db_path = must_opt(dir.path().to_str(), "tempdir path not utf8").to_string();
    // create appstate with keypair (MASTER_KEY_FILE not necessary; AppState::new generates keypair internally)
    let state = Arc::new(must(AppState::new(db_path, b"secret".to_vec())));

    let app = build_app(state.clone());

    // Register
    let reg_body = "{\"username\":\"tester\",\"password\":\"s3cretpass\"}".to_string();
    let req: Request<axum::body::Body> = must(
        Request::builder()
            .method(Method::POST)
            .uri("/auth/register")
            .header("content-type", "application/json")
            .body(axum::body::Body::from(reg_body)),
    );
    let resp: Response<axum::body::Body> = must(app.clone().oneshot(req).await);
    assert!(resp.status().is_success());

    // Login
    let login_body = "{\"username\":\"tester\",\"password\":\"s3cretpass\"}".to_string();
    let req: Request<axum::body::Body> = must(
        Request::builder()
            .method(Method::POST)
            .uri("/auth/login")
            .header("content-type", "application/json")
            .body(axum::body::Body::from(login_body)),
    );
    let resp: Response<axum::body::Body> = must(app.clone().oneshot(req).await);
    assert!(resp.status().is_success());
    let body_bytes = must(axum::body::to_bytes(resp.into_body(), 1024 * 1024).await);
    let v: serde_json::Value = must(serde_json::from_slice(&body_bytes));
    let token = must_opt(v.get("token").and_then(|t| t.as_str()), "missing token");
    assert!(!token.is_empty());

    // Publish plugin
    let manifest: Value = must(serde_json::from_str(
        r#"{ "manifest": { "id": "org.e2e.test", "name": "E2E Test Plugin", "version": "0.0.1", "author": "Tester", "description": "E2E", "published_at": null } }"#,
    ));

    let req: Request<axum::body::Body> = must(
        Request::builder()
            .method(Method::POST)
            .uri("/plugins/publish")
            .header("content-type", "application/json")
            .body(axum::body::Body::from(manifest.to_string())),
    );

    let resp: Response<axum::body::Body> = must(app.clone().oneshot(req).await);
    assert_eq!(resp.status(), StatusCode::CREATED);

    // Search and verify
    let req: Request<axum::body::Body> = must(
        Request::builder()
            .method(Method::GET)
            .uri("/plugins/search?q=E2E")
            .body(axum::body::Body::empty()),
    );
    let resp: Response<axum::body::Body> = must(app.oneshot(req).await);
    let body_bytes = must(axum::body::to_bytes(resp.into_body(), 1024 * 1024).await);
    let v: serde_json::Value = must(serde_json::from_slice(&body_bytes));
    let results = must_opt(v.get("results").and_then(|r| r.as_array()), "no results");
    assert_eq!(results.len(), 1);
    let manifest_json = &results[0];
    let manifest: master_server::plugins::PluginManifest =
        must(serde_json::from_value(manifest_json.clone()));

    // verify signature using server public key
    let pubkey = must_opt(state.keypair.as_ref(), "missing keypair").public;
    let ok = must(verify_manifest(&state.storage, &manifest, &pubkey));
    assert!(ok);
}
