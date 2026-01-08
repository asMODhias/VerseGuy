use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAggregateQuery {
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListAggregatesQuery {
    pub prefix: Option<String>,
}
