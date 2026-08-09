use std::sync::Arc;

use axum::{
    extract::{Extension, Path, Query, State},
    response::Response,
    Json,
};
use sdkwork_intelligence_skills_service::{SkillsRepository, SkillsService};
use sdkwork_routes_skills_common::{
    create_artifact, create_capability, create_category, create_skill_package,
    delete_skill_package, finish_api_json, finish_created_api_json, finish_no_content,
    get_capability, get_category, get_skill_package, list_artifacts, list_capabilities,
    list_categories, list_hub_skills, list_skill_packages, parse_resource_id, update_capability,
    update_category, update_skill_package, ApiProblem, CreateSkillArtifactCommand,
    CreateSkillCapabilityCommand, CreateSkillCategoryCommand, CreateSkillPackageCommand,
    SdkWorkListQuery, SkillCategoryListQuery, UpdateSkillCapabilityCommand,
    UpdateSkillCategoryCommand, UpdateSkillPackageCommand,
};
use sdkwork_skills_contract::SkillCategoryType;
use sdkwork_web_core::WebRequestContext;

use crate::mapper::{artifact_record, capability_record, category_record, package_aggregate};
use crate::SkillsBackendRequestContext;

#[derive(Clone)]
pub struct BackendState<R: SkillsRepository> {
    pub service: Arc<SkillsService<R>>,
}

fn ensure_package_scope(
    context: &SkillsBackendRequestContext,
    organization_id: u64,
) -> Result<(), ApiProblem> {
    if organization_id == context.organization_id {
        Ok(())
    } else {
        Err(ApiProblem::forbidden(
            "skill package is outside the active organization scope",
        ))
    }
}

fn ensure_owned_definition(
    context: &SkillsBackendRequestContext,
    tenant_id: u64,
    organization_id: u64,
    resource: &str,
) -> Result<(), ApiProblem> {
    if tenant_id == context.tenant_id && organization_id == context.organization_id {
        Ok(())
    } else {
        Err(ApiProblem::forbidden(format!(
            "{resource} is inherited or outside the active organization scope"
        )))
    }
}

pub(crate) async fn list_skills<R>(
    ctx: WebRequestContext,
    State(state): State<BackendState<R>>,
    Extension(context): Extension<SkillsBackendRequestContext>,
    Query(query): Query<SdkWorkListQuery>,
) -> Response
where
    R: SkillsRepository + Send + Sync,
{
    finish_api_json(
        &ctx,
        list_hub_skills(
            state.service.as_ref(),
            context.tenant_id,
            context.organization_id,
            context.operator_id,
            &query,
        )
        .await,
    )
}

pub(crate) async fn list_packages<R>(
    ctx: WebRequestContext,
    State(state): State<BackendState<R>>,
    Extension(context): Extension<SkillsBackendRequestContext>,
    Query(query): Query<SdkWorkListQuery>,
) -> Response
where
    R: SkillsRepository + Send + Sync,
{
    finish_api_json(
        &ctx,
        list_skill_packages(state.service.as_ref(), context.tenant_id, &query).await,
    )
}

pub(crate) async fn retrieve_package<R>(
    ctx: WebRequestContext,
    State(state): State<BackendState<R>>,
    Extension(context): Extension<SkillsBackendRequestContext>,
    Path(package_id): Path<String>,
) -> Response
where
    R: SkillsRepository + Send + Sync,
{
    finish_api_json(
        &ctx,
        async {
            let package_id = parse_resource_id(&package_id, "packageId")?;
            let data =
                get_skill_package(state.service.as_ref(), context.tenant_id, package_id).await?;
            ensure_package_scope(&context, data.item.organization_id)?;
            Ok(data)
        }
        .await,
    )
}

pub(crate) async fn create_package<R>(
    ctx: WebRequestContext,
    State(state): State<BackendState<R>>,
    Extension(context): Extension<SkillsBackendRequestContext>,
    Json(body): Json<CreateSkillPackageCommand>,
) -> Response
where
    R: SkillsRepository + Send + Sync,
{
    let (package, initial_artifact) = package_aggregate(
        context.tenant_id,
        context.organization_id,
        context.operator_id,
        body,
    );
    finish_created_api_json(
        &ctx,
        create_skill_package(state.service.as_ref(), package, initial_artifact).await,
    )
}

