use verseguy_application::{ApplicationService, CreateFleetDto};
use verseguy_tests::utils::*;

struct TestContext {
    _temp_dir: tempfile::TempDir,
    app_service: ApplicationService,
    user_id: String,
}

impl TestContext {
    fn new() -> Self {
        let temp_dir = create_test_storage();
        let app_service = ApplicationService::new();

        Self {
            _temp_dir: temp_dir,
            app_service,
            user_id: test_user_id(),
        }
    }
}

#[test]
fn test_create_fleet() {
    let ctx = TestContext::new();
    let dto = CreateFleetDto {
        organization_id: "org_123".to_string(),
        name: "Main Fleet".to_string(),
        description: "Primary combat fleet".to_string(),
    };

    let result = ctx.app_service.create_fleet(dto, ctx.user_id.clone());
    assert!(result.is_ok());
}

#[test]
fn test_get_fleet_after_create() {
    let ctx = TestContext::new();

    let dto = CreateFleetDto {
        organization_id: "org_1".to_string(),
        name: "Fleet A".to_string(),
        description: "".to_string(),
    };

    let fleet = match ctx.app_service.create_fleet(dto, ctx.user_id.clone()) {
        Ok(f) => f,
        Err(_) => panic!("create_fleet failed"),
    };

    let fetched = ctx.app_service.get_fleet(&fleet.id);
    assert!(fetched.is_ok());
    let fetched = match fetched {
        Ok(f) => f,
        Err(_) => panic!("get_fleet failed"),
    };
    assert_eq!(fetched.name, "Fleet A");
}

#[test]
fn test_add_ship_increments_counts() {
    let ctx = TestContext::new();
    let dto = CreateFleetDto {
        organization_id: "org_ship".to_string(),
        name: "ShipFleet".to_string(),
        description: "fleet for ships".to_string(),
    };

    let fleet = match ctx.app_service.create_fleet(dto, ctx.user_id.clone()) {
        Ok(f) => f,
        Err(e) => panic!("create_fleet failed: {:?}", e),
    };

    let add = verseguy_application::AddShipDto {
        fleet_id: fleet.id.clone(),
        manufacturer: "TestMaker".to_string(),
        name: "ShipOne".to_string(),
        variant: None,
        role: "scout".to_string(),
        owner_id: "ownerX".to_string(),
        crew_size: 3,
        cargo_capacity: 100,
    };

    assert!(ctx.app_service.add_ship(add, ctx.user_id.clone()).is_ok());

    let updated = match ctx.app_service.get_fleet(&fleet.id) {
        Ok(u) => u,
        Err(e) => panic!("get_fleet failed: {:?}", e),
    };
    assert_eq!(updated.ship_count, 1);
    assert_eq!(updated.total_crew, 3);
}

#[test]
fn test_add_ship_invalid_fleet_returns_err() {
    let ctx = TestContext::new();
    let add = verseguy_application::AddShipDto {
        fleet_id: "nonexistent".to_string(),
        manufacturer: "X".to_string(),
        name: "ShipX".to_string(),
        variant: None,
        role: "cargo".to_string(),
        owner_id: "ownerY".to_string(),
        crew_size: 2,
        cargo_capacity: 50,
    };

    assert!(ctx.app_service.add_ship(add, ctx.user_id.clone()).is_err());
}

#[test]
fn test_update_and_delete_fleet() {
    let ctx = TestContext::new();
    let dto = CreateFleetDto {
        organization_id: "org_2".to_string(),
        name: "Fleet B".to_string(),
        description: "".to_string(),
    };

    let fleet = match ctx.app_service.create_fleet(dto, ctx.user_id.clone()) {
        Ok(f) => f,
        Err(e) => panic!("create_fleet failed: {:?}", e),
    };

    let updated_res = ctx.app_service.update_fleet(
        &fleet.id,
        Some("Fleet B v2".to_string()),
        ctx.user_id.clone(),
    );
    let updated = match updated_res {
        Ok(u) => u,
        Err(e) => panic!("update_fleet failed: {:?}", e),
    };
    assert_eq!(updated.name, "Fleet B v2");

    let delete_res = ctx.app_service.delete_fleet(&fleet.id, ctx.user_id.clone());
    assert!(delete_res.is_ok(), "delete_fleet failed: {:?}", delete_res);
    assert!(ctx.app_service.get_fleet(&fleet.id).is_err());
}
