use axum::body::Body;
use serde_yaml::Value;
use tower::util::ServiceExt;

#[test]
fn openapi_contains_token_response_schema_and_examples() {
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

        let resp = match app.clone().oneshot(req).await {
            Ok(r) => r,
            Err(e) => panic!("request failed: {}", e),
        };

        assert_eq!(resp.status(), axum::http::StatusCode::OK);

        let bytes = match axum::body::to_bytes(resp.into_body(), 1024 * 1024).await {
            Ok(b) => b,
            Err(e) => panic!("failed to read body: {}", e),
        };

        let s = String::from_utf8_lossy(&bytes);

        let doc: Value = match serde_yaml::from_str(&s) {
            Ok(v) => v,
            Err(e) => panic!("failed to parse openapi.yaml as YAML: {}", e),
        };

        let token_schema_opt = doc
            .get("components")
            .and_then(|c| c.get("schemas"))
            .and_then(|s| s.get("TokenResponse"));
        let token_schema = match token_schema_opt {
            Some(s) => s,
            None => panic!("TokenResponse schema missing"),
        };

        let required_opt = token_schema.get("required").and_then(|r| r.as_sequence());
        let required = match required_opt {
            Some(r) => r,
            None => panic!("required missing or not a sequence"),
        };

        let reqs: Vec<String> = required
            .iter()
            .filter_map(|v| v.as_str().map(|s| s.to_string()))
            .collect();

        assert!(reqs.contains(&"access_token".to_string()));
        assert!(reqs.contains(&"token_type".to_string()));

        let paths_opt = doc.get("paths").and_then(|p| p.get("/oauth/token"));
        let paths = match paths_opt {
            Some(p) => p,
            None => panic!("/oauth/token missing"),
        };

        let responses_opt = paths.get("post").and_then(|p| p.get("responses"));
        let responses = match responses_opt {
            Some(r) => r,
            None => panic!("responses missing"),
        };

        let resp200 = match responses.get("200") {
            Some(r) => r,
            None => panic!("200 response missing"),
        };
        let appjson_opt = resp200.get("content").and_then(|c| c.get("application/json"));
        let appjson = match appjson_opt {
            Some(a) => a,
            None => panic!("application/json content missing"),
        };
        let examples = match appjson.get("examples") {
            Some(e) => e,
            None => panic!("examples missing"),
        };

        assert!(
            examples.get("client_credentials").is_some(),
            "client_credentials example missing"
        );
        assert!(
            examples.get("refresh_token").is_some(),
            "refresh_token example missing"
        );
        assert!(
            examples.get("authorization_code").is_some(),
            "authorization_code example missing"
        );
    });
}
