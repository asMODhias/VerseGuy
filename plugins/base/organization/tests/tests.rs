use chrono::Utc;
use plugins_base_organization::service::OrganizationService;
use plugins_base_organization::types::{Member, Permission, Rank};
use tempfile::tempdir;
use uuid::Uuid;
use verseguy_storage::Storage;
use verseguy_storage::schema::keys;
use verseguy_test_utils::must;

#[test]
fn test_create_organization() {
    let dir = must(tempdir());
    let storage = must(Storage::open(dir.path()));
    let svc = OrganizationService::new(storage);

    let org = match svc.create_organization(
        "Test Org".into(),
        "TST".into(),
        "desc".into(),
        "owner1".into(),
    ) {
        Ok(o) => o,
        Err(e) => panic!("create_organization failed: {}", e),
    };
    assert_eq!(org.name, "Test Org");
}

#[test]
fn test_add_member() {
    let dir = must(tempdir());
    let storage = must(Storage::open(dir.path()));
    let svc = OrganizationService::new(storage.clone());

    let member = Member {
        id: Uuid::new_v4().to_string(),
        org_id: "org1".into(),
        user_id: "u1".into(),
        handle: "bob".into(),
        rank_id: "r1".into(),
        joined_at: Utc::now(),
        notes: None,
    };

    match svc.add_member(member) {
        Ok(_) => {}
        Err(e) => panic!("add_member failed: {}", e),
    }
}

#[test]
fn test_list_members() {
    let dir = must(tempdir());
    let storage = must(Storage::open(dir.path()));
    let svc = OrganizationService::new(storage.clone());

    let m1 = Member {
        id: "m1".into(),
        org_id: "org1".into(),
        user_id: "u1".into(),
        handle: "alice".into(),
        rank_id: "r1".into(),
        joined_at: Utc::now(),
        notes: None,
    };
    let m2 = Member {
        id: "m2".into(),
        org_id: "org1".into(),
        user_id: "u2".into(),
        handle: "bobby".into(),
        rank_id: "r1".into(),
        joined_at: Utc::now(),
        notes: None,
    };
    match svc.add_member(m1) {
        Ok(_) => {}
        Err(e) => panic!("add_member failed: {}", e),
    }
    match svc.add_member(m2) {
        Ok(_) => {}
        Err(e) => panic!("add_member failed: {}", e),
    }

    let list = match svc.list_members("org1") {
        Ok(l) => l,
        Err(e) => panic!("list_members failed: {}", e),
    };
    assert_eq!(list.len(), 2);
}

#[test]
fn test_permissions() {
    let dir = must(tempdir());
    let storage = must(Storage::open(dir.path()));
    let svc = OrganizationService::new(storage.clone());

    // create rank with permissions
    let rank = Rank {
        id: "r1".into(),
        org_id: "org1".into(),
        name: "Admin".into(),
        level: 10,
        permissions: vec![Permission::ManageMembers],
        created_at: Utc::now(),
    };
    match storage.put(keys::rank("org1", &rank.id), &rank) {
        Ok(_) => {}
        Err(e) => panic!("storage.put failed: {}", e),
    };

    // create member assigned to rank
    let member = Member {
        id: "m1".into(),
        org_id: "org1".into(),
        user_id: "u1".into(),
        handle: "alice".into(),
        rank_id: "r1".into(),
        joined_at: Utc::now(),
        notes: None,
    };
    match svc.add_member(member) {
        Ok(_) => {}
        Err(e) => panic!("add_member failed: {}", e),
    };

    let has_perm = match svc.has_permission("u1", Permission::ManageMembers) {
        Ok(h) => h,
        Err(e) => panic!("has_permission failed: {}", e),
    };
    assert!(has_perm);
}
