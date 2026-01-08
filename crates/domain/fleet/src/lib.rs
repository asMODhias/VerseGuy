#![allow(clippy::disallowed_methods)]
//! Fleet Domain (Aggregate Root, Entities, Value Objects)
//!
//! Basic skeleton for TEIL 10: Fleet domain.

pub mod aggregate;
pub mod entity;
pub mod event;
pub mod repo;
pub mod service;
pub mod value_object;

pub use entity::{Fleet, Ship};
pub use event::FleetEvent;
pub use value_object::{Loadout, ShipType};

pub type Result<T> = std::result::Result<T, verseguy_shared_error::AppError>;
