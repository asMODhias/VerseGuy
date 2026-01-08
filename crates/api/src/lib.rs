use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};

/// Build a minimal API router with basic endpoints.
pub fn build_app() -> Router {
    Router::new()
        .route("/health", get(health_handler))
        .route("/metrics", get(metrics_handler))
        .route("/protected", get(protected_handler))
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
            let req = axum::http::Request::builder()
                .method("GET")
                .uri("/health")
                .body(Body::empty())
                .unwrap();

            let resp = app.oneshot(req).await.unwrap();
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
            let req = axum::http::Request::builder()
                .method("GET")
                .uri("/protected")
                .body(Body::empty())
                .unwrap();

            let resp = app.oneshot(req).await.unwrap();
            assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
        });
    }
}
