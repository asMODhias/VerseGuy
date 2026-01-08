use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppAggregate {
    pub id: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
}

impl AppAggregate {
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            created_at: Utc::now(),
        }
    }
}
