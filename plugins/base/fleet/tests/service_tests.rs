use plugins_base_fleet::service::FleetService;
use plugins_base_fleet::types::{Component, Insurance, Loadout, Ship, ShipStatus};
use tempfile::TempDir;
use verseguy_storage::Storage;

#[test]
fn test_add_get_list_ship() {
    let tmp = verseguy_test_utils::must(TempDir::new());
    let storage = verseguy_test_utils::must(Storage::open(tmp.path()));
    let svc = FleetService::new(storage.clone());

    let now = chrono::Utc::now();
    let ship = Ship {
        id: "".into(),
        owner_id: "owner1".into(),
        model: "Carrack".into(),
        manufacturer: "Anvil".into(),
        name: Some("Explorer".into()),
        pledge_date: Some(now),
        cost: Some(250.0),
        insurance: Insurance::LTI,
        status: ShipStatus::Available,
        location: Some("Port Olisar".into()),
        created_at: now,
        updated_at: now,
    };

    verseguy_test_utils::must(svc.add_ship(ship));

    let ships = verseguy_test_utils::must(svc.list_ships_for_owner("owner1"));
    assert_eq!(ships.len(), 1);

    let got = verseguy_test_utils::must(svc.get_ship("owner1", &ships[0].id));
    assert!(got.is_some());
}

#[test]
fn test_add_get_loadout() {
    let tmp = verseguy_test_utils::must(TempDir::new());
    let storage = verseguy_test_utils::must(Storage::open(tmp.path()));
    let svc = FleetService::new(storage.clone());

    let now = chrono::Utc::now();
    let ship = Ship {
        id: "s1".into(),
        owner_id: "owner2".into(),
        model: "F7A".into(),
        manufacturer: "Aegis".into(),
        name: None,
        pledge_date: Some(now),
        cost: None,
        insurance: Insurance::Standard,
        status: ShipStatus::Available,
        location: None,
        created_at: now,
        updated_at: now,
    };

    verseguy_test_utils::must(svc.add_ship(ship.clone()));

    let loadout = Loadout {
        id: "".into(),
        ship_id: ship.id.clone(),
        name: "Default".into(),
        components: vec![Component {
            slot: "Weapon_01".into(),
            item: "Ballistic".into(),
            manufacturer: Some("Aegis".into()),
        }],
        created_at: now,
        updated_at: now,
    };

    verseguy_test_utils::must(svc.add_loadout(loadout));

    let loadouts = verseguy_test_utils::must(svc.get_loadouts_for_ship(&ship.id));
    assert_eq!(loadouts.len(), 1);
}

#[test]
fn test_update_ship_changes_fields() {
    let tmp = verseguy_test_utils::must(TempDir::new());
    let storage = verseguy_test_utils::must(Storage::open(tmp.path()));
    let svc = FleetService::new(storage.clone());

    let now = chrono::Utc::now();
    let mut ship = Ship {
        id: "u1".into(),
        owner_id: "owner3".into(),
        model: "X1".into(),
        manufacturer: "Roberts".into(),
        name: Some("OldName".into()),
        pledge_date: None,
        cost: None,
        insurance: Insurance::Standard,
        status: ShipStatus::Available,
        location: None,
        created_at: now,
        updated_at: now,
    };

    verseguy_test_utils::must(svc.add_ship(ship.clone()));

    ship.name = Some("NewName".into());
    let updated = verseguy_test_utils::must(svc.update_ship(ship.clone()));
    assert_eq!(updated.name, Some("NewName".into()));

    let got = verseguy_test_utils::must(svc.get_ship("owner3", &ship.id));
    match got {
        Some(g) => assert_eq!(g.name, Some("NewName".into())),
        None => panic!("Expected ship to exist after update"),
    }
}

#[test]
fn test_delete_ship_removes_it() {
    let tmp = verseguy_test_utils::must(TempDir::new());
    let storage = verseguy_test_utils::must(Storage::open(tmp.path()));
    let svc = FleetService::new(storage.clone());

    let now = chrono::Utc::now();
    let ship = Ship {
        id: "d1".into(),
        owner_id: "owner4".into(),
        model: "M50".into(),
        manufacturer: "Origin".into(),
        name: None,
        pledge_date: None,
        cost: None,
        insurance: Insurance::Standard,
        status: ShipStatus::Available,
        location: None,
        created_at: now,
        updated_at: now,
    };

    verseguy_test_utils::must(svc.add_ship(ship.clone()));
    verseguy_test_utils::must(svc.delete_ship("owner4", &ship.id));

    let got = verseguy_test_utils::must(svc.get_ship("owner4", &ship.id));
    assert!(got.is_none());
}

#[test]
fn test_create_ship_convenience_returns_valid_ship() {
    let tmp = verseguy_test_utils::must(TempDir::new());
    let storage = verseguy_test_utils::must(Storage::open(tmp.path()));
    let svc = FleetService::new(storage.clone());

    let created = verseguy_test_utils::must(svc.create_ship(
        "owner5".into(),
        "ModelX".into(),
        "Maker".into(),
    ));
    let got = verseguy_test_utils::must(svc.get_ship("owner5", &created.id));
    match got {
        Some(g) => assert_eq!(g.id, created.id),
        None => panic!("Expected created ship to be retrievable"),
    }
}

#[test]
fn test_list_ships_for_owner_filters_by_owner() {
    let tmp = verseguy_test_utils::must(TempDir::new());
    let storage = verseguy_test_utils::must(Storage::open(tmp.path()));
    let svc = FleetService::new(storage.clone());

    let now = chrono::Utc::now();
    let ship_a = Ship {
        id: "a1".into(),
        owner_id: "ownerA".into(),
        model: "X1".into(),
        manufacturer: "Roberts".into(),
        name: None,
        pledge_date: None,
        cost: None,
        insurance: Insurance::Standard,
        status: ShipStatus::Available,
        location: None,
        created_at: now,
        updated_at: now,
    };
    let ship_b = Ship {
        id: "b1".into(),
        owner_id: "ownerB".into(),
        model: "Y1".into(),
        manufacturer: "Origin".into(),
        name: None,
        pledge_date: None,
        cost: None,
        insurance: Insurance::Standard,
        status: ShipStatus::Available,
        location: None,
        created_at: now,
        updated_at: now,
    };

    verseguy_test_utils::must(svc.add_ship(ship_a));
    verseguy_test_utils::must(svc.add_ship(ship_b));

    let a_ships = verseguy_test_utils::must(svc.list_ships_for_owner("ownerA"));
    assert_eq!(a_ships.len(), 1);
    assert_eq!(a_ships[0].owner_id, "ownerA");
}
