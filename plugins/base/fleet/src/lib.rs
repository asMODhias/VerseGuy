pub mod types;
pub mod service;

pub use service::FleetService;

use std::ffi::CString;
use std::os::raw::{c_char, c_void};

pub fn example() -> String {
    "fleet plugin loaded".to_string()
}

// Simple Rust representation of the C IPlugin interface
#[repr(C)]
pub struct IPlugin {
    pub get_id: Option<extern "C" fn(*mut c_void) -> *const c_char>,
    pub get_name: Option<extern "C" fn(*mut c_void) -> *const c_char>,
    pub get_version: Option<extern "C" fn(*mut c_void) -> *const c_char>,
    pub get_required_capabilities: Option<extern "C" fn(*mut c_void) -> u64>,
    pub initialize: Option<extern "C" fn(*mut c_void, *mut c_void) -> bool>,
    pub shutdown: Option<extern "C" fn(*mut c_void)> ,
    pub instance: *mut c_void,
}

pub struct FleetPlugin {
    id: CString,
    name: CString,
    version: CString,
    host: *mut c_void,
}

impl FleetPlugin {
    pub fn new() -> Self {
        Self {
            id: CString::new("org.verseguy.fleet").expect("invalid plugin id string"),
            name: CString::new("Fleet Management").expect("invalid plugin name string"),
            version: CString::new("1.0.0").expect("invalid plugin version string"),
            host: std::ptr::null_mut(),
        }
    }
}

impl Default for FleetPlugin {
    fn default() -> Self {
        Self::new()
    }
}

extern "C" fn get_id_impl(instance: *mut c_void) -> *const c_char {
    if instance.is_null() {
        return std::ptr::null();
    }
    let plugin = unsafe { &*(instance as *mut FleetPlugin) };
    plugin.id.as_ptr()
}

extern "C" fn get_name_impl(instance: *mut c_void) -> *const c_char {
    if instance.is_null() {
        return std::ptr::null();
    }
    let plugin = unsafe { &*(instance as *mut FleetPlugin) };
    plugin.name.as_ptr()
}

extern "C" fn get_version_impl(instance: *mut c_void) -> *const c_char {
    if instance.is_null() {
        return std::ptr::null();
    }
    let plugin = unsafe { &*(instance as *mut FleetPlugin) };
    plugin.version.as_ptr()
}

extern "C" fn get_required_capabilities_impl(_instance: *mut c_void) -> u64 {
    // example: requires storage read/write and UI panel
    (1u64 << 0) | (1u64 << 1) | (1u64 << 4)
}

extern "C" fn initialize_impl(instance: *mut c_void, _host: *mut c_void) -> bool {
    if instance.is_null() {
        return false;
    }
    let plugin = unsafe { &mut *(instance as *mut FleetPlugin) };
    plugin.host = _host;
    true
}

extern "C" fn shutdown_impl(_instance: *mut c_void) {
    // no-op for now
}

#[unsafe(export_name = "PluginInit")]
pub extern "C" fn PluginInit() -> *mut IPlugin {
    let plugin = Box::new(FleetPlugin::new());
    let instance_ptr = Box::into_raw(plugin) as *mut c_void;

    let interface = Box::new(IPlugin {
        get_id: Some(get_id_impl),
        get_name: Some(get_name_impl),
        get_version: Some(get_version_impl),
        get_required_capabilities: Some(get_required_capabilities_impl),
        initialize: Some(initialize_impl),
        shutdown: Some(shutdown_impl),
        instance: instance_ptr,
    });

    Box::into_raw(interface)
}

#[cfg(test)]
mod tests {
    use super::types::Ship;
    use chrono::Utc;

    #[test]
    fn types_roundtrip() {
        let now = Utc::now();
        let ship = Ship {
            id: "s1".into(),
            owner_id: "o1".into(),
            model: "Carrack".into(),
            manufacturer: "Anvil".into(),
            name: Some("MyShip".into()),
            pledge_date: Some(now),
            cost: Some(250.0),
            insurance: super::types::Insurance::LTI,
            status: super::types::ShipStatus::Available,
            location: Some("Port Olisar".into()),
            created_at: now,
            updated_at: now,
        };
        let json = serde_json::to_string(&ship).expect("serialize");
        let ship2: Ship = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(ship.id, ship2.id);
    }
}
