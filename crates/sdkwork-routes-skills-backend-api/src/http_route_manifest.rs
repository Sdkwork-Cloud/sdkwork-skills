use sdkwork_web_core::{HttpMethod, HttpRoute, HttpRouteManifest, RateLimitTier};

const fn skills_admin_route(
    method: HttpMethod,
    path: &'static str,
    operation_id: &'static str,
    permission: &'static str,
) -> HttpRoute {
    HttpRoute::dual_token(method, path, "skills-admin", operation_id)
        .with_required_permission(permission)
}

const fn skills_admin_abuse_route(
    method: HttpMethod,
    path: &'static str,
    operation_id: &'static str,
    permission: &'static str,
) -> HttpRoute {
    skills_admin_route(method, path, operation_id, permission)
        .with_rate_limit_tier(RateLimitTier::AuthCritical)
}

const HTTP_ROUTES: &[HttpRoute] = &[
    skills_admin_route(
        HttpMethod::Get,
        "/backend/v3/api/skill",
        "skills.management.list",
        "skills.marketplace.read",
    ),
    skills_admin_route(
        HttpMethod::Get,
        "/backend/v3/api/skill/package",
        "skillPackages.management.list",
        "skills.packages.manage",
    ),
    skills_admin_route(
        HttpMethod::Post,
        "/backend/v3/api/skill/package",
        "skillPackages.create",
        "skills.packages.manage",
    ),
    skills_admin_route(
        HttpMethod::Put,
        "/backend/v3/api/skill/package/{skillId}",
        "skillPackages.update",
        "skills.packages.manage",
    ),
    skills_admin_abuse_route(
        HttpMethod::Delete,
        "/backend/v3/api/skill/package/{skillId}",
        "skillPackages.delete",
        "skills.packages.manage",
    ),
    skills_admin_route(
        HttpMethod::Get,
        "/backend/v3/api/category",
        "categories.management.list",
        "skills.categories.manage",
    ),
    skills_admin_route(
        HttpMethod::Post,
        "/backend/v3/api/category",
        "categories.create",
        "skills.categories.manage",
    ),
    skills_admin_route(
        HttpMethod::Put,
        "/backend/v3/api/category/{categoryId}",
        "categories.update",
        "skills.categories.manage",
    ),
];

pub fn backend_route_manifest() -> HttpRouteManifest {
    HttpRouteManifest::new(HTTP_ROUTES)
}
