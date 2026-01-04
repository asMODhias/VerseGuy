use crate::state::AppState;
use anyhow::Result;
use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use verseguy_auth::local::LocalAuth;
use verseguy_auth::session::SessionService;
use verseguy_licensing::validate_license;

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct RegisterResponse {
    pub id: String,
    pub username: String,
}

pub async fn register_handler(
    State(state): State<Arc<AppState>>,
    Json(req): Json<RegisterRequest>,
) -> Result<Json<RegisterResponse>, (axum::http::StatusCode, String)> {
    let auth = LocalAuth::new((*state.storage).clone());
    let user = auth
        .register(req.username, req.password)
        .await
        .map_err(|e| (axum::http::StatusCode::BAD_REQUEST, format!("{}", e)))?;
    Ok(Json(RegisterResponse {
        id: user.id,
        username: user.username,
    }))
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
}

pub async fn login_handler(
    State(state): State<Arc<AppState>>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, (axum::http::StatusCode, String)> {
    let auth = LocalAuth::new((*state.storage).clone());
    let user = auth
        .login(&req.username, &req.password)
        .await
        .map_err(|e| (axum::http::StatusCode::UNAUTHORIZED, format!("{}", e)))?;

    let session_service = SessionService::new(state.license_secret.clone());
    let token = session_service
        .create_and_store_session(&user.id, &user.license, 30, &state.storage)
        .map_err(|e| {
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("{}", e),
            )
        })?;

    Ok(Json(LoginResponse { token }))
}

#[derive(Deserialize)]
pub struct LicenseValidateRequest {
    pub token: String,
}

#[derive(Serialize)]
pub struct LicenseValidateResponse {
    pub valid: bool,
}

pub async fn license_validate_handler(
    State(state): State<Arc<AppState>>,
    Json(req): Json<LicenseValidateRequest>,
) -> Result<Json<LicenseValidateResponse>, (axum::http::StatusCode, String)> {
    // Use licensing validate_license
    let valid = validate_license(
        &req.token,
        &state.license_secret,
        chrono::Utc::now().timestamp(),
    )
    .map_err(|e| (axum::http::StatusCode::BAD_REQUEST, format!("{}", e)))?;
    Ok(Json(LicenseValidateResponse { valid }))
}

use crate::plugins::{search_manifests, store_manifest, PluginManifest};
use axum::extract::Query;
use base64::engine::general_purpose;
use base64::Engine;

#[derive(Deserialize)]
pub struct SearchQuery {
    pub q: Option<String>,
}

pub async fn plugins_search_handler(
    State(state): State<Arc<AppState>>,
    Query(query): Query<SearchQuery>,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, String)> {
    let q = query.q.unwrap_or_default();
    let items = search_manifests(&state.storage, &q).map_err(|e| {
        (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("{}", e),
        )
    })?;
    Ok(Json(serde_json::json!({"results": items})))
}

// --- Admin key management handlers ---

pub async fn admin_get_keys(
    State(_state): State<Arc<AppState>>,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, String)> {
    if let Ok(key_path) = std::env::var("MASTER_KEY_FILE") {
        let pk_b64 = crate::keystore::public_key_b64_from_path(std::path::Path::new(&key_path))
            .map_err(|e| {
                (
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    format!("{}", e),
                )
            })?;
        Ok(Json(
            serde_json::json!({"exists": true, "public_key_b64": pk_b64, "path": key_path}),
        ))
    } else {
        Err((
            axum::http::StatusCode::NOT_FOUND,
            "no master key configured".to_string(),
        ))
    }
}

fn require_admin(headers: &axum::http::HeaderMap) -> Result<(), (axum::http::StatusCode, String)> {
    if let Ok(token) = std::env::var("MASTER_ADMIN_TOKEN") {
        let header_token = headers
            .get("x-admin-token")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("");
        if header_token != token {
            return Err((
                axum::http::StatusCode::FORBIDDEN,
                "invalid admin token".to_string(),
            ));
        }
        Ok(())
    } else {
        Err((
            axum::http::StatusCode::FORBIDDEN,
            "admin disabled".to_string(),
        ))
    }
}

