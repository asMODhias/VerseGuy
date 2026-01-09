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
