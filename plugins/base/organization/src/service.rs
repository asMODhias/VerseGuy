use anyhow::{Context, Result};
use crate::types::{Member, Organization};
use verseguy_storage::RocksDBStorage;
use serde::{Serialize, Deserialize};

pub struct OrganizationService {
    storage: RocksDBStorage,
}

impl OrganizationService {
    pub fn new(storage: RocksDBStorage) -> Self {
        Self { storage }
    }

    pub fn create_org(&self, org: &Organization) -> Result<()> {
        let key = format!("org:{}", org.id);
        self.storage.put(key.as_bytes(), org).context("Failed to put org")?;
        Ok(())
    }

    pub fn get_org(&self, id: &str) -> Result<Option<Organization>> {
        let key = format!("org:{}", id);
        let org: Option<Organization> = self.storage.get(key.as_bytes()).context("Failed to get org")?;
        Ok(org)
    }

    pub fn delete_org(&self, id: &str) -> Result<()> {
        let key = format!("org:{}", id);
        self.storage.delete(key.as_bytes()).context("Failed to delete org")?;
        Ok(())
    }

    pub fn list_orgs_prefix(&self, prefix: &str) -> Result<Vec<Organization>> {
        let key_prefix = format!("org:{}", prefix);
        let results: Vec<Organization> = self.storage.prefix_scan(key_prefix.as_bytes()).context("Failed to scan orgs")?;
        Ok(results)
    }

    pub fn add_member(&self, member: &Member) -> Result<()> {
        let key = format!("member:{}", member.id);
        self.storage.put(key.as_bytes(), member).context("Failed to put member")?;
        Ok(())
    }

    pub fn get_member(&self, id: &str) -> Result<Option<Member>> {
        let key = format!("member:{}", id);
        let m: Option<Member> = self.storage.get(key.as_bytes()).context("Failed to get member")?;
        Ok(m)
    }

    pub fn list_members_by_org(&self, org_id: &str) -> Result<Vec<Member>> {
        let key_prefix = format!("member:org:{}:", org_id);
        let results: Vec<Member> = self.storage.prefix_scan(key_prefix.as_bytes()).context("Failed to scan members")?;
        Ok(results)
    }

    pub fn remove_member(&self, id: &str) -> Result<()> {
        let key = format!("member:{}", id);
        self.storage.delete(key.as_bytes()).context("Failed to delete member")?;
        Ok(())
    }
}
