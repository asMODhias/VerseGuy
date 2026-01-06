#![allow(clippy::disallowed_methods)]
use axum::body::Body;
use axum::http::{Request, StatusCode};
use master_server::build_app;
use master_server::state::AppState;
use std::sync::Arc;
use tempfile::tempdir;
use tower::util::ServiceExt;
use verseguy_audit::AuditService;
use verseguy_test_utils::{must, must_opt};

#[tokio::test]
async fn audit_export_and_delete_user_data() {
    let dir = must(tempdir());
    let state = Arc::new(must(AppState::new(dir.path(), vec![0u8; 32])));
    let app = build_app(state.clone());

    // Log some audit events for user
    let audit = AuditService::new(state.storage.clone());
    must(audit.log_event(Some("user-42".to_string()), "action:created".to_string()));
    must(audit.log_event(Some("user-42".to_string()), "action:updated".to_string()));
    must(audit.log_event(Some("other-user".to_string()), "action:other".to_string()));

    // Export for user-42
    let req = must(Request::builder()
        .method("GET")
        .uri("/audit/export/user-42")
        .body(Body::empty()));
    let resp = must(app.clone().oneshot(req).await);
    assert_eq!(resp.status(), StatusCode::OK);
    let bytes = must(axum::body::to_bytes(resp.into_body(), 1024 * 1024).await);
    let v: serde_json::Value = must(serde_json::from_slice(&bytes));
    let entries = must_opt(v.get("entries").and_then(|r| r.as_array()), "missing entries");
    assert_eq!(entries.len(), 2);

    // Add ToS acceptance
    let tos_body = r#"{"user_id":"user-42","accepted_at": 1234567890, "version": "1.0.0"}"#.to_string();
    let req2 = must(Request::builder()
        .method("POST")
        .uri("/auth/tos")
        .header("content-type", "application/json")
        .body(Body::from(tos_body)));
    let resp2 = must(app.clone().oneshot(req2).await);
    assert_eq!(resp2.status(), StatusCode::OK);

    // Delete user data
    let req3 = must(Request::builder()
        .method("DELETE")
        .uri("/users/user-42/data")
        .body(Body::empty()));
    let resp3 = must(app.clone().oneshot(req3).await);
    assert_eq!(resp3.status(), StatusCode::OK);
    let bytes3 = must(axum::body::to_bytes(resp3.into_body(), 1024 * 1024).await);
    let v3: serde_json::Value = must(serde_json::from_slice(&bytes3));
    assert!(must_opt(v3.get("deleted").and_then(|n| n.as_i64()), "missing deleted") >= 2);

    // Export should now be empty for user-42
    let req4 = must(Request::builder()
        .method("GET")
        .uri("/audit/export/user-42")
        .body(Body::empty()));
    let resp4 = must(app.clone().oneshot(req4).await);
    let bytes4 = must(axum::body::to_bytes(resp4.into_body(), 1024 * 1024).await);
    let v4: serde_json::Value = must(serde_json::from_slice(&bytes4));
    let entries4 = must_opt(v4.get("entries").and_then(|r| r.as_array()), "missing entries 2");
    assert_eq!(entries4.len(), 0);

    // GET /auth/tos/user-42 should return 404
    let req5 = must(Request::builder()
        .method("GET")
        .uri("/auth/tos/user-42")
        .body(Body::empty()));
    let resp5 = must(app.oneshot(req5).await);
    assert_eq!(resp5.status(), StatusCode::NOT_FOUND);
}
