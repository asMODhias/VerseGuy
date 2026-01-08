use crate::{entity::AppAggregate, repo::ApplicationRepository};
use std::sync::Arc;

pub struct ApplicationService<R: ApplicationRepository> {
    repo: Arc<R>,
}

impl<R: ApplicationRepository> ApplicationService<R> {
    pub fn new(repo: Arc<R>) -> Self {
        Self { repo }
    }

    pub async fn create(&self, id: String, name: String) -> anyhow::Result<AppAggregate> {
        let a = AppAggregate::new(id, name);
        self.repo.create(&a).await?;
        Ok(a)
    }

    pub async fn get(&self, id: &str) -> anyhow::Result<Option<AppAggregate>> {
        self.repo.get_by_id(id).await
    }

    pub async fn list(&self, prefix: &str) -> anyhow::Result<Vec<AppAggregate>> {
        self.repo.list(prefix).await
    }

    pub async fn update_name(&self, id: &str, name: String) -> anyhow::Result<()> {
        let mut a = self
            .repo
            .get_by_id(id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("not found"))?;
        a.name = name;
        self.repo.update(&a).await?;
        Ok(())
    }

    pub async fn set_metadata(&self, id: &str, key: String, value: String) -> anyhow::Result<()> {
        let mut a = self
            .repo
            .get_by_id(id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("not found"))?;
        a.set_metadata(key, value);
        self.repo.update(&a).await?;
        Ok(())
    }

    pub async fn remove_metadata(&self, id: &str, key: &str) -> anyhow::Result<()> {
        let mut a = self
            .repo
            .get_by_id(id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("not found"))?;
        a.remove_metadata(key);
        self.repo.update(&a).await?;
        Ok(())
    }

    pub async fn add_tag(&self, id: &str, tag: String) -> anyhow::Result<()> {
        let mut a = self
            .repo
            .get_by_id(id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("not found"))?;
        a.add_tag(tag);
        self.repo.update(&a).await?;
        Ok(())
    }

    pub async fn remove_tag(&self, id: &str, tag: &str) -> anyhow::Result<()> {
        let mut a = self
            .repo
            .get_by_id(id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("not found"))?;
        a.remove_tag(tag);
        self.repo.update(&a).await?;
        Ok(())
    }

    pub async fn set_tags(&self, id: &str, tags: Vec<String>) -> anyhow::Result<()> {
        let mut a = self
            .repo
            .get_by_id(id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("not found"))?;
        a.tags = tags;
        self.repo.update(&a).await?;
        Ok(())
    }

    pub async fn delete(&self, id: &str) -> anyhow::Result<()> {
        self.repo.delete(id).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repo::storage_adapter::StorageApplicationRepository;
    use std::sync::Arc;
    use tempfile::TempDir;
    use verseguy_storage::RocksDBStorage;

    #[tokio::test]
    async fn service_crud() -> anyhow::Result<()> {
        let td = TempDir::new()?;
        let storage = Arc::new(RocksDBStorage::open(td.path())?);
        let repo = Arc::new(StorageApplicationRepository::new(storage.clone()));
        let svc = ApplicationService::new(repo.clone());

        let a = svc.create("a-x".into(), "AX".into()).await?;
        let got = svc.get(&a.id).await?.expect("exists");
        assert_eq!(got.name, "AX");

        // Metadata and tags
        svc.set_metadata(&a.id, "env".into(), "prod".into()).await?;
        svc.add_tag(&a.id, "beta".into()).await?;
        let got2 = svc.get(&a.id).await?.expect("exists");
        assert_eq!(got2.metadata.get("env").map(|s| s.as_str()), Some("prod"));
        assert!(got2.tags.contains(&"beta".to_string()));
        Ok(())
    }
}
