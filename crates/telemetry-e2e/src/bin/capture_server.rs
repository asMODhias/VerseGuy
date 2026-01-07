use std::{net::SocketAddr, sync::Arc};
use axum::{body::Bytes, extract::Extension, http::HeaderMap, response::IntoResponse, routing::post, routing::get, Router};
use chrono::Utc;
use tokio::sync::Mutex;
use serde::Serialize;
use std::fs;

#[derive(Clone, Debug, Serialize)]
struct Captured {
    timestamp: String,
    method: String,
    path: String,
    headers: Vec<(String, String)>,
    body_len: usize,
    body_head_hex: String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let captures: Arc<Mutex<Vec<Captured>>> = Arc::new(Mutex::new(Vec::new()));

    let app = Router::new()
        .route("/v1/traces", post(handle_otlp))
        .route("/dump", get(handle_dump))
        .layer(Extension(captures.clone()));

    let addr: SocketAddr = ([127, 0, 0, 1], 4318).into();
    tracing::info!("Starting capture server on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handle_otlp(
    headers: HeaderMap,
    body: Bytes,
    Extension(captures): Extension<Arc<Mutex<Vec<Captured>>>>,
) -> impl IntoResponse {
    let now = Utc::now();
    let body_len = body.len();
    let head_hex = hex::encode(&body.as_ref()[..body_len.min(64)]);
    let headers_vec = headers
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("(invalid utf8)").to_string()))
        .collect::<Vec<_>>();
    let c = Captured {
        timestamp: now.to_rfc3339(),
        method: "POST".to_string(),
        path: "/v1/traces".to_string(),
        headers: headers_vec,
        body_len,
        body_head_hex: head_hex,
    };

    tracing::info!("Captured request: timestamp=%s body_len=%d", c.timestamp, c.body_len);

    // persist capture to disk for postmortem
    if let Ok(mut guard) = captures.lock().await.try_lock() {
        guard.push(c.clone());
    } else {
        // still push via awaiting lock
        let mut guard = captures.lock().await;
        guard.push(c.clone());
    }

    // write a detailed file so CI can fetch it
    let dump = serde_json::to_string_pretty(&c).unwrap_or_else(|_| "{}".into());
    let fname = format!("target/otlp_capture_{}.json", now.timestamp());
    let _ = fs::create_dir_all("target");
    let _ = fs::write(&fname, dump);
    tracing::info!("Wrote capture to {}", fname);

    (axum::http::StatusCode::OK, "OK")
}

async fn handle_dump(Extension(captures): Extension<Arc<Mutex<Vec<Captured>>>>) -> impl IntoResponse {
    let guard = captures.lock().await;
    let body = serde_json::to_string_pretty(&*guard).unwrap_or_else(|_| "[]".into());
    (axum::http::StatusCode::OK, body)
}
