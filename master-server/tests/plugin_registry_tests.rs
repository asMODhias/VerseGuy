use std::sync::Arc;
use axum::body::Body;
use axum::http::{Request, Method, Response};
use serde_json::json;
use master_server::state::AppState;
use master_server::build_app;
use tower::util::ServiceExt;

#[tokio::test]
async fn publish_and_search_plugin() {
    let dir = tempfile::tempdir().unwrap();
    let db_path = dir.path().to_str().unwrap().to_string();
    let state = Arc::new(AppState::new(db_path, b"secret".to_vec()).unwrap());

    let app = build_app(state.clone());

    // Publish
    let manifest = json!({
        "manifest": {
            "id": "org.example.plugin",
            "name": "Example Plugin",
            "version": "1.0.0",
            "author": "Dev",
            "description": "Example",
            "published_at": null
        }
    });

    let req: Request<axum::body::Body> = Request::builder()
        .method(Method::POST)
        .uri("/plugins/publish")
        .header("content-type", "application/json")
        .body(axum::body::Body::from(manifest.to_string()))
        .unwrap();

    let resp: Response<axum::body::Body> = app.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status().as_u16(), 201);

    // Search
    let req: Request<axum::body::Body> = Request::builder()
        .method(Method::GET)
        .uri("/plugins/search?q=Example")
        .body(axum::body::Body::empty())
        .unwrap();

    let resp: Response<axum::body::Body> = app.oneshot(req).await.unwrap();
    assert!(resp.status().is_success());
    // axum::body::to_bytes requires a limit argument; use a generous limit (1MB)
    let body_bytes = axum::body::to_bytes(resp.into_body(), 1024 * 1024).await.unwrap();
    let v: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
    let results = v.get("results").and_then(|r| r.as_array()).unwrap();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].get("id").unwrap().as_str().unwrap(), "org.example.plugin");
}
