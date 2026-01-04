use axum::body::{self, Body};
use axum::http::{Request, StatusCode};
use master_server::build_app;
use master_server::plugins::PluginManifest;
use master_server::state::AppState;
use std::sync::Arc;
use tempfile::tempdir;
use tower::util::ServiceExt;

#[tokio::test]
async fn publish_requires_tos_when_user_header_present() {
    let dir = tempdir().unwrap();
    let state = Arc::new(AppState::new(dir.path(), vec![0u8; 32]).unwrap());
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
    let body = serde_json::json!({"manifest": manifest}).to_string();
    let req = Request::builder()
        .method("POST")
        .uri("/plugins/publish")
        .header("content-type", "application/json")
        .header("x-user-id", "user-123")
        .body(Body::from(body.clone()))
        .unwrap();
    let resp = app.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::FORBIDDEN);

    // Accept ToS for user
    let tos_body = serde_json::json!({"user_id":"user-123","accepted_at": 1234567890, "version": "1.0.0"}).to_string();
    let req2 = Request::builder()
        .method("POST")
        .uri("/auth/tos")
        .header("content-type", "application/json")
        .body(Body::from(tos_body))
        .unwrap();
    let resp2 = app.clone().oneshot(req2).await.unwrap();
    assert_eq!(resp2.status(), StatusCode::OK);

    // Now publish should succeed
    let req3 = Request::builder()
        .method("POST")
        .uri("/plugins/publish")
        .header("content-type", "application/json")
        .header("x-user-id", "user-123")
        .body(Body::from(body))
        .unwrap();
    let resp3 = app.clone().oneshot(req3).await.unwrap();
    assert_eq!(resp3.status(), StatusCode::CREATED);
}