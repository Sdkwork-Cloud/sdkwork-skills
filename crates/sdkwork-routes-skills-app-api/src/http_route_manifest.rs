use sdkwork_skills_contract::{
    OP_CREATE_ARTIFACT, OP_CREATE_INSTALLATION, OP_CREATE_SKILL_PACKAGE, OP_DELETE_SKILL_PACKAGE,
    OP_GET_SKILL, OP_GET_SKILL_PACKAGE, OP_LIST_ARTIFACTS, OP_LIST_CATEGORIES,
    OP_LIST_INSTALLATIONS, OP_LIST_OWNED_SKILL_PACKAGES, OP_LIST_SKILLS, OP_LIST_SKILL_PACKAGES,
    OP_UPDATE_SKILL_PACKAGE, PERM_ARTIFACTS_CREATE, PERM_INSTALLATIONS_READ,
    PERM_MARKETPLACE_READ, PERM_PACKAGES_CREATE, PERM_PACKAGES_DELETE, PERM_PACKAGES_INSTALL,
    PERM_PACKAGES_UPDATE,
};
use sdkwork_web_core::{HttpMethod, HttpRoute, HttpRouteManifest};

const fn skills_route(
    method: HttpMethod,
    path: &'static str,
    operation_id: &'static str,
    permission: &'static str,
) -> HttpRoute {
    HttpRoute::dual_token(method, path, "skills", operation_id).with_required_permission(permission)
}

const HTTP_ROUTES: &[HttpRoute] = &[
    skills_route(
        HttpMethod::Get,
        "/app/v3/api/skills",
        OP_LIST_SKILLS,
        PERM_MARKETPLACE_READ,
    ),
    skills_route(
        HttpMethod::Get,
        "/app/v3/api/skills/{skillKey}",
        OP_GET_SKILL,
        PERM_MARKETPLACE_READ,
    ),
    skills_route(
        HttpMethod::Get,
        "/app/v3/api/skill_packages",
        OP_LIST_SKILL_PACKAGES,
        PERM_MARKETPLACE_READ,
    ),
    skills_route(
        HttpMethod::Post,
        "/app/v3/api/skill_packages",
        OP_CREATE_SKILL_PACKAGE,
        PERM_PACKAGES_CREATE,
    ),
    skills_route(
        HttpMethod::Get,
        "/app/v3/api/skill_packages/owned",
        OP_LIST_OWNED_SKILL_PACKAGES,
        PERM_MARKETPLACE_READ,
    ),
    skills_route(
        HttpMethod::Get,
        "/app/v3/api/skill_packages/{packageId}",
        OP_GET_SKILL_PACKAGE,
        PERM_MARKETPLACE_READ,
    ),
    skills_route(
        HttpMethod::Patch,
        "/app/v3/api/skill_packages/{packageId}",
        OP_UPDATE_SKILL_PACKAGE,
        PERM_PACKAGES_UPDATE,
    ),
    skills_route(
        HttpMethod::Delete,
        "/app/v3/api/skill_packages/{packageId}",
        OP_DELETE_SKILL_PACKAGE,
        PERM_PACKAGES_DELETE,
    ),
    skills_route(
        HttpMethod::Get,
        "/app/v3/api/skill_packages/{packageId}/artifacts",
        OP_LIST_ARTIFACTS,
        PERM_MARKETPLACE_READ,
    ),
    skills_route(
        HttpMethod::Post,
        "/app/v3/api/skill_packages/{packageId}/artifacts",
        OP_CREATE_ARTIFACT,
        PERM_ARTIFACTS_CREATE,
    ),
    skills_route(
        HttpMethod::Get,
        "/app/v3/api/skill_categories",
        OP_LIST_CATEGORIES,
        PERM_MARKETPLACE_READ,
    ),
    skills_route(
        HttpMethod::Post,
        "/app/v3/api/skill_packages/{packageId}/installations",
        OP_CREATE_INSTALLATION,
        PERM_PACKAGES_INSTALL,
    ),
    skills_route(
        HttpMethod::Get,
        "/app/v3/api/skill_installations",
        OP_LIST_INSTALLATIONS,
        PERM_INSTALLATIONS_READ,
    ),
];

pub fn app_route_manifest() -> HttpRouteManifest {
    HttpRouteManifest::new(HTTP_ROUTES)
}
