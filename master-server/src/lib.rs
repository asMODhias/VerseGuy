use axum::{
    routing::{get, post},
    Router,
};
use std::sync::Arc;

pub mod plugins;
pub mod routes;
pub mod state;

use state::AppState;
pub mod admin_cli;
pub mod keystore;
pub mod manifest_tool;

pub fn build_app(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/auth/register", post(routes::register_handler))
        .route("/auth/login", post(routes::login_handler))
        .route("/license/validate", post(routes::license_validate_handler))
        .route("/plugins/search", get(routes::plugins_search_handler))
        .route("/plugins/publish", post(routes::plugins_publish_handler))
        .route("/admin/keys", get(routes::admin_get_keys))
        .route("/admin/keys/rotate", post(routes::admin_rotate_key))
        .route("/admin/keys/import", post(routes::admin_import_key))
        .with_state(state)
}

#[cfg(feature = "run-server")]
pub async fn run_server(state: Arc<AppState>, addr: std::net::SocketAddr) -> anyhow::Result<()> {
    let app = build_app(state);
    let listener = std::net::TcpListener::bind(addr)?;
    listener.set_nonblocking(true)?;
    let server = hyper::Server::from_tcp(listener)?.serve(app.into_make_service());

    tracing::info!("Master server running on {}", addr);
    server.await?;
    Ok(())
}
