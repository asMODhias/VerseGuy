pub mod features;
pub mod validator;

pub use features::features_for_license;
pub use validator::validate_license;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LicenseTier {
    Free,
    Pro,
    Enterprise,
}
