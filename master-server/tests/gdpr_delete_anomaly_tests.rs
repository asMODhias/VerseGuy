#![allow(clippy::disallowed_methods)]
use axum::body::Body;
use axum::http::{Request, StatusCode};
use master_server::build_app;
use master_server::state::AppState;
use std::sync::Arc;
use tempfile::tempdir;
use tower::util::ServiceExt;
use verseguy_test_utils::{must, must_opt};

#[tokio::test]
async fn multiple_gdpr_delete_requests_record_multiple_audit_delete_events() {
    let dir = must(tempdir());
    let state = Arc::new(must(AppState::new(dir.path(), vec![0u8; 32])));
    let app = build_app(state.clone());

    // Log some audit events for user
    let audit = verseguy_audit::AuditService::new(state.storage.clone());
    must(audit.log_event(Some("target-user".to_string()), "action:created".to_string()));

    // Create role `compliance` and assign actor to it
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

    let compliance_role = RoleRec {
        id: "r-compliance".to_string(),
        name: "compliance".to_string(),
        version: 0,
    };
    must(state.storage.put(format!("role:{}", compliance_role.id).as_bytes(), &compliance_role));

    let actor_id = "actor-anom".to_string();
    let assign = AssignmentRec {
        user_id: actor_id.clone(),
        role_id: compliance_role.id.clone(),
        version: 0,
    };
    must(state.storage.put(format!("assignment:{}", assign.user_id).as_bytes(), &assign));

    // Create policy
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

    // Create session token
    let session_service = verseguy_auth::SessionService::new(state.license_secret.clone());
    let token = must(session_service.create_and_store_session(&actor_id, &verseguy_auth::License::Enterprise, 1, &state.storage));

    // Send multiple delete requests
    let mut succeeded = 0;
    let requests = 6;
    for _ in 0..requests {
        let req = must(Request::builder()
            .method("DELETE")
            .uri(format!("/users/{}/data", "target-user"))
            .header("Authorization", format!("Bearer {}", token))
            .body(Body::empty()));
        let resp = must(app.clone().oneshot(req).await);
        if resp.status() == StatusCode::OK {
            succeeded += 1;
        }
    }

    assert_eq!(succeeded, requests);

    // Verify actor has at least `requests` audit.delete events recorded
    let req_actor_audit = must(Request::builder()
        .method("GET")
        .uri(format!("/audit/export/{}", actor_id))
        .body(Body::empty()));
    let resp_actor_audit = must(app.clone().oneshot(req_actor_audit).await);
    assert_eq!(resp_actor_audit.status(), StatusCode::OK);
    let bytes_actor = must(axum::body::to_bytes(resp_actor_audit.into_body(), 1024 * 1024).await);
    let v_actor: serde_json::Value = must(serde_json::from_slice(&bytes_actor));
    let entries_actor = must_opt(v_actor.get("entries").and_then(|r| r.as_array()), "missing actor entries");
    let delete_count = entries_actor.iter().filter(|e| e.get("event").and_then(|ev| ev.as_str()).map(|s| s.contains("audit.delete")).unwrap_or(false)).count();

    assert!(delete_count >= requests);
}
