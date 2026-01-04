use master_server::legal::{AdminCreateLegalRequest, LegalDocument};
use master_server::state::AppState;
use std::sync::Arc;
use tempfile::tempdir;

#[tokio::test]
async fn create_and_get_latest_legal_doc() {
    let dir = tempdir().unwrap();
    let state = Arc::new(AppState::new(dir.path(), b"test-secret".to_vec()).unwrap());

    let req = AdminCreateLegalRequest {
        doc_type: "tos".to_string(),
        version: "1.0.0".to_string(),
        title: "Terms of Service".to_string(),
        content: "These are the terms".to_string(),
        author: Some("Legal Team".to_string()),
    };

    let res = master_server::legal::admin_create_legal_handler(
        axum::extract::State(state.clone()),
        axum::Json(req),
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

    if let axum::Json(val) = latest_res {
        let s = serde_json::to_string(&val).unwrap();
        assert!(s.contains("Terms of Service"));
    } else {
        panic!("unexpected response type");
    }
}
