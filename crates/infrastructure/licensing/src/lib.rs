//! verseguy-licensing: license entities and simple checks

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum LicenseTier {
    Free,
    Pro,
    Enterprise,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct License {
    pub id: String,
    pub product: String,
    pub tier: LicenseTier,
    pub features: Vec<String>,
    pub expires_at: Option<DateTime<Utc>>,
    pub valid: bool,
    pub version: u64,
}

impl verseguy_storage_infra::repository::Entity for License {
    fn entity_type() -> &'static str {
        "license"
    }
    fn id(&self) -> &str {
        &self.id
    }
    fn version(&self) -> u64 {
        self.version
    }
    fn increment_version(&mut self) {
        self.version = self.version.saturating_add(1);
    }
}

impl License {
    pub fn is_valid(&self) -> bool {
        self.valid && self.expires_at.map(|t| t > Utc::now()).unwrap_or(true)
    }

    pub fn has_feature(&self, feat: &str) -> bool {
        self.features.iter().any(|f| f == feat)
    }
}

pub struct LicensingStore {
    repo: verseguy_storage_infra::Repository<License>,
}

impl LicensingStore {
    pub fn new(engine: std::sync::Arc<verseguy_storage_infra::engine::StorageEngine>) -> Self {
        Self {
            repo: verseguy_storage_infra::Repository::new(engine.clone()),
        }
    }

    pub fn create_license(&self, l: &mut License) -> verseguy_storage_infra::prelude::AppResult<()> {
        self.repo.save(l)?;
        Ok(())
    }

    pub fn get_license(&self, id: &str) -> verseguy_storage_infra::prelude::AppResult<Option<License>> {
        self.repo.get(id)
    }

    pub fn license_has_feature(&self, id: &str, feat: &str) -> verseguy_storage_infra::prelude::AppResult<bool> {
        if let Some(l) = self.get_license(id)? {
            Ok(l.is_valid() && l.has_feature(feat))
        } else {
            Ok(false)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;
    use tempfile::TempDir;
    use verseguy_storage_infra::config::StorageConfig;
    use verseguy_storage_infra::engine::StorageEngine;

    #[test]
    fn license_features_and_expiry() -> verseguy_storage_infra::prelude::AppResult<()> {
        let td = TempDir::new()?;
        let cfg = StorageConfig {
            path: td.path().join("ldb"),
            encryption_enabled: false,
            ..Default::default()
        };
        let engine = std::sync::Arc::new(StorageEngine::open(cfg)?);
        let store = LicensingStore::new(engine.clone());

        let mut l = License {
            id: uuid::Uuid::new_v4().to_string(),
            product: "verseguy".into(),
            tier: LicenseTier::Pro,
            features: vec!["feature_a".into(), "feature_b".into()],
            expires_at: Some(Utc::now() + Duration::days(1)),
            valid: true,
            version: 0,
        };

        store.create_license(&mut l)?;

        assert!(store.license_has_feature(&l.id, "feature_a")?);
        assert!(!store.license_has_feature(&l.id, "feature_x")?);

        // expire it and check — fetch stored entity, modify and save (respect version checks)
        if let Some(mut stored) = store.get_license(&l.id)? {
            stored.expires_at = Some(Utc::now() - Duration::days(1));
            store.create_license(&mut stored)?;
            assert!(!store.license_has_feature(&stored.id, "feature_a")?);
        } else {
            panic!("expected stored license");
        }

        Ok(())
    }
}
