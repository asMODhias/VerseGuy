use master_server::state::AppState;
use std::sync::Arc;
use tempfile::tempdir;

#[tokio::test]
async fn create_and_get_latest_legal_doc() {
    let dir = tempdir().unwrap();
    let state = Arc::new(AppState::new(dir.path(), b"test-secret".to_vec()).unwrap());

    // set admin token so handler allows operation
    std::env::set_var("MASTER_ADMIN_TOKEN", "admintoken");

    // Use raw JSON string to avoid needing Serialize on the request type
    let req_json = r#"{"doc_type":"tos","version":"1.0.0","title":"Terms of Service","content":"These are the terms","author":"Legal Team"}"#;
    let http_req = axum::http::Request::builder()
        .method(axum::http::Method::POST)
        .uri("/admin/legal/create")
        .header("content-type", "application/json")
        .header("x-admin-token", "admintoken")
        .body(axum::body::Body::from(req_json))
        .unwrap();

    let _res = master_server::legal::admin_create_legal_handler(
        axum::extract::State(state.clone()),
        http_req,
    )
    .await
    .expect("create should succeed");

    // check latest
    let latest_res = master_server::legal::get_latest_legal_handler(
        axum::extract::State(state.clone()),
        axum::extract::Path("tos".to_string()),
    )
    .await
    .expect("get latest should succeed");

    let axum::Json(val) = latest_res;
    let s = serde_json::to_string(&val).unwrap();
    assert!(s.contains("Terms of Service"));
}
