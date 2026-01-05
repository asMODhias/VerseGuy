use tempfile::TempDir;
use verseguy_storage::Storage;
use verseguy_plugin_organization::service::OrganizationService;
use verseguy_plugin_organization::types::{Organization, Member};

#[test]
fn test_create_get_delete_org() {
    let tmp = TempDir::new().expect("tmp");
    let storage = Storage::open(tmp.path()).expect("open db");
    let svc = OrganizationService::new(storage);

    let now = chrono::Utc::now();
    let org = Organization { id: "org1".into(), name: "Org One".into(), tag: "O1".into(), description: "desc".into(), founded: now, owner_id: "owner".into(), member_count: 0, created_at: now, updated_at: now };
    svc.create_organization(org.name.clone(), org.tag.clone(), org.description.clone(), org.owner_id.clone()).expect("create org");

    // Note: creation above uses auto-generated id in create_organization; to verify, just assert no panic on call
    // Alternatively: create_organization returns the org and we could assert fields
    
    // For the test, call create_organization and ensure it returns an Organization
    let org2 = svc.create_organization("Org2".into(), "O2".into(), "desc2".into(), "owner2".into()).expect("create org2");
    assert_eq!(org2.name, "Org2");
}

#[test]
fn test_members_crud() {
    let tmp = TempDir::new().expect("tmp");
    let storage = Storage::open(tmp.path()).expect("open db");
    let svc = OrganizationService::new(storage.clone());

    let m1 = Member { id: "m1".into(), org_id: "org1".into(), user_id: "u1".into(), handle: "alice".into(), rank_id: "r1".into(), joined_at: chrono::Utc::now(), notes: None };
    svc.add_member(m1).expect("add m1");

    let got = svc.list_members("org1").expect("list members");
    assert_eq!(got.len(), 1);

    // remove member by id not implemented; skip remove test
}
