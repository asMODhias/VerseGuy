use axum::body::Body;
use axum::http::{Request, StatusCode};
use master_server::build_app;
use master_server::state::AppState;
use std::sync::Arc;
use tempfile::tempdir;
use tower::util::ServiceExt;
use verseguy_audit::AuditService;

#[tokio::test]
async fn audit_export_and_delete_user_data() {
    let dir = tempdir().unwrap();
    let state = Arc::new(AppState::new(dir.path(), vec![0u8; 32]).unwrap());
    let app = build_app(state.clone());

    // Log some audit events for user
    let audit = AuditService::new(state.storage.clone());
    audit
        .log_event(Some("user-42".to_string()), "action:created".to_string())
        .unwrap();
    audit
        .log_event(Some("user-42".to_string()), "action:updated".to_string())
        .unwrap();
    audit
        .log_event(Some("other-user".to_string()), "action:other".to_string())
        .unwrap();

    // Export for user-42
    let req = Request::builder()
        .method("GET")
        .uri("/audit/export/user-42")
        .body(Body::empty())
        .unwrap();
    let resp = app.clone().oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let bytes = axum::body::to_bytes(resp.into_body(), 1024 * 1024)
        .await
        .unwrap();
    let v: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    let entries = v.get("entries").unwrap().as_array().unwrap();
    assert_eq!(entries.len(), 2);

    // Add ToS acceptance
    let tos_body =
        serde_json::json!({"user_id":"user-42","accepted_at": 1234567890, "version": "1.0.0"})
            .to_string();
    let req2 = Request::builder()
        .method("POST")
        .uri("/auth/tos")
        .header("content-type", "application/json")
        .body(Body::from(tos_body))
        .unwrap();
    let resp2 = app.clone().oneshot(req2).await.unwrap();
    assert_eq!(resp2.status(), StatusCode::OK);

    // Delete user data
    let req3 = Request::builder()
        .method("DELETE")
        .uri("/users/user-42/data")
        .body(Body::empty())
        .unwrap();
    let resp3 = app.clone().oneshot(req3).await.unwrap();
    assert_eq!(resp3.status(), StatusCode::OK);
    let bytes3 = axum::body::to_bytes(resp3.into_body(), 1024 * 1024)
        .await
        .unwrap();
    let v3: serde_json::Value = serde_json::from_slice(&bytes3).unwrap();
    assert!(v3.get("deleted").unwrap().as_i64().unwrap() >= 2);

    // Export should now be empty for user-42
    let req4 = Request::builder()
        .method("GET")
        .uri("/audit/export/user-42")
        .body(Body::empty())
        .unwrap();
    let resp4 = app.clone().oneshot(req4).await.unwrap();
    let bytes4 = axum::body::to_bytes(resp4.into_body(), 1024 * 1024)
        .await
        .unwrap();
    let v4: serde_json::Value = serde_json::from_slice(&bytes4).unwrap();
    let entries4 = v4.get("entries").unwrap().as_array().unwrap();
    assert_eq!(entries4.len(), 0);

    // GET /auth/tos/user-42 should return 404
    let req5 = Request::builder()
        .method("GET")
        .uri("/auth/tos/user-42")
        .body(Body::empty())
        .unwrap();
    let resp5 = app.oneshot(req5).await.unwrap();
    assert_eq!(resp5.status(), StatusCode::NOT_FOUND);
}
