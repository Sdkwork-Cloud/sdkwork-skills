//! Host-neutral API composition for sdkwork-skills.

use std::collections::BTreeSet;
use std::sync::Arc;

use axum::Router;
use sdkwork_intelligence_skills_repository_sqlx::SqlxSkillsRepository;
use sdkwork_intelligence_skills_service::SkillsService;
use sdkwork_routes_skills_app_api::{
    DenyExternalInstallationTargets, SkillInstallationTargetAuthorizer,
};
use sdkwork_skills_database_host::SkillsDatabaseHost;
use sdkwork_web_bootstrap::{ReadinessCheck, ReadinessFuture};
use sdkwork_web_core::{DomainContextInjector, HttpRoute, HttpRouteManifest};

use crate::{skills_api_route_manifest, SkillsDomainContextInjector};

pub struct ApiAssembly {
    pub router: Router,
    pub route_manifest: HttpRouteManifest,
    pub openapi: serde_json::Value,
    pub permission_catalog: Vec<&'static str>,
    pub readiness_check: Arc<dyn ReadinessCheck>,
    pub domain_context_injectors: Vec<Arc<dyn DomainContextInjector>>,
}

/// Skills-owned App API contribution for a gateway-selected runtime profile.
///
/// The router is raw and contains no Web Framework or infrastructure routes.
pub struct ApiAssemblyContribution {
    pub router: Router,
    pub route_manifest: HttpRouteManifest,
    pub openapi: serde_json::Value,
    pub permission_catalog: Vec<&'static str>,
    pub domain_context_injectors: Vec<Arc<dyn DomainContextInjector>>,
    pub readiness_check: Arc<dyn ReadinessCheck>,
}

#[derive(Clone)]
struct SkillsReadiness {
    database_host: Arc<SkillsDatabaseHost>,
}

impl SkillsReadiness {
    fn new(database_host: Arc<SkillsDatabaseHost>) -> Self {
        Self { database_host }
    }

    async fn check_dependencies(&self) -> Result<(), String> {
        if !self.database_host.node_lease().is_healthy() {
            return Err("skills Snowflake node lease is unhealthy".to_string());
        }
        let connected = self
            .database_host
            .pool()
            .test_connection()
            .await
            .map_err(|error| format!("skills database readiness check failed: {error}"))?;
        if connected {
            Ok(())
        } else {
            Err("skills database readiness query returned no row".to_string())
        }
    }
}

impl ReadinessCheck for SkillsReadiness {
    fn check(&self) -> ReadinessFuture<'_> {
        Box::pin(self.check_dependencies())
    }
}

pub async fn assemble_app_surface_router(
    service: Arc<SkillsService<SqlxSkillsRepository>>,
) -> Router {
    sdkwork_routes_skills_app_api::build_router(service)
}

pub async fn assemble_app_surface_router_with_target_authorizer(
    service: Arc<SkillsService<SqlxSkillsRepository>>,
    target_authorizer: Arc<dyn SkillInstallationTargetAuthorizer>,
) -> Router {
    sdkwork_routes_skills_app_api::build_router_with_target_authorizer(service, target_authorizer)
}

pub async fn assemble_backend_surface_router(
    service: Arc<SkillsService<SqlxSkillsRepository>>,
) -> Router {
    sdkwork_routes_skills_backend_api::build_router(service)
}

pub async fn assemble_api_router(
    service: Arc<SkillsService<SqlxSkillsRepository>>,
    database_host: Arc<SkillsDatabaseHost>,
) -> ApiAssembly {
    let app_router = assemble_app_surface_router(service.clone()).await;
    let backend_router = assemble_backend_surface_router(service).await;
    let route_manifest = skills_api_route_manifest();
    ApiAssembly {
        router: Router::new().merge(app_router).merge(backend_router),
        openapi: sdkwork_web_contract::build_openapi_document(
            "SDKWork Skills API",
            route_manifest.routes(),
        ),
        permission_catalog: permission_catalog(route_manifest.routes()),
        route_manifest,
        readiness_check: Arc::new(SkillsReadiness::new(database_host)),
        domain_context_injectors: vec![Arc::new(SkillsDomainContextInjector)],
    }
}

