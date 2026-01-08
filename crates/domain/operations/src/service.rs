use crate::{
    entity::{Operation, Participant},
    repo::OperationsRepository,
};
use std::sync::Arc;

pub struct OperationsService<R: OperationsRepository> {
    repo: Arc<R>,
}

impl<R: OperationsRepository> OperationsService<R> {
    pub fn new(repo: Arc<R>) -> Self {
        Self { repo }
    }

    pub async fn create_operation(
        &self,
        id: String,
        name: String,
        description: Option<String>,
    ) -> anyhow::Result<Operation> {
        let op = Operation::new(id, name, description);
        self.repo.create(&op).await?;
        Ok(op)
    }

    pub async fn add_participant(&self, op_id: &str, p: Participant) -> anyhow::Result<()> {
        let mut op = self
            .repo
            .get_by_id(op_id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("not found"))?;
        op.add_participant(p);
        self.repo.update(&op).await?;
        Ok(())
    }

    pub async fn update_status(
        &self,
        op_id: &str,
        status: crate::value_object::OperationStatus,
    ) -> anyhow::Result<()> {
        let mut op = self
            .repo
            .get_by_id(op_id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("not found"))?;
        op.update_status(status);
        self.repo.update(&op).await?;
        Ok(())
    }

    pub async fn get_operation(&self, op_id: &str) -> anyhow::Result<Option<Operation>> {
        self.repo.get_by_id(op_id).await
    }

    pub async fn list_operations(&self) -> anyhow::Result<Vec<Operation>> {
        self.repo.list().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repo::storage_adapter::StorageOperationsRepository;
    use std::sync::Arc;
    use tempfile::TempDir;
    use verseguy_storage::RocksDBStorage;

    #[tokio::test]
    async fn service_crud_and_participant_ops() -> anyhow::Result<()> {
        let td = TempDir::new()?;
        let storage = Arc::new(RocksDBStorage::open(td.path())?);
        let repo = Arc::new(StorageOperationsRepository::new(storage.clone()));
        let svc = OperationsService::new(repo.clone());

        let op = svc
            .create_operation("op-x".into(), "OpX".into(), None)
            .await?;
        let part = Participant::new("p-1".into(), "user-1".into(), "manager".into());
        svc.add_participant(&op.id, part).await?;

        let loaded = svc.get_operation(&op.id).await?.expect("exists");
        assert_eq!(loaded.participants.len(), 1);
        Ok(())
    }
}
