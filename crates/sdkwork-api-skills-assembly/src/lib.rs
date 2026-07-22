//! API assembly for sdkwork-skills.
//! Application bootstrap lives in `bootstrap.rs`; route inventory is in `assembly-manifest.json`.
// SDKWORK-ASSEMBLY-LIB-CUSTOM

mod bootstrap;
mod generated;

pub use bootstrap::{
    assemble_api_router, assemble_api_router_from_env, assemble_app_surface_router,
    assemble_backend_surface_router, ApiAssembly, SkillsReadiness,
};

pub fn assembly_route_count() -> usize {
    generated::ROUTE_CRATE_COUNT
}
