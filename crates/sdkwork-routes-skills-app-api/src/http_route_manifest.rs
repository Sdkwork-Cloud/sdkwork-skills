use sdkwork_web_core::{HttpMethod, HttpRoute, HttpRouteManifest};

const HTTP_ROUTES: &[HttpRoute] = &[
    HttpRoute::dual_token(
        HttpMethod::Get,
        "/app/v3/api/skills",
        "skills",
        "skills.list",
    ),
    HttpRoute::dual_token(
        HttpMethod::Get,
        "/app/v3/api/skills/{skillKey}",
        "skills",
        "skills.retrieve",
    ),
    HttpRoute::dual_token(
        HttpMethod::Get,
        "/app/v3/api/skill_packages",
        "skills",
        "skillPackages.list",
    ),
    HttpRoute::dual_token(
        HttpMethod::Get,
        "/app/v3/api/skill_packages/{skillId}",
        "skills",
        "skillPackages.retrieve",
    ),
    HttpRoute::dual_token(
        HttpMethod::Get,
        "/app/v3/api/categories",
        "skills",
        "categories.list",
    ),
    HttpRoute::dual_token(
        HttpMethod::Post,
        "/app/v3/api/user/skills/install",
        "skills",
        "userSkills.install",
    ),
];

pub fn app_route_manifest() -> HttpRouteManifest {
    HttpRouteManifest::new(HTTP_ROUTES)
}
