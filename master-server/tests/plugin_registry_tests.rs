#![allow(clippy::disallowed_methods)]
use axum::http::{Method, Request, Response, StatusCode};
use master_server::build_app;
use master_server::state::AppState;
use serde_json::Value;
use std::sync::Arc;
use tower::util::ServiceExt;
use verseguy_test_utils::{must, must_opt};

#[test]
fn publish_and_search_plugin() {
    let rt = match tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
    {
        Ok(rt) => rt,
        Err(e) => panic!("failed to build runtime: {}", e),
    };

    rt.block_on(async {
        let dir = must(tempfile::tempdir());
    let db_path = must_opt(dir.path().to_str(), "tempdir path not utf8").to_string();
    let state = Arc::new(must(AppState::new(db_path, b"secret".to_vec())));

    let app = build_app(state.clone());

    // Publish
    let manifest: Value = must(serde_json::from_str(
        r#"{ "manifest": { "id": "org.example.plugin", "name": "Example Plugin", "version": "1.0.0", "author": "Dev", "description": "Example", "published_at": null } }"#,
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

    // Search
    let req: Request<axum::body::Body> = must(
        Request::builder()
            .method(Method::GET)
            .uri("/plugins/search?q=Example")
            .body(axum::body::Body::empty()),
    );

    let resp: Response<axum::body::Body> = must(app.oneshot(req).await);
    assert!(resp.status().is_success());

    // axum::body::to_bytes requires a limit argument; use a generous limit (1MB)
    let body_bytes = must(axum::body::to_bytes(resp.into_body(), 1024 * 1024).await);
    let v: serde_json::Value = must(serde_json::from_slice(&body_bytes));
    let results = must_opt(
        v.get("results").and_then(|r| r.as_array()),
        "no results array",
    );
    assert_eq!(results.len(), 1);
    assert_eq!(
        must_opt(
            results[0].get("id").and_then(|id| id.as_str()),
            "missing id"
        ),
        "org.example.plugin"
    );
    });
}
