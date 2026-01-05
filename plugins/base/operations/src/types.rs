use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Operation/Event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Operation {
    pub id: String,
    pub org_id: String,
    pub title: String,
    pub description: String,
    pub operation_type: OperationType,
    pub scheduled_at: DateTime<Utc>,
    pub duration_minutes: i32,
    pub leader_id: String,
    pub participants: Vec<Participant>,
    pub status: OperationStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Operation type
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum OperationType {
    Combat,
    Mining,
    Trading,
    Exploration,
    Racing,
    Social,
    Training,
    Other,
}

/// Operation status
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum OperationStatus {
    Planned,
    InProgress,
    Completed,
    Cancelled,
}

/// Participant in operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Participant {
    pub user_id: String,
    pub role: String,             // e.g., "Pilot", "Gunner", "Engineer"
    pub ship_id: Option<String>,
    pub confirmed: bool,
}