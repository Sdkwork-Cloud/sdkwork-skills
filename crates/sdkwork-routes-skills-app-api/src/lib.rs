mod handlers;
pub mod http_route_manifest;
mod mapper;
mod paths;
mod ports;
mod routes;

pub use handlers::AppState;
pub use http_route_manifest::app_route_manifest;
pub use ports::{
    DenyExternalInstallationTargets, SkillInstallationTargetAuthorizer, SkillsAppRequestContext,
};
pub use routes::{build_router, build_router_with_target_authorizer, router};

use std::sync::Arc;

use axum::Router;
use sdkwork_intelligence_skills_service::{SkillsRepository, SkillsService};
use sdkwork_web_core::HttpRouteManifest;

pub fn gateway_route_manifest() -> HttpRouteManifest {
    app_route_manifest()
}

pub fn gateway_mount<R>(service: Arc<SkillsService<R>>) -> Router
where
    R: SkillsRepository + Clone + Send + Sync + 'static,
{
    build_router(service)
}
