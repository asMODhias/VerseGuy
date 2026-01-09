use crate::types::{Member, Organization, Permission, Rank};
use anyhow::{Context, Result};
use chrono::Utc;
use tracing::info;
use uuid::Uuid;
use verseguy_storage::{Storage, schema::keys};

pub struct OrganizationService {
    storage: Storage,
}

impl OrganizationService {
    pub fn new(storage: Storage) -> Self {
        Self { storage }
    }

    // ===========================================================================
    // ORGANIZATION MANAGEMENT
    // ===========================================================================

    /// Create new organization
    pub fn create_organization(
        &self,
        name: String,
        tag: String,
        description: String,
        owner_id: String,
    ) -> Result<Organization> {
        info!("Creating organization: {} [{}]", name, tag);

        // Validate
        if name.len() < 3 || name.len() > 64 {
            anyhow::bail!("Organization name must be 3-64 characters");
        }
        if tag.len() < 2 || tag.len() > 5 {
            anyhow::bail!("Organization tag must be 2-5 characters");
        }

        // Check if name exists
        let existing: Option<String> = self
            .storage
            .get(keys::organization_by_name(&name))
            .context("Failed to check existing organization")?;

        if existing.is_some() {
            anyhow::bail!("Organization name already exists");
        }

        // Create
        let id = Uuid::new_v4().to_string();
        let now = Utc::now();
        let org = Organization {
            id: id.clone(),
            name: name.clone(),
            tag: tag.clone(),
            description: description.clone(),
            founded: now,
            owner_id: owner_id.clone(),
            member_count: 1,
            created_at: now,
            updated_at: now,
        };

        // Store
        self.storage
            .put(keys::organization(&id), &org)
            .context("Failed to save org")?;
        self.storage
            .put(keys::organization_by_name(&name), &id)
            .context("Failed to save name index")?;

        Ok(org)
    }

    pub fn get_organization(&self, id: &str) -> Result<Option<Organization>> {
        let org_opt: Option<Organization> = self
            .storage
            .get(keys::organization(id))
            .context("Failed to get organization")?;
        Ok(org_opt)
    }

    pub fn delete_organization(&self, id: &str) -> Result<()> {
        self.storage
            .delete(keys::organization(id))
            .context("Failed to delete organization")?;
        Ok(())
    }

    /// Update organization fields (name, tag, description). Will preserve indexes and enforce name uniqueness.
    pub fn update_organization(
        &self,
        id: &str,
        new_name: Option<String>,
        new_tag: Option<String>,
        new_description: Option<String>,
    ) -> Result<Organization> {
        let org_opt: Option<Organization> = self
            .storage
            .get(keys::organization(id))
            .context("Failed to load organization")?;
        let mut org = org_opt.ok_or_else(|| anyhow::anyhow!("Organization not found"))?;

        // If name is changing, ensure uniqueness and update name index
        if let Some(name) = new_name
            && name != org.name
        {
            let existing: Option<String> = self.storage.get(keys::organization_by_name(&name))?;
            if let Some(existing_id) = existing
                && existing_id != org.id
            {
                anyhow::bail!("Organization name already exists");
            }
            // remove old name index and add new one
            self.storage
                .delete(keys::organization_by_name(&org.name))
                .context("Failed to delete old name index")?;
            self.storage
                .put(keys::organization_by_name(&name), &org.id)
                .context("Failed to save name index")?;
            org.name = name;
        }

        if let Some(tag) = new_tag {
            org.tag = tag;
        }
        if let Some(desc) = new_description {
            org.description = desc;
        }

        org.updated_at = Utc::now();

        self.storage
            .put(keys::organization(id), &org)
            .context("Failed to save org")?;

        Ok(org)
    }

    /// Return a page of organizations (offset, limit) using the organization key space.
    pub fn list_orgs_page(&self, offset: usize, limit: usize) -> Result<Vec<Organization>> {
        let all: Vec<Organization> = self
            .storage
            .prefix_scan(keys::organization(""))
            .context("Failed to list organizations")?;
        // simple slice-based pagination
        if offset >= all.len() {
            return Ok(vec![]);
        }
        let end = (offset + limit).min(all.len());
        Ok(all[offset..end].to_vec())
    }

    pub fn add_member(&self, member: Member) -> Result<()> {
        // Validate
        if member.handle.len() < 3 {
            anyhow::bail!("Handle too short");
        }

        self.storage
            .put(keys::member(&member.org_id, &member.user_id), &member)
            .context("Failed to add member")?;
        Ok(())
    }

    pub fn list_members(&self, org_id: &str) -> Result<Vec<Member>> {
        let results: Vec<Member> = self
            .storage
            .prefix_scan(keys::members_prefix(org_id))
            .context("Failed to scan members")?;
        Ok(results)
    }

    pub fn has_permission(&self, user_id: &str, perm: Permission) -> Result<bool> {
        // Simplified: check if any rank assigned to user includes the permission
        // Scan all members under the root member prefix
        let members: Vec<Member> = self.storage.prefix_scan(b"member:")?;
        for m in members {
            if m.user_id == user_id {
                // load rank for this org
                let rank_opt: Option<Rank> = self.storage.get(keys::rank(&m.org_id, &m.rank_id))?;
                if let Some(rank) = rank_opt {
                    return Ok(rank.permissions.contains(&perm));
                }
            }
        }
        Ok(false)
    }

    pub fn list_orgs_prefix(&self, prefix: &str) -> Result<Vec<Organization>> {
        // Use organization's key prefix to scan for organizations matching prefix (empty string for all)
        let out: Vec<Organization> = self
            .storage
            .prefix_scan(keys::organization(prefix))
            .context("Failed to list organizations")?;
        Ok(out)
    }
}
