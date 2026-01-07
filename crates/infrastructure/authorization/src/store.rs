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

    /// Evaluate a named policy for a specific user id by resolving their roles.
    pub fn evaluate_for_user(&self, policy_name: &str, user_id: &str) -> AppResult<bool> {
        let roles = self.get_roles_for_user(user_id)?;
        let role_refs: Vec<&str> = roles.iter().map(String::as_str).collect();
        self.evaluate(policy_name, &role_refs)
    }

    /// Evaluate a named policy for a user with a license-feature checker callback.
    pub fn evaluate_with_license_checker<F>(
        &self,
        policy_name: &str,
        user_id: &str,
        checker: F,
    ) -> AppResult<bool>
    where
        F: Fn(&str) -> anyhow::Result<bool>,
    {
        if let Some(p) = self.policy_repo.find_one(|x| x.name == policy_name)? {
            let roles = self.get_roles_for_user(user_id)?;
            let role_refs: Vec<&str> = roles.iter().map(String::as_str).collect();
            let res = crate::policy::evaluate_policy_with_checker(&p.policy, &role_refs, &checker)?;
            Ok(res)
        } else {
            Ok(false)
        }
    }

    /// Convenience: evaluate with a LicensingStore (checks feature by license id)
    #[cfg(test)]
    pub fn evaluate_with_licensing_store(
        &self,
        policy_name: &str,
        user_id: &str,
        license_store: &verseguy_licensing_infra::LicensingStore,
        license_id: &str,
    ) -> AppResult<bool> {
        let checker = |feat: &str| -> anyhow::Result<bool> { license_store.license_has_feature(license_id, feat) };
        self.evaluate_with_license_checker(policy_name, user_id, checker)
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

        // evaluate_for_user convenience
        assert!(store.evaluate_for_user("admin_only", &a.user_id)?);

        Ok(())
    }

    #[test]
    fn store_composite_policy_flow() -> AppResult<()> {
        let td = TempDir::new()?;
        let cfg = StorageConfig {
            path: td.path().join("db2"),
            encryption_enabled: false,
            ..Default::default()
        };
        let engine = Arc::new(StorageEngine::open(cfg)?);

        let store = AuthStore::new(engine.clone());

        let mut r1 = Role {
            id: uuid::Uuid::new_v4().to_string(),
            name: "admin".to_string(),
            version: 0,
        };
        store.create_role(&mut r1)?;

        let mut r2 = Role {
            id: uuid::Uuid::new_v4().to_string(),
            name: "user".to_string(),
            version: 0,
        };
        store.create_role(&mut r2)?;

        let mut a = Assignment {
            user_id: uuid::Uuid::new_v4().to_string(),
            role_id: r2.id.clone(),
            version: 0,
        };
        store.assign_role(&mut a)?;

        let mut p = Policy {
            id: uuid::Uuid::new_v4().to_string(),
            name: "admin_or_user".to_string(),
            policy: "any(role:admin, role:user)".to_string(),
            version: 0,
        };
        store.create_policy(&mut p)?;

        assert!(store.evaluate_for_user("admin_or_user", &a.user_id)?);

        Ok(())
    }

    #[test]
    fn store_policy_with_license_flow() -> verseguy_storage_infra::prelude::AppResult<()> {
        // Setup storage engine
        let td = TempDir::new()?;
        let cfg = StorageConfig {
            path: td.path().join("db3"),
            encryption_enabled: false,
            ..Default::default()
        };
        let engine = Arc::new(StorageEngine::open(cfg)?);

        // Create stores
        let store = AuthStore::new(engine.clone());
        let license_store = verseguy_licensing_infra::LicensingStore::new(engine.clone());

        // Create role and assign user (user role only)
        let mut r_admin = Role {
            id: uuid::Uuid::new_v4().to_string(),
            name: "admin".to_string(),
            version: 0,
        };
        store.create_role(&mut r_admin)?;

        let mut r_user = Role {
            id: uuid::Uuid::new_v4().to_string(),
            name: "user".to_string(),
            version: 0,
        };
        store.create_role(&mut r_user)?;

        let mut a = Assignment {
            user_id: uuid::Uuid::new_v4().to_string(),
            role_id: r_user.id.clone(),
            version: 0,
        };
        store.assign_role(&mut a)?;

        // Create a license with feature_x
        let mut l = verseguy_licensing_infra::License {
            id: uuid::Uuid::new_v4().to_string(),
            product: "verseguy".into(),
            tier: verseguy_licensing_infra::LicenseTier::Pro,
            features: vec!["feature_x".into()],
            expires_at: None,
            valid: true,
            version: 0,
        };
        license_store.create_license(&mut l)?;

        // Create policy that allows admin role OR license feature_x
        let mut p = Policy {
            id: uuid::Uuid::new_v4().to_string(),
            name: "admin_or_featurex".to_string(),
            policy: "any(role:admin, license:feature_x)".to_string(),
            version: 0,
        };
        store.create_policy(&mut p)?;

        // User does not have admin role -> evaluate_for_user should be false
        assert!(!store.evaluate_for_user("admin_or_featurex", &a.user_id)?);

        // But with license, the policy should pass
        assert!(store.evaluate_with_licensing_store(
            "admin_or_featurex",
            &a.user_id,
            &license_store,
            &l.id
        )?);

        Ok(())
    }
}
