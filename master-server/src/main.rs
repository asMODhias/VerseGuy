#![allow(unused_imports)]
use master_server::build_app;
use master_server::state::AppState;
use std::sync::Arc;

#[cfg(feature = "run-server")]
use master_server::run_server;

#[cfg(not(test))]
#[allow(clippy::disallowed_methods)]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let db_path =
        std::env::var("MASTER_DB_PATH").unwrap_or_else(|_| "./master_server_db".to_string());
    let secret = std::env::var("MASTER_LICENSE_SECRET")
        .unwrap_or_else(|_| "master-secret".to_string())
        .into_bytes();
    // Initialize observability (metrics + tracing)
    let mut state = AppState::new(db_path, secret)?;
    // install metrics recorder and expose handle
    let metrics_handle = match master_server::observability::init_observability() {
        Ok(h) => Some(h),
        Err(e) => {
            tracing::error!("Failed to initialize observability: {}", e);
            None
        }
    };
    state.metrics_handle = metrics_handle;

    let state = Arc::new(state);
    let _app = build_app(state.clone());
    tracing::info!("Master server built. To run, enable the 'run-server' feature or run via workspace run configuration.");

    // If compiled with feature 'run-server', run it by default
    #[cfg(feature = "run-server")]
    {
        // Allow overriding default port via MASTER_SERVER_PORT env var for E2E tests and flexibility
        let port: u16 = std::env::var("MASTER_SERVER_PORT")
            .ok()
            .and_then(|s| s.parse::<u16>().ok())
            .unwrap_or(3000);
        let addr = std::net::SocketAddr::from(([127, 0, 0, 1], port));
        run_server(state, addr).await?;
    }

    Ok(())
}
