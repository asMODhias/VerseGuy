use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Participant {
    pub id: String,
    pub user_id: String,
    pub role: String,
    pub joined_at: DateTime<Utc>,
}

impl Participant {
    pub fn new(id: String, user_id: String, role: String) -> Self {
        Self {
            id,
            user_id,
            role,
            joined_at: Utc::now(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Operation {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub status: crate::value_object::OperationStatus,
    pub participants: Vec<Participant>,
    pub created_at: DateTime<Utc>,
}

impl Operation {
    pub fn new(id: String, name: String, description: Option<String>) -> Self {
        Self {
            id,
            name,
            description,
            status: crate::value_object::OperationStatus::default(),
            participants: Vec::new(),
            created_at: Utc::now(),
        }
    }

    pub fn add_participant(&mut self, p: Participant) {
        if !self.participants.iter().any(|x| x.id == p.id) {
            self.participants.push(p);
        }
    }

    pub fn update_status(&mut self, status: crate::value_object::OperationStatus) {
        self.status = status;
    }
}
