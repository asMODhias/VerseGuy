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
async fn create_and_get_app() {
    let dir = must(tempdir());
    let state = Arc::new(must(AppState::new(dir.path(), vec![0u8; 32])));
    let app = build_app(state.clone());

    // Create app
    let create_body = "{\"name\":\"App-E2E\"}".to_string();
    let req = must(
        Request::builder()
            .method("POST")
            .uri("/v1/apps")
            .header("content-type", "application/json")
            .body(Body::from(create_body)),
    );
    let resp = must(app.clone().oneshot(req).await);
    assert_eq!(resp.status(), StatusCode::OK);
    let bytes = must(body::to_bytes(resp.into_body(), 1024 * 1024).await);
    let created: serde_json::Value = must(serde_json::from_slice(&bytes));
    let id = must_opt(created.get("id").and_then(|v| v.as_str()), "missing id");

    // Get
    let uri = format!("/v1/apps/{}", id);
    let req2 = must(
        Request::builder()
            .method("GET")
            .uri(uri)
            .body(Body::empty()),
    );
    let resp2 = must(app.clone().oneshot(req2).await);
    assert_eq!(resp2.status(), StatusCode::OK);
    let bytes2 = must(body::to_bytes(resp2.into_body(), 1024 * 1024).await);
    let fetched: serde_json::Value = must(serde_json::from_slice(&bytes2));
    assert_eq!(
        fetched.get("name").and_then(|v| v.as_str()).unwrap_or(""),
        "App-E2E"
    );

    // Update name
    let upd_body = "{\"name\":\"App-E2E-Updated\"}".to_string();
    let uri_upd = format!("/v1/apps/{}", id);
    let req3 = must(
        Request::builder()
            .method("PATCH")
            .uri(uri_upd.clone())
            .header("content-type", "application/json")
            .body(Body::from(upd_body)),
    );
    let resp3 = must(app.clone().oneshot(req3).await);
    assert_eq!(resp3.status(), StatusCode::OK);

    // Add metadata and tags via PATCH
    let meta_body =
        r#"{"metadata":{"env":"staging","owner":"team-a"},"tags":["beta","internal"]}"#.to_string();
    let req_meta = must(
        Request::builder()
            .method("PATCH")
            .uri(uri_upd.clone())
            .header("content-type", "application/json")
            .body(Body::from(meta_body)),
    );
    let resp_meta = must(app.clone().oneshot(req_meta).await);
    assert_eq!(resp_meta.status(), StatusCode::OK);

    // Fetch and verify update
    let req4 = must(
        Request::builder()
            .method("GET")
            .uri(uri_upd.clone())
            .body(Body::empty()),
    );
    let resp4 = must(app.clone().oneshot(req4).await);
    let bytes4 = must(body::to_bytes(resp4.into_body(), 1024 * 1024).await);
    let fetched2: serde_json::Value = must(serde_json::from_slice(&bytes4));
    assert_eq!(
        fetched2.get("name").and_then(|v| v.as_str()).unwrap_or(""),
        "App-E2E-Updated"
    );
    // verify metadata
    assert_eq!(
        fetched2
            .get("metadata")
            .and_then(|m| m.get("env"))
            .and_then(|v| v.as_str())
            .unwrap_or(""),
        "staging"
    );
    // verify tags
    let tags = fetched2.get("tags").and_then(|t| t.as_array()).unwrap();
    assert!(tags.iter().any(|v| v.as_str().unwrap_or("") == "beta"));

    // Bulk create apps
    let bulk_body = r#"{"apps":[{"name":"Bulk1"},{"name":"Bulk2"}]}"#.to_string();
    let req_bulk = must(
        Request::builder()
            .method("POST")
            .uri("/v1/apps/bulk")
            .header("content-type", "application/json")
            .body(Body::from(bulk_body)),
    );
    let resp_bulk = must(app.clone().oneshot(req_bulk).await);
    assert_eq!(resp_bulk.status(), StatusCode::OK);
    let bytesb = must(body::to_bytes(resp_bulk.into_body(), 1024 * 1024).await);
    let created: serde_json::Value = must(serde_json::from_slice(&bytesb));
    let apps = must_opt(
        created.get("apps").and_then(|v| v.as_array()),
        "missing apps",
    );
    assert!(apps.len() >= 2);

    // Collect ids and bulk delete
    let ids: Vec<String> = apps
        .iter()
        .map(|a| {
            a.get("id")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string()
        })
        .collect();
    let del_body = serde_json::json!({"ids": ids}).to_string();
    let req_del_bulk = must(
        Request::builder()
            .method("DELETE")
            .uri("/v1/apps/bulk")
            .header("content-type", "application/json")
            .body(Body::from(del_body)),
    );
    let resp_del_bulk = must(app.clone().oneshot(req_del_bulk).await);
    assert_eq!(resp_del_bulk.status(), StatusCode::OK);

    // Fetch should now return null for first bulk id
    let uri_first = format!("/v1/apps/{}", ids[0]);
    let req6 = must(
        Request::builder()
            .method("GET")
            .uri(uri_first.clone())
            .body(Body::empty()),
    );
    let resp6 = must(app.clone().oneshot(req6).await);
    let bytes6 = must(body::to_bytes(resp6.into_body(), 1024 * 1024).await);
    let fetched3: serde_json::Value = must(serde_json::from_slice(&bytes6));
    assert!(fetched3.is_null() || fetched3 == serde_json::Value::Null);
}
