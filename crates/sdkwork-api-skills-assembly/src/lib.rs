//! API assembly for sdkwork-skills.
//! Application bootstrap lives in `bootstrap.rs`; route inventory is in `assembly-manifest.json`.
// SDKWORK-ASSEMBLY-LIB-CUSTOM

mod bootstrap;
mod context;
mod generated;
mod route_manifest;

pub use bootstrap::{
    assemble_api_router, assemble_api_router_from_env, assemble_api_router_with_pool,
    assemble_app_api_contribution, assemble_app_api_contribution_with_pool,
    assemble_app_api_contribution_with_target_authorizer, assemble_app_surface_router,
    assemble_app_surface_router_with_target_authorizer, assemble_backend_surface_router,
    bootstrap_database_from_env, ApiAssembly, ApiAssemblyContribution,
};
pub use context::SkillsDomainContextInjector;
pub use route_manifest::skills_api_route_manifest;
pub use sdkwork_routes_skills_app_api::SkillInstallationTargetAuthorizer;

pub fn assembly_route_count() -> usize {
    generated::ROUTE_CRATE_COUNT
}
