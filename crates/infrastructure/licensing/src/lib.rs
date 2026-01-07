//! verseguy-licensing: simple licensing checks

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct License {
    pub id: String,
    pub product: String,
    pub valid: bool,
}

impl License {
    pub fn is_valid(&self) -> bool {
        self.valid
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn license_validity() {
        let l = License {
            id: Uuid::new_v4().to_string(),
            product: "enterprise".into(),
            valid: true,
        };
        assert!(l.is_valid());
    }
}
