use std::sync::Arc;
use axum::body::Body;
use axum::http::{Request, Method};
use master_server::state::AppState;
use master_server::build_app;
use tower::util::ServiceExt;
use serde_json::json;
use base64::Engine;

#[tokio::test]
async fn rotate_and_import_key() {
    let dir = tempfile::tempdir().unwrap();
    let key_path = dir.path().join("master.key");
    let key_path_str = key_path.to_str().unwrap().to_string();

    // Set env vars
    std::env::set_var("MASTER_KEY_FILE", &key_path_str);
    std::env::set_var("MASTER_ADMIN_TOKEN", "testtoken");

    let db_path = dir.path().join("db").to_str().unwrap().to_string();
    let state = Arc::new(AppState::new(db_path, b"secret".to_vec()).unwrap());
    let app = build_app(state.clone());

    // Rotate key
    let req: Request<axum::body::Body> = Request::builder()
        .method(Method::POST)
        .uri("/admin/keys/rotate")
        .header("x-admin-token", "testtoken")
        .body(Body::empty())
        .unwrap();

    let resp = app.clone().oneshot(req).await.unwrap();
    let status = resp.status();
    eprintln!("rotate status: {}", status);
    let bytes = axum::body::to_bytes(resp.into_body(), 1024*1024).await.unwrap();
    eprintln!("rotate body: {}", std::str::from_utf8(&bytes).unwrap());
    assert!(status.is_success());
    let v: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    let new_pk = v.get("public_key_b64").and_then(|p| p.as_str()).unwrap();
    assert!(!new_pk.is_empty());

    // Import a key (re-import same keypair to ensure import succeeds)
    // Read raw keypair bytes from file and base64 encode
    let key_raw = std::fs::read(&key_path).unwrap();
    let key_b64 = base64::engine::general_purpose::STANDARD.encode(&key_raw);
    let body = json!({"key_b64": key_b64});
    let req: Request<axum::body::Body> = Request::builder()
        .method(Method::POST)
        .uri("/admin/keys/import")
        .header("content-type", "application/json")
        .header("x-admin-token", "testtoken")
        .body(Body::from(body.to_string()))
        .unwrap();

    let resp = app.oneshot(req).await.unwrap();
    let status = resp.status();
    eprintln!("import status: {}", status);
    let bytes = axum::body::to_bytes(resp.into_body(), 1024*1024).await.unwrap();
    eprintln!("import body: {}", std::str::from_utf8(&bytes).unwrap());
    assert!(status.is_success());
    let v: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    let imported_pk = v.get("public_key_b64").and_then(|p| p.as_str()).unwrap();
    assert_eq!(imported_pk, new_pk);
}
