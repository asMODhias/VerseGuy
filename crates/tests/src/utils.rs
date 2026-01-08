use tempfile::TempDir;

/// Create test storage placeholder (ApplicationService does not need external storage in tests)
pub fn create_test_storage() -> TempDir {
    match TempDir::new() {
        Ok(t) => t,
        Err(e) => panic!("Failed to create temp dir: {}", e),
    }
}

/// Create test audit placeholder
pub fn create_test_audit() {
    // placeholder for future integration with real audit
}

/// Generate test user id
pub fn test_user_id() -> String {
    format!("test_user_{}", uuid::Uuid::new_v4())
}
