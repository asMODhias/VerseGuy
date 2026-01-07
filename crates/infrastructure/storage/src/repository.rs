use crate::prelude::*;
use crate::{engine::StorageEngine, error::StorageError};
use serde::{de::DeserializeOwned, Serialize};
use std::marker::PhantomData;
use std::sync::Arc;
use tracing::{debug, warn};

/// Entity trait for storable types
pub trait Entity: Serialize + DeserializeOwned + Send + Sync {
    /// Entity type name (e.g., "user", "organization")
    fn entity_type() -> &'static str;

    /// Entity ID
    fn id(&self) -> &str;

    /// Version for optimistic locking
    fn version(&self) -> u64;

    /// Increment version
    fn increment_version(&mut self);
}

/// Generic repository for type-safe storage operations
pub struct Repository<T: Entity> {
    engine: Arc<StorageEngine>,
    _phantom: PhantomData<T>,
}

impl<T: Entity> Repository<T> {
    /// Create new repository
    pub fn new(engine: Arc<StorageEngine>) -> Self {
        Self {
            engine,
            _phantom: PhantomData,
        }
    }

    /// Save entity (insert or update)
    pub fn save(&self, entity: &mut T) -> AppResult<()> {
        let key = self.make_key(entity.id());

        // Check for version conflict (optimistic locking)
        if let Some(existing) = self.get(entity.id())? {
            if existing.version() != entity.version() {
                return Err(storage_err(format!(
                    "Version conflict for {}/{}",
                    T::entity_type(),
                    entity.id()
                )));
            }
        }

        // Increment version
        entity.increment_version();

        // Serialize
        let value = serde_json::to_vec(entity).map_err(|e| {
            StorageError::Serialization(format!("Failed to serialize {}: {}", T::entity_type(), e))
        })?;

        // Store
        self.engine.put(&key, &value)?;

        debug!(
            entity_type = T::entity_type(),
            id = entity.id(),
            version = entity.version(),
            "Entity saved"
        );

        Ok(())
    }

    /// Get entity by ID
    pub fn get(&self, id: &str) -> AppResult<Option<T>> {
        let key = self.make_key(id);

        match self.engine.get(&key)? {
            Some(data) => {
                let entity: T = serde_json::from_slice(&data).map_err(|e| {
                    StorageError::Deserialization(format!(
                        "Failed to deserialize {}: {}",
                        T::entity_type(),
                        e
                    ))
                })?;

                Ok(Some(entity))
            }
            None => Ok(None),
        }
    }

    /// Get entity by ID (returns error if not found)
    pub fn get_required(&self, id: &str) -> AppResult<T> {
        self.get(id)?
            .ok_or_else(|| storage_err(format!("Not found: {}/{}", T::entity_type(), id)))
    }

    /// Delete entity
    pub fn delete(&self, id: &str) -> AppResult<()> {
        let key = self.make_key(id);
        self.engine.delete(&key)?;

        debug!(entity_type = T::entity_type(), id = id, "Entity deleted");

        Ok(())
    }

    /// Check if entity exists
    pub fn exists(&self, id: &str) -> AppResult<bool> {
        let key = self.make_key(id);
        self.engine.exists(&key)
    }

    /// List all entities
    pub fn list(&self) -> AppResult<Vec<T>> {
        let prefix = format!("{}:", T::entity_type());
        let results = self.engine.scan_prefix(prefix.as_bytes())?;

        let mut entities = Vec::with_capacity(results.len());

        for (_key, value) in results {
            match serde_json::from_slice::<T>(&value) {
                Ok(entity) => entities.push(entity),
                Err(e) => {
                    warn!(
                        entity_type = T::entity_type(),
                        error = %e,
                        "Failed to deserialize entity, skipping"
                    );
                }
            }
        }

        debug!(
            entity_type = T::entity_type(),
            count = entities.len(),
            "Entities listed"
        );

        Ok(entities)
    }

    /// Count entities
    pub fn count(&self) -> AppResult<usize> {
        let prefix = format!("{}:", T::entity_type());
        let results = self.engine.scan_prefix(prefix.as_bytes())?;
        Ok(results.len())
    }

    /// Find entities matching predicate
    pub fn find<F>(&self, predicate: F) -> AppResult<Vec<T>>
    where
        F: Fn(&T) -> bool,
    {
        let all = self.list()?;
        Ok(all.into_iter().filter(|e| predicate(e)).collect())
    }

    /// Find first entity matching predicate
    pub fn find_one<F>(&self, predicate: F) -> AppResult<Option<T>>
    where
        F: Fn(&T) -> bool,
    {
        let all = self.list()?;
        Ok(all.into_iter().find(|e| predicate(e)))
    }

    /// Make storage key for entity
    fn make_key(&self, id: &str) -> Vec<u8> {
        format!("{}:{}", T::entity_type(), id).into_bytes()
    }
}

impl<T: Entity> Clone for Repository<T> {
    fn clone(&self) -> Self {
        Self {
            engine: self.engine.clone(),
            _phantom: PhantomData,
        }
    }
}
