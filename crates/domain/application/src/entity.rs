use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppAggregate {
    pub id: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
    /// Arbitrary metadata (string->string)
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub metadata: HashMap<String, String>,
    /// Tags for quick categorization
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
}

impl AppAggregate {
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            created_at: Utc::now(),
            metadata: HashMap::new(),
            tags: Vec::new(),
        }
    }

    pub fn set_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    }

    pub fn remove_metadata(&mut self, key: &str) {
        self.metadata.remove(key);
    }

    pub fn add_tag(&mut self, tag: String) {
        if !self.tags.contains(&tag) {
            self.tags.push(tag);
        }
    }

    pub fn remove_tag(&mut self, tag: &str) {
        self.tags.retain(|t| t != tag);
    }
}
