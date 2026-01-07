#![allow(clippy::disallowed_methods)]
use axum::body::{self, Body};
use axum::http::{Request, StatusCode};
use master_server::build_app;
use master_server::state::AppState;
use std::sync::Arc;
use tempfile::tempdir;
use tower::util::ServiceExt; // for .oneshot
use verseguy_test_utils::{must, must_opt};

#[tokio::test]
async fn accept_and_get_tos() {
    let dir = must(tempdir());
    let state = Arc::new(must(AppState::new(dir.path(), vec![0u8; 32])));
    let app = build_app(state.clone());

    // POST /auth/tos
    let body = r#"{"user_id":"u1","accepted_at": 1609459200, "version":"1.0"}"#;
    let req = must(
        Request::builder()
            .method("POST")
            .uri("/auth/tos")
            .header("content-type", "application/json")
            .body(Body::from(body)),
    );
    let resp = must(app.clone().oneshot(req).await);
    assert_eq!(resp.status(), StatusCode::OK);

    // GET /auth/tos/u1
    let req2 = must(
        Request::builder()
            .method("GET")
            .uri("/auth/tos/u1")
            .body(Body::empty()),
    );
    let resp2 = must(app.oneshot(req2).await);
    assert_eq!(resp2.status(), StatusCode::OK);
    let bytes = must(body::to_bytes(resp2.into_body(), 1024 * 1024).await);
    let v: serde_json::Value = must(serde_json::from_slice(&bytes));
    assert_eq!(
        must_opt(v.get("user_id").and_then(|n| n.as_str()), "missing user_id"),
        "u1"
    );
}
