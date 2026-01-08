use crate::value_object::Rank;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Organization member (Entity)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Member {
    pub user_id: String,
    pub rank_id: String,
    pub joined_at: DateTime<Utc>,
    pub status: MemberStatus,
    pub contributions: u32,
}

impl Member {
    /// Create new member
    pub fn new(user_id: String, rank: &Rank) -> Self {
        Self {
            user_id,
            rank_id: rank.id.clone(),
            joined_at: Utc::now(),
            status: MemberStatus::Active,
            contributions: 0,
        }
    }

    /// Change rank
    pub fn change_rank(&mut self, new_rank_id: String) {
        self.rank_id = new_rank_id;
    }

    /// Record contribution
    pub fn add_contribution(&mut self) {
        self.contributions += 1;
    }

    /// Mark member as kicked
    pub fn kick(&mut self) {
        self.status = MemberStatus::Kicked;
    }
}

/// Member status
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum MemberStatus {
    Active,
    Inactive,
    Kicked,
    Left,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::value_object::Rank;

    #[test]
    fn test_member_lifecycle() {
        let rank = Rank::member();
        let mut member = Member::new("user-1".to_string(), &rank);

        assert_eq!(member.user_id, "user-1");
        assert_eq!(member.rank_id, rank.id);
        assert_eq!(member.status, MemberStatus::Active);

        member.add_contribution();
        assert_eq!(member.contributions, 1);

        member.change_rank("officer".to_string());
        assert_eq!(member.rank_id, "officer");

        member.kick();
        assert_eq!(member.status, MemberStatus::Kicked);
    }
}