pub async fn assemble_api_router_from_env() -> Result<ApiAssembly, String> {
    let (service, database_host) = bootstrap_owner_runtime_from_env().await?;
    Ok(assemble_api_router(service, database_host).await)
}

/// Builds the Skills App API from the canonical owner repository and database lifecycle.
pub async fn assemble_app_api_contribution() -> Result<ApiAssemblyContribution, String> {
    assemble_app_api_contribution_with_target_authorizer(Arc::new(
        DenyExternalInstallationTargets,
    ))
    .await
}

/// Builds the Skills App API with a composing owner's Project/Agent scope authorizer.
///
/// Skills never imports Agents. The selected gateway supplies this port when
/// Project or Agent installation targets are enabled for that runtime profile.
pub async fn assemble_app_api_contribution_with_target_authorizer(
    target_authorizer: Arc<dyn SkillInstallationTargetAuthorizer>,
) -> Result<ApiAssemblyContribution, String> {
    let (service, database_host) = bootstrap_owner_runtime_from_env().await?;
    let route_manifest = sdkwork_routes_skills_app_api::app_route_manifest();
    let router =
        assemble_app_surface_router_with_target_authorizer(service, target_authorizer).await;
    Ok(ApiAssemblyContribution {
        router,
        openapi: sdkwork_web_contract::build_openapi_document(
            "SDKWork Skills App API",
            route_manifest.routes(),
        ),
        permission_catalog: permission_catalog(route_manifest.routes()),
        route_manifest,
        domain_context_injectors: vec![Arc::new(SkillsDomainContextInjector)],
        readiness_check: Arc::new(SkillsReadiness::new(database_host)),
    })
}

async fn bootstrap_owner_runtime_from_env() -> Result<
    (
        Arc<SkillsService<SqlxSkillsRepository>>,
        Arc<SkillsDatabaseHost>,
    ),
    String,
> {
    let database_host =
        Arc::new(sdkwork_skills_database_host::bootstrap_skills_database_from_env().await?);
    let repository = SqlxSkillsRepository::new(
        database_host.postgres_pool().clone(),
        database_host.id_generator().clone(),
    );
    let service = Arc::new(SkillsService::new(repository));
    Ok((service, database_host))
}

fn permission_catalog(routes: &[HttpRoute]) -> Vec<&'static str> {
    let mut permissions = BTreeSet::new();
    for route in routes {
        if let Some(permission) = route.required_permission {
            permissions.insert(permission);
        }
        if let Some(alternate_permissions) = route.alternate_permissions {
            permissions.extend(alternate_permissions.iter().copied());
        }
    }
    permissions.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use sdkwork_web_contract::{route_inventory_from_openapi, route_inventory_from_routes};

    #[test]
    fn app_api_manifest_openapi_and_auth_inventories_match() {
        let manifest = sdkwork_routes_skills_app_api::app_route_manifest();
        let openapi = sdkwork_web_contract::build_openapi_document(
            "SDKWork Skills App API",
            manifest.routes(),
        );
        assert_eq!(
            route_inventory_from_routes(manifest.routes()),
            route_inventory_from_openapi(&openapi).expect("valid Skills App API OpenAPI inventory")
        );
        assert_permission_catalog_matches(&manifest);
    }

    #[test]
    fn combined_manifest_openapi_and_auth_inventories_match() {
        let manifest = skills_api_route_manifest();
        let openapi =
            sdkwork_web_contract::build_openapi_document("SDKWork Skills API", manifest.routes());
        assert_eq!(
            route_inventory_from_routes(manifest.routes()),
            route_inventory_from_openapi(&openapi)
                .expect("valid combined Skills OpenAPI inventory")
        );
        assert_permission_catalog_matches(&manifest);
    }

    fn assert_permission_catalog_matches(manifest: &HttpRouteManifest) {
        let mut expected = manifest
            .routes()
            .iter()
            .flat_map(|route| {
                route
                    .required_permission
                    .into_iter()
                    .chain(route.alternate_permissions.into_iter().flatten().copied())
            })
            .collect::<Vec<_>>();
        expected.sort_unstable();
        expected.dedup();
        assert_eq!(expected, permission_catalog(manifest.routes()));
    }
}
