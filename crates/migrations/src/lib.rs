use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use verseguy_storage::RocksDBStorage as Storage;

#[derive(Debug, Serialize, Deserialize)]
pub struct AppliedMigration {
    pub version: u32,
    pub name: String,
    pub applied_at: DateTime<Utc>,
}

pub type MigrationFn = fn(&Storage) -> Result<()>;

#[derive(Clone)]
pub struct Migration {
    pub version: u32,
    pub name: &'static str,
    pub up: MigrationFn,
    pub down: Option<MigrationFn>,
}

impl Migration {
    pub fn new(
        version: u32,
        name: &'static str,
        up: MigrationFn,
        down: Option<MigrationFn>,
    ) -> Self {
        Self {
            version,
            name,
            up,
            down,
        }
    }
}

pub struct MigrationManager {
    migrations: Vec<Migration>,
}

impl MigrationManager {
    pub fn new(migrations: Vec<Migration>) -> Self {
        let mut m = migrations;
        m.sort_by_key(|m| m.version);
        Self { migrations: m }
    }

    pub fn current_version(&self, storage: &Storage) -> Result<u32> {
        let opt: Option<u32> = storage.get("migrations:version")?;
        Ok(opt.unwrap_or(0))
    }

    pub fn apply_pending(&self, storage: &Storage) -> Result<Vec<AppliedMigration>> {
        let mut applied = Vec::new();
        let current = self.current_version(storage)?;
        for migration in &self.migrations {
            if migration.version > current {
                tracing::info!(
                    version = migration.version,
                    name = migration.name,
                    "Applying migration"
                );
                (migration.up)(storage).context("migration.up failed")?;
                let am = AppliedMigration {
                    version: migration.version,
                    name: migration.name.to_string(),
                    applied_at: Utc::now(),
                };
                storage.put(format!("migrations:applied:{}", migration.version), &am)?;
                storage.put("migrations:version", &migration.version)?;
                applied.push(am);
            }
        }
        Ok(applied)
    }

    pub fn rollback_last(&self, storage: &Storage) -> Result<Option<AppliedMigration>> {
        let current = self.current_version(storage)?;
        if current == 0 {
            return Ok(None);
        }
        let mig = self
            .migrations
            .iter()
            .rev()
            .find(|m| m.version == current)
            .cloned();
        if let Some(m) = mig {
            if let Some(down) = m.down {
                tracing::info!(version = m.version, name = m.name, "Rolling back migration");
                (down)(storage).context("migration.down failed")?;
                let key = format!("migrations:applied:{}", m.version);
                let am: Option<AppliedMigration> = storage.get(&key)?;
                storage.delete(&key)?;
                // Set version to previous (find prev migration)
                let prev = self
                    .migrations
                    .iter()
                    .filter(|x| x.version < m.version)
                    .map(|x| x.version)
                    .max()
                    .unwrap_or(0);
                storage.put("migrations:version", &prev)?;
                return Ok(am);
            } else {
                anyhow::bail!("No down() defined for migration {}", m.version);
            }
        }
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn create_test_migration() -> Migration {
        fn up(storage: &Storage) -> Result<()> {
            storage.put("example:key", &"example_value")?;
            Ok(())
        }
        fn down(storage: &Storage) -> Result<()> {
            storage.delete("example:key")?;
            Ok(())
        }
        Migration::new(1, "create_example_key", up, Some(down))
    }

    #[test]
    #[allow(clippy::unwrap_used, clippy::disallowed_methods)]
    fn test_apply_and_rollback() -> Result<()> {
        let td = tempdir()?;
        let storage = Storage::open(td.path())?;
        let mgr = MigrationManager::new(vec![create_test_migration()]);

        // apply
        let applied = mgr.apply_pending(&storage)?;
        assert_eq!(applied.len(), 1);
        let v: Option<String> = storage.get("example:key")?;
        assert_eq!(v.unwrap(), "example_value");
        let ver = mgr.current_version(&storage)?;
        assert_eq!(ver, 1);

        // rollback
        let rolled = mgr.rollback_last(&storage)?;
        assert!(rolled.is_some());
        let v2: Option<String> = storage.get("example:key")?;
        assert!(v2.is_none());
        let ver2 = mgr.current_version(&storage)?;
        assert_eq!(ver2, 0);
        Ok(())
    }
}
