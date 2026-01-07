use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Organization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Organization {
    pub id: String,
    pub name: String,
    pub tag: String, // e.g., "TEST"
    pub description: String,
    pub founded: DateTime<Utc>,
    pub owner_id: String,
    pub member_count: usize,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Organization member
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Member {
    pub id: String,
    pub org_id: String,
    pub user_id: String,
    pub handle: String, // Star Citizen handle
    pub rank_id: String,
    pub joined_at: DateTime<Utc>,
    pub notes: Option<String>,
}

/// Rank in organization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rank {
    pub id: String,
    pub org_id: String,
    pub name: String,
    pub level: i32, // Higher = more senior
    pub permissions: Vec<Permission>,
    pub created_at: DateTime<Utc>,
}

/// Permission flags
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Permission {
    // Members
    ViewMembers,
    ManageMembers,
    InviteMembers,
    KickMembers,

    // Ranks
    ViewRanks,
    ManageRanks,
    AssignRanks,

    // Fleet
    ViewFleet,
    ManageFleet,

    // Operations
    ViewOperations,
    ManageOperations,
    CreateOperations,

    // Treasury (Pro+)
    ViewTreasury,
    ManageTreasury,

    // Organization
    ManageOrganization,
    DeleteOrganization,
}

impl Permission {
    pub fn as_str(&self) -> &'static str {
        match self {
            Permission::ViewMembers => "view_members",
            Permission::ManageMembers => "manage_members",
            Permission::InviteMembers => "invite_members",
            Permission::KickMembers => "kick_members",
            Permission::ViewRanks => "view_ranks",
            Permission::ManageRanks => "manage_ranks",
            Permission::AssignRanks => "assign_ranks",
            Permission::ViewFleet => "view_fleet",
            Permission::ManageFleet => "manage_fleet",
            Permission::ViewOperations => "view_operations",
            Permission::ManageOperations => "manage_operations",
            Permission::CreateOperations => "create_operations",
            Permission::ViewTreasury => "view_treasury",
            Permission::ManageTreasury => "manage_treasury",
            Permission::ManageOrganization => "manage_organization",
            Permission::DeleteOrganization => "delete_organization",
        }
    }
}
