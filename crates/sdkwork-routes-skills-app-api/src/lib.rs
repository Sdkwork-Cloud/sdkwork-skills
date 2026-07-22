use std::sync::Arc;

use sdkwork_intelligence_skills_service::{SkillsRepository, SkillsService};
use sdkwork_web_core::HttpRouteManifest;

mod handlers;
pub mod http_route_manifest;
mod mapper;
mod paths;
mod ports;
mod routes;
mod web_bootstrap;

pub use handlers::AppState;
pub use http_route_manifest::app_route_manifest;
pub use ports::{
    DenyExternalInstallationTargets, SkillInstallationTargetAuthorizer, SkillsAppRequestContext,
};
pub use routes::{build_router, build_router_with_target_authorizer, router};
pub use web_bootstrap::{
    skills_public_path_prefixes, wrap_router_with_web_framework,
    wrap_router_with_web_framework_from_env,
};

pub async fn build_router_with_web_framework_from_env<R>(
    service: Arc<SkillsService<R>>,
) -> axum::Router
where
    R: SkillsRepository + Clone + Send + Sync + 'static,
{
    wrap_router_with_web_framework_from_env(build_router(service)).await
}

pub fn gateway_route_manifest() -> HttpRouteManifest {
    app_route_manifest()
}

pub async fn gateway_mount<R>(service: Arc<SkillsService<R>>) -> axum::Router
where
    R: SkillsRepository + Clone + Send + Sync + 'static,
{
    build_router_with_web_framework_from_env(service).await
}
