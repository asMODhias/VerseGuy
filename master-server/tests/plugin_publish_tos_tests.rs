#![allow(clippy::disallowed_methods)]
use axum::body::Body;
use axum::http::{Request, StatusCode};
use master_server::build_app;
use master_server::plugins::PluginManifest;
use master_server::state::AppState;
use std::sync::Arc;
use tempfile::tempdir;
use tower::util::ServiceExt;
use verseguy_test_utils::must;

#[tokio::test]
async fn publish_requires_tos_when_user_header_present() {
    let dir = must(tempdir());
    let state = Arc::new(must(AppState::new(dir.path(), vec![0u8; 32])));
    let app = build_app(state.clone());

    let manifest = PluginManifest {
        id: "org.test.publishtos".to_string(),
        name: "PublishToSTest".to_string(),
        version: "0.1.0".to_string(),
        author: Some("Dev".to_string()),
        description: Some("Test".to_string()),
        published_at: None,
    };

    // Attempt publish with x-user-id header but without ToS acceptance => Forbidden
    let manifest_ser = must(serde_json::to_string(&manifest));
    let body = format!(r#"{{"manifest":{}}}"#, manifest_ser);
    let req = must(Request::builder()
        .method("POST")
        .uri("/plugins/publish")
        .header("content-type", "application/json")
        .header("x-user-id", "user-123")
        .body(Body::from(body.clone())));
    let resp = must(app.clone().oneshot(req).await);
    assert_eq!(resp.status(), StatusCode::FORBIDDEN);

    // Accept ToS for user
    let tos_body = r#"{"user_id":"user-123","accepted_at": 1234567890, "version": "1.0.0"}"#.to_string();
    let req2 = must(Request::builder()
        .method("POST")
        .uri("/auth/tos")
        .header("content-type", "application/json")
        .body(Body::from(tos_body)));
    let resp2 = must(app.clone().oneshot(req2).await);
    assert_eq!(resp2.status(), StatusCode::OK);

    // Now publish should succeed
    let req3 = must(Request::builder()
        .method("POST")
        .uri("/plugins/publish")
        .header("content-type", "application/json")
        .header("x-user-id", "user-123")
        .body(Body::from(body)));
    let resp3 = must(app.clone().oneshot(req3).await);
    assert_eq!(resp3.status(), StatusCode::CREATED);
}
