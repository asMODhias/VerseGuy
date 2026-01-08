#![allow(clippy::disallowed_methods)]
use axum::body::{self, Body};
use axum::http::{Request, StatusCode};
use master_server::build_app;
use master_server::state::AppState;
use std::sync::Arc;
use tempfile::tempdir;
use tower::util::ServiceExt;
use verseguy_test_utils::{must, must_opt};

#[test]
fn create_and_list_orgs() {
    let rt = match tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
    {
        Ok(rt) => rt,
        Err(e) => panic!("failed to build runtime: {}", e),
    };

    rt.block_on(async {
        let dir = must(tempdir());
        let state = Arc::new(must(AppState::new(dir.path(), vec![0u8; 32])));
        let app = build_app(state.clone());

        // Initially list should return empty array
        let req = must(
            Request::builder()
                .method("GET")
                .uri("/v1/orgs")
                .body(Body::empty()),
        );
        let resp = must(app.clone().oneshot(req).await);
        assert_eq!(resp.status(), StatusCode::OK);
        let bytes = must(body::to_bytes(resp.into_body(), 1024 * 1024).await);
        let v: serde_json::Value = must(serde_json::from_slice(&bytes));
        assert!(must_opt(v.get("orgs").and_then(|r| r.as_array()), "missing orgs").is_empty());

        // Create org
        let create_body = "{\"name\":\"TestOrg\",\"tag\":\"TST\"}".to_string();
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
        assert_eq!(
            must_opt(created.get("name").and_then(|n| n.as_str()), "missing name"),
            "TestOrg"
        );

        // GET by id
        let id = must_opt(created.get("id").and_then(|v| v.as_str()), "missing id");
        let uri = format!("/v1/orgs/{}", id);
        let req3 = must(
            Request::builder()
                .method("GET")
                .uri(uri)
                .body(Body::empty()),
        );
        let resp3 = must(app.clone().oneshot(req3).await);
        assert_eq!(resp3.status(), StatusCode::OK);
        let bytes3 = must(body::to_bytes(resp3.into_body(), 1024 * 1024).await);
        let fetched: serde_json::Value = must(serde_json::from_slice(&bytes3));
        assert_eq!(
            must_opt(fetched.get("id").and_then(|v| v.as_str()), "missing id 2"),
            id
        );

        // List again should have one entry
        let req4 = must(
            Request::builder()
                .method("GET")
                .uri("/v1/orgs")
                .body(Body::empty()),
        );
        let resp4 = must(app.clone().oneshot(req4).await);
        assert_eq!(resp4.status(), StatusCode::OK);
        let bytes4 = must(body::to_bytes(resp4.into_body(), 1024 * 1024).await);
        let v4: serde_json::Value = must(serde_json::from_slice(&bytes4));
        assert_eq!(
            must_opt(v4.get("orgs").and_then(|r| r.as_array()), "missing orgs 2").len(),
            1
        );
    });
}