pub(crate) async fn update_package<R>(
    ctx: WebRequestContext,
    State(state): State<BackendState<R>>,
    Extension(context): Extension<SkillsBackendRequestContext>,
    Path(package_id): Path<String>,
    Json(body): Json<UpdateSkillPackageCommand>,
) -> Response
where
    R: SkillsRepository + Send + Sync,
{
    finish_api_json(
        &ctx,
        async {
            let package_id = parse_resource_id(&package_id, "packageId")?;
            let mut record = state
                .service
                .get_skill_package(context.tenant_id, package_id)
                .await
                .map_err(ApiProblem::from)?;
            ensure_package_scope(&context, record.organization_id)?;
            record.version = body.version;
            if let Some(value) = body.display_name {
                record.display_name = value;
            }
            body.summary.apply_to(&mut record.summary);
            body.description.apply_to(&mut record.description);
            if let Some(value) = body.categories {
                record.categories = value;
            }
            if let Some(value) = body.tags {
                record.tags = value;
            }
            if let Some(value) = body.status {
                record.status = value;
            }
            if let Some(value) = body.visibility {
                record.visibility = value;
            }
            if let Some(value) = body.featured {
                record.featured = value;
            }
            if let Some(value) = body.sort_weight {
                record.sort_weight = value;
            }
            update_skill_package(state.service.as_ref(), record).await
        }
        .await,
    )
}

pub(crate) async fn delete_package<R>(
    ctx: WebRequestContext,
    State(state): State<BackendState<R>>,
    Extension(context): Extension<SkillsBackendRequestContext>,
    Path(package_id): Path<String>,
) -> Response
where
    R: SkillsRepository + Send + Sync,
{
    finish_no_content(
        &ctx,
        async {
            let package_id = parse_resource_id(&package_id, "packageId")?;
            let package = state
                .service
                .get_skill_package(context.tenant_id, package_id)
                .await
                .map_err(ApiProblem::from)?;
            ensure_package_scope(&context, package.organization_id)?;
            delete_skill_package(state.service.as_ref(), context.tenant_id, package_id).await
        }
        .await,
    )
}

pub(crate) async fn list_package_artifacts<R>(
    ctx: WebRequestContext,
    State(state): State<BackendState<R>>,
    Extension(context): Extension<SkillsBackendRequestContext>,
    Path(package_id): Path<String>,
    Query(query): Query<SdkWorkListQuery>,
) -> Response
where
    R: SkillsRepository + Send + Sync,
{
    finish_api_json(
        &ctx,
        async {
            let package_id = parse_resource_id(&package_id, "packageId")?;
            let package = state
                .service
                .get_skill_package(context.tenant_id, package_id)
                .await
                .map_err(ApiProblem::from)?;
            ensure_package_scope(&context, package.organization_id)?;
            list_artifacts(
                state.service.as_ref(),
                context.tenant_id,
                package_id,
                &query,
            )
            .await
        }
        .await,
    )
}

pub(crate) async fn create_package_artifact<R>(
    ctx: WebRequestContext,
    State(state): State<BackendState<R>>,
    Extension(context): Extension<SkillsBackendRequestContext>,
    Path(package_id): Path<String>,
    Json(body): Json<CreateSkillArtifactCommand>,
) -> Response
where
    R: SkillsRepository + Send + Sync,
{
    finish_created_api_json(
        &ctx,
        async {
            let package_id = parse_resource_id(&package_id, "packageId")?;
            let package = state
                .service
                .get_skill_package(context.tenant_id, package_id)
                .await
                .map_err(ApiProblem::from)?;
            ensure_package_scope(&context, package.organization_id)?;
            create_artifact(
                state.service.as_ref(),
                artifact_record(context.tenant_id, package_id, body),
            )
            .await
        }
        .await,
    )
}

pub(crate) async fn list_skill_categories<R>(
    ctx: WebRequestContext,
    State(state): State<BackendState<R>>,
    Extension(context): Extension<SkillsBackendRequestContext>,
    Query(query): Query<SkillCategoryListQuery>,
) -> Response
where
    R: SkillsRepository + Send + Sync,
{
    let category_type = query
        .category_type
        .unwrap_or(SkillCategoryType::SkillMarket);
    finish_api_json(
        &ctx,
        list_categories(
            state.service.as_ref(),
            context.tenant_id,
            category_type.as_str(),
            &query.pagination,
        )
        .await,
    )
}

pub(crate) async fn retrieve_category<R>(
    ctx: WebRequestContext,
    State(state): State<BackendState<R>>,
    Extension(context): Extension<SkillsBackendRequestContext>,
    Path(category_id): Path<String>,
) -> Response
where
    R: SkillsRepository + Send + Sync,
{
    finish_api_json(
        &ctx,
        async {
            let category_id = parse_resource_id(&category_id, "categoryId")?;
            get_category(state.service.as_ref(), context.tenant_id, category_id).await
        }
        .await,
    )
}

