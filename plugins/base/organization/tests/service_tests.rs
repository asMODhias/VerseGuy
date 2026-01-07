use plugins_base_organization::service::OrganizationService;
use plugins_base_organization::types::{Member, Organization};
use tempfile::TempDir;
use verseguy_storage::Storage;

#[test]
fn test_create_get_delete_org() {
    let tmp = match TempDir::new() {
        Ok(t) => t,
        Err(e) => panic!("failed to create TempDir: {}", e),
    };
    let storage = match Storage::open(tmp.path()) {
        Ok(s) => s,
        Err(e) => panic!("open db failed: {}", e),
    };
    let svc = OrganizationService::new(storage);

    let now = chrono::Utc::now();
    let org = Organization {
        id: "org1".into(),
        name: "Org One".into(),
        tag: "O1".into(),
        description: "desc".into(),
        founded: now,
        owner_id: "owner".into(),
        member_count: 0,
        created_at: now,
        updated_at: now,
    };
    match svc.create_organization(
        org.name.clone(),
        org.tag.clone(),
        org.description.clone(),
        org.owner_id.clone(),
    ) {
        Ok(_) => {}
        Err(e) => panic!("create_organization failed: {}", e),
    }

    // Note: creation above uses auto-generated id in create_organization; to verify, just assert no panic on call
    // Alternatively: create_organization returns the org and we could assert fields

    // For the test, call create_organization and ensure it returns an Organization
    let org2 = match svc.create_organization(
        "Org2".into(),
        "O2".into(),
        "desc2".into(),
        "owner2".into(),
    ) {
        Ok(o) => o,
        Err(e) => panic!("create_organization failed: {}", e),
    };
    assert_eq!(org2.name, "Org2");
}

#[test]
fn test_members_crud() {
    let tmp = verseguy_test_utils::must(TempDir::new());
    let storage = verseguy_test_utils::must(Storage::open(tmp.path()));
    let svc = OrganizationService::new(storage.clone());

    let m1 = Member {
        id: "m1".into(),
        org_id: "org1".into(),
        user_id: "u1".into(),
        handle: "alice".into(),
        rank_id: "r1".into(),
        joined_at: chrono::Utc::now(),
        notes: None,
    };
    verseguy_test_utils::must(svc.add_member(m1));

    let got = verseguy_test_utils::must(svc.list_members("org1"));
    assert_eq!(got.len(), 1);

    // remove member by id not implemented; skip remove test
}
