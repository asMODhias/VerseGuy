use crate::entity::Teil13Aggregate;
use crate::repo::Teil13Repository;
use std::sync::Arc;

#[derive(Clone)]
pub struct Teil13Service<R: Teil13Repository> {
    repo: Arc<R>,
}

impl<R: Teil13Repository> Teil13Service<R> {
    pub fn new(repo: Arc<R>) -> Self {
        Self { repo }
    }

    pub async fn create(&self, name: impl Into<String>) -> anyhow::Result<Teil13Aggregate> {
        let agg = Teil13Aggregate::new(name);
        self.repo.put(&agg).await?;
        Ok(agg)
    }

    pub async fn get(&self, id: &str) -> anyhow::Result<Option<Teil13Aggregate>> {
        self.repo.get(id).await
    }
}
