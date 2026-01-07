use serde::{Deserialize, Serialize};
use uuid::Uuid;

use verseguy_storage_infra::repository::Entity;

/// Simple user model for authentication layer
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct User {
    /// UUID as string for storage key
    pub id: String,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub version: u64,
}

impl Entity for User {
    fn entity_type() -> &'static str {
        "user"
    }

    fn id(&self) -> &str {
        &self.id
    }

    fn version(&self) -> u64 {
        self.version
    }

    fn increment_version(&mut self) {
        self.version = self.version.saturating_add(1);
    }
}

/// Repository trait for user management (implemented using storage layer)
pub trait UserRepository: Send + Sync {
    fn create_user(&self, user: &mut User) -> anyhow::Result<()>;
    fn get_user_by_id(&self, id: &str) -> anyhow::Result<Option<User>>;
    fn get_user_by_username(&self, username: &str) -> anyhow::Result<Option<User>>;
    fn delete_user(&self, id: &str) -> anyhow::Result<()>;
}

pub mod repo;

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn user_model_roundtrip() -> Result<()> {
        let u = User {
            id: Uuid::new_v4().to_string(),
            username: "alice".to_string(),
            email: "alice@example.com".to_string(),
            password_hash: "hash".to_string(),
            version: 0,
        };

        let s = serde_json::to_string(&u)?;
        let r: User = serde_json::from_str(&s)?;
        assert_eq!(u, r);
        Ok(())
    }
}