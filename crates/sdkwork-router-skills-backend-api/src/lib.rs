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
    routing::{delete, get, post, put},
    Json, Router,
};
use sdkwork_intelligence_skills_service::SkillsService;
use sdkwork_skills_contract::{
    SkillCategoryRecord, SkillCategoryType, SkillInvocationKind, SkillLifecycleStatus,
    SkillPackageRecord, SkillVisibility,
};
use serde::Deserialize;
use sqlx::PgPool;

pub use handlers::{
    delete_skill_package, list_categories, list_hub_skills, list_skill_packages, resolve_tenant_id,
    upsert_category, upsert_skill_package, SharedSkillsService,
};
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
    State(state): State<BackendState<R>>,
    headers: HeaderMap,
    context: Option<Extension<SkillsBackendRequestContext>>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)>
where
    R: sdkwork_intelligence_skills_service::SkillsRepository + Send + Sync,
{
    let tenant_id = resolve_request_tenant_id(context.as_ref(), &headers, state.default_tenant_id);
    let payload = list_hub_skills(state.service.as_ref(), tenant_id).await?;
    Ok(Json(payload))
}

async fn list_admin_packages<R>(
    State(state): State<BackendState<R>>,
    headers: HeaderMap,
    context: Option<Extension<SkillsBackendRequestContext>>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)>
where
    R: sdkwork_intelligence_skills_service::SkillsRepository + Send + Sync,
{
    let tenant_id = resolve_request_tenant_id(context.as_ref(), &headers, state.default_tenant_id);
    let payload = list_skill_packages(state.service.as_ref(), tenant_id).await?;
    Ok(Json(payload))
}

