use verseguy_tests::utils::*;
use verseguy_application::{ApplicationService, CreateOperationDto};
use chrono::{Utc, Duration};

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
fn test_create_operation() {
    let ctx = TestContext::new();
    let start = chrono::Utc::now() + chrono::Duration::hours(1);
    let end = start + chrono::Duration::hours(2);

    let dto = CreateOperationDto {
        organization_id: "org_123".to_string(),
        name: "Mining Run Alpha".to_string(),
        description: "Quantanium mining".to_string(),
        operation_type: "mining".to_string(),
        scheduled_start: start,
        scheduled_end: end,
    };

    let result = ctx.app_service.create_operation(dto, ctx.user_id.clone());
    assert!(result.is_ok());
}
