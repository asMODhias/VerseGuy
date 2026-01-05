pub mod types;

use anyhow::{Context, Result};
use chrono::Utc;
use tracing::{debug, info};
use uuid::Uuid;
use verseguy_storage::{Storage, schema::keys};

pub use types::{Operation, OperationStatus, OperationType, Participant};

/// Operations service
pub struct OperationsService {
    storage: Storage,
}

impl OperationsService {
    pub fn new(storage: Storage) -> Self {
        Self { storage }
    }
    
    /// Create operation
    #[allow(clippy::too_many_arguments)]
    pub fn create_operation(
        &self,
        org_id: String,
        title: String,
        description: String,
        operation_type: OperationType,
        scheduled_at: chrono::DateTime<Utc>,
        duration_minutes: i32,
        leader_id: String,
    ) -> Result<Operation> {
        info!("Creating operation: {}", title);
        
        let operation_id = Uuid::new_v4().to_string();
        let now = Utc::now();
        
        let operation = Operation {
            id: operation_id.clone(),
            org_id: org_id.clone(),
            title,
            description,
            operation_type,
            scheduled_at,
            duration_minutes,
            leader_id,
            participants: Vec::new(),
            status: OperationStatus::Planned,
            created_at: now,
            updated_at: now,
        };
        
        self.storage
            .put(keys::operation(&org_id, &operation_id), &operation)
            .context("Failed to save operation")?;
        
        info!("Operation created: {}", operation_id);
        
        Ok(operation)
    }
    
    /// Get operation by ID
    pub fn get_operation(&self, org_id: &str, operation_id: &str) -> Result<Option<Operation>> {
        self.storage
            .get(keys::operation(org_id, operation_id))
            .context("Failed to get operation")
    }
    
    /// List all operations for organization
    pub fn list_operations(&self, org_id: &str) -> Result<Vec<Operation>> {
        let mut operations: Vec<Operation> = self.storage
            .prefix_scan(keys::operations_prefix(org_id))
            .context("Failed to list operations")?;
        
        // Sort by scheduled time (soonest first)
        operations.sort_by(|a, b| a.scheduled_at.cmp(&b.scheduled_at));
        
        Ok(operations)
    }
    
    /// Update operation
    pub fn update_operation(&self, operation: &Operation) -> Result<()> {
        debug!("Updating operation: {}", operation.title);
        
        let mut updated = operation.clone();
        updated.updated_at = Utc::now();
        
        self.storage
            .put(keys::operation(&operation.org_id, &operation.id), &updated)
            .context("Failed to update operation")?;
        
        Ok(())
    }
    
    /// Delete operation
    pub fn delete_operation(&self, org_id: &str, operation_id: &str) -> Result<()> {
        info!("Deleting operation: {}", operation_id);
        
        self.storage
            .delete(keys::operation(org_id, operation_id))
            .context("Failed to delete operation")?;
        
        Ok(())
    }
    
    /// Add participant to operation
    pub fn add_participant(
        &self,
        org_id: &str,
        operation_id: &str,
        user_id: String,
        role: String,
        ship_id: Option<String>,
    ) -> Result<()> {
        debug!("Adding participant to operation: {}", user_id);
        
        let mut operation = self.get_operation(org_id, operation_id)?
            .ok_or_else(|| anyhow::anyhow!("Operation not found"))?;
        
        // Check if already participating
        if operation.participants.iter().any(|p| p.user_id == user_id) {
            anyhow::bail!("User already participating");
        }
        
        let participant = Participant {
            user_id,
            role,
            ship_id,
            confirmed: false,
        };
        
        operation.participants.push(participant);
        self.update_operation(&operation)?;
        
        Ok(())
    }
    
    /// Remove participant from operation
    pub fn remove_participant(
        &self,
        org_id: &str,
        operation_id: &str,
        user_id: &str,
    ) -> Result<()> {
        debug!("Removing participant from operation: {}", user_id);
        
        let mut operation = self.get_operation(org_id, operation_id)?
            .ok_or_else(|| anyhow::anyhow!("Operation not found"))?;
        
        operation.participants.retain(|p| p.user_id != user_id);
        self.update_operation(&operation)?;
        
        Ok(())
    }
    
    /// Confirm participation
    pub fn confirm_participant(
        &self,
        org_id: &str,
        operation_id: &str,
        user_id: &str,
    ) -> Result<()> {
        debug!("Confirming participant: {}", user_id);
        
        let mut operation = self.get_operation(org_id, operation_id)?
            .ok_or_else(|| anyhow::anyhow!("Operation not found"))?;
        
        let participant = operation.participants.iter_mut()
            .find(|p| p.user_id == user_id)
            .ok_or_else(|| anyhow::anyhow!("Participant not found"))?;
        
        participant.confirmed = true;
        self.update_operation(&operation)?;
        
        Ok(())
    }
    
    /// Set operation status
    pub fn set_status(
        &self,
        org_id: &str,
        operation_id: &str,
        status: OperationStatus,
    ) -> Result<()> {
        debug!("Setting operation status: {:?}", status);
        
        let mut operation = self.get_operation(org_id, operation_id)?
            .ok_or_else(|| anyhow::anyhow!("Operation not found"))?;
        
        operation.status = status;
        self.update_operation(&operation)?;
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    
    fn setup() -> (TempDir, OperationsService) {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let storage = Storage::open(temp_dir.path()).expect("Failed to open storage");
        let service = OperationsService::new(storage);
        (temp_dir, service)
    }
    
    #[test]
    fn test_create_operation() {
        let (_temp_dir, service) = setup();
        
        let operation = service.create_operation(
            "org123".to_string(),
            "Mining Op".to_string(),
            "Quantanium mining".to_string(),
            OperationType::Mining,
            Utc::now(),
            120,
            "leader123".to_string(),
        ).expect("Failed to create operation");
        
        assert_eq!(operation.title, "Mining Op");
        assert_eq!(operation.operation_type, OperationType::Mining);
        assert_eq!(operation.status, OperationStatus::Planned);
    }
    
    #[test]
    fn test_add_participant() {
        let (_temp_dir, service) = setup();
        
        let operation = service.create_operation(
            "org123".to_string(),
            "Test Op".to_string(),
            "Description".to_string(),
            OperationType::Combat,
            Utc::now(),
            60,
            "leader123".to_string(),
        ).expect("Failed to create operation");
        
        service.add_participant(
            &operation.org_id,
            &operation.id,
            "user123".to_string(),
            "Pilot".to_string(),
            Some("ship123".to_string()),
        ).expect("Failed to add participant");
        
        let updated = service.get_operation(&operation.org_id, &operation.id)
            .expect("failed to get operation")
            .expect("operation not found");
        
        assert_eq!(updated.participants.len(), 1);
        assert_eq!(updated.participants[0].role, "Pilot");
    }
    
    #[test]
    fn test_confirm_participant() {
        let (_temp_dir, service) = setup();
        
        let operation = service.create_operation(
            "org123".to_string(),
            "Test Op".to_string(),
            "Description".to_string(),
            OperationType::Combat,
            Utc::now(),
            60,
            "leader123".to_string(),
        ).expect("Failed to create operation");
        
        service.add_participant(
            &operation.org_id,
            &operation.id,
            "user123".to_string(),
            "Pilot".to_string(),
            None,
        ).expect("failed to add participant");
        
        service.confirm_participant(&operation.org_id, &operation.id, "user123")
            .expect("Failed to confirm participant");
        
        let updated = service.get_operation(&operation.org_id, &operation.id)
            .expect("failed to get operation")
            .expect("operation not found");
        
        assert!(updated.participants[0].confirmed);
    }
}
