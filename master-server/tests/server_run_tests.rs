use master_server::build_app;
use master_server::state::AppState;
use std::sync::Arc;
use tempfile::tempdir;

#[tokio::test]
async fn starts_and_serves_plugins_search() {
    // create temp db path
    let dir = tempdir().unwrap();
    let db_path = dir.path().to_str().unwrap().to_string();
    let secret = b"integration-secret".to_vec();

    let state = Arc::new(AppState::new(db_path, secret).unwrap());
    let app = build_app(state.clone());

    // Call the router directly using ServiceExt::oneshot to avoid network bindings and hyper version conflicts
    use axum::http::Request;
    use tower::util::ServiceExt; // for .oneshot()

    let req: Request<axum::body::Body> = Request::builder()
        .method("GET")
        .uri("/plugins/search")
        .body(axum::body::Body::empty())
        .unwrap();

    let resp = app.oneshot(req).await.unwrap();
    let status = resp.status();
    assert!(status.is_success());
}
