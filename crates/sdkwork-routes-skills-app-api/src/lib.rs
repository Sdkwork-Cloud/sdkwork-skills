use std::sync::Arc;

use sdkwork_web_core::HttpRouteManifest;

mod health;
pub mod http_route_manifest;
mod paths;
mod ports;
mod web_bootstrap;

use axum::{
    extract::{Extension, Path, Query, State},
    http::StatusCode,
    response::Response,
    routing::{get, post},
    Json, Router,
};
use sdkwork_intelligence_skills_service::SkillsService;
use sdkwork_skills_contract::{SkillCategoryType, UserSkillInstallRecord};
use sdkwork_web_core::WebRequestContext;
use sqlx::PgPool;

pub use sdkwork_routes_skills_common::{
    finish_api_json, get_skill, get_skill_package, install_skill, list_categories,
    list_hub_skills, list_skill_packages, ok_json, resolve_tenant_id, InstallSkillCommand,
    SdkWorkListQuery,
};
pub type SharedSkillsService<R> = std::sync::Arc<
    sdkwork_intelligence_skills_service::SkillsService<R>,
>;
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
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, String)>
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
    headers: &axum::http::HeaderMap,
    default_tenant_id: u64,
) -> u64 {
    context
        .map(|extension| extension.0.tenant_id)
        .unwrap_or_else(|| resolve_tenant_id(headers, default_tenant_id))
}

async fn list_skills_handler<R>(
    ctx: WebRequestContext,
    State(state): State<AppState<R>>,
    headers: axum::http::HeaderMap,
    Query(query): Query<SdkWorkListQuery>,
    context: Option<Extension<SkillsAppRequestContext>>,
) -> Response
where
    R: sdkwork_intelligence_skills_service::SkillsRepository + Send + Sync,
{
    finish_api_json(
        &ctx,
        async {
            let tenant_id =
                resolve_request_tenant_id(context.as_ref(), &headers, state.default_tenant_id);
            ok_json(
                list_hub_skills(state.service.as_ref(), tenant_id, &query).await?,
            )
        }
        .await,
    )
}

async fn get_skill_handler<R>(
    ctx: WebRequestContext,
    State(state): State<AppState<R>>,
    headers: axum::http::HeaderMap,
    Path(skill_key): Path<String>,
    context: Option<Extension<SkillsAppRequestContext>>,
) -> Response
where
    R: sdkwork_intelligence_skills_service::SkillsRepository + Send + Sync,
{
    finish_api_json(
        &ctx,
        async {
            let tenant_id =
                resolve_request_tenant_id(context.as_ref(), &headers, state.default_tenant_id);
            ok_json(get_skill(state.service.as_ref(), tenant_id, skill_key.as_str()).await?)
        }
        .await,
    )
}

async fn list_skill_packages_handler<R>(
    ctx: WebRequestContext,
    State(state): State<AppState<R>>,
    headers: axum::http::HeaderMap,
    Query(query): Query<SdkWorkListQuery>,
    context: Option<Extension<SkillsAppRequestContext>>,
) -> Response
where
    R: sdkwork_intelligence_skills_service::SkillsRepository + Send + Sync,
{
    finish_api_json(
        &ctx,
        async {
            let tenant_id =
                resolve_request_tenant_id(context.as_ref(), &headers, state.default_tenant_id);
            ok_json(
                list_skill_packages(state.service.as_ref(), tenant_id, &query).await?,
            )
        }
        .await,
    )
}

async fn get_skill_package_handler<R>(
    ctx: WebRequestContext,
    State(state): State<AppState<R>>,
    headers: axum::http::HeaderMap,
    Path(skill_id): Path<String>,
    context: Option<Extension<SkillsAppRequestContext>>,
) -> Response
where
    R: sdkwork_intelligence_skills_service::SkillsRepository + Send + Sync,
{
    finish_api_json(
        &ctx,
        async {
            let tenant_id =
                resolve_request_tenant_id(context.as_ref(), &headers, state.default_tenant_id);
            ok_json(
                get_skill_package(state.service.as_ref(), tenant_id, skill_id.as_str()).await?,
            )
        }
        .await,
    )
}

async fn list_categories_handler<R>(
    ctx: WebRequestContext,
    State(state): State<AppState<R>>,
    headers: axum::http::HeaderMap,
    Query(query): Query<SdkWorkListQuery>,
    context: Option<Extension<SkillsAppRequestContext>>,
) -> Response
where
    R: sdkwork_intelligence_skills_service::SkillsRepository + Send + Sync,
{
    finish_api_json(
        &ctx,
        async {
            let tenant_id =
                resolve_request_tenant_id(context.as_ref(), &headers, state.default_tenant_id);
            ok_json(
                list_categories(
                    state.service.as_ref(),
                    tenant_id,
                    SkillCategoryType::SkillMarket.as_str(),
                    &query,
                )
                .await?,
            )
        }
        .await,
    )
}

async fn install_skill_handler<R>(
    ctx: WebRequestContext,
    State(state): State<AppState<R>>,
    headers: axum::http::HeaderMap,
    context: Option<Extension<SkillsAppRequestContext>>,
    Json(body): Json<InstallSkillCommand>,
) -> Response
where
    R: sdkwork_intelligence_skills_service::SkillsRepository + Send + Sync,
{
    finish_api_json(
        &ctx,
        async {
            let tenant_id =
                resolve_request_tenant_id(context.as_ref(), &headers, state.default_tenant_id);
            let actor_id = context
                .as_ref()
                .and_then(|value| value.0.actor_id)
                .ok_or_else(|| {
                    sdkwork_routes_skills_common::ApiProblem::bad_request(
                        "authenticated actor id is required",
                    )
                })?;
            let record = UserSkillInstallRecord {
                id: 0,
                tenant_id,
                organization_id: context
                    .as_ref()
                    .and_then(|value| value.0.organization_id)
                    .unwrap_or(0),
                user_id: actor_id,
                skill_id: body.skill_id as u64,
                package_id: body.package_id.map(|value| value as u64),
                install_status: "installed".to_string(),
                enabled: true,
                config_json: "{}".to_string(),
                installed_at: String::new(),
                updated_at: String::new(),
            };
            ok_json(install_skill(state.service.as_ref(), record).await?)
        }
        .await,
    )
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

pub fn gateway_route_manifest() -> HttpRouteManifest {
    app_route_manifest()
}

pub async fn gateway_mount<R>(
    service: std::sync::Arc<SkillsService<R>>,
    default_tenant_id: u64,
    pool: sqlx::PgPool,
) -> axum::Router
where
    R: sdkwork_intelligence_skills_service::SkillsRepository + Clone + Send + Sync + 'static,
{
    build_router_with_web_framework_from_env(service, default_tenant_id, pool).await
}
