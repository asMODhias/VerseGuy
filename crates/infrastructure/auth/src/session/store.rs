use std::sync::Arc;
use verseguy_storage_infra::{Repository as StorageRepository, StorageEngine};
use super::{Session, SessionStore};

pub struct StorageSessionStore {
    repo: StorageRepository<Session>,
}

impl StorageSessionStore {
    pub fn new(engine: Arc<StorageEngine>) -> Self {
        Self {
            repo: StorageRepository::new(engine),
        }
    }
}

impl SessionStore for StorageSessionStore {
    fn create_session(&self, session: &mut Session) -> anyhow::Result<()> {
        self.repo.save(session)?;
        Ok(())
    }

    fn get_session(&self, id: &str) -> anyhow::Result<Option<Session>> {
        Ok(self.repo.get(id)?)
    }

    fn revoke_session(&self, id: &str) -> anyhow::Result<()> {
        self.repo.delete(id)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use verseguy_storage_infra::config::StorageConfig;
    use std::sync::Arc;

    #[test]
    fn storage_session_store_crud() -> anyhow::Result<()> {
        let td = TempDir::new()?;
        let cfg = StorageConfig { path: td.path().join("db"), encryption_enabled: false, ..Default::default() };
        let engine = Arc::new(StorageEngine::open(cfg)?);

        let store = StorageSessionStore::new(engine.clone());

        let mut s = Session {
            id: uuid::Uuid::new_v4().to_string(),
            user_id: uuid::Uuid::new_v4().to_string(),
            created: chrono::Utc::now(),
            expires: chrono::Utc::now(),
            version: 0,
        };

        store.create_session(&mut s)?;
        let loaded = store.get_session(&s.id)?;
        let loaded = match loaded {
            Some(l) => l,
            None => panic!("session exists"),
        };
        assert_eq!(loaded.user_id, s.user_id);

        store.revoke_session(&s.id)?;
        assert!(store.get_session(&s.id)?.is_none());

        Ok(())
    }
}