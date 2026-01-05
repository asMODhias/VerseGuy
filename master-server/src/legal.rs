use crate::state::AppState;
use anyhow::Result;
use axum::body::Body;
use axum::http::Request;
use axum::{extract::State, Json};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct AdminCreateLegalRequest {
    pub doc_type: String,
    pub version: String,
    pub title: String,
    pub content: String,
    pub author: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LegalDocument {
    pub id: String,
    pub doc_type: String,
    pub version: String,
    pub title: String,
    pub content_hash: String,
    pub content: String,
    pub author: Option<String>,
    pub created_at: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Revocation {
    pub id: String,
    pub doc_type: String,
    pub version: String,
    pub reason: String,
    pub revoked_at: i64,
}

fn sha256_hex(s: &str) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(s.as_bytes());
    format!("{:x}", hasher.finalize())
}

// Storage key helpers
fn key_for_doc(doc_type: &str, version: &str) -> String {
    format!("legal:doc:{}:{}", doc_type, version)
}
fn key_latest(doc_type: &str) -> String {
    format!("legal:latest:{}", doc_type)
}
fn key_revoked(id: &str) -> String {
    format!("legal:revoked:{}", id)
}

// Admin: create legal doc
pub async fn admin_create_legal_handler(
    State(state): State<Arc<AppState>>,
    req: Request<Body>,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, String)> {
    // require admin
    let headers = req.headers();
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
    } else {
        return Err((
            axum::http::StatusCode::FORBIDDEN,
            "admin disabled".to_string(),
        ));
    }

    // read body
    let bytes = axum::body::to_bytes(req.into_body(), 1024 * 1024)
        .await
        .map_err(|e| {
            (
                axum::http::StatusCode::BAD_REQUEST,
                format!("failed to read body: {}", e),
            )
        })?;
    let req_json: AdminCreateLegalRequest = serde_json::from_slice(&bytes).map_err(|e| {
        (
            axum::http::StatusCode::BAD_REQUEST,
            format!("invalid json: {}", e),
        )
    })?;

    if req_json.doc_type.trim().is_empty() || req_json.version.trim().is_empty() {
        return Err((
            axum::http::StatusCode::BAD_REQUEST,
            "doc_type and version required".to_string(),
        ));
    }

    let id = Uuid::new_v4().to_string();
    let hash = sha256_hex(&req_json.content);
    let doc = LegalDocument {
        id: id.clone(),
        doc_type: req_json.doc_type.clone(),
        version: req_json.version.clone(),
        title: req_json.title.clone(),
        content_hash: hash,
        content: req_json.content.clone(),
        author: req_json.author.clone(),
        created_at: Utc::now().timestamp(),
    };

    let k = key_for_doc(&doc.doc_type, &doc.version);
    state.storage.put(k.as_bytes(), &doc).map_err(|e| {
        (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("storage error: {}", e),
        )
    })?;

    // also update latest mapping
    let latest_k = key_latest(&doc.doc_type);
    state.storage.put(latest_k.as_bytes(), &doc).map_err(|e| {
        (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("storage error: {}", e),
        )
    })?;

    Ok(Json(serde_json::json!({"ok": true, "id": id, "doc": doc})))
}

// Admin: get by id (scan)
pub async fn admin_get_legal_handler(
    State(state): State<Arc<AppState>>,
    req: Request<Body>,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, String)> {
    // require admin
    let headers = req.headers();
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
    } else {
        return Err((
            axum::http::StatusCode::FORBIDDEN,
            "admin disabled".to_string(),
        ));
    }

    // naive scan for key with id
    let id = req
        .uri()
        .path()
        .rsplit('/')
        .next()
        .unwrap_or("")
        .to_string();
    let items: Vec<LegalDocument> = state.storage.prefix_scan(b"legal:doc:").map_err(|e| {
        (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("storage: {}", e),
        )
    })?;
    for it in items {
        if it.id == id {
            return Ok(Json(serde_json::json!({"doc": it})));
        }
    }
    Err((
        axum::http::StatusCode::NOT_FOUND,
        "document not found".to_string(),
    ))
}

