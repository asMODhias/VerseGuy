use crate::entity::AppAggregate;
use async_trait::async_trait;

pub mod storage_adapter;

#[async_trait]
pub trait ApplicationRepository: Send + Sync {
    async fn create(&self, a: &AppAggregate) -> anyhow::Result<()>;
    async fn get_by_id(&self, id: &str) -> anyhow::Result<Option<AppAggregate>>;
    async fn list(&self, prefix: &str) -> anyhow::Result<Vec<AppAggregate>>;
    async fn update(&self, a: &AppAggregate) -> anyhow::Result<()>;
    async fn delete(&self, id: &str) -> anyhow::Result<()>;

    /// Bulk create multiple apps. Fails if any item cannot be stored.
    async fn bulk_create(&self, apps: &[AppAggregate]) -> anyhow::Result<()>;

    /// Bulk delete by ids. Fails if any delete fails.
    async fn bulk_delete(&self, ids: &[String]) -> anyhow::Result<()>;
}

#[cfg(test)]
mod tests {
    // No imports required for this compile-time test

    #[tokio::test]
    async fn repo_trait_compiles() {
        // This test ensures the trait compiles in test context.
    }
}
