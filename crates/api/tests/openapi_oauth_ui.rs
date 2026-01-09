use axum::body::Body;
use tower::util::ServiceExt;

#[test]
fn interactive_ui_exposes_oauth_helpers() {
    let rt = match tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
    {
        Ok(rt) => rt,
        Err(e) => panic!("failed to build runtime: {}", e),
    };

    rt.block_on(async {
        use verseguy_api::build_app;
        let app = build_app();

        let req = match axum::http::Request::builder()
            .method("GET")
            .uri("/static/swagger-ui/interactive.js")
            .body(Body::empty())
        {
            Ok(r) => r,
            Err(e) => panic!("failed to build request: {}", e),
        };

        let resp: axum::http::Response<axum::body::Body> = match app.clone().oneshot(req).await {
            Ok(r) => r,
            Err(e) => panic!("request failed: {}", e),
        };
        assert_eq!(resp.status(), axum::http::StatusCode::OK);

        let b = match axum::body::to_bytes(resp.into_body(), 1024 * 1024).await {
            Ok(b) => b,
            Err(e) => panic!("failed to read body: {}", e),
        };
        let s = String::from_utf8_lossy(&b);
        // ensure helper functions are present for client credentials and auth code flow
        assert!(s.contains("fetchTokenClientCredentials"));
        assert!(s.contains("setAuthHeader"));
        assert!(s.contains("performAuthorizationCodeFlow"));

        // callback page should be present and contain postMessage handler
        let req_cb = match axum::http::Request::builder()
            .method("GET")
            .uri("/static/swagger-ui/oauth-callback.html")
            .body(Body::empty())
        {
            Ok(r) => r,
            Err(e) => panic!("failed to build request: {}", e),
        };
        let resp_cb: axum::http::Response<axum::body::Body> =
            match app.clone().oneshot(req_cb).await {
                Ok(r) => r,
                Err(e) => panic!("request failed: {}", e),
            };
        assert_eq!(resp_cb.status(), axum::http::StatusCode::OK);
        let bcb = match axum::body::to_bytes(resp_cb.into_body(), 1024 * 1024).await {
            Ok(b) => b,
            Err(e) => panic!("failed to read body: {}", e),
        };
        let scb = String::from_utf8_lossy(&bcb);
        assert!(scb.contains("postMessage"));
    });
}