// Admin: list docs (by type optional)
pub async fn admin_list_legal_handler(
    State(state): State<Arc<AppState>>,
    req: Request<Body>,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, String)> {
    // require admin
    let headers = req.headers();
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
    } else {
        return Err((
            axum::http::StatusCode::FORBIDDEN,
            "admin disabled".to_string(),
        ));
    }

    let items: Vec<LegalDocument> = state.storage.prefix_scan(b"legal:doc:").map_err(|e| {
        (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("storage: {}", e),
        )
    })?;
    Ok(Json(serde_json::json!({"documents": items})))
}

#[derive(Deserialize)]
pub struct RevokeReq {
    pub id: String,
    pub reason: String,
}

pub async fn admin_revoke_legal_handler(
    State(state): State<Arc<AppState>>,
    req: Request<Body>,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, String)> {
    // require admin
    let headers = req.headers();
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
    } else {
        return Err((
            axum::http::StatusCode::FORBIDDEN,
            "admin disabled".to_string(),
        ));
    }

    // read body
    let bytes = axum::body::to_bytes(req.into_body(), 1024 * 1024)
        .await
        .map_err(|e| {
            (
                axum::http::StatusCode::BAD_REQUEST,
                format!("failed to read body: {}", e),
            )
        })?;
    let r: RevokeReq = serde_json::from_slice(&bytes).map_err(|e| {
        (
            axum::http::StatusCode::BAD_REQUEST,
            format!("invalid json: {}", e),
        )
    })?;

    // create revocation record
    let rev = Revocation {
        id: r.id.clone(),
        doc_type: String::new(),
        version: String::new(),
        reason: r.reason.clone(),
        revoked_at: Utc::now().timestamp(),
    };
    let k = key_revoked(&r.id);
    state.storage.put(k.as_bytes(), &rev).map_err(|e| {
        (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("storage: {}", e),
        )
    })?;
    Ok(Json(serde_json::json!({"ok": true})))
}

// Client: get latest for type
pub async fn get_latest_legal_handler(
    State(state): State<Arc<AppState>>,
    axum::extract::Path(doc_type): axum::extract::Path<String>,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, String)> {
    let k = key_latest(&doc_type);
    let rec: Option<LegalDocument> = state.storage.get(k.as_bytes()).map_err(|e| {
        (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("storage: {}", e),
        )
    })?;
    if let Some(r) = rec {
        Ok(Json(serde_json::json!({"doc": r})))
    } else {
        Err((axum::http::StatusCode::NOT_FOUND, "not found".to_string()))
    }
}

// Helper: check if a given user has accepted latest ToS
pub fn user_has_accepted_latest(storage: &crate::state::AppState, user_id: &str) -> Result<bool> {
    // read latest tos for the user
    let key = format!("tos:latest:{}", user_id);
    let rec: Option<verseguy_compliance::tos_validator::TosAcceptance> = storage
        .storage
        .get(key.as_bytes())
        .map_err(|e| anyhow::anyhow!("storage: {}", e))?;
    Ok(rec.is_some())
}

// Client: get specific version
pub async fn get_legal_version_handler(
    State(state): State<Arc<AppState>>,
    axum::extract::Path((doc_type, version)): axum::extract::Path<(String, String)>,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, String)> {
    let k = key_for_doc(&doc_type, &version);
    let rec: Option<LegalDocument> = state.storage.get(k.as_bytes()).map_err(|e| {
        (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("storage: {}", e),
        )
    })?;
    if let Some(r) = rec {
        Ok(Json(serde_json::json!({"doc": r})))
    } else {
        Err((axum::http::StatusCode::NOT_FOUND, "not found".to_string()))
    }
}
