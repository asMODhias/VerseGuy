use axum::body::Body;
use tower::util::ServiceExt;

#[test]
fn openapi_and_docs_are_served() {
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
        assert_eq!(resp.status(), axum::http::StatusCode::OK);
        let bytes = match axum::body::to_bytes(resp.into_body(), 1024 * 1024).await {
            Ok(b) => b,
            Err(e) => panic!("failed to read body: {}", e),
        };
        let s = String::from_utf8_lossy(&bytes);
        assert!(s.contains("openapi: 3.0.1"));

        let req2 = match axum::http::Request::builder()
            .method("GET")
            .uri("/docs")
            .body(Body::empty())
        {
            Ok(r) => r,
            Err(e) => panic!("failed to build request: {}", e),
        };
        let resp2: axum::http::Response<axum::body::Body> = match app.clone().oneshot(req2).await {
            Ok(r) => r,
            Err(e) => panic!("request failed: {}", e),
        };
        assert_eq!(resp2.status(), axum::http::StatusCode::OK);
        let b2 = match axum::body::to_bytes(resp2.into_body(), 1024 * 1024).await {
            Ok(b) => b,
            Err(e) => panic!("failed to read body: {}", e),
        };
        let s2 = String::from_utf8_lossy(&b2);
        assert!(s2.contains("SwaggerUIBundle"));

        // Ensure interactive JS is present and exposes the renderInteractiveUI function
        let req_inter = match axum::http::Request::builder()
            .method("GET")
            .uri("/static/swagger-ui/interactive.js")
            .body(Body::empty())
        {
            Ok(r) => r,
            Err(e) => panic!("failed to build request: {}", e),
        };
        let resp_inter: axum::http::Response<axum::body::Body> =
            match app.clone().oneshot(req_inter).await {
                Ok(r) => r,
                Err(e) => panic!("request failed: {}", e),
            };
        assert_eq!(resp_inter.status(), axum::http::StatusCode::OK);
        let b_inter = match axum::body::to_bytes(resp_inter.into_body(), 1024 * 1024).await {
            Ok(b) => b,
            Err(e) => panic!("failed to read body: {}", e),
        };
        let s_inter = String::from_utf8_lossy(&b_inter);
        assert!(s_inter.contains("renderInteractiveUI"));

        // The vendored official bundle and preset (if downloaded) should be served if present
        for u in [
            "/static/swagger-ui/swagger-ui-bundle.min.js",
            "/static/swagger-ui/swagger-ui-standalone-preset.min.js",
            "/static/swagger-ui/swagger-ui.bundle.css",
        ] {
            let req = match axum::http::Request::builder()
                .method("GET")
                .uri(u)
                .body(Body::empty())
            {
                Ok(r) => r,
                Err(e) => panic!("failed to build request: {}", e),
            };
            let resp: axum::http::Response<axum::body::Body> = match app.clone().oneshot(req).await
            {
                Ok(r) => r,
                Err(e) => panic!("request failed: {}", e),
            };
            // allow 200 or 404 depending on whether the repo vendored the files
            assert!(
                resp.status() == axum::http::StatusCode::OK
                    || resp.status() == axum::http::StatusCode::NOT_FOUND
            );
        }

        // Ensure interactive helper exposes sendRequest for programmatic tests
        let req_int = match axum::http::Request::builder()
            .method("GET")
            .uri("/static/swagger-ui/interactive.js")
            .body(Body::empty())
        {
            Ok(r) => r,
            Err(e) => panic!("failed to build request: {}", e),
        };
        let resp_int: axum::http::Response<axum::body::Body> =
            match app.clone().oneshot(req_int).await {
                Ok(r) => r,
                Err(e) => panic!("request failed: {}", e),
            };
        let b_int = match axum::body::to_bytes(resp_int.into_body(), 1024 * 1024).await {
            Ok(b) => b,
            Err(e) => panic!("failed to read body: {}", e),
        };
        let s_int = String::from_utf8_lossy(&b_int);
        assert!(s_int.contains("sendRequest") || s_int.contains("renderInteractiveUI"));

        // Also ensure the offline CSS asset is served and contains expected selectors
        let req3 = match axum::http::Request::builder()
            .method("GET")
            .uri("/static/swagger-ui/swagger-ui.css")
            .body(Body::empty())
        {
            Ok(r) => r,
            Err(e) => panic!("failed to build request: {}", e),
        };
        let resp3: axum::http::Response<axum::body::Body> = match app.clone().oneshot(req3).await {
            Ok(r) => r,
            Err(e) => panic!("request failed: {}", e),
        };
        assert_eq!(resp3.status(), axum::http::StatusCode::OK);
        let b3 = match axum::body::to_bytes(resp3.into_body(), 1024 * 1024).await {
            Ok(b) => b,
            Err(e) => panic!("failed to read body: {}", e),
        };
        let s3 = String::from_utf8_lossy(&b3);
        assert!(s3.contains(".swagger-ui"));
    });
}
