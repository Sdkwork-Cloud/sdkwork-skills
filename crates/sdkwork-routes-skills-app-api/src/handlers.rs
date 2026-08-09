use std::sync::Arc;

use axum::{
    extract::{Extension, Path, Query, State},
    response::Response,
    Json,
};
use sdkwork_intelligence_skills_service::{SkillsRepository, SkillsService};
use sdkwork_routes_skills_common::mapper::{artifact_record, self_service_package_aggregate};
use sdkwork_routes_skills_common::{
    create_artifact, create_skill_package, delete_skill_package, finish_api_json,
    finish_created_api_json, finish_no_content, get_marketplace_skill_package, get_skill,
    install_skill, list_categories, list_hub_skills, list_installable_artifacts,
    list_installations, list_marketplace_skill_packages, list_owned_skill_packages, ok_json,
    parse_resource_id, update_skill_package, ApiProblem, CreateSkillArtifactCommand,
    CreateSkillInstallationCommand, CreateSkillPackageCommand, SdkWorkListQuery,
    SkillInstallationListQuery, UpdateOwnSkillPackageCommand,
};
use sdkwork_skills_contract::{
    SkillCategoryType, SkillInstallationSubjectKind, SkillPackageRecord, PERM_INSTALLATIONS_MANAGE,
};
use sdkwork_web_core::WebRequestContext;

use crate::mapper::installation_record;
use crate::{SkillInstallationTargetAuthorizer, SkillsAppRequestContext};

#[derive(Clone)]
pub struct AppState<R: SkillsRepository> {
    pub service: Arc<SkillsService<R>>,
    pub target_authorizer: Arc<dyn SkillInstallationTargetAuthorizer>,
}

pub(crate) async fn list_package_artifacts<R>(
    ctx: WebRequestContext,
    State(state): State<AppState<R>>,
    Extension(context): Extension<SkillsAppRequestContext>,
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
            list_installable_artifacts(
                state.service.as_ref(),
                context.tenant_id,
                context.organization_id,
                context.actor_id,
                package_id,
                &query,
            )
            .await
        }
        .await,
    )
}

pub(crate) async fn list_skills<R>(
    ctx: WebRequestContext,
    State(state): State<AppState<R>>,
    Extension(context): Extension<SkillsAppRequestContext>,
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
            context.actor_id,
            &query,
        )
        .await,
    )
}

pub(crate) async fn retrieve_skill<R>(
    ctx: WebRequestContext,
    State(state): State<AppState<R>>,
    Extension(context): Extension<SkillsAppRequestContext>,
    Path(skill_key): Path<String>,
) -> Response
where
    R: SkillsRepository + Send + Sync,
{
    finish_api_json(
        &ctx,
        get_skill(
            state.service.as_ref(),
            context.tenant_id,
            context.organization_id,
            context.actor_id,
            skill_key.as_str(),
        )
        .await,
    )
}

pub(crate) async fn list_packages<R>(
    ctx: WebRequestContext,
    State(state): State<AppState<R>>,
    Extension(context): Extension<SkillsAppRequestContext>,
    Query(query): Query<SdkWorkListQuery>,
) -> Response
where
    R: SkillsRepository + Send + Sync,
{
    finish_api_json(
        &ctx,
        list_marketplace_skill_packages(
            state.service.as_ref(),
            context.tenant_id,
            context.organization_id,
            context.actor_id,
            &query,
        )
        .await,
    )
}

pub(crate) async fn retrieve_package<R>(
    ctx: WebRequestContext,
    State(state): State<AppState<R>>,
    Extension(context): Extension<SkillsAppRequestContext>,
    Path(package_id): Path<String>,
) -> Response
where
    R: SkillsRepository + Send + Sync,
{
    finish_api_json(
        &ctx,
        async {
            let package_id = parse_resource_id(&package_id, "packageId")?;
            get_marketplace_skill_package(
                state.service.as_ref(),
                context.tenant_id,
                context.organization_id,
                context.actor_id,
                package_id,
            )
            .await
        }
        .await,
    )
}

