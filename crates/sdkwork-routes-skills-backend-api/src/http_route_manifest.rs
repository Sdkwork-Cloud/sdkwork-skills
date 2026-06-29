use sdkwork_web_core::{HttpMethod, HttpRoute, HttpRouteManifest, RateLimitTier};

const fn abuse_sensitive_route(
    method: HttpMethod,
    path: &'static str,
    tag: &'static str,
    operation_id: &'static str,
) -> HttpRoute {
    HttpRoute::dual_token(method, path, tag, operation_id)
        .with_rate_limit_tier(RateLimitTier::AuthCritical)
}

const HTTP_ROUTES: &[HttpRoute] = &[
    HttpRoute::dual_token(
        HttpMethod::Get,
        "/backend/v3/api/skill",
        "skills-admin",
        "skills.management.list",
    ),
    HttpRoute::dual_token(
        HttpMethod::Get,
        "/backend/v3/api/skill/package",
        "skills-admin",
        "skillPackages.management.list",
    ),
    HttpRoute::dual_token(
        HttpMethod::Post,
        "/backend/v3/api/skill/package",
        "skills-admin",
        "skillPackages.create",
    ),
    HttpRoute::dual_token(
        HttpMethod::Put,
        "/backend/v3/api/skill/package/{skillId}",
        "skills-admin",
        "skillPackages.update",
    ),
    abuse_sensitive_route(
        HttpMethod::Delete,
        "/backend/v3/api/skill/package/{skillId}",
        "skills-admin",
        "skillPackages.delete",
    ),
    HttpRoute::dual_token(
        HttpMethod::Get,
        "/backend/v3/api/category",
        "skills-admin",
        "categories.management.list",
    ),
    HttpRoute::dual_token(
        HttpMethod::Post,
        "/backend/v3/api/category",
        "skills-admin",
        "categories.create",
    ),
    HttpRoute::dual_token(
        HttpMethod::Put,
        "/backend/v3/api/category/{categoryId}",
        "skills-admin",
        "categories.update",
    ),
];

pub fn backend_route_manifest() -> HttpRouteManifest {
    HttpRouteManifest::new(HTTP_ROUTES)
}
