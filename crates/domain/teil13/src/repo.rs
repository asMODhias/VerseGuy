use crate::entity::Teil13Aggregate;
use async_trait::async_trait;

#[async_trait]
pub trait Teil13Repository: Send + Sync {
    async fn put(&self, item: &Teil13Aggregate) -> anyhow::Result<()>;
    async fn get(&self, id: &str) -> anyhow::Result<Option<Teil13Aggregate>>;
    // TODO: add more repository operations (list, delete, query)
}

#[cfg(test)]
pub mod mock {
    use super::*;
    use async_trait::async_trait;
    use std::collections::HashMap;
    use tokio::sync::Mutex;

    #[derive(Default)]
    pub struct InMemoryRepo {
        store: Mutex<HashMap<String, Teil13Aggregate>>,
    }

    impl InMemoryRepo {
        pub fn new() -> Self {
            Default::default()
        }
    }

    #[async_trait]
    impl Teil13Repository for InMemoryRepo {
        async fn put(&self, item: &Teil13Aggregate) -> anyhow::Result<()> {
            let mut s = self.store.lock().await;
            s.insert(item.id.to_string(), item.clone());
            Ok(())
        }

        async fn get(&self, id: &str) -> anyhow::Result<Option<Teil13Aggregate>> {
            let s = self.store.lock().await;
            Ok(s.get(id).cloned())
        }
    }
}