/// Self-service package ownership guard: only the authenticated creator may
/// mutate a package within the active tenant.
fn ensure_owned_package(
    context: &SkillsAppRequestContext,
    package: &SkillPackageRecord,
) -> Result<(), ApiProblem> {
    if package.tenant_id == context.tenant_id && package.owner_user_id == context.actor_id {
        Ok(())
    } else {
        Err(ApiProblem::forbidden(
            "skill package is outside the authenticated user's ownership scope",
        ))
    }
}

pub(crate) async fn list_owned_packages<R>(
    ctx: WebRequestContext,
    State(state): State<AppState<R>>,
    Extension(context): Extension<SkillsAppRequestContext>,
    Query(query): Query<SdkWorkListQuery>,
) -> Response
where
    R: SkillsRepository + Send + Sync,
{
    finish_api_json(
        &ctx,
        list_owned_skill_packages(
            state.service.as_ref(),
            context.tenant_id,
            context.actor_id,
            &query,
        )
        .await,
    )
}

pub(crate) async fn create_package<R>(
    ctx: WebRequestContext,
    State(state): State<AppState<R>>,
    Extension(context): Extension<SkillsAppRequestContext>,
    Json(body): Json<CreateSkillPackageCommand>,
) -> Response
where
    R: SkillsRepository + Send + Sync,
{
    let (package, initial_artifact) = self_service_package_aggregate(
        context.tenant_id,
        context.organization_id,
        context.actor_id,
        body,
    );
    finish_created_api_json(
        &ctx,
        create_skill_package(state.service.as_ref(), package, initial_artifact).await,
    )
}

pub(crate) async fn update_package<R>(
    ctx: WebRequestContext,
    State(state): State<AppState<R>>,
    Extension(context): Extension<SkillsAppRequestContext>,
    Path(package_id): Path<String>,
    Json(body): Json<UpdateOwnSkillPackageCommand>,
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
            ensure_owned_package(&context, &record)?;
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
            update_skill_package(state.service.as_ref(), record).await
        }
        .await,
    )
}

pub(crate) async fn delete_package<R>(
    ctx: WebRequestContext,
    State(state): State<AppState<R>>,
    Extension(context): Extension<SkillsAppRequestContext>,
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
            ensure_owned_package(&context, &package)?;
            delete_skill_package(state.service.as_ref(), context.tenant_id, package_id).await
        }
        .await,
    )
}

pub(crate) async fn create_package_artifact<R>(
    ctx: WebRequestContext,
    State(state): State<AppState<R>>,
    Extension(context): Extension<SkillsAppRequestContext>,
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
            ensure_owned_package(&context, &package)?;
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
    State(state): State<AppState<R>>,
    Extension(context): Extension<SkillsAppRequestContext>,
    Query(query): Query<SdkWorkListQuery>,
) -> Response
where
    R: SkillsRepository + Send + Sync,
{
    finish_api_json(
        &ctx,
        list_categories(
            state.service.as_ref(),
            context.tenant_id,
            SkillCategoryType::SkillMarket.as_str(),
            &query,
        )
        .await,
    )
}

