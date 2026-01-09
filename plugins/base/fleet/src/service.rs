use anyhow::{Context, Result};
use chrono::Utc;
use tracing::info;
use uuid::Uuid;

use crate::types::{Loadout, Ship};
use verseguy_storage::{Storage, schema::keys};

pub struct FleetService {
    storage: Storage,
}

impl FleetService {
    pub fn new(storage: Storage) -> Self {
        Self { storage }
    }

    pub fn add_ship(&self, mut ship: Ship) -> Result<()> {
        info!("Adding ship: {}", ship.id);
        if ship.id.is_empty() {
            ship.id = Uuid::new_v4().to_string();
        }
        let now = Utc::now();
        ship.created_at = now;
        ship.updated_at = now;
        self.storage
            .put(keys::ship(&ship.owner_id, &ship.id), &ship)
            .context("Failed to save ship")?;
        Ok(())
    }

    /// Convenience: create a ship with minimal fields and return it
    pub fn create_ship(
        &self,
        owner_id: String,
        model: String,
        manufacturer: String,
    ) -> Result<Ship> {
        let now = Utc::now();
        let ship = Ship {
            id: Uuid::new_v4().to_string(),
            owner_id: owner_id.clone(),
            model,
            manufacturer,
            name: None,
            pledge_date: None,
            cost: None,
            insurance: crate::types::Insurance::Standard,
            status: crate::types::ShipStatus::Available,
            location: None,
            created_at: now,
            updated_at: now,
        };
        self.add_ship(ship.clone())?;
        Ok(ship)
    }

    pub fn get_ship(&self, owner_id: &str, ship_id: &str) -> Result<Option<Ship>> {
        let got: Option<Ship> = self
            .storage
            .get(keys::ship(owner_id, ship_id))
            .context("Failed to get ship")?;
        Ok(got)
    }

    pub fn list_ships_for_owner(&self, owner_id: &str) -> Result<Vec<Ship>> {
        let out: Vec<Ship> = self
            .storage
            .prefix_scan(keys::ships_prefix(owner_id))
            .context("Failed to list ships")?;
        Ok(out)
    }

    pub fn add_loadout(&self, mut loadout: Loadout) -> Result<()> {
        info!(
            "Adding loadout: {} for ship {}",
            loadout.id, loadout.ship_id
        );
        if loadout.id.is_empty() {
            loadout.id = Uuid::new_v4().to_string();
        }
        let now = Utc::now();
        loadout.created_at = now;
        loadout.updated_at = now;
        self.storage
            .put(keys::loadout(&loadout.ship_id, &loadout.id), &loadout)
            .context("Failed to save loadout")?;
        Ok(())
    }

    pub fn get_loadouts_for_ship(&self, ship_id: &str) -> Result<Vec<Loadout>> {
        let out: Vec<Loadout> = self
            .storage
            .prefix_scan(keys::loadouts_prefix(ship_id))
            .context("Failed to list loadouts")?;
        Ok(out)
    }

    /// Update an existing ship. Returns the updated ship or an error if it doesn't exist.
    pub fn update_ship(&self, mut ship: Ship) -> Result<Ship> {
        let existing = self.get_ship(&ship.owner_id, &ship.id)?;
        if existing.is_none() {
            anyhow::bail!("Ship not found");
        }
        ship.updated_at = Utc::now();
        self.storage
            .put(keys::ship(&ship.owner_id, &ship.id), &ship)
            .context("Failed to update ship")?;
        Ok(ship)
    }

    /// Delete a ship owned by `owner_id` with id `ship_id`.
    pub fn delete_ship(&self, owner_id: &str, ship_id: &str) -> Result<()> {
        self.storage
            .delete(keys::ship(owner_id, ship_id))
            .context("Failed to delete ship")?;
        Ok(())
    }
}