async fn list_admin_categories<R>(
    State(state): State<BackendState<R>>,
    headers: HeaderMap,
    context: Option<Extension<SkillsBackendRequestContext>>,
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
struct CreatePackageRequest {
    skill_id: String,
    package_key: String,
    code: String,
    display_name: String,
    summary: Option<String>,
    description: Option<String>,
    invocation_kind: SkillInvocationKind,
    package_ref: String,
    entrypoint: String,
    input_schema_json: Option<String>,
    output_schema_json: Option<String>,
    capability_ids: Vec<String>,
    categories: Vec<String>,
    tags: Vec<String>,
    security_profile_id: Option<String>,
    category_id: Option<u64>,
    visibility: Option<SkillVisibility>,
}

async fn create_package<R>(
    State(state): State<BackendState<R>>,
    headers: HeaderMap,
    context: Option<Extension<SkillsBackendRequestContext>>,
    Json(body): Json<CreatePackageRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)>
where
    R: sdkwork_intelligence_skills_service::SkillsRepository + Send + Sync,
{
    let tenant_id = resolve_request_tenant_id(context.as_ref(), &headers, state.default_tenant_id);
    let record = SkillPackageRecord {
        id: 0,
        tenant_id,
        organization_id: 0,
        owner_user_id: context
            .as_ref()
            .and_then(|value| value.0.operator_id)
            .unwrap_or(0),
        skill_id: body.skill_id,
        package_key: body.package_key,
        code: body.code,
        display_name: body.display_name,
        summary: body.summary,
        description: body.description,
        invocation_kind: body.invocation_kind,
        package_ref: body.package_ref,
        entrypoint: body.entrypoint,
        input_schema_json: body.input_schema_json.unwrap_or_else(|| "{}".to_string()),
        output_schema_json: body.output_schema_json.unwrap_or_else(|| "{}".to_string()),
        capability_ids: body.capability_ids,
        categories: body.categories,
        tags: body.tags,
        security_profile_id: body.security_profile_id,
        category_id: body.category_id,
        status: SkillLifecycleStatus::Active,
        visibility: body.visibility.unwrap_or(SkillVisibility::Tenant),
        version: 1,
        created_at: String::new(),
        updated_at: String::new(),
        deleted_at: None,
    };
    let payload = upsert_skill_package(state.service.as_ref(), record).await?;
    Ok(Json(payload))
}

#[derive(Debug, Deserialize)]
struct UpdatePackageRequest {
    package_key: Option<String>,
    code: Option<String>,
    display_name: Option<String>,
    summary: Option<String>,
    description: Option<String>,
    invocation_kind: Option<SkillInvocationKind>,
    package_ref: Option<String>,
    entrypoint: Option<String>,
    input_schema_json: Option<String>,
    output_schema_json: Option<String>,
    capability_ids: Option<Vec<String>>,
    categories: Option<Vec<String>>,
    tags: Option<Vec<String>>,
    security_profile_id: Option<String>,
    category_id: Option<u64>,
    status: Option<SkillLifecycleStatus>,
    visibility: Option<SkillVisibility>,
}

async fn update_package<R>(
    State(state): State<BackendState<R>>,
    headers: HeaderMap,
    Path(skill_id): Path<String>,
    context: Option<Extension<SkillsBackendRequestContext>>,
    Json(body): Json<UpdatePackageRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)>
where
    R: sdkwork_intelligence_skills_service::SkillsRepository + Send + Sync,
{
    let tenant_id = resolve_request_tenant_id(context.as_ref(), &headers, state.default_tenant_id);
    let mut record = state
        .service
        .get_skill_package(tenant_id, skill_id.as_str())
        .await
        .map_err(crate::handlers::service_error_response)?;
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
    if let Some(value) = body.description {
        record.description = Some(value);
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
    if let Some(value) = body.input_schema_json {
        record.input_schema_json = value;
    }
    if let Some(value) = body.output_schema_json {
        record.output_schema_json = value;
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
    if let Some(value) = body.security_profile_id {
        record.security_profile_id = Some(value);
    }
    if let Some(value) = body.category_id {
        record.category_id = Some(value);
    }
    if let Some(value) = body.status {
        record.status = value;
    }
    if let Some(value) = body.visibility {
        record.visibility = value;
    }
    let payload = upsert_skill_package(state.service.as_ref(), record).await?;
    Ok(Json(payload))
}

async fn delete_package<R>(
    State(state): State<BackendState<R>>,
    headers: HeaderMap,
    Path(skill_id): Path<String>,
    context: Option<Extension<SkillsBackendRequestContext>>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)>
where
    R: sdkwork_intelligence_skills_service::SkillsRepository + Send + Sync,
{
    let tenant_id = resolve_request_tenant_id(context.as_ref(), &headers, state.default_tenant_id);
    let payload = delete_skill_package(state.service.as_ref(), tenant_id, skill_id.as_str()).await?;
    Ok(Json(payload))
}

#[derive(Debug, Deserialize)]
struct CreateCategoryRequest {
    code: String,
    name: String,
    description: Option<String>,
    parent_id: Option<u64>,
    sort_weight: Option<i32>,
}

async fn create_category<R>(
    State(state): State<BackendState<R>>,
    headers: HeaderMap,
    context: Option<Extension<SkillsBackendRequestContext>>,
    Json(body): Json<CreateCategoryRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)>
where
    R: sdkwork_intelligence_skills_service::SkillsRepository + Send + Sync,
{
    let tenant_id = resolve_request_tenant_id(context.as_ref(), &headers, state.default_tenant_id);
    let record = SkillCategoryRecord {
        id: 0,
        tenant_id,
        organization_id: 0,
        category_type: SkillCategoryType::SkillMarket.as_str().to_string(),
        code: body.code,
        name: body.name,
        description: body.description,
        parent_id: body.parent_id,
        sort_weight: body.sort_weight.unwrap_or(0),
        visible: true,
        status: 1,
    };
    let payload = upsert_category(state.service.as_ref(), record).await?;
    Ok(Json(payload))
}

#[derive(Debug, Deserialize)]
struct UpdateCategoryRequest {
    name: Option<String>,
    description: Option<String>,
    parent_id: Option<u64>,
    sort_weight: Option<i32>,
    visible: Option<bool>,
    status: Option<i16>,
}

async fn update_category<R>(
    State(state): State<BackendState<R>>,
    headers: HeaderMap,
    Path(category_id): Path<u64>,
    context: Option<Extension<SkillsBackendRequestContext>>,
    Json(body): Json<UpdateCategoryRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)>
where
    R: sdkwork_intelligence_skills_service::SkillsRepository + Send + Sync,
{
    let tenant_id = resolve_request_tenant_id(context.as_ref(), &headers, state.default_tenant_id);
    let categories = state
        .service
        .list_categories(tenant_id, SkillCategoryType::SkillMarket.as_str())
        .await
        .map_err(crate::handlers::service_error_response)?;
    let Some(mut record) = categories.into_iter().find(|item| item.id == category_id) else {
        return Err((StatusCode::NOT_FOUND, format!("category {category_id} not found")));
    };
    if let Some(value) = body.name {
        record.name = value;
    }
    if let Some(value) = body.description {
        record.description = Some(value);
    }
    if let Some(value) = body.parent_id {
        record.parent_id = Some(value);
    }
    if let Some(value) = body.sort_weight {
        record.sort_weight = value;
    }
    if let Some(value) = body.visible {
        record.visible = value;
    }
    if let Some(value) = body.status {
        record.status = value;
    }
    let payload = upsert_category(state.service.as_ref(), record).await?;
    Ok(Json(payload))
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
