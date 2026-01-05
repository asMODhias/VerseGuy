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

#[derive(Deserialize)]
pub struct TosAcceptRequest {
    pub user_id: String,
    pub accepted_at: i64,
    pub version: String,
}

#[derive(Serialize)]
pub struct TosAcceptResponse {
    pub ok: bool,
}

pub async fn tos_accept_handler(
    State(state): State<Arc<AppState>>,
    Json(req): Json<TosAcceptRequest>,
) -> Result<Json<TosAcceptResponse>, (axum::http::StatusCode, String)> {
    // validate using compliance crate
    let payload = serde_json::json!({"user_id": req.user_id, "accepted_at": req.accepted_at, "version": req.version});
    let s = payload.to_string();
    let t = verseguy_compliance::validate_tos_acceptance(&s)
        .map_err(|e| (axum::http::StatusCode::BAD_REQUEST, format!("{}", e)))?;

    // store in storage under key `tos:{user_id}:{version}` and `tos:latest:{user_id}`
    let key = format!("tos:{}:{}", t.user_id, t.version);
    state.storage.put(key.as_bytes(), &t).map_err(|e| {
        (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("{}", e),
        )
    })?;

    let latest_key = format!("tos:latest:{}", t.user_id);
    state.storage.put(latest_key.as_bytes(), &t).map_err(|e| {
        (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("{}", e),
        )
    })?;

    Ok(Json(TosAcceptResponse { ok: true }))
}

use axum::extract::Path;

pub async fn tos_get_handler(
    State(state): State<Arc<AppState>>,
    Path(user_id): Path<String>,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, String)> {
    let latest_key = format!("tos:latest:{}", user_id);
    let rec: Option<verseguy_compliance::tos_validator::TosAcceptance> =
        state.storage.get(latest_key.as_bytes()).map_err(|e| {
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("{}", e),
            )
        })?;
    if let Some(r) = rec {
        Ok(Json(
            serde_json::json!({"user_id": r.user_id, "accepted_at": r.accepted_at, "version": r.version}),
        ))
    } else {
        Err((
            axum::http::StatusCode::NOT_FOUND,
            "tos acceptance not found".to_string(),
        ))
    }
}

