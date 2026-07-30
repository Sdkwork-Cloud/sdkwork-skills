mod handlers;
pub mod http_route_manifest;
mod mapper;
mod paths;
mod ports;
mod routes;

pub use handlers::BackendState;
pub use http_route_manifest::backend_route_manifest;
pub use ports::SkillsBackendRequestContext;
pub use routes::{build_router, router};

use std::sync::Arc;

use axum::Router;
use sdkwork_intelligence_skills_service::{SkillsRepository, SkillsService};
use sdkwork_web_core::HttpRouteManifest;

pub fn gateway_route_manifest() -> HttpRouteManifest {
    backend_route_manifest()
}

pub fn gateway_mount<R>(service: Arc<SkillsService<R>>) -> Router
where
    R: SkillsRepository + Clone + Send + Sync + 'static,
{
    build_router(service)
}
