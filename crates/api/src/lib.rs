use axum::{
    extract::{Form, Json},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
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

/// OAuth2 token endpoint (initial: client_credentials grant)
#[derive(Deserialize)]
struct TokenRequest {
    grant_type: String,
    client_id: Option<String>,
    client_secret: Option<String>,
    scope: Option<String>,
}

#[derive(Serialize)]
struct TokenResponse {
    access_token: String,
    token_type: &'static str,
    expires_in: u64,
    scope: Option<String>,
}

async fn token_handler(
    Form(req): Form<TokenRequest>,
) -> Result<Json<TokenResponse>, (StatusCode, &'static str)> {
    // Only client_credentials supported for now
    if req.grant_type != "client_credentials" {
        return Err((StatusCode::BAD_REQUEST, "unsupported grant_type"));
    }

    // Very small client store (placeholder): client_id: demo, client_secret: secret
    match (req.client_id.as_deref(), req.client_secret.as_deref()) {
        (Some("demo"), Some("secret")) => {
            let token = Uuid::new_v4().to_string();
            let resp = TokenResponse {
                access_token: token,
                token_type: "bearer",
                expires_in: 3600,
                scope: req.scope.clone(),
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

            let resp = match app.oneshot(req).await {
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
            assert_eq!(v.get("token_type").and_then(|t| t.as_str()), Some("bearer"));
        });
    }
}
