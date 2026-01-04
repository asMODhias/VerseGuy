//! Registry plugin placeholder

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PluginInfo {
    pub id: String,
    pub name: String,
    pub version: String,
}

pub fn list_plugins() -> Vec<PluginInfo> {
    Vec::new()
}
