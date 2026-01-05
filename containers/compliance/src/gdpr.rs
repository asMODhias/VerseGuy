use anyhow::Result;
use serde::Serialize;
use verseguy_auth::{User, Session};
use verseguy_storage::RocksDBStorage;

#[derive(Serialize)]
pub struct UserExport {
    pub id: String,
    pub username: String,
    pub user: User,
    pub sessions: Vec<Session>,
}

pub fn export_user_data(storage: &RocksDBStorage, user_id: &str) -> Result<String> {
    // Load user
    let key = format!("user:id:{}", user_id);
    let user_opt: Option<User> = storage.get(key.as_bytes())?;
    let user = user_opt.ok_or_else(|| anyhow::anyhow!("user not found"))?;

    // Find sessions for this user
    let sessions: Vec<Session> = storage.prefix_scan(b"session:")?;
    let user_sessions: Vec<Session> = sessions
        .into_iter()
        .filter(|s| s.user_id == user_id)
        .collect();

    let export = UserExport {
        id: user_id.to_string(),
        username: user.username.clone(),
        user,
        sessions: user_sessions,
    };

    Ok(serde_json::to_string_pretty(&export)?)
}

pub fn delete_user_data(storage: &RocksDBStorage, user_id: &str) -> Result<bool> {
    // Load user to find username
    let key = format!("user:id:{}", user_id);
    let user_opt: Option<User> = storage.get(key.as_bytes())?;
    let user = match user_opt {
        Some(u) => u,
        None => return Ok(false),
    };

    // Delete user records
    storage.delete(format!("user:id:{}", user_id).as_bytes())?;
    storage.delete(format!("user:username:{}", user.username).as_bytes())?;

    // Delete sessions
    let sessions: Vec<Session> = storage.prefix_scan(b"session:")?;
    for s in sessions.into_iter().filter(|s| s.user_id == user_id) {
        storage.delete(format!("session:{}", s.id).as_bytes())?;
    }

    Ok(true)
}
