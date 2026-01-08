#![allow(clippy::disallowed_methods)]
//! Operations domain skeleton

pub mod entity;
pub mod repo;
pub mod service;
pub mod value_object;

pub use entity::{Operation, Participant};
pub use value_object::OperationStatus;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke() {
        let o = Operation::new("op-1".into(), "TestOp".into(), None);
        assert_eq!(o.id, "op-1");
    }
}
