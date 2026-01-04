use axum::body::{self, Body};
use axum::http::{Request, StatusCode};
use master_server::build_app;
use master_server::state::AppState;
use std::sync::Arc;
use tempfile::tempdir;
use tower::util::ServiceExt; // for .oneshot

#[tokio::test]
async fn accept_and_get_tos() {
    let dir = tempdir().unwrap();
    let state = Arc::new(AppState::new(dir.path(), vec![0u8; 32]).unwrap());
    let app = build_app(state.clone());

    // POST /auth/tos
    let body = r#"{"user_id":"u1","accepted_at": 1609459200, "version":"1.0"}"#;
    let req = Request::builder()
        .method("POST")
        .uri("/auth/tos")
        .header("content-type", "application/json")
        .body(Body::from(body))
        .unwrap();
    let resp = app.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    // GET /auth/tos/u1
    let req2 = Request::builder()
        .method("GET")
        .uri("/auth/tos/u1")
        .body(Body::empty())
        .unwrap();
    let resp2 = app.oneshot(req2).await.unwrap();
    assert_eq!(resp2.status(), StatusCode::OK);
    let bytes = body::to_bytes(resp2.into_body(), 1024 * 1024)
        .await
        .unwrap();
    let v: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(v.get("user_id").unwrap().as_str().unwrap(), "u1");
}
