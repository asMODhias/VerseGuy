pub mod service;
pub mod types;

use std::ffi::CString;
use std::os::raw::{c_char, c_void};

pub use service::OrganizationService;

pub fn example() -> String {
    "organization plugin loaded".to_string()
}

// Simple Rust representation of the C IPlugin interface
#[repr(C)]
pub struct IPlugin {
    pub get_id: Option<extern "C" fn(*mut c_void) -> *const c_char>,
    pub get_name: Option<extern "C" fn(*mut c_void) -> *const c_char>,
    pub get_version: Option<extern "C" fn(*mut c_void) -> *const c_char>,
    pub get_required_capabilities: Option<extern "C" fn(*mut c_void) -> u64>,
    pub initialize: Option<extern "C" fn(*mut c_void, *mut c_void) -> bool>,
    pub shutdown: Option<extern "C" fn(*mut c_void)>,
    pub instance: *mut c_void,
}

pub struct OrganizationPlugin {
    id: CString,
    name: CString,
    version: CString,
    host: *mut c_void,
}

impl OrganizationPlugin {
    pub fn new() -> Self {
        Self {
            id: match CString::new("org.verseguy.organization") {
                Ok(c) => c,
                Err(e) => panic!("invalid plugin id string: {}", e),
            },
            name: match CString::new("Organization Management") {
                Ok(c) => c,
                Err(e) => panic!("invalid plugin name string: {}", e),
            },
            version: match CString::new("2.0.0") {
                Ok(c) => c,
                Err(e) => panic!("invalid plugin version string: {}", e),
            },
            host: std::ptr::null_mut(),
        }
    }
}

impl Default for OrganizationPlugin {
    fn default() -> Self {
        Self::new()
    }
}

extern "C" fn get_id_impl(instance: *mut c_void) -> *const c_char {
    if instance.is_null() {
        return std::ptr::null();
    }
    let plugin = unsafe { &*(instance as *mut OrganizationPlugin) };
    plugin.id.as_ptr()
}

extern "C" fn get_name_impl(instance: *mut c_void) -> *const c_char {
    if instance.is_null() {
        return std::ptr::null();
    }
    let plugin = unsafe { &*(instance as *mut OrganizationPlugin) };
    plugin.name.as_ptr()
}

extern "C" fn get_version_impl(instance: *mut c_void) -> *const c_char {
    if instance.is_null() {
        return std::ptr::null();
    }
    let plugin = unsafe { &*(instance as *mut OrganizationPlugin) };
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
    let plugin = unsafe { &mut *(instance as *mut OrganizationPlugin) };
    plugin.host = _host;
    true
}

extern "C" fn shutdown_impl(_instance: *mut c_void) {
    // no-op for now
}

#[unsafe(no_mangle)]
pub extern "C" fn PluginInit() -> *mut IPlugin {
    let plugin = Box::new(OrganizationPlugin::new());
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
    use super::types::{Member, Organization, Permission, Rank};
    use chrono::Utc;

    #[test]
    fn types_roundtrip() {
        let now = Utc::now();
        let org = Organization {
            id: "org1".into(),
            name: "Org".into(),
            tag: "ORG".into(),
            description: "desc".into(),
            founded: now,
            owner_id: "owner".into(),
            member_count: 5,
            treasury_balance: 0,
            created_at: now,
            updated_at: now,
        };
        let mem = Member {
            id: "m1".into(),
            org_id: "org1".into(),
            user_id: "u1".into(),
            handle: "bob".into(),
            rank_id: "r1".into(),
            joined_at: now,
            notes: None,
        };
        let rank = Rank {
            id: "r1".into(),
            org_id: "org1".into(),
            name: "Admin".into(),
            level: 10,
            permissions: vec![Permission::ManageMembers],
            created_at: now,
        };
        assert_eq!(org.id, "org1");
        assert_eq!(mem.org_id, "org1");
        assert_eq!(rank.org_id, "org1");
    }
}
