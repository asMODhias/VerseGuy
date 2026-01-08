#![allow(clippy::disallowed_methods)]
pub mod command;
pub mod entity;
pub mod query;
pub mod repo;
pub mod service;

pub use entity::AppAggregate;
pub use service::ApplicationService;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke() {
        let a = AppAggregate::new("a1".into(), "Demo".into());
        assert_eq!(a.id, "a1");
    }
}
