use std::sync::Arc;

mod handlers;
mod health;
pub mod http_route_manifest;
mod paths;
mod ports;
mod web_bootstrap;

use axum::{
    extract::{Extension, Path, State},
    http::{HeaderMap, StatusCode},
    routing::{get, post},
    Json, Router,
};
use sdkwork_intelligence_skills_service::SkillsService;
use sdkwork_skills_contract::{SkillCategoryType, UserSkillInstallRecord};
use serde::Deserialize;
use sqlx::PgPool;

pub use handlers::{
    get_skill_package, install_skill, list_categories, list_hub_skills, list_skill_packages,
    resolve_tenant_id, SharedSkillsService,
};
pub use health::DbReadinessCheck;
pub use http_route_manifest::app_route_manifest;
pub use ports::SkillsAppRequestContext;
pub use web_bootstrap::{
    skills_public_path_prefixes, wrap_router_with_web_framework,
    wrap_router_with_web_framework_from_env,
};

#[derive(Clone)]
pub struct AppState<R: sdkwork_intelligence_skills_service::SkillsRepository> {
    pub service: SharedSkillsService<R>,
    pub default_tenant_id: u64,
    pub readiness: Option<DbReadinessCheck>,
}

pub fn router<R>(state: AppState<R>) -> Router
where
    R: sdkwork_intelligence_skills_service::SkillsRepository + Clone + Send + Sync + 'static,
{
    Router::new()
        .route(paths::LIVEZ, get(health::livez))
        .route(paths::READYZ, get(readyz_handler::<R>))
        .route(paths::HEALTHZ, get(healthz_handler::<R>))
        .route(paths::SKILLS_LIST, get(list_skills_handler))
        .route(paths::SKILL_GET, get(get_skill_handler))
        .route(paths::SKILL_PACKAGES_LIST, get(list_skill_packages_handler))
        .route(paths::SKILL_PACKAGE_GET, get(get_skill_package_handler))
        .route(paths::CATEGORIES_LIST, get(list_categories_handler))
        .route(paths::USER_SKILL_INSTALL, post(install_skill_handler))
        .with_state(state)
}

async fn readyz_handler<R>(
    State(state): State<AppState<R>>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)>
where
    R: sdkwork_intelligence_skills_service::SkillsRepository + Send + Sync,
{
    health::readyz_with_state(state.readiness.clone()).await
}

async fn healthz_handler<R>(
    State(state): State<AppState<R>>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)>
where
    R: sdkwork_intelligence_skills_service::SkillsRepository + Send + Sync,
{
    health::healthz_with_state(state.readiness.clone()).await
}

fn resolve_request_tenant_id(
    context: Option<&Extension<SkillsAppRequestContext>>,
    headers: &HeaderMap,
    default_tenant_id: u64,
) -> u64 {
    context
        .map(|extension| extension.0.tenant_id)
        .unwrap_or_else(|| resolve_tenant_id(headers, default_tenant_id))
}

async fn list_skills_handler<R>(
    State(state): State<AppState<R>>,
    headers: HeaderMap,
    context: Option<Extension<SkillsAppRequestContext>>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)>
where
    R: sdkwork_intelligence_skills_service::SkillsRepository + Send + Sync,
{
    let tenant_id = resolve_request_tenant_id(context.as_ref(), &headers, state.default_tenant_id);
    let payload = list_hub_skills(state.service.as_ref(), tenant_id).await?;
    Ok(Json(payload))
}

async fn get_skill_handler<R>(
    State(state): State<AppState<R>>,
    headers: HeaderMap,
    Path(skill_key): Path<String>,
    context: Option<Extension<SkillsAppRequestContext>>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)>
where
    R: sdkwork_intelligence_skills_service::SkillsRepository + Send + Sync,
{
    let tenant_id = resolve_request_tenant_id(context.as_ref(), &headers, state.default_tenant_id);
    let record = state
        .service
        .get_skill(tenant_id, skill_key.as_str())
        .await
        .map_err(crate::handlers::service_error_response)?;
    Ok(Json(crate::handlers::record_response(record)))
}

