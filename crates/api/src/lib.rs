use axum::{
    extract::Json,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use serde::Serialize;
use uuid::Uuid;

/// Build a minimal API router with basic endpoints.
pub fn build_app() -> Router {
    Router::new()
        .route("/health", get(health_handler))
        .route("/metrics", get(metrics_handler))
        .route("/protected", get(protected_handler))
        .route("/oauth/token", post(token_handler))
        .route("/oauth/authorize", get(authorize_handler))
}

async fn health_handler() -> impl IntoResponse {
    (StatusCode::OK, "ok")
}

async fn metrics_handler() -> impl IntoResponse {
    // placeholder; in real implementation this would return metrics
    (StatusCode::OK, "metrics: {}")
}

/// Minimal protected endpoint: requires header `x-api-key: secret` (placeholder auth)
async fn protected_handler(req: axum::http::Request<axum::body::Body>) -> impl IntoResponse {
    if let Some(v) = req.headers().get("x-api-key") {
        // simple placeholder check: accept any non-empty header value
        if !v.as_bytes().is_empty() {
            return (StatusCode::OK, "authorized");
        }
    }

    (StatusCode::UNAUTHORIZED, "unauthorized")
}

use chrono::{DateTime, Utc};
/// OAuth2 token endpoint (initial: client_credentials grant)
use std::collections::HashMap;

#[derive(Serialize)]
struct TokenResponse {
    access_token: String,
    token_type: &'static str,
    expires_in: u64,
    refresh_token: Option<String>,
    scope: Option<String>,
}

pub mod store;
use crate::store::{TokenRecord, TOKEN_STORE};
use once_cell::sync::Lazy;
use std::sync::Mutex;

struct CodeRecord {
    client_id: String,
    redirect_uri: String,
    expires_at: DateTime<Utc>,
}

static CODE_STORE: Lazy<Mutex<HashMap<String, CodeRecord>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

async fn authorize_handler(req: axum::http::Request<axum::body::Body>) -> impl IntoResponse {
    // parse query params
    let q = req.uri().query().unwrap_or("");
    let params: HashMap<String, String> = match serde_urlencoded::from_str(q) {
        Ok(m) => m,
        Err(_) => return (StatusCode::BAD_REQUEST, "invalid_query").into_response(),
    };

    // require response_type=code
    if params.get("response_type").map(|s| s.as_str()) != Some("code") {
        return (StatusCode::BAD_REQUEST, "unsupported response_type").into_response();
    }

    // validate client_id and redirect_uri (simplified)
    let client = params.get("client_id").map(|s| s.as_str()).unwrap_or("");
    let redirect = match params.get("redirect_uri") {
        Some(r) => r.clone(),
        None => return (StatusCode::BAD_REQUEST, "missing redirect_uri").into_response(),
    };
    if client != "demo" {
        return (StatusCode::BAD_REQUEST, "unknown client").into_response();
    }

    // generate code and store it
    let code = Uuid::new_v4().to_string();
    let expires_at = Utc::now() + chrono::Duration::seconds(300);
    let crec = CodeRecord {
        client_id: client.to_string(),
        redirect_uri: redirect.clone(),
        expires_at,
    };
    match CODE_STORE.lock() {
        Ok(mut store) => {
            store.insert(code.clone(), crec);
        }
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "lock_error").into_response(),
    }

    // build redirect URL: redirect_uri?code=...&state=... (if provided)
    let mut location = format!("{}?code={}", redirect, code);
    if let Some(state) = params.get("state") {
        location = format!("{}&state={}", location, state);
    }

    // redirect
    let res = match axum::http::Response::builder()
        .status(StatusCode::FOUND)
        .header(axum::http::header::LOCATION, location)
        .body(axum::body::Body::empty())
    {
        Ok(r) => r,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "build_error").into_response(),
    };
    res.into_response()
}

