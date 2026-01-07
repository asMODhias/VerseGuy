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
