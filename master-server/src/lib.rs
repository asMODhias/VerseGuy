use axum::{
    routing::{get, post},
    Router,
};
use std::sync::Arc;

pub mod legal;
pub mod plugins;
pub mod routes;
pub mod state;
pub mod observability;

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
        .route("/v1/orgs", get(routes::orgs_list_handler).post(routes::orgs_create_handler))
        .route("/v1/orgs/{id}", get(routes::orgs_get_handler))
        .route("/admin/keys", get(routes::admin_get_keys))
        .route("/admin/keys/rotate", post(routes::admin_rotate_key))
        .route("/admin/keys/import", post(routes::admin_import_key))
        .route("/auth/tos", post(routes::tos_accept_handler))
        .route("/auth/tos/{user_id}", get(routes::tos_get_handler))
        .route("/verify/plugin", post(routes::verify_plugin_handler))
        .route("/verify/revoke", post(routes::revoke_handler))
        .route("/verify/revocations", get(routes::revocations_list_handler))
        // Observability
        .route("/healthz", get(routes::health_handler))
        .route("/metrics", get(routes::metrics_handler))
        // Legal / admin legal endpoints
        .route("/admin/legal", post(legal::admin_create_legal_handler))
        .route("/admin/legal/{id}", get(legal::admin_get_legal_handler))
        .route("/admin/legal", get(legal::admin_list_legal_handler))
        .route(
            "/admin/legal/{id}/revoke",
            post(legal::admin_revoke_legal_handler),
        )
        .route("/legal/latest/{type}", get(legal::get_latest_legal_handler))
        .route(
            "/legal/{type}/{version}",
            get(legal::get_legal_version_handler),
        )
        // GDPR / Audit endpoints
        .route("/audit/export/{user_id}", get(routes::audit_export_handler))
        .route(
            "/users/{user_id}/data",
            axum::routing::delete(routes::user_data_delete_handler),
        )
        .with_state(state)
}

#[cfg(feature = "run-server")]
pub async fn run_server(_state: Arc<AppState>, _addr: std::net::SocketAddr) -> anyhow::Result<()> {
    // The CI and tests call the Router directly using ServiceExt::oneshot to avoid
    // binding to network sockets and to prevent Hyper version conflicts during
    // test builds. If you enable `run-server`, implement a platform-specific
    // runner that uses a compatible hyper version.
    tracing::info!("run_server() is a no-op in this build configuration");
    Err(anyhow::anyhow!(
        "run-server is not implemented in this build"
    ))
}