pub async fn admin_rotate_key(
    State(_state): State<Arc<AppState>>,
    req: axum::http::Request<axum::body::Body>,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, String)> {
    let headers = req.headers();
    require_admin(headers)?;

    let key_path = std::env::var("MASTER_KEY_FILE").map_err(|_| {
        (
            axum::http::StatusCode::NOT_FOUND,
            "no master key configured".to_string(),
        )
    })?;
    let kp = crate::keystore::rotate_key(std::path::Path::new(&key_path)).map_err(|e| {
        (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("{}", e),
        )
    })?;
    let pk_b64 = general_purpose::STANDARD.encode(kp.public.to_bytes());
    Ok(Json(
        serde_json::json!({"ok": true, "public_key_b64": pk_b64}),
    ))
}

#[derive(Deserialize)]
pub struct ImportBody {
    pub key_b64: String,
}

pub async fn admin_import_key(
    State(_state): State<Arc<AppState>>,
    req: axum::http::Request<axum::body::Body>,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, String)> {
    let headers = req.headers();
    require_admin(headers)?;

    // read body
    let bytes = axum::body::to_bytes(req.into_body(), 1024 * 1024)
        .await
        .map_err(|e| {
            (
                axum::http::StatusCode::BAD_REQUEST,
                format!("failed to read body: {}", e),
            )
        })?;
    let b: ImportBody = serde_json::from_slice(&bytes).map_err(|e| {
        (
            axum::http::StatusCode::BAD_REQUEST,
            format!("invalid json: {}", e),
        )
    })?;

    let key_path = std::env::var("MASTER_KEY_FILE").map_err(|_| {
        (
            axum::http::StatusCode::NOT_FOUND,
            "no master key configured".to_string(),
        )
    })?;
    let kp = crate::keystore::import_key_b64(std::path::Path::new(&key_path), &b.key_b64)
        .map_err(|e| (axum::http::StatusCode::BAD_REQUEST, format!("{}", e)))?;
    let pk_b64 = general_purpose::STANDARD.encode(kp.public.to_bytes());
    Ok(Json(
        serde_json::json!({"ok": true, "public_key_b64": pk_b64}),
    ))
}

#[derive(Deserialize)]
pub struct PublishRequest {
    pub manifest: PluginManifest,
}

pub async fn plugins_publish_handler(
    State(state): State<Arc<AppState>>,
    req: axum::http::Request<axum::body::Body>,
) -> Result<(axum::http::StatusCode, Json<serde_json::Value>), (axum::http::StatusCode, String)> {
    // Simple publisher auth: if MASTER_PLUGIN_PUBLISH_KEY is set, require X-Plugin-Token header to match
    if let Ok(key) = std::env::var("MASTER_PLUGIN_PUBLISH_KEY") {
        let header_token = req
            .headers()
            .get("x-plugin-token")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("");
        if header_token != key {
            return Err((
                axum::http::StatusCode::FORBIDDEN,
                "Invalid plugin publish token".to_string(),
            ));
        }
    }

    // Parse JSON body into PublishRequest
    let bytes = axum::body::to_bytes(req.into_body(), 1024 * 1024)
        .await
        .map_err(|e| {
            (
                axum::http::StatusCode::BAD_REQUEST,
                format!("failed to read body: {}", e),
            )
        })?;
    let req_json: PublishRequest = serde_json::from_slice(&bytes).map_err(|e| {
        (
            axum::http::StatusCode::BAD_REQUEST,
            format!("invalid json: {}", e),
        )
    })?;

    let manifest = req_json.manifest.with_published();

    // Sign the manifest with master server keypair if available
    let kp_opt = state.keypair.as_ref();
    store_manifest(&state.storage, &manifest, kp_opt).map_err(|e| {
        (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("{}", e),
        )
    })?;
    Ok((
        axum::http::StatusCode::CREATED,
        Json(serde_json::json!({"ok": true, "manifest": manifest})),
    ))
}
