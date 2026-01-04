use anyhow::Result;
use serde::{Serialize, Deserialize};
use chrono::{Utc};

#[derive(Serialize, Deserialize)]
pub struct TosAcceptance {
    pub user_id: String,
    pub accepted_at: i64,
    pub version: String,
}

pub fn validate_tos_acceptance(json: &str) -> Result<TosAcceptance> {
    let t: TosAcceptance = serde_json::from_str(json)?;
    // Basic validation: timestamp not in future
    if t.accepted_at > Utc::now().timestamp() {
        anyhow::bail!("accepted_at in future");
    }
    Ok(t)
}
