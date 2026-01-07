#![allow(clippy::disallowed_methods)]

use axum::{
    body::Bytes, extract::Extension, http::HeaderMap, response::IntoResponse, routing::get,
    routing::post, Router,
};
use chrono::Utc;
use serde::Serialize;
use std::fs;
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::Mutex;

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
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let captures: Arc<Mutex<Vec<Captured>>> = Arc::new(Mutex::new(Vec::new()));

    let app = Router::new()
        .route("/v1/traces", post(handle_otlp))
        .route("/dump", get(handle_dump))
        .layer(Extension(captures.clone()));

    // Allow overriding the listen port via $CAPTURE_PORT (useful to avoid conflicts with collector)
    let port: u16 = std::env::var("CAPTURE_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(4318u16);
    let addr: SocketAddr = ([127, 0, 0, 1], port).into();
    tracing::info!(
        "Starting capture server on {} (CAPTURE_PORT={})",
        addr,
        port
    );

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}

#[axum::debug_handler]
async fn handle_otlp(
    headers: HeaderMap,
    Extension(captures): Extension<Arc<Mutex<Vec<Captured>>>>,
    body: Bytes,
) -> impl IntoResponse {
    let now = Utc::now();
    let body_len = body.len();
    let head_hex = hex::encode(&body.as_ref()[..body_len.min(64)]);
    let headers_vec = headers
        .iter()
        .map(|(k, v)| {
            (
                k.to_string(),
                v.to_str().unwrap_or("(invalid utf8)").to_string(),
            )
        })
        .collect::<Vec<_>>();
    let c = Captured {
        timestamp: now.to_rfc3339(),
        method: "POST".to_string(),
        path: "/v1/traces".to_string(),
        headers: headers_vec,
        body_len,
        body_head_hex: head_hex,
    };

    tracing::info!(
        "Captured request: timestamp={} body_len={}",
        c.timestamp,
        c.body_len
    );

    // persist capture to disk for postmortem
    let mut guard = captures.lock().await;
    guard.push(c.clone());

    // write a detailed JSON file so CI can fetch it
    let dump = serde_json::to_string_pretty(&c).unwrap_or_else(|_| "{}".into());
    let ts = Utc::now();
    let fname = format!("target/otlp_capture_{}.json", ts.timestamp());
    let _ = fs::create_dir_all("target");
    let _ = fs::write(&fname, &dump);
    tracing::info!("Wrote capture to {}", fname);

    // also write the full raw body to a .bin file for exact repro (filename included in JSON)
    let bin_fname = format!("target/otlp_capture_{}.bin", ts.timestamp());
    let _ = fs::write(&bin_fname, &body);
    tracing::info!("Wrote raw body to {} ({} bytes)", bin_fname, body_len);

    // update the JSON file to include the bin filename (best-effort)
    let mut json_val: serde_json::Value =
        serde_json::from_str(&dump).unwrap_or_else(|_| serde_json::json!({}));
    if let serde_json::Value::Object(ref mut m) = json_val {
        m.insert(
            "bin_file".into(),
            serde_json::Value::String(bin_fname.clone()),
        );
        let _ = fs::write(
            &fname,
            serde_json::to_string_pretty(&json_val).unwrap_or_else(|_| "{}".into()),
        );
    }

    (axum::http::StatusCode::OK, "OK")
}

#[axum::debug_handler]
async fn handle_dump(
    Extension(captures): Extension<Arc<Mutex<Vec<Captured>>>>,
) -> impl IntoResponse {
    let guard = captures.lock().await;
    let body = serde_json::to_string_pretty(&*guard).unwrap_or_else(|_| "[]".into());
    (axum::http::StatusCode::OK, body)
}
