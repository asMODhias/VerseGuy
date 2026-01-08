use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FleetEvent {
    FleetCreated {
        fleet_id: String,
        timestamp: DateTime<Utc>,
    },
    ShipAdded {
        fleet_id: String,
        ship_id: String,
        timestamp: DateTime<Utc>,
    },
    ShipRemoved {
        fleet_id: String,
        ship_id: String,
        timestamp: DateTime<Utc>,
    },
    LoadoutAssigned {
        fleet_id: String,
        ship_id: String,
        loadout_id: String,
        timestamp: DateTime<Utc>,
    },
}
