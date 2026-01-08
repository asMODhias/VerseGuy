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
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;

#[derive(Serialize)]
struct TokenResponse {
    access_token: String,
    token_type: &'static str,
    expires_in: u64,
    refresh_token: Option<String>,
    scope: Option<String>,
}

struct TokenRecord {
    access_token: String,
    refresh_token: String,
    expires_at: DateTime<Utc>,
}

static TOKEN_STORE: Lazy<Mutex<HashMap<String, TokenRecord>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

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
            let mut store = match TOKEN_STORE.lock() {
                Ok(s) => s,
                Err(_) => return Err((StatusCode::INTERNAL_SERVER_ERROR, "lock_error")),
            };
            if let Some(rec) = store.get_mut(rtok) {
                // check expiry
                if rec.expires_at < Utc::now() {
                    return Err((StatusCode::UNAUTHORIZED, "expired_refresh_token"));
                }

                let new_access = Uuid::new_v4().to_string();
                rec.access_token = new_access.clone();
                rec.expires_at = Utc::now() + chrono::Duration::seconds(3600);

                let resp = TokenResponse {
                    access_token: new_access,
                    token_type: "bearer",
                    expires_in: 3600,
                    refresh_token: Some(rec.refresh_token.clone()),
                    scope: None,
                };
                return Ok(Json(resp));
            }
        }
        return Err((StatusCode::UNAUTHORIZED, "invalid_refresh_token"));
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
            match TOKEN_STORE.lock() {
                Ok(mut store) => {
                    store.insert(refresh_token.clone(), rec);
                }
                Err(_) => return Err((StatusCode::INTERNAL_SERVER_ERROR, "lock_error")),
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
            if resp2.status() != StatusCode::OK {
                let status = resp2.status();
                let b = axum::body::to_bytes(resp2.into_body(), 1024 * 1024).await;
                let bytes2 = match b {
                    Ok(b) => b,
                    Err(e) => panic!("failed to read body: {}", e),
                };
                panic!(
                    "refresh request failed: {} - {}",
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
