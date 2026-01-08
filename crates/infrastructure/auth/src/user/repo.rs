use std::sync::Arc;
use verseguy_storage_infra::{Repository as StorageRepository, StorageEngine};
use super::{User, UserRepository};

pub struct StorageUserRepository {
    repo: StorageRepository<User>,
}

impl StorageUserRepository {
    pub fn new(engine: Arc<StorageEngine>) -> Self {
        Self {
            repo: StorageRepository::new(engine),
        }
    }
}

impl UserRepository for StorageUserRepository {
    fn create_user(&self, user: &mut User) -> anyhow::Result<()> {
        self.repo.save(user)?;
        Ok(())
    }

    fn get_user_by_id(&self, id: &str) -> anyhow::Result<Option<User>> {
        Ok(self.repo.get(id)?)
    }

    fn get_user_by_username(&self, username: &str) -> anyhow::Result<Option<User>> {
        Ok(self.repo.find_one(|u| u.username == username)?)
    }

    fn delete_user(&self, id: &str) -> anyhow::Result<()> {
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
    fn storage_user_repo_crud() -> anyhow::Result<()> {
        let td = TempDir::new()?;
        let cfg = StorageConfig { path: td.path().join("db"), encryption_enabled: false, ..Default::default() };
        let engine = Arc::new(StorageEngine::open(cfg)?);

        let repo = StorageUserRepository::new(engine.clone());

        let mut user = User {
            id: uuid::Uuid::new_v4().to_string(),
            username: "bob".to_string(),
            email: "bob@example.com".to_string(),
            password_hash: "h".to_string(),
            version: 0,
        };

        repo.create_user(&mut user)?;
        let loaded = repo.get_user_by_id(&user.id)?;
        let loaded = match loaded {
            Some(u) => u,
            None => panic!("user exists"),
        };
        assert_eq!(loaded.username, "bob");

        let by_name = repo.get_user_by_username("bob")?;
        let by_name = match by_name {
            Some(b) => b,
            None => panic!("found by username"),
        };
        assert_eq!(by_name.email, "bob@example.com");

        repo.delete_user(&user.id)?;
        assert!(repo.get_user_by_id(&user.id)?.is_none());

        Ok(())
    }
}