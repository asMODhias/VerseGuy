use thiserror::Error;

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("Database error: {0}")]
    Database(String),
    
    #[error("Entity not found: {entity_type}/{id}")]
    NotFound { entity_type: String, id: String },
    
    #[error("Serialization error: {0}")]
    Serialization(String),
    
    #[error("Deserialization error: {0}")]
    Deserialization(String),
    
    #[error("Encryption error: {0}")]
    Encryption(String),
    
    #[error("Decryption error: {0}")]
    Decryption(String),
    
    #[error("Transaction error: {0}")]
    Transaction(String),
    
    #[error("Constraint violation: {0}")]
    ConstraintViolation(String),
    
    #[error("Migration error: {0}")]
    Migration(String),
    
    #[error("Backup error: {0}")]
    Backup(String),
    
    #[error("Restore error: {0}")]
    Restore(String),
    
    #[error("Configuration error: {0}")]
    Configuration(String),
}


impl From<rocksdb::Error> for StorageError {
    fn from(err: rocksdb::Error) -> Self {
        StorageError::Database(err.to_string())
    }
}

impl From<serde_json::Error> for StorageError {
    fn from(err: serde_json::Error) -> Self {
        StorageError::Serialization(err.to_string())
    }
}
