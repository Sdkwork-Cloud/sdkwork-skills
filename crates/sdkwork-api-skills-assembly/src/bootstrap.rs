//! Gateway bootstrap for sdkwork-skills.
//! Split-surface hosts call surface-specific assemblers; unified ingress can merge both surfaces later.

use std::sync::Arc;

use axum::Router;
use sdkwork_intelligence_skills_repository_sqlx::SqlxSkillsRepository;
use sdkwork_intelligence_skills_service::SkillsService;
use sqlx::PgPool;

pub struct ApiAssembly {
    pub router: Router,
}

fn web_framework_enabled() -> bool {
    if std::env::var("SDKWORK_SKILLS_ENVIRONMENT")
        .map(|value| value.eq_ignore_ascii_case("production"))
        .unwrap_or(false)
    {
        return true;
    }
    std::env::var("SDKWORK_SKILLS_WEB_FRAMEWORK")
        .map(|value| value != "0" && value != "false")
        .unwrap_or(true)
}

pub async fn assemble_app_surface_router(
    service: Arc<SkillsService<SqlxSkillsRepository>>,
    default_tenant_id: u64,
    pool: PgPool,
) -> Router {
    if web_framework_enabled() {
        sdkwork_routes_skills_app_api::build_router_with_web_framework_from_env(
            service,
            default_tenant_id,
            pool,
        )
        .await
    } else {
        sdkwork_routes_skills_app_api::build_router_with_readiness(service, default_tenant_id, pool)
    }
}

pub async fn assemble_backend_surface_router(
    service: Arc<SkillsService<SqlxSkillsRepository>>,
    default_tenant_id: u64,
    pool: PgPool,
) -> Router {
    if web_framework_enabled() {
        sdkwork_routes_skills_backend_api::build_router_with_web_framework_from_env(
            service,
            default_tenant_id,
            pool,
        )
        .await
    } else {
        sdkwork_routes_skills_backend_api::build_router_with_readiness(
            service,
            default_tenant_id,
            pool,
        )
    }
}

pub async fn assemble_api_router(
    service: Arc<SkillsService<SqlxSkillsRepository>>,
    default_tenant_id: u64,
    pool: PgPool,
) -> ApiAssembly {
    let app_router =
        assemble_app_surface_router(service.clone(), default_tenant_id, pool.clone()).await;
    let backend_router =
        assemble_backend_surface_router(service, default_tenant_id, pool).await;
    ApiAssembly {
        router: Router::new()
            .merge(app_router)
            .merge(backend_router),
    }
}
