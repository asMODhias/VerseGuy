use serde::{Deserialize, Serialize};

/// Organization tag (2-5 uppercase alphanumeric)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct OrganizationTag(String);

impl OrganizationTag {
    /// Create new tag (validates format)
    pub fn new(tag: String) -> crate::Result<Self> {
        // Validate: 2-5 chars, uppercase alphanumeric
        if tag.len() < 2 || tag.len() > 5 {
            return Err(verseguy_shared_error::AppError::Validation(
                "Tag must be 2-5 characters".into(),
            ));
        }

        if !tag
            .chars()
            .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit())
        {
            return Err(verseguy_shared_error::AppError::Validation(
                "Tag must be uppercase alphanumeric".into(),
            ));
        }

        Ok(Self(tag))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for OrganizationTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Organization rank
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Rank {
    pub id: String,
    pub name: String,
    pub level: u32,
    pub can_invite: bool,
    pub can_kick: bool,
    pub can_manage_ranks: bool,
    pub can_manage_treasury: bool,
    pub can_create_operations: bool,
}

impl Rank {
    /// Create new rank
    pub fn new(name: String, level: u32) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            level,
            can_invite: false,
            can_kick: false,
            can_manage_ranks: false,
            can_manage_treasury: false,
            can_create_operations: false,
        }
    }

    /// Default member rank
    pub fn member() -> Self {
        Self {
            id: "member".to_string(),
            name: "Member".to_string(),
            level: 1,
            can_invite: false,
            can_kick: false,
            can_manage_ranks: false,
            can_manage_treasury: false,
            can_create_operations: false,
        }
    }

    /// Default officer rank
    pub fn officer() -> Self {
        Self {
            id: "officer".to_string(),
            name: "Officer".to_string(),
            level: 5,
            can_invite: true,
            can_kick: false,
            can_manage_ranks: false,
            can_manage_treasury: false,
            can_create_operations: true,
        }
    }

    /// Default leader rank
    pub fn leader() -> Self {
        Self {
            id: "leader".to_string(),
            name: "Leader".to_string(),
            level: 10,
            can_invite: true,
            can_kick: true,
            can_manage_ranks: true,
            can_manage_treasury: true,
            can_create_operations: true,
        }
    }
}

/// Treasury (organization funds)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Treasury {
    pub balance: i64,
    pub currency: Currency,
}

impl Treasury {
    pub fn new() -> Self {
        Self {
            balance: 0,
            currency: Currency::AUEC,
        }
    }

    /// Add funds
    pub fn deposit(&mut self, amount: i64) -> crate::Result<()> {
        if amount <= 0 {
            return Err(verseguy_shared_error::AppError::Validation(
                "Amount must be positive".into(),
            ));
        }

        self.balance = self.balance.checked_add(amount).ok_or_else(|| {
            verseguy_shared_error::AppError::Other(anyhow::anyhow!("Treasury overflow"))
        })?;

        Ok(())
    }

    /// Remove funds
    pub fn withdraw(&mut self, amount: i64) -> crate::Result<()> {
        if amount <= 0 {
            return Err(verseguy_shared_error::AppError::Validation(
                "Amount must be positive".into(),
            ));
        }

        if self.balance < amount {
            return Err(verseguy_shared_error::AppError::Other(anyhow::anyhow!(
                "Insufficient funds"
            )));
        }

        self.balance -= amount;
        Ok(())
    }
}

impl Default for Treasury {
    fn default() -> Self {
        Self::new()
    }
}

/// Currency type
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Currency {
    #[serde(rename = "aUEC")]
    AUEC, // Alpha United Earth Credits
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tag_validation_ok() {
        assert!(OrganizationTag::new("ABC".to_string()).is_ok());
        assert!(OrganizationTag::new("TEST".to_string()).is_ok());
        assert!(OrganizationTag::new("T3ST".to_string()).is_ok());
    }

    #[test]
    fn test_tag_validation_err() {
        assert!(OrganizationTag::new("A".to_string()).is_err()); // Too short
        assert!(OrganizationTag::new("TOOLONG".to_string()).is_err()); // Too long
        assert!(OrganizationTag::new("test".to_string()).is_err()); // Lowercase
        assert!(OrganizationTag::new("TE-ST".to_string()).is_err()); // Special char
    }

    #[test]
    fn test_treasury() {
        let mut treasury = Treasury::new();

        assert!(treasury.deposit(1000).is_ok());
        assert_eq!(treasury.balance, 1000);

        assert!(treasury.withdraw(500).is_ok());
        assert_eq!(treasury.balance, 500);

        assert!(treasury.withdraw(1000).is_err()); // Insufficient
    }
}
