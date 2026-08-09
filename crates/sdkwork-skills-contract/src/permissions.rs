pub const PERM_MARKETPLACE_READ: &str = "skills.marketplace.read";
pub const PERM_PACKAGES_INSTALL: &str = "skills.packages.install";
/// Self-service create of skill packages owned by the authenticated user.
pub const PERM_PACKAGES_CREATE: &str = "skills.packages.create";
/// Self-service update of skill packages owned by the authenticated user.
pub const PERM_PACKAGES_UPDATE: &str = "skills.packages.update";
/// Self-service delete of skill packages owned by the authenticated user.
pub const PERM_PACKAGES_DELETE: &str = "skills.packages.delete";
/// Self-service artifact attachment to skill packages owned by the authenticated user.
pub const PERM_ARTIFACTS_CREATE: &str = "skills.artifacts.create";
pub const PERM_PACKAGES_MANAGE: &str = "skills.packages.manage";
pub const PERM_CATEGORIES_MANAGE: &str = "skills.categories.manage";
pub const PERM_CAPABILITIES_MANAGE: &str = "skills.capabilities.manage";
pub const PERM_ARTIFACTS_MANAGE: &str = "skills.artifacts.manage";
pub const PERM_INSTALLATIONS_READ: &str = "skills.installations.read";
pub const PERM_INSTALLATIONS_MANAGE: &str = "skills.installations.manage";

pub fn package_manage_permission_for_category(category_code: &str) -> String {
    format!("skills.packages.manage.{category_code}")
}

pub fn resolve_category_package_permission(
    category_code: &str,
    permission_code: Option<&str>,
) -> String {
    permission_code
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_string)
        .unwrap_or_else(|| package_manage_permission_for_category(category_code))
}
