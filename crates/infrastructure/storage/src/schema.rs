//! Database schema and key generation
//!
//! All keys follow the pattern: `entity_type:id`
//!
//! Examples:
//! - `user:uuid`
//! - `organization:uuid`
//! - `member:org_id:user_id`
//! - `ship:org_id:ship_id`

/// User keys
pub mod user {
    pub fn by_id(id: &str) -> String {
        format!("user:{}", id)
    }

    pub fn by_username(username: &str) -> String {
        format!("user_by_username:{}", username)
    }

    pub fn by_email(email: &str) -> String {
        format!("user_by_email:{}", email)
    }
}

/// Organization keys
pub mod organization {
    pub fn by_id(id: &str) -> String {
        format!("organization:{}", id)
    }

    pub fn by_tag(tag: &str) -> String {
        format!("organization_by_tag:{}", tag)
    }
}

/// Member keys
pub mod member {
    pub fn by_id(org_id: &str, user_id: &str) -> String {
        format!("member:{}:{}", org_id, user_id)
    }

    pub fn list_by_org(org_id: &str) -> String {
        format!("member:{}:", org_id)
    }
}

/// Ship keys
pub mod ship {
    pub fn by_id(org_id: &str, ship_id: &str) -> String {
        format!("ship:{}:{}", org_id, ship_id)
    }
}
