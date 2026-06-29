use std::sync::Arc;

use sdkwork_web_core::HttpRouteManifest;

mod health;
pub mod http_route_manifest;
mod paths;
mod ports;
mod web_bootstrap;

use axum::{
    extract::{Extension, Path, Query, State},
    http::{HeaderMap, StatusCode},
    response::Response,
    routing::{delete, get, post, put},
    Json, Router,
};
use sdkwork_intelligence_skills_service::SkillsService;
use sdkwork_skills_contract::{
    SkillCategoryRecord, SkillCategoryType, SkillLifecycleStatus, SkillPackageRecord,
    SkillVisibility, package_manage_permission_for_category,
};
use sdkwork_web_core::WebRequestContext;
use sqlx::PgPool;

pub use sdkwork_routes_skills_common::{
    delete_skill_package, finish_api_json, list_categories, list_hub_skills, list_skill_packages,
    ok_json, resolve_tenant_id, upsert_category, upsert_skill_package, ApiProblem,
    CreateSkillCategoryCommand, CreateSkillPackageCommand, SdkWorkListQuery,
    UpdateSkillCategoryCommand, UpdateSkillPackageCommand,
};
pub type SharedSkillsService<R> = std::sync::Arc<
    sdkwork_intelligence_skills_service::SkillsService<R>,
>;
pub use health::DbReadinessCheck;
pub use http_route_manifest::backend_route_manifest;
pub use ports::SkillsBackendRequestContext;
pub use web_bootstrap::{
    skills_backend_public_path_prefixes, wrap_router_with_web_framework,
    wrap_router_with_web_framework_from_env,
};

#[derive(Clone)]
pub struct BackendState<R: sdkwork_intelligence_skills_service::SkillsRepository> {
    pub service: SharedSkillsService<R>,
    pub default_tenant_id: u64,
    pub readiness: Option<DbReadinessCheck>,
}

pub fn router<R>(state: BackendState<R>) -> Router
where
    R: sdkwork_intelligence_skills_service::SkillsRepository + Clone + Send + Sync + 'static,
{
    Router::new()
        .route(paths::LIVEZ, get(health::livez))
        .route(paths::READYZ, get(readyz_handler::<R>))
        .route(paths::HEALTHZ, get(healthz_handler::<R>))
        .route(paths::ADMIN_SKILLS_LIST, get(list_admin_skills))
        .route(paths::ADMIN_PACKAGES_LIST, get(list_admin_packages))
        .route(paths::ADMIN_PACKAGE_CREATE, post(create_package))
        .route(paths::ADMIN_PACKAGE_UPDATE, put(update_package))
        .route(paths::ADMIN_PACKAGE_DELETE, delete(delete_package))
        .route(paths::ADMIN_CATEGORIES_LIST, get(list_admin_categories))
        .route(paths::ADMIN_CATEGORY_CREATE, post(create_category))
        .route(paths::ADMIN_CATEGORY_UPDATE, put(update_category))
        .with_state(state)
}

async fn readyz_handler<R>(
    State(state): State<BackendState<R>>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)>
where
    R: sdkwork_intelligence_skills_service::SkillsRepository + Send + Sync,
{
    health::readyz_with_state(state.readiness.clone()).await
}

async fn healthz_handler<R>(
    State(state): State<BackendState<R>>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)>
where
    R: sdkwork_intelligence_skills_service::SkillsRepository + Send + Sync,
{
    health::healthz_with_state(state.readiness.clone()).await
}

fn resolve_request_tenant_id(
    context: Option<&Extension<SkillsBackendRequestContext>>,
    headers: &HeaderMap,
    default_tenant_id: u64,
) -> u64 {
    context
        .map(|extension| extension.0.tenant_id)
        .unwrap_or_else(|| resolve_tenant_id(headers, default_tenant_id))
}

