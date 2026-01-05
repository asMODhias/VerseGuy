use verseguy_storage::RocksDBStorage;
use tempfile::TempDir;
use plugins_base_organization::service::OrganizationService;
use plugins_base_organization::types::{Organization, Member};

#[test]
fn test_create_get_delete_org() {
    let tmp = TempDir::new().expect("tmp");
    let storage = RocksDBStorage::open(tmp.path()).expect("open db");
    let svc = OrganizationService::new(storage);

    let org = Organization { id: "org1".into(), name: "Org One".into(), tag: "O1".into(), member_count: 0 };
    svc.create_org(&org).expect("create org");

    let got = svc.get_org("org1").expect("get org");
    assert!(got.is_some());
    assert_eq!(got.unwrap().name, "Org One");

    svc.delete_org("org1").expect("delete org");
    let got2 = svc.get_org("org1").expect("get org");
    assert!(got2.is_none());
}

#[test]
fn test_members_crud() {
    let tmp = TempDir::new().expect("tmp");
    let storage = RocksDBStorage::open(tmp.path()).expect("open db");
    let svc = OrganizationService::new(storage);

    let m1 = Member { id: "m1".into(), org_id: "org1".into(), handle: "alice".into(), rank_id: "r1".into() };
    svc.add_member(&m1).expect("add m1");

    let got = svc.get_member("m1").expect("get m1");
    assert!(got.is_some());
    assert_eq!(got.unwrap().handle, "alice");

    svc.remove_member("m1").expect("remove m1");
    assert!(svc.get_member("m1").expect("get m1").is_none());
}
