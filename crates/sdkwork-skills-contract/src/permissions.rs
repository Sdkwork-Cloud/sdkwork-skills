pub const PERM_MARKETPLACE_READ: &str = "skills.marketplace.read";
pub const PERM_PACKAGES_INSTALL: &str = "skills.packages.install";
pub const PERM_PACKAGES_MANAGE: &str = "skills.packages.manage";
pub const PERM_CATEGORIES_MANAGE: &str = "skills.categories.manage";

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
