use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Teil13Aggregate {
    pub id: Uuid,
    pub name: String,
    // TODO: add domain-specific fields
}

impl Teil13Aggregate {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
        }
    }
}
