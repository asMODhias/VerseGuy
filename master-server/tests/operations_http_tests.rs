#![allow(clippy::disallowed_methods)]
use axum::body::{self, Body};
use axum::http::{Request, StatusCode};
use master_server::build_app;
use master_server::state::AppState;
use std::sync::Arc;
use tempfile::tempdir;
use tower::util::ServiceExt;
use verseguy_test_utils::{must, must_opt};

#[tokio::test]
async fn create_and_manage_operation() {
    let dir = must(tempdir());
    let state = Arc::new(must(AppState::new(dir.path(), vec![0u8; 32])));
    let app = build_app(state.clone());

    // Create operation
    let create_body = "{\"name\":\"Op-E2E\",\"description\":\"Test operation\"}".to_string();
    let req = must(
        Request::builder()
            .method("POST")
            .uri("/v1/operations")
            .header("content-type", "application/json")
            .body(Body::from(create_body)),
    );
    let resp = must(app.clone().oneshot(req).await);
    assert_eq!(resp.status(), StatusCode::OK);
    let bytes = must(body::to_bytes(resp.into_body(), 1024 * 1024).await);
    let created: serde_json::Value = must(serde_json::from_slice(&bytes));
    let id = must_opt(created.get("id").and_then(|v| v.as_str()), "missing id");

    // Add participant
    let add_body = "{\"user_id\":\"u-1\",\"role\":\"engineer\"}".to_string();
    let uri = format!("/v1/operations/{}/participants", id);
    let req2 = must(
        Request::builder()
            .method("POST")
            .uri(uri)
            .header("content-type", "application/json")
            .body(Body::from(add_body)),
    );
    let resp2 = must(app.clone().oneshot(req2).await);
    assert_eq!(resp2.status(), StatusCode::OK);

    // Update status
    let status_body = "{\"status\":\"Running\"}".to_string();
    let uri2 = format!("/v1/operations/{}/status", id);
    let req3 = must(
        Request::builder()
            .method("POST")
            .uri(uri2)
            .header("content-type", "application/json")
            .body(Body::from(status_body)),
    );
    let resp3 = must(app.clone().oneshot(req3).await);
    assert_eq!(resp3.status(), StatusCode::OK);

    // Fetch operation and verify
    let uri3 = format!("/v1/operations/{}", id);
    let req4 = must(
        Request::builder()
            .method("GET")
            .uri(uri3)
            .body(Body::empty()),
    );
    let resp4 = must(app.clone().oneshot(req4).await);
    assert_eq!(resp4.status(), StatusCode::OK);
    let bytes4 = must(body::to_bytes(resp4.into_body(), 1024 * 1024).await);
    let fetched: serde_json::Value = must(serde_json::from_slice(&bytes4));

    let parts = must_opt(
        fetched.get("participants").and_then(|v| v.as_array()),
        "missing parts",
    );
    assert_eq!(parts.len(), 1);
    assert_eq!(
        fetched.get("status").and_then(|v| v.as_str()).unwrap_or(""),
        "Running"
    );
}
