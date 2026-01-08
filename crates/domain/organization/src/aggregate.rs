use crate::entity::Member;
use crate::value_object::OrganizationTag;
use crate::value_object::Rank;
use crate::value_object::Treasury;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Organization {
    pub id: String,
    pub name: String,
    pub tag: String,
    pub founder_id: String,
    pub ranks: Vec<Rank>,
    pub members: Vec<Member>,
    pub treasury: Treasury,
    pub created_at: DateTime<Utc>,
    pub version: u64,
}

impl Organization {
    pub fn new(id: String, name: String, tag: OrganizationTag, founder_id: String) -> Self {
        Self {
            id,
            name,
            tag: tag.value().to_string(),
            founder_id,
            ranks: vec![Rank::member(), Rank::officer(), Rank::leader()],
            members: vec![],
            treasury: Treasury::new(),
            created_at: Utc::now(),
            version: 0,
        }
    }

    pub fn add_member(&mut self, member: Member) {
        self.members.push(member);
    }

    pub fn remove_member(&mut self, user_id: &str) {
        self.members.retain(|m| m.user_id != user_id);
    }
}

impl verseguy_storage_infra::repository::Entity for Organization {
    fn entity_type() -> &'static str {
        "organization"
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::value_object::OrganizationTag;
    use uuid::Uuid;

    #[test]
    fn create_and_modify_org() {
        let tag = OrganizationTag::new("ORG".to_string()).expect("valid tag");
        let id = Uuid::new_v4().to_string();
        let mut org = Organization::new(
            id.clone(),
            "TestOrg".to_string(),
            tag,
            "founder".to_string(),
        );

        assert_eq!(org.id, id);
        assert_eq!(org.name, "TestOrg");
        assert_eq!(org.treasury.balance, 0);

        org.treasury.deposit(100).unwrap();
        assert_eq!(org.treasury.balance, 100);

        org.add_member(Member::new("user-1".to_string(), &org.ranks[0]));
        assert_eq!(org.members.len(), 1);

        org.remove_member("user-1");
        assert_eq!(org.members.len(), 0);
    }
}
