#![allow(clippy::disallowed_methods)]
use axum::body::{self, Body};
use axum::http::{Request, StatusCode};
use master_server::build_app;
use master_server::state::AppState;
use std::sync::Arc;
use tempfile::tempdir;
use tower::util::ServiceExt;
use verseguy_test_utils::{must, must_opt};

#[tokio::test]
async fn org_treasury_and_members() {
    let dir = must(tempdir());
    let state = Arc::new(must(AppState::new(dir.path(), vec![0u8; 32])));
    let app = build_app(state.clone());

    // Create org
    let create_body = "{\"name\":\"OpsOrg\",\"tag\":\"OPS\"}".to_string();
    let req2 = must(
        Request::builder()
            .method("POST")
            .uri("/v1/orgs")
            .header("content-type", "application/json")
            .body(Body::from(create_body)),
    );
    let resp2 = must(app.clone().oneshot(req2).await);
    assert_eq!(resp2.status(), StatusCode::OK);
    let bytes2 = must(body::to_bytes(resp2.into_body(), 1024 * 1024).await);
    let created: serde_json::Value = must(serde_json::from_slice(&bytes2));
    let id = must_opt(created.get("id").and_then(|v| v.as_str()), "missing id");

    // Deposit 200
    let deposit_body = "{\"amount\":200}".to_string();
    let uri = format!("/v1/orgs/{}/deposit", id);
    let reqd = must(
        Request::builder()
            .method("POST")
            .uri(uri)
            .header("content-type", "application/json")
            .body(Body::from(deposit_body)),
    );
    let respd = must(app.clone().oneshot(reqd).await);
    assert_eq!(respd.status(), StatusCode::OK);
    let bytesd = must(body::to_bytes(respd.into_body(), 1024 * 1024).await);
    let bal: serde_json::Value = must(serde_json::from_slice(&bytesd));
    assert_eq!(
        must_opt(
            bal.get("balance").and_then(|v| v.as_i64()),
            "missing balance"
        ),
        200
    );

    // Withdraw 50
    let withdraw_body = "{\"amount\":50}".to_string();
    let uriw = format!("/v1/orgs/{}/withdraw", id);
    let reqw = must(
        Request::builder()
            .method("POST")
            .uri(uriw)
            .header("content-type", "application/json")
            .body(Body::from(withdraw_body)),
    );
    let respw = must(app.clone().oneshot(reqw).await);
    assert_eq!(respw.status(), StatusCode::OK);
    let bytesw = must(body::to_bytes(respw.into_body(), 1024 * 1024).await);
    let bal2: serde_json::Value = must(serde_json::from_slice(&bytesw));
    assert_eq!(
        must_opt(
            bal2.get("balance").and_then(|v| v.as_i64()),
            "missing balance"
        ),
        150
    );

    // Add member
    let member_body = "{\"user_id\":\"user-1\",\"rank_id\":null}".to_string();
    let urim = format!("/v1/orgs/{}/members", id);
    let reqm = must(
        Request::builder()
            .method("POST")
            .uri(urim)
            .header("content-type", "application/json")
            .body(Body::from(member_body)),
    );
    let respm = must(app.clone().oneshot(reqm).await);
    assert_eq!(respm.status(), StatusCode::OK);

    // Fetch org and verify member
    let urig = format!("/v1/orgs/{}", id);
    let reqg = must(
        Request::builder()
            .method("GET")
            .uri(urig)
            .body(Body::empty()),
    );
    let respg = must(app.clone().oneshot(reqg).await);
    assert_eq!(respg.status(), StatusCode::OK);
    let bytesg = must(body::to_bytes(respg.into_body(), 1024 * 1024).await);
    let fetched: serde_json::Value = must(serde_json::from_slice(&bytesg));
    let members = must_opt(
        fetched.get("members").and_then(|m| m.as_array()),
        "missing members",
    );
    assert_eq!(members.len(), 1);
}
