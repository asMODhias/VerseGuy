use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Organization domain events
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum OrganizationEvent {
    Created {
        organization_id: String,
        name: String,
        tag: String,
        founder_id: String,
        timestamp: DateTime<Utc>,
    },

    Updated {
        organization_id: String,
        changes: Vec<String>,
        timestamp: DateTime<Utc>,
    },

    MemberAdded {
        organization_id: String,
        user_id: String,
        rank_id: String,
        invited_by: String,
        timestamp: DateTime<Utc>,
    },

    MemberRemoved {
        organization_id: String,
        user_id: String,
        reason: String,
        removed_by: String,
        timestamp: DateTime<Utc>,
    },

    MemberRankChanged {
        organization_id: String,
        user_id: String,
        old_rank_id: String,
        new_rank_id: String,
        changed_by: String,
        timestamp: DateTime<Utc>,
    },

    RankCreated {
        organization_id: String,
        rank_id: String,
        rank_name: String,
        timestamp: DateTime<Utc>,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn event_serialization_roundtrip() {
        let e = OrganizationEvent::Created {
            organization_id: "org-1".to_string(),
            name: "My Org".to_string(),
            tag: "ORG".to_string(),
            founder_id: "user-1".to_string(),
            timestamp: Utc::now(),
        };

        let s = serde_json::to_string(&e).expect("serialize");
        let back: OrganizationEvent = serde_json::from_str(&s).expect("deserialize");
        assert_eq!(format!("{:?}", e), format!("{:?}", back));
    }
}
