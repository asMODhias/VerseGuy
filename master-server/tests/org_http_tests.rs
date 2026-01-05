use axum::body::{self, Body};
use axum::http::{Request, StatusCode};
use master_server::build_app;
use master_server::state::AppState;
use std::sync::Arc;
use tempfile::tempdir;
use tower::util::ServiceExt;

#[tokio::test]
async fn create_and_list_orgs() {
    let dir = tempdir().unwrap();
    let state = Arc::new(AppState::new(dir.path(), vec![0u8; 32]).unwrap());
    let app = build_app(state.clone());

    // Initially list should return empty array
    let req = Request::builder()
        .method("GET")
        .uri("/v1/orgs")
        .body(Body::empty())
        .unwrap();
    let resp = app.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let bytes = body::to_bytes(resp.into_body(), 1024 * 1024).await.unwrap();
    let v: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert!(v.get("orgs").unwrap().as_array().unwrap().is_empty());

    // Create org
    let create_body = serde_json::json!({"name": "TestOrg", "tag": "TST"}).to_string();
    let req2 = Request::builder()
        .method("POST")
        .uri("/v1/orgs")
        .header("content-type", "application/json")
        .body(Body::from(create_body))
        .unwrap();
    let resp2 = app.clone().oneshot(req2).await.unwrap();
    assert_eq!(resp2.status(), StatusCode::OK);
    let bytes2 = body::to_bytes(resp2.into_body(), 1024 * 1024).await.unwrap();
    let created: serde_json::Value = serde_json::from_slice(&bytes2).unwrap();
    assert_eq!(created.get("name").unwrap().as_str().unwrap(), "TestOrg");

    // GET by id
    let id = created.get("id").unwrap().as_str().unwrap();
    let uri = format!("/v1/orgs/{}", id);
    let req3 = Request::builder().method("GET").uri(uri).body(Body::empty()).unwrap();
    let resp3 = app.clone().oneshot(req3).await.unwrap();
    assert_eq!(resp3.status(), StatusCode::OK);
    let bytes3 = body::to_bytes(resp3.into_body(), 1024 * 1024).await.unwrap();
    let fetched: serde_json::Value = serde_json::from_slice(&bytes3).unwrap();
    assert_eq!(fetched.get("id").unwrap().as_str().unwrap(), id);

    // List again should have one entry
    let req4 = Request::builder().method("GET").uri("/v1/orgs").body(Body::empty()).unwrap();
    let resp4 = app.clone().oneshot(req4).await.unwrap();
    assert_eq!(resp4.status(), StatusCode::OK);
    let bytes4 = body::to_bytes(resp4.into_body(), 1024 * 1024).await.unwrap();
    let v4: serde_json::Value = serde_json::from_slice(&bytes4).unwrap();
    assert_eq!(v4.get("orgs").unwrap().as_array().unwrap().len(), 1);
}
