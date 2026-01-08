use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAggregateCommand {
    pub id: Option<String>,
    pub name: String,
}
