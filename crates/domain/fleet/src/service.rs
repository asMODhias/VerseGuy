use crate::{
    entity::{Fleet, Ship},
    repo::FleetRepository,
};
use std::sync::Arc;

pub struct FleetService<R: FleetRepository> {
    repo: Arc<R>,
}

impl<R: FleetRepository> FleetService<R> {
    pub fn new(repo: Arc<R>) -> Self {
        Self { repo }
    }

    pub async fn create_fleet(&self, id: String, name: String) -> anyhow::Result<Fleet> {
        let f = Fleet::new(id, name);
        self.repo.create(&f).await?;
        Ok(f)
    }

    pub async fn add_ship(&self, fleet_id: &str, ship: Ship) -> anyhow::Result<()> {
        let mut f = self
            .repo
            .get_by_id(fleet_id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("not found"))?;
        f.add_ship(ship);
        self.repo.create(&f).await?; // save updated
        Ok(())
    }

    pub async fn get_fleet(&self, id: &str) -> anyhow::Result<Option<Fleet>> {
        self.repo.get_by_id(id).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::Ship;
    use crate::repo::storage_adapter::StorageFleetRepository;
    use std::sync::Arc;
    use tempfile::TempDir;
    use verseguy_storage::RocksDBStorage;

    #[tokio::test]
    async fn service_crud_and_ship_ops() -> anyhow::Result<()> {
        let td = TempDir::new()?;
        let storage = Arc::new(RocksDBStorage::open(td.path())?);
        let repo = Arc::new(StorageFleetRepository::new(storage.clone()));
        let svc = FleetService::new(repo.clone());

        let f = svc.create_fleet("f-x".into(), "FleetX".into()).await?;
        let ship = Ship::new("s-1".into(), "st-1".into(), "MyShip".into());
        svc.add_ship(&f.id, ship.clone()).await?;

        let loaded = svc.get_fleet(&f.id).await?.expect("exists");
        assert_eq!(loaded.ships.len(), 1);
        assert_eq!(loaded.ships[0].id, "s-1");
        Ok(())
    }
}
