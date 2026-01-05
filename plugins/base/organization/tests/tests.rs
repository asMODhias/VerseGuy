use tempfile::tempdir;
use verseguy_storage::Storage;
use verseguy_storage::schema::keys;
use verseguy_plugin_organization::service::OrganizationService;
use verseguy_plugin_organization::types::{Member, Permission, Rank};
use chrono::Utc;
use uuid::Uuid;

#[test]
fn test_create_organization() {
    let dir = tempdir().unwrap();
    let storage = Storage::open(dir.path()).unwrap();
    let svc = OrganizationService::new(storage);

    let org = svc.create_organization("Test Org".into(), "TST".into(), "desc".into(), "owner1".into()).unwrap();
    assert_eq!(org.name, "Test Org");
}

#[test]
fn test_add_member() {
    let dir = tempdir().unwrap();
    let storage = Storage::open(dir.path()).unwrap();
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

    svc.add_member(member).unwrap();
}

#[test]
fn test_list_members() {
    let dir = tempdir().unwrap();
    let storage = Storage::open(dir.path()).unwrap();
    let svc = OrganizationService::new(storage.clone());

    let m1 = Member { id: "m1".into(), org_id: "org1".into(), user_id: "u1".into(), handle: "alice".into(), rank_id: "r1".into(), joined_at: Utc::now(), notes: None };
    let m2 = Member { id: "m2".into(), org_id: "org1".into(), user_id: "u2".into(), handle: "bobby".into(), rank_id: "r1".into(), joined_at: Utc::now(), notes: None };
    svc.add_member(m1).unwrap();
    svc.add_member(m2).unwrap();

    let list = svc.list_members("org1").unwrap();
    assert_eq!(list.len(), 2);
}

#[test]
fn test_permissions() {
    let dir = tempdir().unwrap();
    let storage = Storage::open(dir.path()).unwrap();
    let svc = OrganizationService::new(storage.clone());

    // create rank with permissions
    let rank = Rank { id: "r1".into(), org_id: "org1".into(), name: "Admin".into(), level: 10, permissions: vec![Permission::ManageMembers], created_at: Utc::now() };
    storage.put(keys::rank("org1", &rank.id), &rank).unwrap();

    // create member assigned to rank
    let member = Member { id: "m1".into(), org_id: "org1".into(), user_id: "u1".into(), handle: "alice".into(), rank_id: "r1".into(), joined_at: Utc::now(), notes: None };
    svc.add_member(member).unwrap();

    let has_perm = svc.has_permission("u1", Permission::ManageMembers).unwrap();
    assert!(has_perm);
}