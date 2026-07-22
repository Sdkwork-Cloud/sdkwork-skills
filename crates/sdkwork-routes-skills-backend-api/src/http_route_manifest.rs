use sdkwork_skills_contract::{
    OP_CREATE_ARTIFACT, OP_CREATE_CAPABILITY, OP_CREATE_CATEGORY, OP_CREATE_SKILL_PACKAGE,
    OP_DELETE_SKILL_PACKAGE, OP_GET_CAPABILITY, OP_GET_CATEGORY, OP_GET_SKILL_PACKAGE,
    OP_LIST_ARTIFACTS, OP_LIST_CAPABILITIES, OP_LIST_CATEGORIES, OP_LIST_SKILLS,
    OP_LIST_SKILL_PACKAGES, OP_UPDATE_CAPABILITY, OP_UPDATE_CATEGORY, OP_UPDATE_SKILL_PACKAGE,
    PERM_ARTIFACTS_MANAGE, PERM_CAPABILITIES_MANAGE, PERM_CATEGORIES_MANAGE, PERM_MARKETPLACE_READ,
    PERM_PACKAGES_MANAGE,
};
use sdkwork_web_core::{HttpMethod, HttpRoute, HttpRouteManifest, RateLimitTier};

const fn skills_admin_route(
    method: HttpMethod,
    path: &'static str,
    operation_id: &'static str,
    permission: &'static str,
) -> HttpRoute {
    HttpRoute::dual_token(method, path, "skills", operation_id).with_required_permission(permission)
}

const fn sensitive_skills_admin_route(
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
        "/backend/v3/api/skills",
        OP_LIST_SKILLS,
        PERM_MARKETPLACE_READ,
    ),
    skills_admin_route(
        HttpMethod::Get,
        "/backend/v3/api/skill_packages",
        OP_LIST_SKILL_PACKAGES,
        PERM_PACKAGES_MANAGE,
    ),
    skills_admin_route(
        HttpMethod::Post,
        "/backend/v3/api/skill_packages",
        OP_CREATE_SKILL_PACKAGE,
        PERM_PACKAGES_MANAGE,
    ),
    skills_admin_route(
        HttpMethod::Get,
        "/backend/v3/api/skill_packages/{packageId}",
        OP_GET_SKILL_PACKAGE,
        PERM_PACKAGES_MANAGE,
    ),
    skills_admin_route(
        HttpMethod::Patch,
        "/backend/v3/api/skill_packages/{packageId}",
        OP_UPDATE_SKILL_PACKAGE,
        PERM_PACKAGES_MANAGE,
    ),
    sensitive_skills_admin_route(
        HttpMethod::Delete,
        "/backend/v3/api/skill_packages/{packageId}",
        OP_DELETE_SKILL_PACKAGE,
        PERM_PACKAGES_MANAGE,
    ),
    skills_admin_route(
        HttpMethod::Get,
        "/backend/v3/api/skill_packages/{packageId}/artifacts",
        OP_LIST_ARTIFACTS,
        PERM_ARTIFACTS_MANAGE,
    ),
    skills_admin_route(
        HttpMethod::Post,
        "/backend/v3/api/skill_packages/{packageId}/artifacts",
        OP_CREATE_ARTIFACT,
        PERM_ARTIFACTS_MANAGE,
    ),
    skills_admin_route(
        HttpMethod::Get,
        "/backend/v3/api/skill_capabilities",
        OP_LIST_CAPABILITIES,
        PERM_CAPABILITIES_MANAGE,
    ),
    skills_admin_route(
        HttpMethod::Post,
        "/backend/v3/api/skill_capabilities",
        OP_CREATE_CAPABILITY,
        PERM_CAPABILITIES_MANAGE,
    ),
    skills_admin_route(
        HttpMethod::Get,
        "/backend/v3/api/skill_capabilities/{capabilityId}",
        OP_GET_CAPABILITY,
        PERM_CAPABILITIES_MANAGE,
    ),
    skills_admin_route(
        HttpMethod::Patch,
        "/backend/v3/api/skill_capabilities/{capabilityId}",
        OP_UPDATE_CAPABILITY,
        PERM_CAPABILITIES_MANAGE,
    ),
    skills_admin_route(
        HttpMethod::Get,
        "/backend/v3/api/skill_categories",
        OP_LIST_CATEGORIES,
        PERM_CATEGORIES_MANAGE,
    ),
    skills_admin_route(
        HttpMethod::Post,
        "/backend/v3/api/skill_categories",
        OP_CREATE_CATEGORY,
        PERM_CATEGORIES_MANAGE,
    ),
    skills_admin_route(
        HttpMethod::Get,
        "/backend/v3/api/skill_categories/{categoryId}",
        OP_GET_CATEGORY,
        PERM_CATEGORIES_MANAGE,
    ),
    skills_admin_route(
        HttpMethod::Patch,
        "/backend/v3/api/skill_categories/{categoryId}",
        OP_UPDATE_CATEGORY,
        PERM_CATEGORIES_MANAGE,
    ),
];

pub fn backend_route_manifest() -> HttpRouteManifest {
    HttpRouteManifest::new(HTTP_ROUTES)
}