pub(crate) async fn create_skill_category<R>(
    ctx: WebRequestContext,
    State(state): State<BackendState<R>>,
    Extension(context): Extension<SkillsBackendRequestContext>,
    Json(body): Json<CreateSkillCategoryCommand>,
) -> Response
where
    R: SkillsRepository + Send + Sync,
{
    finish_created_api_json(
        &ctx,
        create_category(state.service.as_ref(), category_record(&context, body)).await,
    )
}

pub(crate) async fn update_skill_category<R>(
    ctx: WebRequestContext,
    State(state): State<BackendState<R>>,
    Extension(context): Extension<SkillsBackendRequestContext>,
    Path(category_id): Path<String>,
    Json(body): Json<UpdateSkillCategoryCommand>,
) -> Response
where
    R: SkillsRepository + Send + Sync,
{
    finish_api_json(
        &ctx,
        async {
            let category_id = parse_resource_id(&category_id, "categoryId")?;
            let mut record = state
                .service
                .get_category(context.tenant_id, category_id)
                .await
                .map_err(ApiProblem::from)?;
            ensure_owned_definition(
                &context,
                record.tenant_id,
                record.organization_id,
                "skill category",
            )?;
            record.version = body.version;
            if let Some(value) = body.name {
                record.name = value;
            }
            body.description.apply_to(&mut record.description);
            body.parent_id.apply_to(&mut record.parent_id);
            if let Some(value) = body.sort_weight {
                record.sort_weight = value;
            }
            if let Some(value) = body.permission_code {
                record.permission_code = value;
            }
            if let Some(value) = body.visible {
                record.visible = value;
            }
            if let Some(value) = body.status {
                record.status = value;
            }
            update_category(state.service.as_ref(), record).await
        }
        .await,
    )
}

pub(crate) async fn list_skill_capabilities<R>(
    ctx: WebRequestContext,
    State(state): State<BackendState<R>>,
    Extension(context): Extension<SkillsBackendRequestContext>,
    Query(query): Query<SdkWorkListQuery>,
) -> Response
where
    R: SkillsRepository + Send + Sync,
{
    finish_api_json(
        &ctx,
        list_capabilities(state.service.as_ref(), context.tenant_id, &query).await,
    )
}

pub(crate) async fn retrieve_capability<R>(
    ctx: WebRequestContext,
    State(state): State<BackendState<R>>,
    Extension(context): Extension<SkillsBackendRequestContext>,
    Path(capability_id): Path<String>,
) -> Response
where
    R: SkillsRepository + Send + Sync,
{
    finish_api_json(
        &ctx,
        async {
            let capability_id = parse_resource_id(&capability_id, "capabilityId")?;
            get_capability(state.service.as_ref(), context.tenant_id, capability_id).await
        }
        .await,
    )
}

pub(crate) async fn create_skill_capability<R>(
    ctx: WebRequestContext,
    State(state): State<BackendState<R>>,
    Extension(context): Extension<SkillsBackendRequestContext>,
    Json(body): Json<CreateSkillCapabilityCommand>,
) -> Response
where
    R: SkillsRepository + Send + Sync,
{
    finish_created_api_json(
        &ctx,
        create_capability(state.service.as_ref(), capability_record(&context, body)).await,
    )
}

pub(crate) async fn update_skill_capability<R>(
    ctx: WebRequestContext,
    State(state): State<BackendState<R>>,
    Extension(context): Extension<SkillsBackendRequestContext>,
    Path(capability_id): Path<String>,
    Json(body): Json<UpdateSkillCapabilityCommand>,
) -> Response
where
    R: SkillsRepository + Send + Sync,
{
    finish_api_json(
        &ctx,
        async {
            let capability_id = parse_resource_id(&capability_id, "capabilityId")?;
            let mut record = state
                .service
                .get_capability(context.tenant_id, capability_id)
                .await
                .map_err(ApiProblem::from)?;
            ensure_owned_definition(
                &context,
                record.tenant_id,
                record.organization_id,
                "skill capability",
            )?;
            record.version = body.version;
            if let Some(value) = body.display_name {
                record.display_name = value;
            }
            body.description.apply_to(&mut record.description);
            if let Some(value) = body.risk_level {
                record.risk_level = value;
            }
            if let Some(value) = body.status {
                record.status = value;
            }
            update_capability(state.service.as_ref(), record).await
        }
        .await,
    )
}
