use master_server::build_app;
use std::sync::Arc;
use tower::util::ServiceExt;
use axum::http::{Method, Request, Response, StatusCode};
use tempfile::TempDir;
use verseguy_test_utils::{must, must_opt};

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
    let req: Request<axum::body::Body> = must(
        Request::builder()
            .method(Method::POST)
            .uri("/plugins/publish")
            .header("content-type", "application/json")
            .body(axum::body::Body::from(manifest_body.to_string())),
    );
    let resp: Response<axum::body::Body> = must(app.clone().oneshot(req).await);
    assert_eq!(resp.status(), StatusCode::FORBIDDEN);

    // with wrong token -> forbidden
    let req: Request<axum::body::Body> = must(
        Request::builder()
            .method(Method::POST)
            .uri("/plugins/publish")
            .header("content-type", "application/json")
            .header("x-plugin-token", "wrongtoken")
            .body(axum::body::Body::from(manifest_body.to_string())),
    );
    let resp: Response<axum::body::Body> = must(app.clone().oneshot(req).await);
    assert_eq!(resp.status(), StatusCode::FORBIDDEN);

    // with correct token -> created
    let req: Request<axum::body::Body> = must(
        Request::builder()
            .method(Method::POST)
            .uri("/plugins/publish")
            .header("content-type", "application/json")
            .header("x-plugin-token", "secrettoken123")
            .body(axum::body::Body::from(manifest_body.to_string())),
    );
    let resp: Response<axum::body::Body> = must(app.clone().oneshot(req).await);
    assert_eq!(resp.status(), StatusCode::CREATED);
}
