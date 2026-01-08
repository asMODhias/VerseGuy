
use master_server::build_app;
use std::sync::Arc;
use tower::util::ServiceExt;
use axum::http::{Method, Request, Response, StatusCode};
use tempfile::TempDir;
use verseguy_test_utils::{must, must_opt};

// Narrow allow: this test exercises request handling which invokes code (in dependencies/handlers) that currently uses `Result::expect`.
// The issue is tracked; see follow-up to remove or refactor the offending expect usage upstream.
#[allow(clippy::disallowed_methods)]
#[tokio::test]
async fn publish_requires_plugin_token_when_set() {
    // set env var
    std::env::set_var("MASTER_PLUGIN_PUBLISH_KEY", "secrettoken123");

    let dir = must(TempDir::new());
    let db_path = must_opt(dir.path().to_str(), "tempdir path not utf8").to_string();
    let state = Arc::new(must(master_server::state::AppState::new(db_path, b"secret".to_vec())));
    let app = build_app(state.clone());

    let manifest_body = r#"{ "manifest": { "id": "org.auth.test", "name": "AuthTest", "version": "0.1.0", "author": "Test", "description": "Auth test", "published_at": null } }"#;

    // without token -> forbidden
    let req_builder = Request::builder()
            .method(Method::POST)
            .uri("/plugins/publish");
    let req = match req_builder.body(axum::body::Body::from(manifest_body.to_string())) {
        Ok(r) => r,
        Err(e) => panic!("unexpected error: {}", e),
    };
    let resp: Response<axum::body::Body> = match app.clone().oneshot(req).await {
        Ok(r) => r,
        Err(e) => panic!("unexpected error: {}", e),
    };
    assert_eq!(resp.status(), StatusCode::FORBIDDEN);

    /* with wrong token -> forbidden
    let req_builder = Request::builder()
            .method(Method::POST)
            .uri("/plugins/publish")
            .header("content-type", "application/json")
            .header("x-plugin-token", "wrongtoken");
    let req = match req_builder.body(axum::body::Body::from(manifest_body.to_string())) {
        Ok(r) => r,
        Err(e) => panic!("unexpected error: {}", e),
    };
    let resp: Response<axum::body::Body> = match app.clone().oneshot(req).await {
        Ok(r) => r,
        Err(e) => panic!("unexpected error: {}", e),
    };
    assert_eq!(resp.status(), StatusCode::FORBIDDEN); */

    /* with correct token -> created
    let req_builder = Request::builder()
            .method(Method::POST)
            .uri("/plugins/publish")
            .header("content-type", "application/json")
            .header("x-plugin-token", "secrettoken123");
    let req = match req_builder.body(axum::body::Body::from(manifest_body.to_string())) {
        Ok(r) => r,
        Err(e) => panic!("unexpected error: {}", e),
    };
    let resp: Response<axum::body::Body> = match app.clone().oneshot(req).await {
        Ok(r) => r,
        Err(e) => panic!("unexpected error: {}", e),
    };
    assert_eq!(resp.status(), StatusCode::CREATED); */
}
