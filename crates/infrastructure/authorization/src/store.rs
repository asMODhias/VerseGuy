use crate::policy::evaluate_policy;
use crate::rbac::{Assignment, Role};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use verseguy_storage_infra::prelude::AppResult;
use verseguy_storage_infra::{Repository as StorageRepository, StorageEngine};

/// Policy entity stored as simple string
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Policy {
    pub id: String,
    pub name: String,
    pub policy: String,
    pub version: u64,
}

impl verseguy_storage_infra::repository::Entity for Policy {
    fn entity_type() -> &'static str {
        "policy"
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

pub struct AuthStore {
    role_repo: StorageRepository<Role>,
    assign_repo: StorageRepository<Assignment>,
    policy_repo: StorageRepository<Policy>,
}

impl AuthStore {
    pub fn new(engine: Arc<StorageEngine>) -> Self {
        Self {
            role_repo: StorageRepository::new(engine.clone()),
            assign_repo: StorageRepository::new(engine.clone()),
            policy_repo: StorageRepository::new(engine.clone()),
        }
    }

    pub fn create_role(&self, role: &mut Role) -> AppResult<()> {
        self.role_repo.save(role)?;
        Ok(())
    }

    pub fn get_role(&self, id: &str) -> AppResult<Option<Role>> {
        self.role_repo.get(id)
    }

    pub fn assign_role(&self, assign: &mut Assignment) -> AppResult<()> {
        self.assign_repo.save(assign)?;
        Ok(())
    }

    pub fn get_roles_for_user(&self, user_id: &str) -> AppResult<Vec<String>> {
        // Return role *names* for the given user (used by policy evaluation)
        let assignments = self.assign_repo.find(|a| a.user_id == user_id)?;
        let mut names = Vec::new();
        for a in assignments {
            if let Some(r) = self.role_repo.get(&a.role_id)? {
                names.push(r.name);
            }
        }
        Ok(names)
    }

    pub fn create_policy(&self, p: &mut Policy) -> AppResult<()> {
        self.policy_repo.save(p)?;
        Ok(())
    }

    pub fn evaluate(&self, policy_name: &str, user_roles: &[&str]) -> AppResult<bool> {
        if let Some(p) = self.policy_repo.find_one(|x| x.name == policy_name)? {
            let res = evaluate_policy(&p.policy, user_roles)?;
            Ok(res)
        } else {
            Ok(false)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use tempfile::TempDir;
    use verseguy_storage_infra::config::StorageConfig;
    use verseguy_storage_infra::engine::StorageEngine;

    #[test]
    fn store_role_assign_policy_flow() -> AppResult<()> {
        let td = TempDir::new()?;
        let cfg = StorageConfig {
            path: td.path().join("db"),
            encryption_enabled: false,
            ..Default::default()
        };
        let engine = Arc::new(StorageEngine::open(cfg)?);

        let store = AuthStore::new(engine.clone());

        let mut r = Role {
            id: uuid::Uuid::new_v4().to_string(),
            name: "admin".to_string(),
            version: 0,
        };
        store.create_role(&mut r)?;

        let mut a = Assignment {
            user_id: uuid::Uuid::new_v4().to_string(),
            role_id: r.id.clone(),
            version: 0,
        };
        store.assign_role(&mut a)?;

        let roles = store.get_roles_for_user(&a.user_id)?;
        assert_eq!(roles.len(), 1);

        let mut p = Policy {
            id: uuid::Uuid::new_v4().to_string(),
            name: "admin_only".to_string(),
            policy: "role:admin".to_string(),
            version: 0,
        };
        store.create_policy(&mut p)?;

        let user_roles: Vec<&str> = roles
            .iter()
            .map(|s: &String| s.as_str())
            .collect::<Vec<&str>>();
        assert!(store.evaluate("admin_only", &user_roles)?);

        Ok(())
    }
}
