use crate::LicenseTier;

pub fn features_for_license(tier: LicenseTier) -> Vec<&'static str> {
    match tier {
        LicenseTier::Free => vec!["core"],
        LicenseTier::Pro => vec!["core", "advanced_analytics", "recruitment"],
        LicenseTier::Enterprise => vec!["core", "advanced_analytics", "rbac", "multi_org"],
    }
}
