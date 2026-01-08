use crate::entity::Operation;
use crate::repo::OperationsRepository;
use async_trait::async_trait;
use std::sync::Arc;
use verseguy_storage::RocksDBStorage;

pub struct StorageOperationsRepository {
    storage: Arc<RocksDBStorage>,
}

impl StorageOperationsRepository {
    pub fn new(storage: Arc<RocksDBStorage>) -> Self {
        Self { storage }
    }

    fn key(id: &str) -> Vec<u8> {
        format!("operation:{}", id).into_bytes()
    }
}

#[async_trait]
impl OperationsRepository for StorageOperationsRepository {
    async fn create(&self, op: &Operation) -> anyhow::Result<()> {
        self.storage.put(Self::key(&op.id), op)?;
        Ok(())
    }

    async fn get_by_id(&self, id: &str) -> anyhow::Result<Option<Operation>> {
        let res: Option<Operation> = self.storage.get(Self::key(id))?;
        Ok(res)
    }

    async fn list(&self) -> anyhow::Result<Vec<Operation>> {
        let out: Vec<Operation> = self.storage.prefix_scan(b"operation:")?;
        Ok(out)
    }

    async fn update(&self, op: &Operation) -> anyhow::Result<()> {
        self.storage.put(Self::key(&op.id), op)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::Operation;
    use std::sync::Arc;
    use tempfile::TempDir;
    use verseguy_storage::RocksDBStorage;

    #[tokio::test]
    async fn storage_operations_repo_crud() -> anyhow::Result<()> {
        let td = TempDir::new()?;
        let storage = Arc::new(RocksDBStorage::open(td.path())?);
        let repo = StorageOperationsRepository::new(storage.clone());

        let mut op = Operation::new("op-1".into(), "TestOp".into(), None);
        repo.create(&op).await?;

        let loaded = repo.get_by_id(&op.id).await?.expect("exists");
        assert_eq!(loaded.name, "TestOp");

        op.name = "Updated".into();
        repo.update(&op).await?;

        let loaded2 = repo.get_by_id(&op.id).await?.expect("exists");
        assert_eq!(loaded2.name, "Updated");

        repo.update(&Operation::new("op-x".into(), "Noop".into(), None))
            .await?; // should just write
        Ok(())
    }
}