async fn token_handler(
    req: axum::http::Request<axum::body::Body>,
) -> Result<Json<TokenResponse>, (StatusCode, &'static str)> {
    // Read body and parse urlencoded form
    let bytes = match axum::body::to_bytes(req.into_body(), 1024 * 1024).await {
        Ok(b) => b,
        Err(_) => return Err((StatusCode::BAD_REQUEST, "invalid_body")),
    };
    let params: HashMap<String, String> = match serde_urlencoded::from_bytes(&bytes) {
        Ok(m) => m,
        Err(_) => return Err((StatusCode::BAD_REQUEST, "invalid_form")),
    };

    let grant = params.get("grant_type").map(|s| s.as_str()).unwrap_or("");

    // Handle refresh_token grant
    if grant == "refresh_token" {
        if let Some(rtok) = params.get("refresh_token") {
            match TOKEN_STORE.get(rtok) {
                Ok(Some(rec)) => {
                    // check expiry
                    if rec.expires_at < Utc::now() {
                        return Err((StatusCode::UNAUTHORIZED, "expired_refresh_token"));
                    }

                    let new_access = Uuid::new_v4().to_string();
                    let new_rec = TokenRecord {
                        access_token: new_access.clone(),
                        refresh_token: rec.refresh_token.clone(),
                        expires_at: Utc::now() + chrono::Duration::seconds(3600),
                    };
                    if TOKEN_STORE.insert(rtok.to_string(), new_rec).is_err() {
                        return Err((StatusCode::INTERNAL_SERVER_ERROR, "store_error"));
                    }

                    let resp = TokenResponse {
                        access_token: new_access,
                        token_type: "bearer",
                        expires_in: 3600,
                        refresh_token: Some(rec.refresh_token.clone()),
                        scope: None,
                    };
                    return Ok(Json(resp));
                }
                Ok(None) => return Err((StatusCode::UNAUTHORIZED, "invalid_refresh_token")),
                Err(_) => return Err((StatusCode::INTERNAL_SERVER_ERROR, "store_error")),
            }
        }
        return Err((StatusCode::UNAUTHORIZED, "invalid_refresh_token"));
    }

    // Authorization Code grant
    if grant == "authorization_code" {
        if let Some(code) = params.get("code") {
            let mut store = match CODE_STORE.lock() {
                Ok(s) => s,
                Err(_) => return Err((StatusCode::INTERNAL_SERVER_ERROR, "lock_error")),
            };
            if let Some(crec) = store.remove(code) {
                // check expiry
                if crec.expires_at < Utc::now() {
                    return Err((StatusCode::UNAUTHORIZED, "expired_code"));
                }
                // verify client and redirect_uri
                let client_ok =
                    params.get("client_id").map(|s| s.as_str()) == Some(crec.client_id.as_str());
                let redirect_ok = params.get("redirect_uri").map(|s| s.as_str())
                    == Some(crec.redirect_uri.as_str());
                if !client_ok || !redirect_ok {
                    return Err((StatusCode::BAD_REQUEST, "invalid_request"));
                }
                // issue tokens
                let access_token = Uuid::new_v4().to_string();
                let refresh_token = Uuid::new_v4().to_string();
                let expires_at = Utc::now() + chrono::Duration::seconds(3600);
                let trec = TokenRecord {
                    access_token: access_token.clone(),
                    refresh_token: refresh_token.clone(),
                    expires_at,
                };
                if TOKEN_STORE.insert(refresh_token.clone(), trec).is_err() {
                    return Err((StatusCode::INTERNAL_SERVER_ERROR, "store_error"));
                }
                let resp = TokenResponse {
                    access_token,
                    token_type: "bearer",
                    expires_in: 3600,
                    refresh_token: Some(refresh_token),
                    scope: None,
                };
                return Ok(Json(resp));
            }
        }
        return Err((StatusCode::BAD_REQUEST, "invalid_code"));
    }

    // client_credentials
    if grant != "client_credentials" {
        return Err((StatusCode::BAD_REQUEST, "unsupported grant_type"));
    }

    let client_id = params.get("client_id").map(|s| s.as_str());
    let client_secret = params.get("client_secret").map(|s| s.as_str());

    match (client_id, client_secret) {
        (Some("demo"), Some("secret")) => {
            let access_token = Uuid::new_v4().to_string();
            let refresh_token = Uuid::new_v4().to_string();
            let expires_at = Utc::now() + chrono::Duration::seconds(3600);

            // Store record
            let rec = TokenRecord {
                access_token: access_token.clone(),
                refresh_token: refresh_token.clone(),
                expires_at,
            };
            if TOKEN_STORE.insert(refresh_token.clone(), rec).is_err() {
                return Err((StatusCode::INTERNAL_SERVER_ERROR, "store_error"));
            }

            let resp = TokenResponse {
                access_token,
                token_type: "bearer",
                expires_in: 3600,
                refresh_token: Some(refresh_token),
                scope: params.get("scope").cloned(),
            };
            Ok(Json(resp))
        }
        _ => Err((StatusCode::UNAUTHORIZED, "invalid_client")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use tower::util::ServiceExt;

    #[test]
    fn health_endpoint_returns_ok() {
        let rt = match tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
        {
            Ok(rt) => rt,
            Err(e) => panic!("failed to build runtime: {}", e),
        };

        rt.block_on(async {
            let app = build_app();
            let req = match axum::http::Request::builder()
                .method("GET")
                .uri("/health")
                .body(Body::empty())
            {
                Ok(r) => r,
                Err(e) => panic!("failed to build request: {}", e),
            };

            let resp = match app.oneshot(req).await {
                Ok(r) => r,
                Err(e) => panic!("request failed: {}", e),
            };
            assert_eq!(resp.status(), StatusCode::OK);
        });
    }

    #[test]
    fn protected_endpoint_requires_auth() {
        let rt = match tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
        {
            Ok(rt) => rt,
            Err(e) => panic!("failed to build runtime: {}", e),
        };

        rt.block_on(async {
            let app = build_app();
            let req = match axum::http::Request::builder()
                .method("GET")
                .uri("/protected")
                .body(Body::empty())
            {
                Ok(r) => r,
                Err(e) => panic!("failed to build request: {}", e),
            };

            let resp = match app.oneshot(req).await {
                Ok(r) => r,
                Err(e) => panic!("request failed: {}", e),
            };
            assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
        });
    }

    #[test]
    fn token_endpoint_client_credentials() {
        let rt = match tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
        {
            Ok(rt) => rt,
            Err(e) => panic!("failed to build runtime: {}", e),
        };

        rt.block_on(async {
            let app = build_app();
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
            assert_eq!(resp.status(), StatusCode::OK);
            let bytes = match axum::body::to_bytes(resp.into_body(), 1024 * 1024).await {
                Ok(b) => b,
                Err(e) => panic!("failed to read body: {}", e),
            };
            let v: serde_json::Value = match serde_json::from_slice(&bytes) {
                Ok(j) => j,
                Err(e) => panic!("invalid json response: {}", e),
            };
            assert!(v.get("access_token").is_some());
            assert!(v.get("refresh_token").is_some());
            assert_eq!(v.get("token_type").and_then(|t| t.as_str()), Some("bearer"));

            // Now attempt refresh
            let rtok = match v.get("refresh_token").and_then(|r| r.as_str()) {
                Some(s) => s.to_string(),
                None => panic!("no refresh_token in response: {:?}", v),
            };

            // perform refresh token exchange
            let body_refresh = format!("grant_type=refresh_token&refresh_token={}", rtok);
            let req_refresh = match axum::http::Request::builder()
                .method("POST")
                .uri("/oauth/token")
                .header("content-type", "application/x-www-form-urlencoded")
                .body(Body::from(body_refresh))
            {
                Ok(r) => r,
                Err(e) => panic!("failed to build request: {}", e),
            };

            let resp_refresh = match app.clone().oneshot(req_refresh).await {
                Ok(r) => r,
                Err(e) => panic!("refresh request failed: {}", e),
            };
            assert_eq!(resp_refresh.status(), StatusCode::OK);

            // Now test authorization code flow: request code via /oauth/authorize
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

            // exchange code for token
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
            if resp2.status() != StatusCode::OK {
                let status = resp2.status();
                let b = axum::body::to_bytes(resp2.into_body(), 1024 * 1024).await;
                let bytes2 = match b {
                    Ok(b) => b,
                    Err(e) => panic!("failed to read body: {}", e),
                };
                panic!(
                    "auth-code token exchange failed: {} - {}",
                    status,
                    String::from_utf8_lossy(&bytes2)
                );
            }
            let bytes2 = match axum::body::to_bytes(resp2.into_body(), 1024 * 1024).await {
                Ok(b) => b,
                Err(e) => panic!("failed to read body: {}", e),
            };
            let v2: serde_json::Value = match serde_json::from_slice(&bytes2) {
                Ok(j) => j,
                Err(e) => panic!("invalid json response: {}", e),
            };
            assert!(v2.get("access_token").is_some());
        });
    }
}