use crate::plugins::{
    revoke_manifest, search_manifests, store_manifest, verify_manifest, PluginManifest,
};
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
    tracing::info!("require_admin called");
    if let Ok(token) = std::env::var("MASTER_ADMIN_TOKEN") {
        let header_token = headers
            .get("x-admin-token")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("");
        tracing::info!(
            "admin header present: {}",
            if header_token.is_empty() { "no" } else { "yes" }
        );
        if header_token != token {
            tracing::warn!("invalid admin token");
            return Err((
                axum::http::StatusCode::FORBIDDEN,
                "invalid admin token".to_string(),
            ));
        }
        Ok(())
    } else {
        tracing::warn!("admin endpoints disabled (MASTER_ADMIN_TOKEN not set)");
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
    tracing::info!("admin_rotate_key called");
    let headers = req.headers();
    require_admin(headers)?;

    let key_path = std::env::var("MASTER_KEY_FILE").map_err(|_| {
        (
            axum::http::StatusCode::NOT_FOUND,
            "no master key configured".to_string(),
        )
    })?;
    tracing::info!("rotating key at path: {}", key_path);
    let kp = crate::keystore::rotate_key(std::path::Path::new(&key_path)).map_err(|e| {
        (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("{}", e),
        )
    })?;
    let pk_b64 = general_purpose::STANDARD.encode(kp.public.to_bytes());
    tracing::info!("rotation complete");
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
    tracing::info!("admin_import_key called");
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
    tracing::info!("importing key into path: {}", key_path);
    let kp = crate::keystore::import_key_b64(std::path::Path::new(&key_path), &b.key_b64)
        .map_err(|e| (axum::http::StatusCode::BAD_REQUEST, format!("{}", e)))?;
    let pk_b64 = general_purpose::STANDARD.encode(kp.public.to_bytes());
    tracing::info!("import complete");
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
    tracing::info!("plugins_publish_handler called");
    // If request contains x-user-id, ensure that user accepted latest ToS
    if let Some(uid_hdr) = req.headers().get("x-user-id") {
        if let Ok(uid) = uid_hdr.to_str() {
            tracing::info!("publish request by user id present: {}", uid);
            let accepted = crate::legal::user_has_accepted_latest(&state, uid).map_err(|e| {
                (
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    format!("tos check failed: {}", e),
                )
            })?;
            if !accepted {
                tracing::warn!("user {} must accept latest ToS", uid);
                return Err((
                    axum::http::StatusCode::FORBIDDEN,
                    "user must accept latest ToS".to_string(),
                ));
            }
        }
    }

    // Simple publisher auth: if MASTER_PLUGIN_PUBLISH_KEY is set, require X-Plugin-Token header to match
    if let Ok(key) = std::env::var("MASTER_PLUGIN_PUBLISH_KEY") {
        let header_token = req
            .headers()
            .get("x-plugin-token")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("");
        tracing::info!(
            "plugin publish token header present: {}",
            if header_token.is_empty() { "no" } else { "yes" }
        );
        if header_token != key {
            tracing::warn!("Invalid plugin publish token provided");
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
    tracing::info!("Publishing manifest {}:{}", manifest.id, manifest.version);

    // Sign the manifest with master server keypair if available
    let kp_opt = state.keypair.as_ref();
    if kp_opt.is_some() {
        tracing::info!("Signing manifest with master keypair");
    } else {
        tracing::warn!("No master keypair configured; storing unsigned manifest");
    }
    store_manifest(&state.storage, &manifest, kp_opt).map_err(|e| {
        (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("{}", e),
        )
    })?;
    tracing::info!("Manifest stored");
    Ok((
        axum::http::StatusCode::CREATED,
        Json(serde_json::json!({"ok": true, "manifest": manifest})),
    ))
}

#[derive(Deserialize)]
pub struct VerifyPluginRequest {
    pub manifest: PluginManifest,
    pub public_key_b64: String,
}

#[derive(Serialize)]
pub struct VerifyPluginResponse {
    pub valid: bool,
    pub revoked: bool,
}

pub async fn verify_plugin_handler(
    State(state): State<Arc<AppState>>,
    Json(req): Json<VerifyPluginRequest>,
) -> Result<Json<VerifyPluginResponse>, (axum::http::StatusCode, String)> {
    // parse public key
    let pub_bytes = general_purpose::STANDARD
        .decode(req.public_key_b64.trim())
        .map_err(|e| {
            (
                axum::http::StatusCode::BAD_REQUEST,
                format!("invalid pubkey: {}", e),
            )
        })?;
    let pubk = ed25519_dalek::PublicKey::from_bytes(&pub_bytes).map_err(|e| {
        (
            axum::http::StatusCode::BAD_REQUEST,
            format!("invalid pubkey: {}", e),
        )
    })?;

    let valid = verify_manifest(&state.storage, &req.manifest, &pubk).map_err(|e| {
        (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("{}", e),
        )
    })?;

    let revoked =
        crate::plugins::is_revoked(&state.storage, &req.manifest.id, &req.manifest.version)
            .map_err(|e| {
                (
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    format!("{}", e),
                )
            })?;

    Ok(Json(VerifyPluginResponse { valid, revoked }))
}

#[derive(Deserialize)]
pub struct RevokeRequest {
    pub id: String,
    pub version: String,
    pub reason: String,
}

pub async fn revoke_handler(
    State(state): State<Arc<AppState>>,
    req: axum::http::Request<axum::body::Body>,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, String)> {
    let headers = req.headers();
    require_admin(headers)?;

    let bytes = axum::body::to_bytes(req.into_body(), 1024 * 1024)
        .await
        .map_err(|e| {
            (
                axum::http::StatusCode::BAD_REQUEST,
                format!("failed to read body: {}", e),
            )
        })?;
    let r: RevokeRequest = serde_json::from_slice(&bytes).map_err(|e| {
        (
            axum::http::StatusCode::BAD_REQUEST,
            format!("invalid json: {}", e),
        )
    })?;

    revoke_manifest(&state.storage, &r.id, &r.version, &r.reason).map_err(|e| {
        (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("{}", e),
        )
    })?;

    Ok(Json(serde_json::json!({"ok": true})))
}

pub async fn revocations_list_handler(
    State(state): State<Arc<AppState>>,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, String)> {
    let items: Vec<serde_json::Value> =
        state.storage.prefix_scan(b"plugin_revoked:").map_err(|e| {
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("{}", e),
            )
        })?;
    Ok(Json(serde_json::json!({"revocations": items})))
}

// GET /audit/export/{user_id}
pub async fn audit_export_handler(
    State(state): State<Arc<AppState>>,
    axum::extract::Path(user_id): axum::extract::Path<String>,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, String)> {
    // Use AuditService from containers/audit
    let audit_service = verseguy_audit::AuditService::new(state.storage.clone());
    let items = audit_service.export_for_user(&user_id).map_err(|e| {
        (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("{}", e),
        )
    })?;
    Ok(Json(serde_json::json!({"entries": items})))
}

// DELETE /users/{user_id}/data
pub async fn user_data_delete_handler(
    State(state): State<Arc<AppState>>,
    axum::extract::Path(user_id): axum::extract::Path<String>,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, String)> {
    // Delete ToS entries for user
    let tos_prefix = format!("tos:{}:", user_id);
    let mut deleted = 0usize;
    match state.storage.prefix_delete(tos_prefix.as_bytes()) {
        Ok(n) => deleted += n,
        Err(e) => {
            return Err((
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("failed deleting tos: {}", e),
            ))
        }
    }
    // delete latest tos pointer
    let latest_key = format!("tos:latest:{}", user_id);
    if let Err(e) = state.storage.delete(latest_key.as_bytes()) {
        // ignore not found, but return error on other errors
        // RocksDB delete returns Ok even if missing, so just count
        return Err((
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("failed deleting tos latest: {}", e),
        ));
    }

    // Delete audit entries by scanning all audits and deleting those matching user_id
    let audit_items: Vec<verseguy_audit::AuditEntry> = match state.storage.prefix_scan(b"audit:") {
        Ok(v) => v,
        Err(e) => {
            return Err((
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("failed scanning audits: {}", e),
            ))
        }
    };
    let mut audit_deleted = 0usize;
    for a in audit_items {
        if a.user_id.as_deref() == Some(&user_id) {
            let key = format!("audit:{}", a.id);
            if let Err(e) = state.storage.delete(key.as_bytes()) {
                return Err((
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    format!("failed deleting audit {}: {}", a.id, e),
                ));
            }
            audit_deleted += 1;
        }
    }

    deleted += audit_deleted;

    Ok(Json(serde_json::json!({"ok": true, "deleted": deleted})))
}
