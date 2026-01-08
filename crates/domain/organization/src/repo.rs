use crate::aggregate::Organization as Org;
use async_trait::async_trait;

/// Repository trait for organizations
#[async_trait]
pub trait OrganizationRepository: Send + Sync {
    async fn create(&self, org: &mut Org) -> anyhow::Result<()>;
    async fn get_by_id(&self, id: &str) -> anyhow::Result<Option<Org>>;
    async fn delete(&self, id: &str) -> anyhow::Result<()>;
}

pub mod storage_adapter;
