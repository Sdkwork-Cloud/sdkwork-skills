use sdkwork_web_core::{HttpMethod, HttpRoute, HttpRouteManifest, RateLimitTier};

const fn skills_route(
    method: HttpMethod,
    path: &'static str,
    operation_id: &'static str,
    permission: &'static str,
) -> HttpRoute {
    HttpRoute::dual_token(method, path, "skills", operation_id)
        .with_required_permission(permission)
}

const HTTP_ROUTES: &[HttpRoute] = &[
    skills_route(
        HttpMethod::Get,
        "/app/v3/api/skills",
        "skills.list",
        "skills.marketplace.read",
    ),
    skills_route(
        HttpMethod::Get,
        "/app/v3/api/skills/{skillKey}",
        "skills.retrieve",
        "skills.marketplace.read",
    ),
    skills_route(
        HttpMethod::Get,
        "/app/v3/api/skill_packages",
        "skillPackages.list",
        "skills.marketplace.read",
    ),
    skills_route(
        HttpMethod::Get,
        "/app/v3/api/skill_packages/{skillId}",
        "skillPackages.retrieve",
        "skills.marketplace.read",
    ),
    skills_route(
        HttpMethod::Get,
        "/app/v3/api/categories",
        "categories.list",
        "skills.marketplace.read",
    ),
    skills_route(
        HttpMethod::Post,
        "/app/v3/api/user/skills/install",
        "userSkills.install",
        "skills.packages.install",
    ),
];

pub fn app_route_manifest() -> HttpRouteManifest {
    HttpRouteManifest::new(HTTP_ROUTES)
}
