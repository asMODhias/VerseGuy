use crate::engine::StorageEngine;
use crate::prelude::*;

/// Simple Migration struct: version and an apply function
pub type MigrationFn = Box<dyn Fn(&StorageEngine) -> AppResult<()>>;

pub struct Migration {
    pub version: u32,
    pub description: &'static str,
    pub apply: MigrationFn,
}

pub struct MigrationManager {
    migrations: Vec<Migration>,
}

impl MigrationManager {
    pub fn new() -> Self {
        Self {
            migrations: Vec::new(),
        }
    }

    pub fn register<F>(&mut self, version: u32, description: &'static str, f: F)
    where
        F: Fn(&StorageEngine) -> AppResult<()> + 'static,
    {
        self.migrations.push(Migration {
            version,
            description,
            apply: Box::new(f),
        });
        // keep migrations sorted by version
        self.migrations.sort_by_key(|m| m.version);
    }

    /// Run all pending migrations against the provided storage engine
    pub fn run(&self, engine: &StorageEngine) -> AppResult<()> {
        for m in &self.migrations {
            let key = format!("migration:applied:{}", m.version);
            // check if applied
            if let Some(_v) = engine.get(key.as_bytes())? {
                // already applied
                continue;
            }

            // apply migration
            (m.apply)(engine)?;

            // mark as applied
            let ts = chrono::Utc::now().to_rfc3339();
            engine.put(key.as_bytes(), ts.as_bytes())?;
        }
        Ok(())
    }
}

impl Default for MigrationManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::StorageConfig;
    use crate::engine::StorageEngine;
    use tempfile::TempDir;

    #[test]
    fn test_migration_manager_runs() -> AppResult<()> {
        let td = TempDir::new()?;
        let cfg = StorageConfig {
            path: td.path().join("db_migration_test"),
            encryption_enabled: false,
            ..Default::default()
        };
        let engine = StorageEngine::open(cfg)?;

        let mut mgr = MigrationManager::new();
        mgr.register(1, "create:migration-test", |eng| {
            eng.put(b"migrant:1", b"ok")?;
            Ok(())
        });

        mgr.run(&engine)?;

        // assert applied
        let got = engine.get(b"migrant:1")?;
        assert_eq!(got, Some(b"ok".to_vec()));

        // re-run: should be idempotent
        mgr.run(&engine)?;

        Ok(())
    }
}
