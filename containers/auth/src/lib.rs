pub mod local;
pub mod session;

pub use local::LocalAuth;
pub use session::{SessionClaims, SessionService};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum AuthMethod {
    Local {
        username: String,
        password_hash: String,
    },
    OAuth {
        provider: String,
        token: String,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum License {
    Free,
    Pro,
    Enterprise,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: String,
    pub username: String,
    pub auth_method: AuthMethod,
    pub license: License,
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Session {
    pub id: String,
    pub user_id: String,
    pub license: License,
    pub expires_at: DateTime<Utc>,
}
