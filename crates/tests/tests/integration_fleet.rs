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
