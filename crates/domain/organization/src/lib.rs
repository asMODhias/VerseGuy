#![allow(clippy::disallowed_methods)]
//! Organization Domain (Aggregate Root, Entities, Value Objects)

pub mod aggregate;
pub mod entity;
pub mod event;
pub mod repo;
pub mod service;
pub mod value_object;

pub use aggregate::Organization;
pub use entity::{Member, MemberStatus};
pub use event::OrganizationEvent;
pub use service::OrganizationService;
pub use value_object::{Currency, OrganizationTag, Rank, Treasury};
pub type Result<T> = std::result::Result<T, verseguy_shared_error::AppError>;
