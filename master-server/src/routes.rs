#![allow(clippy::disallowed_methods)]

use crate::state::AppState;
use anyhow::Result;
use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use verseguy_auth::local::LocalAuth;
use verseguy_auth::SessionService;
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
        .create_and_store_session(&user.id, &user.license, 7, &(*state.storage).clone())
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

#[allow(clippy::disallowed_methods)]
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

#[allow(clippy::disallowed_methods)]
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

#[allow(clippy::disallowed_methods)]
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

#[allow(clippy::disallowed_methods)]
pub async fn plugins_publish_handler(
    State(state): State<Arc<AppState>>,
    req: axum::http::Request<axum::body::Body>,
) -> Result<(axum::http::StatusCode, Json<serde_json::Value>), (axum::http::StatusCode, String)> {
    // If X-User-Id header present, require ToS acceptance for that user
    if let Some(user_id) = req.headers().get("x-user-id").and_then(|v| v.to_str().ok()) {
        let tos_key = format!("tos:{}", user_id);
        let tos: Option<serde_json::Value> =
            (*state.storage).get(tos_key.as_bytes()).map_err(|e| {
                (
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    format!("{}", e),
                )
            })?;
        if tos.is_none() {
            return Err((
                axum::http::StatusCode::FORBIDDEN,
                "ToS acceptance required".to_string(),
            ));
        }
    }

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

// --- Organization endpoints ---
use axum::extract::Path;
use axum::http::StatusCode;
use plugins_base_organization::service::OrganizationService;
use plugins_base_organization::types::Organization as OrgType;

#[derive(Serialize)]
pub struct OrgListResponse {
    pub orgs: Vec<OrgType>,
}

#[allow(clippy::disallowed_methods)]
pub async fn orgs_list_handler(
    State(state): State<Arc<AppState>>,
) -> Result<Json<OrgListResponse>, (axum::http::StatusCode, String)> {
    let svc = OrganizationService::new((*state.storage).clone());
    let orgs = svc
        .list_orgs_prefix("")
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", e)))?;
    Ok(Json(OrgListResponse { orgs }))
}

/// Simple health check endpoint for orchestration / k8s
#[allow(clippy::disallowed_methods)]
pub async fn health_handler(
) -> Result<(axum::http::StatusCode, Json<serde_json::Value>), (axum::http::StatusCode, String)> {
    Ok((
        axum::http::StatusCode::OK,
        Json(serde_json::json!({"status":"ok"})),
    ))
}

use crate::observability;

/// Metrics endpoint for Prometheus scraping
#[allow(clippy::disallowed_methods)]
pub async fn metrics_handler(
    State(state): State<Arc<AppState>>,
) -> Result<(axum::http::StatusCode, String), (axum::http::StatusCode, String)> {
    if let Some(handle) = &state.metrics_handle {
        let body = observability::render_metrics(handle);
        Ok((axum::http::StatusCode::OK, body))
    } else {
        Err((
            axum::http::StatusCode::NOT_IMPLEMENTED,
            "metrics not enabled".to_string(),
        ))
    }
}

#[derive(Deserialize)]
pub struct CreateOrgRequest {
    pub name: String,
    pub tag: String,
}

#[allow(clippy::disallowed_methods)]
pub async fn orgs_create_handler(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateOrgRequest>,
) -> Result<Json<OrgType>, (axum::http::StatusCode, String)> {
    let svc = OrganizationService::new((*state.storage).clone());
    let created = svc
        .create_organization(req.name, req.tag, "".into(), "system".into())
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", e)))?;
    Ok(Json(created))
}

#[allow(clippy::disallowed_methods)]
pub async fn orgs_get_handler(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, String)> {
    let svc = OrganizationService::new((*state.storage).clone());
    let org_opt = svc
        .get_organization(&id)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", e)))?;

    if let Some(org) = org_opt {
        let members = svc
            .list_members(&org.id)
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", e)))?;
        let mut obj = serde_json::to_value(&org)
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", e)))?;
        if let serde_json::Value::Object(ref mut map) = obj {
            map.insert(
                "members".to_string(),
                serde_json::to_value(&members).unwrap(),
            );
        }
        Ok(Json(obj))
    } else {
        Ok(Json(serde_json::json!(null)))
    }
}

#[allow(clippy::disallowed_methods)]
pub async fn orgs_delete_handler(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<(axum::http::StatusCode, Json<serde_json::Value>), (axum::http::StatusCode, String)> {
    let svc = OrganizationService::new((*state.storage).clone());
    svc.delete_organization(&id)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", e)))?;
    Ok((StatusCode::OK, Json(serde_json::json!({"deleted": true}))))
}

#[derive(Deserialize)]
pub struct DepositRequest {
    pub amount: i64,
}

#[allow(clippy::disallowed_methods)]
pub async fn orgs_deposit_handler(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(req): Json<DepositRequest>,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, String)> {
    let svc = OrganizationService::new((*state.storage).clone());
    svc.deposit(&id, req.amount)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", e)))?;
    let org = svc
        .get_organization(&id)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", e)))?
        .ok_or_else(|| (StatusCode::NOT_FOUND, "not found".to_string()))?;
    Ok(Json(
        serde_json::json!({"id": org.id, "balance": org.treasury_balance}),
    ))
}

#[allow(clippy::disallowed_methods)]
pub async fn orgs_withdraw_handler(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(req): Json<DepositRequest>,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, String)> {
    let svc = OrganizationService::new((*state.storage).clone());
    svc.withdraw(&id, req.amount).map_err(|e| {
        // treat treasury errors as bad request
        (StatusCode::BAD_REQUEST, format!("{}", e))
    })?;
    let org = svc
        .get_organization(&id)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", e)))?
        .ok_or_else(|| (StatusCode::NOT_FOUND, "not found".to_string()))?;
    Ok(Json(
        serde_json::json!({"id": org.id, "balance": org.treasury_balance}),
    ))
}

#[derive(Deserialize)]
pub struct AddMemberRequest {
    pub user_id: String,
    pub rank_id: Option<String>,
}

#[allow(clippy::disallowed_methods)]
pub async fn orgs_add_member_handler(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(req): Json<AddMemberRequest>,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, String)> {
    let svc = OrganizationService::new((*state.storage).clone());
    // fetch org to ensure exists
    let _org = svc
        .get_organization(&id)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", e)))?
        .ok_or_else(|| (StatusCode::NOT_FOUND, "not found".to_string()))?;

    // pick rank id or default to empty
    let rank_id = req.rank_id.clone().unwrap_or_default();

    let member = plugins_base_organization::types::Member {
        id: uuid::Uuid::new_v4().to_string(),
        org_id: id.clone(),
        user_id: req.user_id.clone(),
        handle: req.user_id.clone(),
        rank_id,
        joined_at: chrono::Utc::now(),
        notes: None,
    };
    svc.add_member(member)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", e)))?;
    Ok(Json(serde_json::json!({"ok": true})))
}

// --- Fleet endpoints ---

#[derive(Deserialize)]
pub struct CreateFleetRequest {
    pub id: Option<String>,
    pub name: String,
}

#[allow(clippy::disallowed_methods)]
pub async fn fleets_create_handler(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateFleetRequest>,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, String)> {
    // Repo and service
    let repo = std::sync::Arc::new(
        verseguy_domain_fleet::repo::storage_adapter::StorageFleetRepository::new(
            state.storage.clone(),
        ),
    );
    let svc = verseguy_domain_fleet::service::FleetService::new(repo);

    let id = req
        .id
        .clone()
        .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
    let created = svc
        .create_fleet(id.clone(), req.name)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", e)))?;
    Ok(Json(serde_json::to_value(&created).map_err(|e| {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", e))
    })?))
}

#[allow(clippy::disallowed_methods)]
pub async fn fleets_list_handler(
    State(state): State<Arc<AppState>>,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, String)> {
    let items: Vec<serde_json::Value> = (*state.storage)
        .prefix_scan(b"fleet:")
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", e)))?;
    Ok(Json(serde_json::json!({"fleets": items})))
}

// --- Operations endpoints ---

#[derive(Deserialize)]
pub struct CreateOperationRequest {
    pub id: Option<String>,
    pub name: String,
    pub description: Option<String>,
}

#[allow(clippy::disallowed_methods)]
pub async fn operations_create_handler(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateOperationRequest>,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, String)> {
    let repo = std::sync::Arc::new(
        verseguy_domain_operations::repo::storage_adapter::StorageOperationsRepository::new(
            state.storage.clone(),
        ),
    );
    let svc = verseguy_domain_operations::service::OperationsService::new(repo);

    let id = req
        .id
        .clone()
        .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
    let created = svc
        .create_operation(id.clone(), req.name, req.description)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", e)))?;
    Ok(Json(serde_json::to_value(&created).map_err(|e| {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", e))
    })?))
}

#[allow(clippy::disallowed_methods)]
pub async fn operations_list_handler(
    State(state): State<Arc<AppState>>,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, String)> {
    let items: Vec<serde_json::Value> = (*state.storage)
        .prefix_scan(b"operation:")
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", e)))?;
    Ok(Json(serde_json::json!({"operations": items})))
}

// --- Application endpoints ---

#[derive(Deserialize)]
pub struct CreateAppRequest {
    pub id: Option<String>,
    pub name: String,
}

#[allow(clippy::disallowed_methods)]
pub async fn apps_create_handler(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateAppRequest>,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, String)> {
    let repo = std::sync::Arc::new(
        verseguy_domain_application::repo::storage_adapter::StorageApplicationRepository::new(
            state.storage.clone(),
        ),
    );
    let svc = verseguy_domain_application::service::ApplicationService::new(repo);

    let id = req
        .id
        .clone()
        .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
    let created = svc
        .create(id.clone(), req.name)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", e)))?;
    Ok(Json(serde_json::to_value(&created).map_err(|e| {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", e))
    })?))
}

#[allow(clippy::disallowed_methods)]
pub async fn apps_list_handler(
    State(state): State<Arc<AppState>>,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, String)> {
    // list with empty prefix
    let repo = std::sync::Arc::new(
        verseguy_domain_application::repo::storage_adapter::StorageApplicationRepository::new(
            state.storage.clone(),
        ),
    );
    let svc = verseguy_domain_application::service::ApplicationService::new(repo);
    let items = svc
        .list("")
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", e)))?;
    Ok(Json(serde_json::json!({"apps": items})))
}

#[allow(clippy::disallowed_methods)]
pub async fn apps_get_handler(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, String)> {
    let repo = std::sync::Arc::new(
        verseguy_domain_application::repo::storage_adapter::StorageApplicationRepository::new(
            state.storage.clone(),
        ),
    );
    let svc = verseguy_domain_application::service::ApplicationService::new(repo);
    let opt = svc
        .get(&id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", e)))?;
    if let Some(a) = opt {
        Ok(Json(serde_json::to_value(&a).map_err(|e| {
            (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", e))
        })?))
    } else {
        Ok(Json(serde_json::json!(null)))
    }
}

#[derive(Deserialize)]
pub struct UpdateAppRequest {
    pub name: String,
}

#[allow(clippy::disallowed_methods)]
pub async fn apps_update_handler(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(req): Json<UpdateAppRequest>,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, String)> {
    let repo = std::sync::Arc::new(
        verseguy_domain_application::repo::storage_adapter::StorageApplicationRepository::new(
            state.storage.clone(),
        ),
    );
    let svc = verseguy_domain_application::service::ApplicationService::new(repo);
    svc.update_name(&id, req.name)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", e)))?;
    Ok(Json(serde_json::json!({"ok": true})))
}

#[allow(clippy::disallowed_methods)]
pub async fn apps_delete_handler(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, String)> {
    let repo = std::sync::Arc::new(
        verseguy_domain_application::repo::storage_adapter::StorageApplicationRepository::new(
            state.storage.clone(),
        ),
    );
    let svc = verseguy_domain_application::service::ApplicationService::new(repo);
    svc.delete(&id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", e)))?;
    Ok(Json(serde_json::json!({"deleted": true})))
}
#[allow(clippy::disallowed_methods)]
pub async fn fleets_get_handler(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, String)> {
    let repo = std::sync::Arc::new(
        verseguy_domain_fleet::repo::storage_adapter::StorageFleetRepository::new(
            state.storage.clone(),
        ),
    );
    let svc = verseguy_domain_fleet::service::FleetService::new(repo);
    let fopt = svc
        .get_fleet(&id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", e)))?;
    if let Some(f) = fopt {
        Ok(Json(serde_json::to_value(&f).map_err(|e| {
            (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", e))
        })?))
    } else {
        Ok(Json(serde_json::json!(null)))
    }
}

#[allow(clippy::disallowed_methods)]
pub async fn operations_get_handler(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, String)> {
    let repo = std::sync::Arc::new(
        verseguy_domain_operations::repo::storage_adapter::StorageOperationsRepository::new(
            state.storage.clone(),
        ),
    );
    let svc = verseguy_domain_operations::service::OperationsService::new(repo);
    let opt = svc
        .get_operation(&id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", e)))?;
    if let Some(o) = opt {
        Ok(Json(serde_json::to_value(&o).map_err(|e| {
            (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", e))
        })?))
    } else {
        Ok(Json(serde_json::json!(null)))
    }
}

#[derive(Deserialize)]
pub struct AddFleetShipRequest {
    pub id: Option<String>,
    pub ship_type_id: String,
    pub name: Option<String>,
}

#[allow(clippy::disallowed_methods)]
pub async fn fleets_add_ship_handler(
    State(state): State<Arc<AppState>>,
    Path(fleet_id): Path<String>,
    Json(req): Json<AddFleetShipRequest>,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, String)> {
    let repo = std::sync::Arc::new(
        verseguy_domain_fleet::repo::storage_adapter::StorageFleetRepository::new(
            state.storage.clone(),
        ),
    );
    let svc = verseguy_domain_fleet::service::FleetService::new(repo);

    let ship_id = req
        .id
        .clone()
        .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
    let ship = verseguy_domain_fleet::entity::Ship::new(
        ship_id,
        req.ship_type_id.clone(),
        req.name.clone().unwrap_or_default(),
    );
    svc.add_ship(&fleet_id, ship)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", e)))?;
    Ok(Json(serde_json::json!({"ok": true})))
}

#[derive(Deserialize)]
pub struct AddOperationParticipantRequest {
    pub id: Option<String>,
    pub user_id: String,
    pub role: String,
}

#[allow(clippy::disallowed_methods)]
pub async fn operations_add_participant_handler(
    State(state): State<Arc<AppState>>,
    Path(op_id): Path<String>,
    Json(req): Json<AddOperationParticipantRequest>,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, String)> {
    let repo = std::sync::Arc::new(
        verseguy_domain_operations::repo::storage_adapter::StorageOperationsRepository::new(
            state.storage.clone(),
        ),
    );
    let svc = verseguy_domain_operations::service::OperationsService::new(repo);

    let id = req
        .id
        .clone()
        .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
    let part = verseguy_domain_operations::entity::Participant::new(
        id,
        req.user_id.clone(),
        req.role.clone(),
    );
    svc.add_participant(&op_id, part)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", e)))?;
    Ok(Json(serde_json::json!({"ok": true})))
}

#[derive(Deserialize)]
pub struct UpdateOperationStatusRequest {
    pub status: String,
}

#[allow(clippy::disallowed_methods)]
pub async fn operations_update_status_handler(
    State(state): State<Arc<AppState>>,
    Path(op_id): Path<String>,
    Json(req): Json<UpdateOperationStatusRequest>,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, String)> {
    let repo = std::sync::Arc::new(
        verseguy_domain_operations::repo::storage_adapter::StorageOperationsRepository::new(
            state.storage.clone(),
        ),
    );
    let svc = verseguy_domain_operations::service::OperationsService::new(repo);

    let status = match req.status.as_str() {
        "Planned" => verseguy_domain_operations::value_object::OperationStatus::Planned,
        "Running" => verseguy_domain_operations::value_object::OperationStatus::Running,
        "Completed" => verseguy_domain_operations::value_object::OperationStatus::Completed,
        "Cancelled" => verseguy_domain_operations::value_object::OperationStatus::Cancelled,
        _ => return Err((StatusCode::BAD_REQUEST, "invalid status".to_string())),
    };

    svc.update_status(&op_id, status)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", e)))?;
    Ok(Json(serde_json::json!({"ok": true})))
}

// --- Stubs for handlers referenced by older routes ---
#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct TosAcceptance {
    pub user_id: String,
    pub accepted_at: i64,
    pub version: String,
}

#[allow(clippy::disallowed_methods)]
pub async fn tos_accept_handler(
    State(state): State<Arc<AppState>>,
    Json(body): Json<TosAcceptance>,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, String)> {
    // Store ToS acceptance in storage under key tos:{user_id}
    let key = format!("tos:{}", body.user_id);
    (*state.storage).put(key.as_bytes(), &body).map_err(|e| {
        (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("{}", e),
        )
    })?;
    Ok(Json(serde_json::json!({"ok": true})))
}

#[allow(clippy::disallowed_methods)]
pub async fn tos_get_handler(
    State(state): State<Arc<AppState>>,
    Path(user_id): Path<String>,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, String)> {
    let key = format!("tos:{}", user_id);
    let got: Option<TosAcceptance> = (*state.storage).get(key.as_bytes()).map_err(|e| {
        (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("{}", e),
        )
    })?;
    match got {
        Some(t) => Ok(Json(serde_json::json!(t))),
        None => Err((
            axum::http::StatusCode::NOT_FOUND,
            "tos not found".to_string(),
        )),
    }
}

#[derive(serde::Deserialize)]
pub struct VerifyRequest {
    pub manifest: PluginManifest,
    pub public_key_b64: String,
}

#[allow(clippy::disallowed_methods)]
pub async fn verify_plugin_handler(
    State(state): State<Arc<AppState>>,
    Json(req): Json<VerifyRequest>,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, String)> {
    use crate::ed25519_compat::PublicKey;
    let pub_bytes = base64::engine::general_purpose::STANDARD
        .decode(&req.public_key_b64)
        .map_err(|e| {
            (
                axum::http::StatusCode::BAD_REQUEST,
                format!("invalid base64: {}", e),
            )
        })?;
    let mut pk_arr = [0u8; 32];
    pk_arr.copy_from_slice(&pub_bytes[..32]);
    let pubkey = PublicKey::from_bytes(&pk_arr).map_err(|e| {
        (
            axum::http::StatusCode::BAD_REQUEST,
            format!("invalid public key: {}", e),
        )
    })?;

    let ok =
        crate::plugins::verify_manifest(&state.storage, &req.manifest, &pubkey).map_err(|e| {
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("{}", e),
            )
        })?;
    Ok(Json(serde_json::json!({"valid": ok})))
}

#[derive(serde::Deserialize)]
struct RevokeRequest {
    id: String,
    version: String,
    reason: String,
}

#[allow(clippy::disallowed_methods)]
pub async fn revoke_handler(
    State(state): State<Arc<AppState>>,
    req: axum::http::Request<axum::body::Body>,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, String)> {
    require_admin(req.headers())?;
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
    crate::plugins::revoke_manifest(&state.storage, &r.id, &r.version, &r.reason).map_err(|e| {
        (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("{}", e),
        )
    })?;
    Ok(Json(serde_json::json!({"ok": true})))
}

#[allow(clippy::disallowed_methods)]
pub async fn revocations_list_handler(
    State(state): State<Arc<AppState>>,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, String)> {
    // Prefix scan for plugin_revoked:
    let items: Vec<serde_json::Value> =
        (*state.storage)
            .prefix_scan(b"plugin_revoked:")
            .map_err(|e| {
                (
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    format!("{}", e),
                )
            })?;
    Ok(Json(serde_json::json!({"revocations": items})))
}

#[allow(clippy::disallowed_methods)]
pub async fn audit_export_handler(
    State(state): State<Arc<AppState>>,
    Path(user_id): Path<String>,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, String)> {
    let audit = verseguy_audit::AuditService::new(state.storage.clone());
    let entries = audit.export_for_user(&user_id).map_err(|e| {
        (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("{}", e),
        )
    })?;
    Ok(Json(serde_json::json!({"entries": entries})))
}

use axum::http::HeaderMap;
use uuid::Uuid;

#[derive(serde::Deserialize)]
#[allow(dead_code)]
struct AssignmentRec {
    pub user_id: String,
    pub role_id: String,
    pub version: u64,
}

#[allow(dead_code)]
#[derive(serde::Deserialize)]
struct RoleRec {
    pub id: String,
    pub name: String,
    pub version: u64,
}

#[allow(clippy::disallowed_methods)]
pub async fn user_data_delete_handler(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path(user_id): Path<String>,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, String)> {
    // Extract Authorization header (Bearer)
    let auth_header = headers
        .get("authorization")
        .and_then(|v| v.to_str().ok())
        .ok_or((
            StatusCode::UNAUTHORIZED,
            "missing authorization".to_string(),
        ))?;

    let token = auth_header.strip_prefix("Bearer ").ok_or((
        StatusCode::UNAUTHORIZED,
        "invalid authorization format".to_string(),
    ))?;

    // Validate session token
    let session_service = SessionService::new(state.license_secret.clone());
    let token_data = session_service
        .validate_token_and_storage(token, &state.storage)
        .map_err(|e| (StatusCode::UNAUTHORIZED, format!("{}", e)))?;
    let actor_id = token_data.claims.sub;

    // Evaluate a named policy `compliance:delete` for the actor (fallback to admin role if policy not found)
    // Gather the actor's role names
    let assignments: Vec<AssignmentRec> = (*state.storage)
        .prefix_scan(b"assignment:")
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", e)))?;
    let mut actor_roles: Vec<String> = Vec::new();
    for a in assignments.into_iter().filter(|a| a.user_id == actor_id) {
        let role_opt: Option<RoleRec> = (*state.storage)
            .get(format!("role:{}", a.role_id).as_bytes())
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", e)))?;
        if let Some(r) = role_opt {
            actor_roles.push(r.name);
        }
    }

    // Try to find a policy named `compliance:delete`
    let mut policy_opt: Option<verseguy_authorization::store::Policy> = None;
    let policies: Vec<serde_json::Value> = (*state.storage)
        .prefix_scan(b"policy:")
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", e)))?;
    for p in policies {
        // Attempt to deserialize into Policy; ignore deserialization errors
        if let Ok(pol) = serde_json::from_value::<verseguy_authorization::store::Policy>(
            serde_json::to_value(&p).unwrap_or(serde_json::Value::Null),
        ) {
            if pol.name == "compliance:delete" {
                policy_opt = Some(pol);
                break;
            }
        }
    }

    let authorized = if let Some(pol) = policy_opt {
        // Evaluate using the existing policy engine
        let role_refs: Vec<&str> = actor_roles.iter().map(String::as_str).collect();
        verseguy_authorization::policy::evaluate_policy(&pol.policy, &role_refs)
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", e)))?
    } else {
        // Fallback: require admin role
        actor_roles.iter().any(|r| r == "admin")
    };

    if !authorized {
        return Err((StatusCode::FORBIDDEN, "forbidden".to_string()));
    }

    // Proceed with deletion of personal data
    let deleted_records = verseguy_compliance::gdpr::delete_user_data(&state.storage, &user_id)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", e)))?;

    // Delete audit entries for the principal and capture how many were deleted
    let audit = verseguy_audit::AuditService::new(state.storage.clone());
    let deleted_audit = audit
        .delete_for_user(&user_id)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", e)))?;

    // Delete ToS acceptance if present
    let tos_key = format!("tos:{}", user_id);
    let _ = (*state.storage).delete(tos_key.as_bytes());

    // Record an immutable audit event for this delete action in the protected namespace
    let deleted_count = deleted_audit + if deleted_records { 1 } else { 0 };
    let request_id = Uuid::new_v4().to_string();
    let event = serde_json::json!({
        "action": "audit.delete",
        "principal_id": user_id,
        "request_id": request_id,
        "deleted_count": deleted_count
    })
    .to_string();

    // increment metrics: total gdpr requests and number of audit events deleted
    metrics::increment_counter!("gdpr_delete_requests_total");
    metrics::counter!("audit_events_deleted_total", deleted_count as u64);

    let _ = audit
        .log_delete_event(Some(actor_id), event)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", e)))?;

    Ok(Json(serde_json::json!({ "deleted": deleted_count })))
}
