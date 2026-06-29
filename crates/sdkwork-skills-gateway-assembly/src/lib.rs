//! Gateway assembly for sdkwork-skills.
//! Application bootstrap lives in `bootstrap.rs`; route inventory is in `assembly-manifest.json`.

mod bootstrap;
mod generated;

pub use bootstrap::{
    assemble_app_surface_router, assemble_application_business_router,
    assemble_backend_surface_router, ApplicationAssembly,
};

pub fn assembly_route_count() -> usize {
    generated::ROUTE_CRATE_COUNT
}

pub fn assembly_route_packages() -> &'static [&'static str] {
    generated::ROUTE_CRATE_PACKAGES
}
