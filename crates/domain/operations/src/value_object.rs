use serde::{Deserialize, Serialize};

/// Operation status value object
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum OperationStatus {
    #[default]
    Planned,
    Running,
    Completed,
    Cancelled,
}
