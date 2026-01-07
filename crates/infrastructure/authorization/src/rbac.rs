use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Role {
    pub id: String,
    pub name: String,
    pub version: u64,
}

impl verseguy_storage_infra::repository::Entity for Role {
    fn entity_type() -> &'static str {
        "role"
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Assignment {
    pub user_id: String,
    pub role_id: String,
    pub version: u64,
}

impl verseguy_storage_infra::repository::Entity for Assignment {
    fn entity_type() -> &'static str {
        "assignment"
    }
    fn id(&self) -> &str {
        &self.user_id
    }
    fn version(&self) -> u64 {
        self.version
    }
    fn increment_version(&mut self) {
        self.version = self.version.saturating_add(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use uuid::Uuid;

    #[test]
    fn simple_role_assignment() -> Result<()> {
        let r = Role {
            id: Uuid::new_v4().to_string(),
            name: "admin".into(),
            version: 0,
        };
        let a = Assignment {
            user_id: Uuid::new_v4().to_string(),
            role_id: r.id.clone(),
            version: 0,
        };
        assert_eq!(a.role_id, r.id);
        Ok(())
    }
}
