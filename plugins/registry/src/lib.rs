//! Registry plugin

use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PluginInfo {
    pub id: String,
    pub name: String,
    pub version: String,
}

#[derive(Clone, Default)]
pub struct Registry {
    plugins: Arc<Mutex<Vec<PluginInfo>>>,
}

impl Registry {
    pub fn new() -> Self {
        Self {
            plugins: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn register(&self, p: PluginInfo) {
        let mut lock = match self.plugins.lock() {
            Ok(l) => l,
            Err(e) => panic!("plugins mutex poisoned: {}", e),
        };
        // replace existing with same id
        if let Some(pos) = lock.iter().position(|x| x.id == p.id) {
            lock[pos] = p;
        } else {
            lock.push(p);
        }
    }

    pub fn list(&self) -> Vec<PluginInfo> {
        let lock = match self.plugins.lock() {
            Ok(l) => l,
            Err(e) => panic!("plugins mutex poisoned: {}", e),
        };
        lock.clone()
    }

    pub fn find(&self, id: &str) -> Option<PluginInfo> {
        let lock = match self.plugins.lock() {
            Ok(l) => l,
            Err(e) => panic!("plugins mutex poisoned: {}", e),
        };
        lock.iter().find(|p| p.id == id).cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn register_and_list_plugins() {
        let r = Registry::new();
        let a = PluginInfo {
            id: "a".into(),
            name: "A".into(),
            version: "0.1".into(),
        };
        let b = PluginInfo {
            id: "b".into(),
            name: "B".into(),
            version: "0.1".into(),
        };
        r.register(a.clone());
        r.register(b.clone());
        let list = r.list();
        assert_eq!(list.len(), 2);
        assert!(list.contains(&a));
        assert!(list.contains(&b));
    }

    #[test]
    fn find_returns_correct_plugin() {
        let r = Registry::new();
        let a = PluginInfo {
            id: "x".into(),
            name: "X".into(),
            version: "1.2".into(),
        };
        r.register(a.clone());
        let found = match r.find("x") {
            Some(p) => p,
            None => panic!("should find plugin"),
        };
        assert_eq!(found, a);
        assert!(r.find("missing").is_none());
    }
}
