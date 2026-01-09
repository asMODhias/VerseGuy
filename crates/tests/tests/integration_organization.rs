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

#[test]
fn test_get_organization_after_create() {
    let ctx = TestContext::new();

    let dto = CreateOrganizationDto {
        name: "GetOrg".to_string(),
        tag: "GEO".to_string(),
        description: "".to_string(),
    };

    let org = match ctx
        .app_service
        .create_organization(dto, ctx.user_id.clone())
    {
        Ok(o) => o,
        Err(_) => panic!("create_organization failed"),
    };

    let fetched = ctx.app_service.get_organization(&org.id);
    assert!(fetched.is_ok());
    let fetched = match fetched {
        Ok(f) => f,
        Err(_) => panic!("get_organization failed"),
    };
    assert_eq!(fetched.name, "GetOrg");
}

#[test]
fn test_create_organization_invalid_tag() {
    let ctx = TestContext::new();

    let dto = CreateOrganizationDto {
        name: "Bad".to_string(),
        tag: "bad".to_string(), // lowercase not allowed per ApplicationService
        description: "".to_string(),
    };

    let result = ctx
        .app_service
        .create_organization(dto, ctx.user_id.clone());
    assert!(result.is_err());
}

#[test]
fn test_add_member_increments_member_count() {
    let ctx = TestContext::new();

    let dto = CreateOrganizationDto {
        name: "MembersOrg".to_string(),
        tag: "MEM".to_string(),
        description: "".to_string(),
    };

    let org = match ctx
        .app_service
        .create_organization(dto, ctx.user_id.clone())
    {
        Ok(o) => o,
        Err(_) => panic!("create_organization failed"),
    };

    let before = org.member_count;
    let add = verseguy_application::AddMemberDto {
        organization_id: org.id.clone(),
        user_id: "u1".to_string(),
    };

    let res = ctx.app_service.add_member(add, ctx.user_id.clone());
    assert!(res.is_ok());

    let fetched = match ctx.app_service.get_organization(&org.id) {
        Ok(f) => f,
        Err(_) => panic!("get_organization failed"),
    };
    assert_eq!(fetched.member_count, before + 1);
}

#[test]
fn test_treasury_deposit_withdraw() {
    let ctx = TestContext::new();

    let dto = CreateOrganizationDto {
        name: "BankOrg".to_string(),
        tag: "BANK".to_string(),
        description: "".to_string(),
    };

    let org = match ctx
        .app_service
        .create_organization(dto, ctx.user_id.clone())
    {
        Ok(o) => o,
        Err(_) => panic!("create_organization failed"),
    };

    let deposit = verseguy_application::TreasuryOperationDto {
        organization_id: org.id.clone(),
        amount: 1000,
        reason: None,
    };

    let res = ctx.app_service.deposit_funds(deposit, ctx.user_id.clone());
    assert!(res.is_ok());

    let fetched = match ctx.app_service.get_organization(&org.id) {
        Ok(f) => f,
        Err(_) => panic!("get_organization failed"),
    };
    assert_eq!(fetched.treasury_balance, 1000);

    let withdraw_ok = verseguy_application::TreasuryOperationDto {
        organization_id: org.id.clone(),
        amount: 500,
        reason: None,
    };
    let res = ctx
        .app_service
        .withdraw_funds(withdraw_ok, ctx.user_id.clone());
    assert!(res.is_ok());
    let fetched = match ctx.app_service.get_organization(&org.id) {
        Ok(f) => f,
        Err(_) => panic!("get_organization failed"),
    };
    assert_eq!(fetched.treasury_balance, 500);

    // overdraft should fail
    let withdraw_bad = verseguy_application::TreasuryOperationDto {
        organization_id: org.id.clone(),
        amount: 1000,
        reason: None,
    };
    assert!(ctx
        .app_service
        .withdraw_funds(withdraw_bad, ctx.user_id.clone())
        .is_err());
}

#[test]
fn test_get_nonexistent_org_returns_err() {
    let ctx = TestContext::new();

    let res = ctx.app_service.get_organization("does-not-exist");
    assert!(res.is_err());
}

#[test]
fn test_tag_not_unique_allows_create() {
    let ctx = TestContext::new();

    let dto1 = CreateOrganizationDto {
        name: "OrgA".to_string(),
        tag: "TAGX".to_string(),
        description: "".to_string(),
    };
    let dto2 = CreateOrganizationDto {
        name: "OrgB".to_string(),
        tag: "TAGX".to_string(), // same tag as OrgA
        description: "".to_string(),
    };

    let a = match ctx
        .app_service
        .create_organization(dto1, ctx.user_id.clone())
    {
        Ok(o) => o,
        Err(_) => panic!("create_organization failed for OrgA"),
    };

    let b = match ctx
        .app_service
        .create_organization(dto2, ctx.user_id.clone())
    {
        Ok(o) => o,
        Err(_) => panic!("create_organization failed for OrgB"),
    };

    assert_ne!(a.id, b.id);
    let fetched_a = match ctx.app_service.get_organization(&a.id) {
        Ok(f) => f,
        Err(_) => panic!("get_organization failed for OrgA"),
    };
    let fetched_b = match ctx.app_service.get_organization(&b.id) {
        Ok(f) => f,
        Err(_) => panic!("get_organization failed for OrgB"),
    };

    assert_eq!(fetched_a.tag, "TAGX");
    assert_eq!(fetched_b.tag, "TAGX");
}
