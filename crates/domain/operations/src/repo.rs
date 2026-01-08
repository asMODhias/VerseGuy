use crate::entity::Operation;
use async_trait::async_trait;

pub mod storage_adapter;

#[async_trait]
pub trait OperationsRepository: Send + Sync {
    async fn create(&self, op: &Operation) -> anyhow::Result<()>;
    async fn get_by_id(&self, id: &str) -> anyhow::Result<Option<Operation>>;
    async fn list(&self) -> anyhow::Result<Vec<Operation>>;
    async fn update(&self, op: &Operation) -> anyhow::Result<()>;
}

#[cfg(test)]
mod tests {
    // No imports needed in this test module

    #[tokio::test]
    async fn repo_trait_compiles() {
        // This test ensures the trait compiles in test context.
        // No runtime assertions required.
    }
}
