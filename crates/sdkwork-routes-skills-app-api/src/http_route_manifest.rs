use sdkwork_web_core::{HttpMethod, HttpRoute, HttpRouteManifest};

const HTTP_ROUTES: &[HttpRoute] = &[
    HttpRoute::dual_token(
        HttpMethod::Get,
        "/app/v3/api/skills",
        "skills",
        "skills.listSkills",
    ),
    HttpRoute::dual_token(
        HttpMethod::Get,
        "/app/v3/api/skills/{skillKey}",
        "skills",
        "skills.getSkill",
    ),
    HttpRoute::dual_token(
        HttpMethod::Get,
        "/app/v3/api/skill_packages",
        "skills",
        "skills.listSkillPackages",
    ),
    HttpRoute::dual_token(
        HttpMethod::Get,
        "/app/v3/api/skill_packages/{skillId}",
        "skills",
        "skills.getSkillPackage",
    ),
    HttpRoute::dual_token(
        HttpMethod::Get,
        "/app/v3/api/categories",
        "skills",
        "skills.listCategories",
    ),
    HttpRoute::dual_token(
        HttpMethod::Post,
        "/app/v3/api/user/skills/install",
        "skills",
        "skills.installSkill",
    ),
];

pub fn app_route_manifest() -> HttpRouteManifest {
    HttpRouteManifest::new(HTTP_ROUTES)
}
