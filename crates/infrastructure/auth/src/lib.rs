//! verseguy-auth: Authentication infrastructure crate

pub mod user;
pub mod session;
pub mod oauth;

pub mod prelude {
    pub use crate::user::*;
    pub use crate::session::*;
    pub use crate::oauth::*;
}