#![allow(clippy::disallowed_methods)]
use master_server::build_app;
use master_server::state::AppState;
use std::sync::Arc;
use tempfile::tempdir;
use verseguy_test_utils::{must, must_opt};

#[test]
fn starts_and_serves_plugins_search() {
    let rt = match tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
    {
        Ok(rt) => rt,
        Err(e) => panic!("failed to build runtime: {}", e),
    };

    rt.block_on(async {
        // create temp db path
        let dir = must(tempdir());
        let db_path = must_opt(dir.path().to_str(), "tempdir path not utf8").to_string();
        let secret = b"integration-secret".to_vec();

        let state = Arc::new(must(AppState::new(db_path, secret)));
        let app = build_app(state.clone());

        // Call the router directly using ServiceExt::oneshot to avoid network bindings and hyper version conflicts
        use axum::http::Request;
        use tower::util::ServiceExt; // for .oneshot()

        let req: Request<axum::body::Body> = must(
            Request::builder()
                .method("GET")
                .uri("/plugins/search")
                .body(axum::body::Body::empty()),
        );

        let resp = must(app.clone().oneshot(req).await);
        let status = resp.status();
        assert!(status.is_success());

        // Health check
        let req2: Request<axum::body::Body> = must(
            Request::builder()
                .method("GET")
                .uri("/healthz")
                .body(axum::body::Body::empty()),
        );

        let resp2 = must(app.clone().oneshot(req2).await);
        assert!(resp2.status().is_success());

        // Metrics (may be enabled or not depending on initialization)
        let req3: Request<axum::body::Body> = must(
            Request::builder()
                .method("GET")
                .uri("/metrics")
                .body(axum::body::Body::empty()),
        );

        let _ = must(app.clone().oneshot(req3).await);
    });
}
