use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{Utc, DateTime};
use verseguy_storage_infra::repository::Entity;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: String,
    pub user_id: String,
    pub created: DateTime<Utc>,
    pub expires: DateTime<Utc>,
    pub version: u64,
}

impl Entity for Session {
    fn entity_type() -> &'static str {
        "session"
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

pub mod store;

pub trait SessionStore: Send + Sync {
    fn create_session(&self, session: &mut Session) -> anyhow::Result<()>;
    fn get_session(&self, id: &str) -> anyhow::Result<Option<Session>>;
    fn revoke_session(&self, id: &str) -> anyhow::Result<()>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn session_model_roundtrip() -> Result<()> {
        let s = Session {
            id: Uuid::new_v4().to_string(),
            user_id: Uuid::new_v4().to_string(),
            created: Utc::now(),
            expires: Utc::now(),
            version: 0,
        };
        let s2 = serde_json::to_string(&s)?;
        let r: Session = serde_json::from_str(&s2)?;
        assert_eq!(s.user_id, r.user_id);
        Ok(())
    }
}