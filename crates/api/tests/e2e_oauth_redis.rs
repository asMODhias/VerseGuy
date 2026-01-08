use axum::body::Body;
use serde_json::Value;
use tower::util::ServiceExt;
#[test]
fn e2e_oauth_with_redis() {
    let rt = match tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
    {
        Ok(rt) => rt,
        Err(e) => panic!("failed to build runtime: {}", e),
    };

    rt.block_on(async {
        // Prepare Redis-backed store
        let url = std::env::var("VERSEGUY_API_TOKEN_STORE_URL").unwrap_or_else(|_| "redis://127.0.0.1/".to_string());
        // quick reachability check for Redis (skip if not available)
        match redis::Client::open(url.clone()) {
            Ok(client) => match client.get_connection() {
                Ok(_) => (),
                Err(e) => { eprintln!("Skipping e2e Redis test (redis connection failed): {:?}", e); return; }
            },
            Err(e) => { eprintln!("Skipping e2e Redis test (redis client open failed): {:?}", e); return; }
        }

        let rstore = match verseguy_api::store::RedisTokenStore::new(&url) {
            Ok(s) => s,
            Err(e) => { eprintln!("Skipping e2e Redis test (client init failed): {:?}", e); return; }
        };

        let store_arc: std::sync::Arc<dyn verseguy_api::store::TokenStore> = std::sync::Arc::new(rstore);

        let app = verseguy_api::build_app_with_store(store_arc.clone());

        // client_credentials flow
        let body = "grant_type=client_credentials&client_id=demo&client_secret=secret";
        let req = match axum::http::Request::builder()
            .method("POST")
            .uri("/oauth/token")
            .header("content-type", "application/x-www-form-urlencoded")
            .body(Body::from(body))
        {
            Ok(r) => r,
            Err(e) => panic!("failed to build request: {}", e),
        };

        let resp = match app.clone().oneshot(req).await {
            Ok(r) => r,
            Err(e) => panic!("request failed: {}", e),
        };
        assert_eq!(resp.status(), axum::http::StatusCode::OK);
        let bytes = match axum::body::to_bytes(resp.into_body(), 1024 * 1024).await {
            Ok(b) => b,
            Err(e) => panic!("failed to read body: {}", e),
        };
        let v: Value = match serde_json::from_slice(&bytes) {
            Ok(j) => j,
            Err(e) => panic!("invalid json response: {}", e),
        };
        let rtok = match v.get("refresh_token").and_then(|r| r.as_str()) {
            Some(s) => s.to_string(),
            None => panic!("no refresh_token in response: {:?}", v),
        };

        // Ensure refresh token persists in Redis store
        match store_arc.get(&rtok) {
            Ok(Some(rec)) => assert_eq!(rec.refresh_token, rtok),
            Ok(None) => panic!("refresh token not found in redis store"),
            Err(e) => panic!("store get failed: {:?}", e),
        }

        // refresh flow
        let body2 = format!("grant_type=refresh_token&refresh_token={}", rtok);
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

        // authorization code flow
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
                let map: std::collections::HashMap<String, String> = serde_urlencoded::from_str(q).unwrap_or_default();
                match map.get("code").cloned() {
                    Some(c) => c,
                    None => panic!("no code"),
                }
            }
            None => panic!("no query in redirect location"),
        };

        // exchange code for token
        let body3 = format!("grant_type=authorization_code&code={}&redirect_uri=https://example.com/cb&client_id=demo&client_secret=secret", code);
        let req3 = match axum::http::Request::builder()
            .method("POST")
            .uri("/oauth/token")
            .header("content-type", "application/x-www-form-urlencoded")
            .body(Body::from(body3))
        {
            Ok(r) => r,
            Err(e) => panic!("failed to build request: {}", e),
        };

        let resp3 = match app.clone().oneshot(req3).await {
            Ok(r) => r,
            Err(e) => panic!("request failed: {}", e),
        };
        assert_eq!(resp3.status(), axum::http::StatusCode::OK);

        // parse token response
        let b3 = match axum::body::to_bytes(resp3.into_body(), 1024 * 1024).await {
            Ok(b) => b,
            Err(e) => panic!("failed to read body: {}", e),
        };
        let v3: Value = match serde_json::from_slice(&b3) {
            Ok(j) => j,
            Err(e) => panic!("invalid json response: {}", e),
        };
        assert!(v3.get("access_token").is_some());
    });
}
