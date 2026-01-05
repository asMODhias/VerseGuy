use crate::{AuthMethod, License, User};
use anyhow::Result;
use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
};
use chrono::Utc;
use rand::rngs::OsRng;
use uuid::Uuid;
use verseguy_storage::RocksDBStorage;

pub struct LocalAuth {
    storage: RocksDBStorage,
}

impl LocalAuth {
    pub fn new(storage: RocksDBStorage) -> Self {
        Self { storage }
    }

    pub async fn register(&self, username: String, password: String) -> Result<User> {
        // Validate
        if username.len() < 3 {
            anyhow::bail!("Username too short");
        }
        if password.len() < 8 {
            anyhow::bail!("Password too short");
        }

        // Check existing
        let key = format!("user:username:{}", username);
        if self.storage.get::<_, User>(key.as_bytes())?.is_some() {
            anyhow::bail!("Username exists");
        }

        // Hash password
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| anyhow::anyhow!(e))?
            .to_string();

        // Create user
        let user = User {
            id: Uuid::new_v4().to_string(),
            username: username.clone(),
            email: None,
            password_hash: Some(password_hash.clone()),
            auth_method: AuthMethod::Local {
                username: username.clone(),
                password_hash: password_hash.clone(),
            },
            license: License::Free,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        // Save
        let user_key = format!("user:id:{}", user.id);
        self.storage.put(user_key.as_bytes(), &user)?;
        self.storage
            .put(format!("user:username:{}", username).as_bytes(), &user)?;

        Ok(user)
    }

    pub async fn login(&self, username: &str, password: &str) -> Result<User> {
        let key = format!("user:username:{}", username);
        let user = self
            .storage
            .get::<_, User>(key.as_bytes())?
            .ok_or_else(|| anyhow::anyhow!("Invalid credentials"))?;

        let hash = match &user.auth_method {
            AuthMethod::Local { password_hash, .. } => password_hash,
            _ => anyhow::bail!("Not a local user"),
        };

        let parsed = PasswordHash::new(hash).map_err(|e| anyhow::anyhow!(e))?;
        Argon2::default()
            .verify_password(password.as_bytes(), &parsed)
            .map_err(|_| anyhow::anyhow!("Invalid credentials"))?;

        Ok(user)
    }

    pub async fn change_password(
        &self,
        user_id: &str,
        old_password: &str,
        new_password: &str,
    ) -> Result<()> {
        // Fetch user
        let user_key = format!("user:id:{}", user_id);
        let mut user: User = self
            .storage
            .get::<_, User>(user_key.as_bytes())?
            .ok_or_else(|| anyhow::anyhow!("User not found"))?;

        // Verify old password
        let current_hash = match &user.auth_method {
            AuthMethod::Local { password_hash, .. } => password_hash,
            _ => anyhow::bail!("User does not use local auth"),
        };

        let parsed = PasswordHash::new(current_hash).map_err(|e| anyhow::anyhow!(e))?;
        Argon2::default()
            .verify_password(old_password.as_bytes(), &parsed)
            .map_err(|_| anyhow::anyhow!("Current password is incorrect"))?;

        // Validate new password
        if new_password.len() < 8 {
            anyhow::bail!("New password must be at least 8 characters");
        }

        // Hash new password
        let salt = SaltString::generate(&mut OsRng);
        let new_hash = Argon2::default()
            .hash_password(new_password.as_bytes(), &salt)
            .map_err(|e| anyhow::anyhow!(e))?
            .to_string();

        // Update user
        user.auth_method = AuthMethod::Local {
            username: user.username.clone(),
            password_hash: new_hash.clone(),
        };

        // Save
        self.storage.put(user_key.as_bytes(), &user)?;
        self.storage
            .put(format!("user:username:{}", user.username).as_bytes(), &user)?;

        Ok(())
    }
}
