use crate::aggregate::Organization;
use crate::repo::OrganizationRepository;
use async_trait::async_trait;
use std::sync::Arc;
use verseguy_storage_infra::engine::StorageEngine;
use verseguy_storage_infra::Repository as StorageRepository;

pub struct StorageOrganizationRepository {
    repo: StorageRepository<Organization>,
}

impl StorageOrganizationRepository {
    pub fn new(engine: Arc<StorageEngine>) -> Self {
        Self {
            repo: StorageRepository::new(engine),
        }
    }
}

#[async_trait]
impl OrganizationRepository for StorageOrganizationRepository {
    async fn create(&self, org: &mut Organization) -> anyhow::Result<()> {
        self.repo.save(org)?;
        Ok(())
    }

    async fn get_by_id(&self, id: &str) -> anyhow::Result<Option<Organization>> {
        Ok(self.repo.get(id)?)
    }

    async fn delete(&self, id: &str) -> anyhow::Result<()> {
        self.repo.delete(id)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::value_object::OrganizationTag;
    use std::sync::Arc;
    use tempfile::TempDir;
    use uuid::Uuid;
    use verseguy_storage_infra::config::StorageConfig;

    #[tokio::test]
    async fn storage_org_repo_crud() -> anyhow::Result<()> {
        let td = TempDir::new()?;
        let cfg = StorageConfig {
            path: td.path().join("db"),
            encryption_enabled: false,
            ..Default::default()
        };
        let engine = Arc::new(verseguy_storage_infra::engine::StorageEngine::open(cfg)?);

        let repo = StorageOrganizationRepository::new(engine.clone());

        let tag = OrganizationTag::new("ORG".to_string()).expect("valid tag");
        let mut org = Organization::new(
            Uuid::new_v4().to_string(),
            "TestOrg".to_string(),
            tag,
            "founder".to_string(),
        );

        repo.create(&mut org).await?;

        let loaded = repo.get_by_id(&org.id).await?.expect("exists");
        assert_eq!(loaded.name, "TestOrg");

        repo.delete(&org.id).await?;
        assert!(repo.get_by_id(&org.id).await?.is_none());

        Ok(())
    }
}