async fn authorize_target(
    ctx: &WebRequestContext,
    context: &SkillsAppRequestContext,
    authorizer: &dyn SkillInstallationTargetAuthorizer,
    subject_kind: SkillInstallationSubjectKind,
    subject_id: u64,
) -> Result<(SkillInstallationSubjectKind, u64), ApiProblem> {
    if subject_id == 0 {
        return Err(ApiProblem::bad_request(
            "installation target id must be a positive Snowflake id",
        ));
    }
    if subject_kind == SkillInstallationSubjectKind::User {
        return if subject_id == context.actor_id {
            Ok((subject_kind, subject_id))
        } else {
            Err(ApiProblem::forbidden(
                "a user installation target must be the authenticated user",
            ))
        };
    }
    if !ctx.has_permission(PERM_INSTALLATIONS_MANAGE) {
        return Err(ApiProblem::forbidden(format!(
            "{} is required for non-user installation targets",
            PERM_INSTALLATIONS_MANAGE
        )));
    }
    if subject_kind == SkillInstallationSubjectKind::Organization {
        return if context.organization_id > 0 && subject_id == context.organization_id {
            Ok((subject_kind, subject_id))
        } else {
            Err(ApiProblem::forbidden(
                "an organization installation target must be the authenticated organization",
            ))
        };
    }
    if !authorizer
        .authorize(context, subject_kind, subject_id)
        .await
    {
        return Err(ApiProblem::forbidden(
            "installation target is outside the authenticated principal's authorized scope",
        ));
    }
    Ok((subject_kind, subject_id))
}

#[cfg(test)]
mod target_authority_tests {
    use super::*;

    #[test]
    fn canonical_subject_kinds_keep_iam_and_agents_authorities_distinct() {
        assert_eq!(SkillInstallationSubjectKind::User.as_str(), "user");
        assert_eq!(
            SkillInstallationSubjectKind::Organization.as_str(),
            "organization"
        );
        assert_eq!(SkillInstallationSubjectKind::Project.as_str(), "project");
        assert_eq!(SkillInstallationSubjectKind::Agent.as_str(), "agent");
        assert!(SkillInstallationSubjectKind::parse("workspace").is_none());
    }
}

pub(crate) async fn create_installation<R>(
    ctx: WebRequestContext,
    State(state): State<AppState<R>>,
    Extension(context): Extension<SkillsAppRequestContext>,
    Path(package_id): Path<String>,
    Json(body): Json<CreateSkillInstallationCommand>,
) -> Response
where
    R: SkillsRepository + Send + Sync,
{
    finish_created_api_json(
        &ctx,
        async {
            let package_id = parse_resource_id(&package_id, "packageId")?;
            let (subject_kind, subject_id) = match body.target {
                Some(target) => {
                    authorize_target(
                        &ctx,
                        &context,
                        state.target_authorizer.as_ref(),
                        target.kind,
                        target.id,
                    )
                    .await?
                }
                None => (SkillInstallationSubjectKind::User, context.actor_id),
            };
            let record = installation_record(
                &context,
                package_id,
                body.artifact_id,
                subject_kind,
                subject_id,
                body.config,
            );
            install_skill(state.service.as_ref(), record).await
        }
        .await,
    )
}

pub(crate) async fn list_skill_installations<R>(
    ctx: WebRequestContext,
    State(state): State<AppState<R>>,
    Extension(context): Extension<SkillsAppRequestContext>,
    Query(query): Query<SkillInstallationListQuery>,
) -> Response
where
    R: SkillsRepository + Send + Sync,
{
    finish_api_json(
        &ctx,
        async {
            let (subject_kind, subject_id) = match (query.subject_kind, query.subject_id) {
                (None, None) | (Some(SkillInstallationSubjectKind::User), None) => {
                    (SkillInstallationSubjectKind::User, context.actor_id)
                }
                (None, Some(_)) => {
                    return Err(ApiProblem::bad_request(
                        "subject_kind is required when subject_id is provided",
                    ));
                }
                (Some(kind), Some(id)) => {
                    authorize_target(&ctx, &context, state.target_authorizer.as_ref(), kind, id)
                        .await?
                }
                (Some(_), None) => {
                    return Err(ApiProblem::bad_request(
                        "subject_id is required for non-user installation targets",
                    ));
                }
            };
            ok_json(
                list_installations(
                    state.service.as_ref(),
                    context.tenant_id,
                    context.organization_id,
                    subject_kind.as_str(),
                    subject_id,
                    &query.pagination,
                )
                .await?,
            )
        }
        .await,
    )
}
