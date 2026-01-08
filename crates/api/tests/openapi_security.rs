use axum::body::Body;
use tower::util::ServiceExt;

#[test]
fn openapi_contains_oauth2_security() {
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
            .uri("/openapi.yaml")
            .body(Body::empty())
        {
            Ok(r) => r,
            Err(e) => panic!("failed to build request: {}", e),
        };

        let resp: axum::http::Response<axum::body::Body> = match app.clone().oneshot(req).await {
            Ok(r) => r,
            Err(e) => panic!("request failed: {}", e),
        };
        let bytes = match axum::body::to_bytes(resp.into_body(), 1024 * 1024).await {
            Ok(b) => b,
            Err(e) => panic!("failed to read body: {}", e),
        };
        let s = String::from_utf8_lossy(&bytes);
        assert!(s.contains("securitySchemes"));
        assert!(s.contains("authorizationCode"));
        assert!(s.contains("clientCredentials"));
    });
}
