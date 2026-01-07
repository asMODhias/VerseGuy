use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Ship in hangar
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ship {
    pub id: String,
    pub owner_id: String,
    pub model: String,        // e.g., "Anvil Carrack"
    pub manufacturer: String, // e.g., "Anvil Aerospace"
    pub name: Option<String>, // Custom ship name
    pub pledge_date: Option<DateTime<Utc>>,
    pub cost: Option<f64>, // USD
    pub insurance: Insurance,
    pub status: ShipStatus,
    pub location: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Ship insurance type
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Insurance {
    None,
    Standard,
    LTI, // Lifetime Insurance
}

/// Ship operational status
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ShipStatus {
    Available,
    InUse,
    Maintenance,
    Destroyed,
    Unknown,
}

/// Ship loadout
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Loadout {
    pub id: String,
    pub ship_id: String,
    pub name: String,
    pub components: Vec<Component>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Ship component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Component {
    pub slot: String, // e.g., "PowerPlant", "Shield", "Weapon_01"
    pub item: String, // e.g., "Genoa", "FR-76 Shield"
    pub manufacturer: Option<String>,
}