async fn list_admin_skills<R>(
    ctx: WebRequestContext,
    State(state): State<BackendState<R>>,
    headers: HeaderMap,
    Query(query): Query<SdkWorkListQuery>,
    context: Option<Extension<SkillsBackendRequestContext>>,
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

async fn list_admin_packages<R>(
    ctx: WebRequestContext,
    State(state): State<BackendState<R>>,
    headers: HeaderMap,
    Query(query): Query<SdkWorkListQuery>,
    context: Option<Extension<SkillsBackendRequestContext>>,
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

async fn list_admin_categories<R>(
    ctx: WebRequestContext,
    State(state): State<BackendState<R>>,
    headers: HeaderMap,
    Query(query): Query<SdkWorkListQuery>,
    context: Option<Extension<SkillsBackendRequestContext>>,
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

async fn create_package<R>(
    ctx: WebRequestContext,
    State(state): State<BackendState<R>>,
    headers: HeaderMap,
    context: Option<Extension<SkillsBackendRequestContext>>,
    Json(body): Json<CreateSkillPackageCommand>,
) -> Response
where
    R: sdkwork_intelligence_skills_service::SkillsRepository + Send + Sync,
{
    finish_api_json(
        &ctx,
        async {
            let tenant_id =
                resolve_request_tenant_id(context.as_ref(), &headers, state.default_tenant_id);
            let package_key = body.resolved_package_key();
            let capability_ids = body.resolved_capability_ids();
            let categories = body.resolved_categories();
            let tags = body.resolved_tags();
            let record = SkillPackageRecord {
                id: 0,
                tenant_id,
                organization_id: 0,
                owner_user_id: context
                    .as_ref()
                    .and_then(|value| value.0.operator_id)
                    .unwrap_or(0),
                skill_id: body.skill_id,
                package_key,
                code: body.code,
                display_name: body.display_name,
                summary: body.summary,
                description: None,
                invocation_kind: body.invocation_kind,
                package_ref: body.package_ref,
                entrypoint: body.entrypoint,
                input_schema_json: "{}".to_string(),
                output_schema_json: "{}".to_string(),
                capability_ids,
                categories,
                tags,
                security_profile_id: None,
                status: SkillLifecycleStatus::Active,
                visibility: SkillVisibility::Tenant,
                version: 1,
                created_at: String::new(),
                updated_at: String::new(),
                deleted_at: None,
            };
            ok_json(upsert_skill_package(state.service.as_ref(), record).await?)
        }
        .await,
    )
}

async fn update_package<R>(
    ctx: WebRequestContext,
    State(state): State<BackendState<R>>,
    headers: HeaderMap,
    Path(skill_id): Path<String>,
    context: Option<Extension<SkillsBackendRequestContext>>,
    Json(body): Json<UpdateSkillPackageCommand>,
) -> Response
where
    R: sdkwork_intelligence_skills_service::SkillsRepository + Send + Sync,
{
    finish_api_json(
        &ctx,
        async {
            let tenant_id =
                resolve_request_tenant_id(context.as_ref(), &headers, state.default_tenant_id);
            let mut record = state
                .service
                .get_skill_package(tenant_id, skill_id.as_str())
                .await
                .map_err(ApiProblem::from)?;
            if let Some(value) = body.package_key {
                record.package_key = value;
            }
            if let Some(value) = body.code {
                record.code = value;
            }
            if let Some(value) = body.display_name {
                record.display_name = value;
            }
            if let Some(value) = body.summary {
                record.summary = Some(value);
            }
            if let Some(value) = body.invocation_kind {
                record.invocation_kind = value;
            }
            if let Some(value) = body.package_ref {
                record.package_ref = value;
            }
            if let Some(value) = body.entrypoint {
                record.entrypoint = value;
            }
            if let Some(value) = body.capability_ids {
                record.capability_ids = value;
            }
            if let Some(value) = body.categories {
                record.categories = value;
            }
            if let Some(value) = body.tags {
                record.tags = value;
            }
            if let Some(value) = body.visibility {
                record.visibility = value;
            }
            ok_json(upsert_skill_package(state.service.as_ref(), record).await?)
        }
        .await,
    )
}

async fn delete_package<R>(
    ctx: WebRequestContext,
    State(state): State<BackendState<R>>,
    headers: HeaderMap,
    Path(skill_id): Path<String>,
    context: Option<Extension<SkillsBackendRequestContext>>,
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
                delete_skill_package(state.service.as_ref(), tenant_id, skill_id.as_str()).await?,
            )
        }
        .await,
    )
}

async fn create_category<R>(
    ctx: WebRequestContext,
    State(state): State<BackendState<R>>,
    headers: HeaderMap,
    context: Option<Extension<SkillsBackendRequestContext>>,
    Json(body): Json<CreateSkillCategoryCommand>,
) -> Response
where
    R: sdkwork_intelligence_skills_service::SkillsRepository + Send + Sync,
{
    finish_api_json(
        &ctx,
        async {
            let tenant_id =
                resolve_request_tenant_id(context.as_ref(), &headers, state.default_tenant_id);
            let record = SkillCategoryRecord {
                id: 0,
                tenant_id,
                organization_id: 0,
                category_type: SkillCategoryType::SkillMarket.as_str().to_string(),
                code: body.code.clone(),
                name: body.name,
                description: body.description,
                parent_id: None,
                sort_weight: body.sort_weight.unwrap_or(0),
                permission_code: body
                    .permission_code
                    .unwrap_or_else(|| package_manage_permission_for_category(body.code.as_str())),
                visible: true,
                status: 1,
            };
            ok_json(upsert_category(state.service.as_ref(), record).await?)
        }
        .await,
    )
}

async fn update_category<R>(
    ctx: WebRequestContext,
    State(state): State<BackendState<R>>,
    headers: HeaderMap,
    Path(category_id): Path<u64>,
    context: Option<Extension<SkillsBackendRequestContext>>,
    Json(body): Json<UpdateSkillCategoryCommand>,
) -> Response
where
    R: sdkwork_intelligence_skills_service::SkillsRepository + Send + Sync,
{
    finish_api_json(
        &ctx,
        async {
            let tenant_id =
                resolve_request_tenant_id(context.as_ref(), &headers, state.default_tenant_id);
            let categories = state
                .service
                .list_categories(tenant_id, SkillCategoryType::SkillMarket.as_str())
                .await
                .map_err(ApiProblem::from)?;
            let Some(mut record) = categories.into_iter().find(|item| item.id == category_id)
            else {
                return Err(ApiProblem::not_found(format!(
                    "category {category_id} not found"
                )));
            };
            if let Some(value) = body.name {
                record.name = value;
            }
            if let Some(value) = body.description {
                record.description = Some(value);
            }
            if let Some(value) = body.sort_weight {
                record.sort_weight = value;
            }
            if let Some(value) = body.permission_code {
                record.permission_code = value;
            }
            ok_json(upsert_category(state.service.as_ref(), record).await?)
        }
        .await,
    )
}

pub fn build_router<R>(service: Arc<SkillsService<R>>, default_tenant_id: u64) -> Router
where
    R: sdkwork_intelligence_skills_service::SkillsRepository + Clone + Send + Sync + 'static,
{
    router(BackendState {
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
    router(BackendState {
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
    backend_route_manifest()
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
