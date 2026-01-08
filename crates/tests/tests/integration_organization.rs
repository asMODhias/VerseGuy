use verseguy_application::{ApplicationService, CreateOrganizationDto};
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
fn test_create_organization() {
    let ctx = TestContext::new();

    let dto = CreateOrganizationDto {
        name: "Test Organization".to_string(),
        tag: "TEST".to_string(),
        description: "A test organization".to_string(),
    };

    let result = ctx
        .app_service
        .create_organization(dto, ctx.user_id.clone());
    assert!(result.is_ok());
    let org = match result {
        Ok(o) => o,
        Err(_) => panic!("create_organization returned Err"),
    };
    assert_eq!(org.name, "Test Organization");
}
