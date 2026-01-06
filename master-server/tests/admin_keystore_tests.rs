#![allow(clippy::disallowed_methods)]
use axum::body::Body;
use axum::http::{Method, Request};
use base64::Engine;
use master_server::build_app;
use master_server::state::AppState;
use std::sync::Arc;
use tower::util::ServiceExt;
use verseguy_test_utils::{must, must_opt};

#[tokio::test]
async fn rotate_and_import_key() {
    let dir = must(tempfile::tempdir());
    let key_path = dir.path().join("master.key");
    let key_path_str = must_opt(key_path.to_str(), "key path not utf8").to_string();

    // Set env vars
    std::env::set_var("MASTER_KEY_FILE", &key_path_str);
    std::env::set_var("MASTER_ADMIN_TOKEN", "testtoken");

    let db_path = must_opt(dir.path().join("db").to_str(), "db path not utf8").to_string();
    let state = Arc::new(must(AppState::new(db_path, b"secret".to_vec())));
    let app = build_app(state.clone());

    // Rotate key
    let req: Request<axum::body::Body> = must(Request::builder()
        .method(Method::POST)
        .uri("/admin/keys/rotate")
        .header("x-admin-token", "testtoken")
        .body(Body::empty()));

    let resp = must(app.clone().oneshot(req).await);
    let status = resp.status();
    eprintln!("rotate status: {}", status);
    let bytes = must(axum::body::to_bytes(resp.into_body(), 1024 * 1024).await);
    eprintln!("rotate body: {}", must(std::str::from_utf8(&bytes)));
    assert!(status.is_success());
    let v: serde_json::Value = must(serde_json::from_slice(&bytes));
    let new_pk = must_opt(v.get("public_key_b64").and_then(|p| p.as_str()), "missing pubkey");
    assert!(!new_pk.is_empty());

    // Import a key (re-import same keypair to ensure import succeeds)
    // Read raw keypair bytes from file and base64 encode
    let key_raw = must(std::fs::read(&key_path));
    let key_b64 = base64::engine::general_purpose::STANDARD.encode(&key_raw);
    let body = format!(r#"{{"key_b64":"{}"}}"#, key_b64);
    let req: Request<axum::body::Body> = must(Request::builder()
        .method(Method::POST)
        .uri("/admin/keys/import")
        .header("content-type", "application/json")
        .header("x-admin-token", "testtoken")
        .body(Body::from(body)));

    let resp = must(app.oneshot(req).await);
    let status = resp.status();
    eprintln!("import status: {}", status);
    let bytes = must(axum::body::to_bytes(resp.into_body(), 1024 * 1024).await);
    eprintln!("import body: {}", must(std::str::from_utf8(&bytes)));
    assert!(status.is_success());
    let v: serde_json::Value = must(serde_json::from_slice(&bytes));
    let imported_pk = must_opt(v.get("public_key_b64").and_then(|p| p.as_str()), "missing imported pk");
    assert_eq!(imported_pk, new_pk);
}
