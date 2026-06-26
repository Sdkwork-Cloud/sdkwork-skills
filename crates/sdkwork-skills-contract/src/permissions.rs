pub const PERM_CATEGORY_MANAGE: &str = "skills.admin.category.manage";
pub const PERM_PACKAGE_MANAGE: &str = "skills.admin.package.manage";
pub const PERM_MARKETPLACE_READ: &str = "skills.admin.marketplace.read";

pub fn package_manage_permission_for_category(category_code: &str) -> String {
    format!("skills.admin.package.manage.{category_code}")
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
