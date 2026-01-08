use crate::entity::Fleet;
use async_trait::async_trait;

pub mod storage_adapter;

#[async_trait]
pub trait FleetRepository: Send + Sync {
    async fn create(&self, fleet: &Fleet) -> anyhow::Result<()>;
    async fn get_by_id(&self, id: &str) -> anyhow::Result<Option<Fleet>>;
    async fn delete(&self, id: &str) -> anyhow::Result<()>;
}

#[cfg(test)]
mod tests {
    // No imports required for this compile-time test

    #[tokio::test]
    async fn dummy_repo_trait() {
        // This test simply ensures the trait compiles in test context.
    }
}
