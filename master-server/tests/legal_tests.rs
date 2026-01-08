use master_server::state::AppState;
use std::sync::Arc;
use tempfile::tempdir;
use verseguy_test_utils::must;

#[test]
#[allow(clippy::disallowed_methods)]
fn create_and_get_latest_legal_doc() {
    let rt = match tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
    {
        Ok(rt) => rt,
        Err(e) => panic!("failed to build runtime: {}", e),
    };

    rt.block_on(async {
        let dir = must(tempdir());
    let state = Arc::new(must(AppState::new(dir.path(), b"test-secret".to_vec())));

    // set admin token so handler allows operation
    std::env::set_var("MASTER_ADMIN_TOKEN", "admintoken");

    // Use raw JSON string to avoid needing Serialize on the request type
    let req_json = r#"{"doc_type":"tos","version":"1.0.0","title":"Terms of Service","content":"These are the terms","author":"Legal Team"}"#;
    let http_req = must(
        axum::http::Request::builder()
            .method(axum::http::Method::POST)
            .uri("/admin/legal/create")
            .header("content-type", "application/json")
            .header("x-admin-token", "admintoken")
            .body(axum::body::Body::from(req_json)),
    );

    match master_server::legal::admin_create_legal_handler(
        axum::extract::State(state.clone()),
        http_req,
    )
    .await
    {
        Ok(_) => {}
        Err((status, msg)) => panic!("create failed: {} {}", status, msg),
    }

    // check latest
    let latest_res = match master_server::legal::get_latest_legal_handler(
        axum::extract::State(state.clone()),
        axum::extract::Path("tos".to_string()),
    )
    .await
    {
        Ok(r) => r,
        Err((status, msg)) => panic!("get latest failed: {} {}", status, msg),
    };

    let axum::Json(val) = latest_res;
    let s = must(serde_json::to_string(&val));
    assert!(s.contains("Terms of Service"));
    });
}
