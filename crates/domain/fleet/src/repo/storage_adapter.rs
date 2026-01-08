use crate::entity::Fleet;
use crate::repo::FleetRepository;
use async_trait::async_trait;
use std::sync::Arc;
use verseguy_storage::RocksDBStorage;

pub struct StorageFleetRepository {
    storage: Arc<RocksDBStorage>,
}

impl StorageFleetRepository {
    pub fn new(storage: Arc<RocksDBStorage>) -> Self {
        Self { storage }
    }

    fn key(id: &str) -> Vec<u8> {
        format!("fleet:{}", id).into_bytes()
    }
}

#[async_trait]
impl FleetRepository for StorageFleetRepository {
    async fn create(&self, fleet: &Fleet) -> anyhow::Result<()> {
        self.storage.put(Self::key(&fleet.id), fleet)?;
        Ok(())
    }

    async fn get_by_id(&self, id: &str) -> anyhow::Result<Option<Fleet>> {
        let res: Option<Fleet> = self.storage.get(Self::key(id))?;
        Ok(res)
    }

    async fn delete(&self, id: &str) -> anyhow::Result<()> {
        self.storage.delete(Self::key(id))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::Fleet;
    use std::sync::Arc;
    use tempfile::TempDir;
    use verseguy_storage::RocksDBStorage;

    #[tokio::test]
    async fn storage_fleet_repo_crud() -> anyhow::Result<()> {
        let td = TempDir::new()?;
        let storage = Arc::new(RocksDBStorage::open(td.path())?);
        let repo = StorageFleetRepository::new(storage.clone());

        let f = Fleet::new("f1".into(), "TestFleet".into());
        repo.create(&f).await?;

        let loaded = repo.get_by_id(&f.id).await?.expect("exists");
        assert_eq!(loaded.name, "TestFleet");

        repo.delete(&f.id).await?;
        assert!(repo.get_by_id(&f.id).await?.is_none());
        Ok(())
    }
}
