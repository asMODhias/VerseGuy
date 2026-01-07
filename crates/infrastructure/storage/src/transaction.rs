use crate::engine::StorageEngine;
use crate::prelude::*;
use std::sync::{Arc, Mutex};

/// Transaction for atomic operations
pub struct Transaction {
    engine: Arc<StorageEngine>,
    operations: Arc<Mutex<Vec<Operation>>>,
    committed: Arc<Mutex<bool>>,
}

enum Operation {
    Put { key: Vec<u8>, value: Vec<u8> },
    Delete { key: Vec<u8> },
}

impl Transaction {
    /// Create new transaction
    pub fn new(engine: Arc<StorageEngine>) -> Self {
        Self {
            engine,
            operations: Arc::new(Mutex::new(Vec::new())),
            committed: Arc::new(Mutex::new(false)),
        }
    }

    /// Add put operation
    pub fn put(&self, key: &[u8], value: &[u8]) -> AppResult<()> {
        let mut ops = self
            .operations
            .lock()
            .map_err(|e| internal_err(format!("Failed to lock operations: {}", e)))?;

        ops.push(Operation::Put {
            key: key.to_vec(),
            value: value.to_vec(),
        });

        Ok(())
    }

    /// Add delete operation
    pub fn delete(&self, key: &[u8]) -> AppResult<()> {
        let mut ops = self
            .operations
            .lock()
            .map_err(|e| internal_err(format!("Failed to lock operations: {}", e)))?;

        ops.push(Operation::Delete { key: key.to_vec() });

        Ok(())
    }

    /// Commit transaction
    pub fn commit(self) -> AppResult<()> {
        let ops = self
            .operations
            .lock()
            .map_err(|e| internal_err(format!("Failed to lock operations: {}", e)))?;

        // Execute all operations
        for op in ops.iter() {
            match op {
                Operation::Put { key, value } => {
                    self.engine.put(key, value)?;
                }
                Operation::Delete { key } => {
                    self.engine.delete(key)?;
                }
            }
        }

        // Mark as committed
        *self
            .committed
            .lock()
            .map_err(|e| internal_err(format!("Failed to lock committed flag: {}", e)))? = true;

        // Flush to ensure durability
        self.engine.flush()?;

        tracing::info!(operations = ops.len(), "Transaction committed");

        Ok(())
    }

    /// Rollback transaction (automatic on drop if not committed)
    pub fn rollback(self) {
        // Operations are not applied, just dropped
        tracing::info!("Transaction rolled back");
    }
}

impl Drop for Transaction {
    fn drop(&mut self) {
        if let Ok(committed) = self.committed.lock() {
            if !*committed {
                tracing::warn!("Transaction dropped without commit");
            }
        }
    }
}
