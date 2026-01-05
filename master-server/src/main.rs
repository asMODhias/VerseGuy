#![allow(unused_imports)]
use master_server::build_app;
use master_server::state::AppState;
use std::sync::Arc;

#[cfg(feature = "run-server")]
use master_server::run_server;

#[cfg(not(test))]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let db_path =
        std::env::var("MASTER_DB_PATH").unwrap_or_else(|_| "./master_server_db".to_string());
    let secret = std::env::var("MASTER_LICENSE_SECRET")
        .unwrap_or_else(|_| "master-secret".to_string())
        .into_bytes();
    let state = Arc::new(AppState::new(db_path, secret)?);

    let _app = build_app(state.clone());
    tracing::info!("Master server built. To run, enable the 'run-server' feature or run via workspace run configuration.");

    // If compiled with feature 'run-server', run it by default
    #[cfg(feature = "run-server")]
    {
        let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
        run_server(state, addr).await?;
    }

    Ok(())
}
