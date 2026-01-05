use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Authentication method
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AuthMethod {
    /// Local username/password authentication
    Local {
        username: String,
        password_hash: String,
    },
    /// OAuth authentication
    OAuth {
        provider: String,
        token: String,
        refresh_token: Option<String>,
        expires_at: DateTime<Utc>,
    },
}

/// License tier
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum License {
    Free,
    Pro,
    Enterprise,
}

/// User account
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: Option<String>,
    /// Password hash (Argon2) - only for local auth
    pub password_hash: Option<String>,
    pub auth_method: AuthMethod,
    pub license: License,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: String,
    pub user_id: String,
    pub license: License,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

impl Session {
    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_at
    }
}