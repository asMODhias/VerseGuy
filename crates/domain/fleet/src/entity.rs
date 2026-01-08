use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ship {
    pub id: String,
    pub ship_type_id: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
}

impl Ship {
    pub fn new(id: String, ship_type_id: String, name: String) -> Self {
        Self {
            id,
            ship_type_id,
            name,
            created_at: Utc::now(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fleet {
    pub id: String,
    pub name: String,
    pub ships: Vec<Ship>,
    pub created_at: DateTime<Utc>,
}

impl Fleet {
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            ships: Vec::new(),
            created_at: Utc::now(),
        }
    }

    pub fn add_ship(&mut self, ship: Ship) {
        if !self.ships.iter().any(|s| s.id == ship.id) {
            self.ships.push(ship);
        }
    }

    pub fn remove_ship(&mut self, ship_id: &str) {
        self.ships.retain(|s| s.id != ship_id);
    }
}
