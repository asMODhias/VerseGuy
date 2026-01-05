use anyhow::Result;
use tempfile::TempDir;
use verseguy_auth::{LocalAuth, SessionManager};
use verseguy_storage::Storage;
use verseguy_plugin_organization::service::OrganizationService;
use verseguy_plugin_fleet::service::FleetService;
use verseguy_plugin_operations::OperationsService;

#[tokio::test]
async fn test_complete_user_flow() -> Result<()> {
    // Setup
    let temp_dir = TempDir::new()?;
    let storage = Storage::open(temp_dir.path())?;

    // Auth
    let auth = LocalAuth::new(storage.clone());
    let session_mgr = SessionManager::new(
        SessionManager::generate_secret(),
        storage.clone(),
    );

    // Register user
    let user = auth
        .register("testuser".to_string(), "password123".to_string())
        .await?;

    // Create session
    let token = session_mgr.create_session(user.id.clone(), user.license)?;

    // Validate session
    let session = session_mgr.validate_token(&token)?;
    assert_eq!(session.user_id, user.id);

    // Create organization
    let org_service = OrganizationService::new(storage.clone());
    let org = org_service.create_organization(
        "Test Org".to_string(),
        "TEST".to_string(),
        "A test organization".to_string(),
        user.id.clone(),
    )?;

    // Add ships via fleet service convenience method
    let fleet_service = FleetService::new(storage.clone());
    let ship = fleet_service.create_ship(user.id.clone(), "Carrack".to_string(), "Anvil Aerospace".to_string())?;

    // Create operation
    let ops_service = OperationsService::new(storage.clone());
    let operation = ops_service.create_operation(
        org.id.clone(),
        "Test Operation".to_string(),
        "Description".to_string(),
        verseguy_plugin_operations::OperationType::Mining,
        chrono::Utc::now(),
        120,
        user.id.clone(),
    )?;

    // Verify everything
    assert!(org_service.get_organization(&org.id)?.is_some());
    assert!(fleet_service.get_ship(&user.id, &ship.id)?.is_some());
    assert!(ops_service.get_operation(&org.id, &operation.id)?.is_some());

    Ok(())
}

#[tokio::test]
async fn test_concurrent_operations() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let storage = Storage::open(temp_dir.path())?;
    let auth = LocalAuth::new(storage.clone());

    // Create multiple users concurrently
    let mut handles = vec![];

    for i in 0..10 {
        let auth_clone = auth.clone();
        let handle = tokio::spawn(async move {
            auth_clone.register(format!("user{}", i), "password123".to_string()).await
        });
        handles.push(handle);
    }

    // Wait for all
    for handle in handles {
        handle.await??;
    }

    Ok(())
}