async fn list_skill_packages_handler<R>(
    State(state): State<AppState<R>>,
    headers: HeaderMap,
    context: Option<Extension<SkillsAppRequestContext>>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)>
where
    R: sdkwork_intelligence_skills_service::SkillsRepository + Send + Sync,
{
    let tenant_id = resolve_request_tenant_id(context.as_ref(), &headers, state.default_tenant_id);
    let payload = list_skill_packages(state.service.as_ref(), tenant_id).await?;
    Ok(Json(payload))
}

async fn get_skill_package_handler<R>(
    State(state): State<AppState<R>>,
    headers: HeaderMap,
    Path(skill_id): Path<String>,
    context: Option<Extension<SkillsAppRequestContext>>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)>
where
    R: sdkwork_intelligence_skills_service::SkillsRepository + Send + Sync,
{
    let tenant_id = resolve_request_tenant_id(context.as_ref(), &headers, state.default_tenant_id);
    let payload = get_skill_package(state.service.as_ref(), tenant_id, skill_id.as_str()).await?;
    Ok(Json(payload))
}

async fn list_categories_handler<R>(
    State(state): State<AppState<R>>,
    headers: HeaderMap,
    context: Option<Extension<SkillsAppRequestContext>>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)>
where
    R: sdkwork_intelligence_skills_service::SkillsRepository + Send + Sync,
{
    let tenant_id = resolve_request_tenant_id(context.as_ref(), &headers, state.default_tenant_id);
    let payload = list_categories(
        state.service.as_ref(),
        tenant_id,
        SkillCategoryType::SkillMarket.as_str(),
    )
    .await?;
    Ok(Json(payload))
}

#[derive(Debug, Deserialize)]
struct InstallSkillRequest {
    user_id: u64,
    skill_id: u64,
    package_id: Option<u64>,
    config_json: Option<String>,
}

async fn install_skill_handler<R>(
    State(state): State<AppState<R>>,
    headers: HeaderMap,
    context: Option<Extension<SkillsAppRequestContext>>,
    Json(body): Json<InstallSkillRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)>
where
    R: sdkwork_intelligence_skills_service::SkillsRepository + Send + Sync,
{
    let tenant_id = resolve_request_tenant_id(context.as_ref(), &headers, state.default_tenant_id);
    let actor_id = context
        .as_ref()
        .and_then(|value| value.0.actor_id)
        .unwrap_or(body.user_id);
    let record = UserSkillInstallRecord {
        id: 0,
        tenant_id,
        organization_id: 0,
        user_id: actor_id,
        skill_id: body.skill_id,
        package_id: body.package_id,
        install_status: "installed".to_string(),
        enabled: true,
        config_json: body.config_json.unwrap_or_else(|| "{}".to_string()),
        installed_at: String::new(),
        updated_at: String::new(),
    };
    let payload = install_skill(state.service.as_ref(), record).await?;
    Ok(Json(payload))
}

pub fn build_router<R>(service: Arc<SkillsService<R>>, default_tenant_id: u64) -> Router
where
    R: sdkwork_intelligence_skills_service::SkillsRepository + Clone + Send + Sync + 'static,
{
    router(AppState {
        service,
        default_tenant_id,
        readiness: None,
    })
}

pub fn build_router_with_readiness<R>(
    service: Arc<SkillsService<R>>,
    default_tenant_id: u64,
    pool: PgPool,
) -> Router
where
    R: sdkwork_intelligence_skills_service::SkillsRepository + Clone + Send + Sync + 'static,
{
    router(AppState {
        service,
        default_tenant_id,
        readiness: Some(DbReadinessCheck::new(pool)),
    })
}

pub async fn build_router_with_web_framework_from_env<R>(
    service: Arc<SkillsService<R>>,
    default_tenant_id: u64,
    pool: PgPool,
) -> Router
where
    R: sdkwork_intelligence_skills_service::SkillsRepository + Clone + Send + Sync + 'static,
{
    wrap_router_with_web_framework_from_env(build_router_with_readiness(
        service,
        default_tenant_id,
        pool,
    ))
    .await
}
