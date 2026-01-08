use axum::body::Body;
use std::collections::HashMap;
use tower::util::ServiceExt;

#[test]
fn e2e_authorization_code_simulated_flow() {
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

        // Step 1: request authorization code
        let auth_uri = "/oauth/authorize?response_type=code&client_id=demo&redirect_uri=https://example.com/cb&state=xyz";
        let req_auth = match axum::http::Request::builder()
            .method("GET")
            .uri(auth_uri)
            .body(Body::empty())
        {
            Ok(r) => r,
            Err(e) => panic!("failed to build request: {}", e),
        };

        let resp_auth = match app.clone().oneshot(req_auth).await {
            Ok(r) => r,
            Err(e) => panic!("authorize request failed: {}", e),
        };
        assert!(resp_auth.status().is_redirection());
        let loc = match resp_auth.headers().get(axum::http::header::LOCATION) {
            Some(h) => match h.to_str() { Ok(s)=>s.to_string(), Err(_) => panic!("invalid location header") },
            None => panic!("missing location header"),
        };
        // extract code from location query
        let code = match loc.split('?').nth(1) {
            Some(q) => {
                let map: HashMap<String, String> = serde_urlencoded::from_str(q).unwrap_or_default();
                match map.get("code").cloned() {
                    Some(c) => c,
                    None => panic!("no code"),
                }
            }
            None => panic!("no query in redirect location"),
        };

        // Step 2: exchange code for token
        let body2 = format!("grant_type=authorization_code&code={}&redirect_uri=https://example.com/cb&client_id=demo&client_secret=secret", code);
        let req2 = match axum::http::Request::builder()
            .method("POST")
            .uri("/oauth/token")
            .header("content-type", "application/x-www-form-urlencoded")
            .body(Body::from(body2))
        {
            Ok(r) => r,
            Err(e) => panic!("failed to build request: {}", e),
        };

        let resp2 = match app.clone().oneshot(req2).await {
            Ok(r) => r,
            Err(e) => panic!("request failed: {}", e),
        };
        assert_eq!(resp2.status(), axum::http::StatusCode::OK);
        let bytes2 = match axum::body::to_bytes(resp2.into_body(), 1024 * 1024).await {
            Ok(b) => b,
            Err(e) => panic!("failed to read body: {}", e),
        };
        let v2: serde_json::Value = match serde_json::from_slice(&bytes2) {
            Ok(j) => j,
            Err(e) => panic!("invalid json response: {}", e),
        };
        let access = match v2.get("access_token").and_then(|a| a.as_str()).map(|s| s.to_string()) {
            Some(s) => s,
            None => panic!("access_token missing"),
        };

        // Step 3: access protected resource with Authorization header
        let req_prot = match axum::http::Request::builder()
            .method("GET")
            .uri("/protected")
            .header("Authorization", format!("Bearer {}", access).as_str())
            .body(Body::empty())
        {
            Ok(r) => r,
            Err(e) => panic!("failed to build request: {}", e),
        };

        let resp_prot = match app.clone().oneshot(req_prot).await {
            Ok(r) => r,
            Err(e) => panic!("request failed: {}", e),
        };
        assert_eq!(resp_prot.status(), axum::http::StatusCode::OK);
    });
}
