/// Database key prefixes for different entity types
pub mod prefixes {
    pub const USER: &[u8] = b"user:";
    pub const USER_BY_USERNAME: &[u8] = b"user_by_username:";
    pub const USER_BY_EMAIL: &[u8] = b"user_by_email:";
    pub const SESSION: &[u8] = b"session:";
    pub const SESSION_BY_USER: &[u8] = b"session_by_user:";
    pub const ORGANIZATION: &[u8] = b"org:";
    pub const ORGANIZATION_BY_NAME: &[u8] = b"org_by_name:";
    pub const MEMBER: &[u8] = b"member:";
    pub const RANK: &[u8] = b"rank:";
    pub const SHIP: &[u8] = b"ship:";
    pub const LOADOUT: &[u8] = b"loadout:";
    pub const OPERATION: &[u8] = b"operation:";
    pub const AUDIT_LOG: &[u8] = b"audit:";
    pub const CONFIG: &[u8] = b"config:";
    pub const CACHE: &[u8] = b"cache:";
}

/// Key generation functions for different entity types
pub mod keys {
    use super::prefixes;

    /// Generate key for user by ID
    pub fn user(id: &str) -> Vec<u8> {
        [prefixes::USER, id.as_bytes()].concat()
    }

    /// Generate key for username lookup
    pub fn user_by_username(username: &str) -> Vec<u8> {
        [prefixes::USER_BY_USERNAME, username.as_bytes()].concat()
    }

    /// Generate key for email lookup
    pub fn user_by_email(email: &str) -> Vec<u8> {
        [prefixes::USER_BY_EMAIL, email.as_bytes()].concat()
    }

    /// Generate key for session by ID
    pub fn session(id: &str) -> Vec<u8> {
        [prefixes::SESSION, id.as_bytes()].concat()
    }

    /// Generate key for session lookup by user
    pub fn session_by_user(user_id: &str) -> Vec<u8> {
        [prefixes::SESSION_BY_USER, user_id.as_bytes()].concat()
    }

    /// Generate key for organization by ID
    pub fn organization(id: &str) -> Vec<u8> {
        [prefixes::ORGANIZATION, id.as_bytes()].concat()
    }

    /// Generate key for organization lookup by name
    pub fn organization_by_name(name: &str) -> Vec<u8> {
        [prefixes::ORGANIZATION_BY_NAME, name.as_bytes()].concat()
    }

    /// Generate key for member
    pub fn member(org_id: &str, user_id: &str) -> Vec<u8> {
        [
            prefixes::MEMBER,
            org_id.as_bytes(),
            b":",
            user_id.as_bytes(),
        ]
        .concat()
    }

    /// Generate prefix for all members of org
    pub fn members_prefix(org_id: &str) -> Vec<u8> {
        [prefixes::MEMBER, org_id.as_bytes(), b":"].concat()
    }

    /// Generate key for rank
    pub fn rank(org_id: &str, rank_id: &str) -> Vec<u8> {
        [prefixes::RANK, org_id.as_bytes(), b":", rank_id.as_bytes()].concat()
    }

    /// Generate prefix for all ranks of org
    pub fn ranks_prefix(org_id: &str) -> Vec<u8> {
        [prefixes::RANK, org_id.as_bytes(), b":"].concat()
    }

    /// Generate key for ship
    pub fn ship(owner_id: &str, ship_id: &str) -> Vec<u8> {
        [
            prefixes::SHIP,
            owner_id.as_bytes(),
            b":",
            ship_id.as_bytes(),
        ]
        .concat()
    }

    /// Generate prefix for all ships of owner
    pub fn ships_prefix(owner_id: &str) -> Vec<u8> {
        [prefixes::SHIP, owner_id.as_bytes(), b":"].concat()
    }

    /// Generate key for loadout
    pub fn loadout(ship_id: &str, loadout_id: &str) -> Vec<u8> {
        [
            prefixes::LOADOUT,
            ship_id.as_bytes(),
            b":",
            loadout_id.as_bytes(),
        ]
        .concat()
    }

    /// Generate prefix for all loadouts of ship
    pub fn loadouts_prefix(ship_id: &str) -> Vec<u8> {
        [prefixes::LOADOUT, ship_id.as_bytes(), b":"].concat()
    }

    /// Generate key for operation
    pub fn operation(org_id: &str, operation_id: &str) -> Vec<u8> {
        [
            prefixes::OPERATION,
            org_id.as_bytes(),
            b":",
            operation_id.as_bytes(),
        ]
        .concat()
    }

    /// Generate prefix for all operations of org
    pub fn operations_prefix(org_id: &str) -> Vec<u8> {
        [prefixes::OPERATION, org_id.as_bytes(), b":"].concat()
    }
}
