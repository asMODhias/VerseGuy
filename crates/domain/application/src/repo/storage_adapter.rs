use crate::entity::AppAggregate;
use crate::repo::ApplicationRepository;
use async_trait::async_trait;
use std::sync::Arc;
use verseguy_storage::RocksDBStorage;

pub struct StorageApplicationRepository {
    storage: Arc<RocksDBStorage>,
}

impl StorageApplicationRepository {
    pub fn new(storage: Arc<RocksDBStorage>) -> Self {
        Self { storage }
    }

    fn key(id: &str) -> Vec<u8> {
        format!("app:{}", id).into_bytes()
    }
}

#[async_trait]
impl ApplicationRepository for StorageApplicationRepository {
    async fn create(&self, a: &AppAggregate) -> anyhow::Result<()> {
        self.storage.put(Self::key(&a.id), a)?;
        Ok(())
    }

    async fn get_by_id(&self, id: &str) -> anyhow::Result<Option<AppAggregate>> {
        let res: Option<AppAggregate> = self.storage.get(Self::key(id))?;
        Ok(res)
    }

    async fn list(&self, prefix: &str) -> anyhow::Result<Vec<AppAggregate>> {
        let k = format!("app:{}", prefix);
        let out: Vec<AppAggregate> = self.storage.prefix_scan(k.as_bytes())?;
        Ok(out)
    }

    async fn update(&self, a: &AppAggregate) -> anyhow::Result<()> {
        self.storage.put(Self::key(&a.id), a)?;
        Ok(())
    }

    async fn delete(&self, id: &str) -> anyhow::Result<()> {
        self.storage.delete(Self::key(id))?;
        Ok(())
    }

    async fn bulk_create(&self, apps: &[AppAggregate]) -> anyhow::Result<()> {
        for a in apps {
            self.storage.put(Self::key(&a.id), a)?;
        }
        Ok(())
    }

    async fn bulk_delete(&self, ids: &[String]) -> anyhow::Result<()> {
        for id in ids {
            self.storage.delete(Self::key(id))?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::AppAggregate;
    use std::sync::Arc;
    use tempfile::TempDir;
    use verseguy_storage::RocksDBStorage;

    #[tokio::test]
    async fn storage_app_repo_crud() -> anyhow::Result<()> {
        let td = TempDir::new()?;
        let storage = Arc::new(RocksDBStorage::open(td.path())?);
        let repo = StorageApplicationRepository::new(storage.clone());

        let a = AppAggregate::new("a1".into(), "Demo".into());
        repo.create(&a).await?;

        let loaded = repo.get_by_id(&a.id).await?.expect("exists");
        assert_eq!(loaded.name, "Demo");

        let list = repo.list("").await?;
        assert!(!list.is_empty());
        Ok(())
    }
}
