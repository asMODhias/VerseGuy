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
    let req = must(
        Request::builder()
            .method("GET")
            .uri("/audit/export/user-42")
            .body(Body::empty()),
    );
    let resp = must(app.clone().oneshot(req).await);
    assert_eq!(resp.status(), StatusCode::OK);
    let bytes = must(axum::body::to_bytes(resp.into_body(), 1024 * 1024).await);
    let v: serde_json::Value = must(serde_json::from_slice(&bytes));
    let entries = must_opt(
        v.get("entries").and_then(|r| r.as_array()),
        "missing entries",
    );
    assert_eq!(entries.len(), 2);

    // Add ToS acceptance
    let tos_body =
        r#"{"user_id":"user-42","accepted_at": 1234567890, "version": "1.0.0"}"#.to_string();
    let req2 = must(
        Request::builder()
            .method("POST")
            .uri("/auth/tos")
            .header("content-type", "application/json")
            .body(Body::from(tos_body)),
    );
    let resp2 = must(app.clone().oneshot(req2).await);
    assert_eq!(resp2.status(), StatusCode::OK);

    // Create an admin role and assign an actor (admin) user
    // Create role & assignment records directly in storage (avoid adding a crate dependency)
    #[derive(serde::Serialize)]
    struct RoleRec {
        id: String,
        name: String,
        version: u64,
    }
    #[derive(serde::Serialize)]
    struct AssignmentRec {
        user_id: String,
        role_id: String,
        version: u64,
    }

    // Create role `compliance` and assign actor to it
    let compliance_role = RoleRec {
        id: "r-compliance".to_string(),
        name: "compliance".to_string(),
        version: 0,
    };
    must(state.storage.put(
        format!("role:{}", compliance_role.id).as_bytes(),
        &compliance_role,
    ));

    let actor_id = "actor-1".to_string();
    let assign = AssignmentRec {
        user_id: actor_id.clone(),
        role_id: compliance_role.id.clone(),
        version: 0,
    };
    must(
        state
            .storage
            .put(format!("assignment:{}", assign.user_id).as_bytes(), &assign),
    );

    // Create a policy `compliance:delete` that requires role:compliance
    #[derive(serde::Serialize)]
    struct PolicyRec {
        id: String,
        name: String,
        policy: String,
        version: u64,
    }
    let p = PolicyRec {
        id: "p1".to_string(),
        name: "compliance:delete".to_string(),
        policy: "role:compliance".to_string(),
        version: 0,
    };
    must(state.storage.put(format!("policy:{}", p.id).as_bytes(), &p));

    // Create a session token for the actor and store session via SessionService
    let session_service = verseguy_auth::SessionService::new(state.license_secret.clone());
    let token = must(session_service.create_and_store_session(
        &actor_id,
        &verseguy_auth::License::Enterprise,
        1,
        &state.storage,
    ));

    // Delete user data with Authorization header
    let req3 = must(
        Request::builder()
            .method("DELETE")
            .uri("/users/user-42/data")
            .header("Authorization", format!("Bearer {}", token))
            .body(Body::empty()),
    );
    let resp3 = must(app.clone().oneshot(req3).await);
    assert_eq!(resp3.status(), StatusCode::OK);
    let bytes3 = must(axum::body::to_bytes(resp3.into_body(), 1024 * 1024).await);
    let v3: serde_json::Value = must(serde_json::from_slice(&bytes3));
    assert!(
        must_opt(
            v3.get("deleted").and_then(|n| n.as_i64()),
            "missing deleted"
        ) >= 2
    );

    // Verify an audit.delete event was recorded for the actor
    let req_actor_audit = must(
        Request::builder()
            .method("GET")
            .uri(format!("/audit/export/{}", actor_id))
            .body(Body::empty()),
    );
    let resp_actor_audit = must(app.clone().oneshot(req_actor_audit).await);
    assert_eq!(resp_actor_audit.status(), StatusCode::OK);
    let bytes_actor = must(axum::body::to_bytes(resp_actor_audit.into_body(), 1024 * 1024).await);
    let v_actor: serde_json::Value = must(serde_json::from_slice(&bytes_actor));
    let entries_actor = must_opt(
        v_actor.get("entries").and_then(|r| r.as_array()),
        "missing actor entries",
    );
    // there should be at least one entry and at least one should contain "audit.delete"
    assert!(!entries_actor.is_empty());
    assert!(entries_actor.iter().any(|e| e
        .get("event")
        .and_then(|ev| ev.as_str())
        .map(|s| s.contains("audit.delete"))
        .unwrap_or(false)));

    // Export should now be empty for user-42
    let req4 = must(
        Request::builder()
            .method("GET")
            .uri("/audit/export/user-42")
            .body(Body::empty()),
    );
    let resp4 = must(app.clone().oneshot(req4).await);
    let bytes4 = must(axum::body::to_bytes(resp4.into_body(), 1024 * 1024).await);
    let v4: serde_json::Value = must(serde_json::from_slice(&bytes4));
    let entries4 = must_opt(
        v4.get("entries").and_then(|r| r.as_array()),
        "missing entries 2",
    );
    assert_eq!(entries4.len(), 0);

    // GET /auth/tos/user-42 should return 404
    let req5 = must(
        Request::builder()
            .method("GET")
            .uri("/auth/tos/user-42")
            .body(Body::empty()),
    );
    let resp5 = must(app.oneshot(req5).await);
    assert_eq!(resp5.status(), StatusCode::NOT_FOUND);
}
