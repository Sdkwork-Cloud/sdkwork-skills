use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};
use sdkwork_intelligence_skills_service::{SkillsRepository, SkillsService};

use crate::handlers::{
    create_installation, list_package_artifacts, list_packages, list_skill_categories,
    list_skill_installations, list_skills, retrieve_package, retrieve_skill, AppState,
};
use crate::paths;
use crate::{DenyExternalInstallationTargets, SkillInstallationTargetAuthorizer};

pub fn router<R>(state: AppState<R>) -> Router
where
    R: SkillsRepository + Clone + Send + Sync + 'static,
{
    Router::new()
        .route(paths::SKILLS, get(list_skills::<R>))
        .route(paths::SKILL, get(retrieve_skill::<R>))
        .route(paths::SKILL_PACKAGES, get(list_packages::<R>))
        .route(paths::SKILL_PACKAGE, get(retrieve_package::<R>))
        .route(paths::PACKAGE_ARTIFACTS, get(list_package_artifacts::<R>))
        .route(paths::SKILL_CATEGORIES, get(list_skill_categories::<R>))
        .route(paths::PACKAGE_INSTALLATIONS, post(create_installation::<R>))
        .route(
            paths::SKILL_INSTALLATIONS,
            get(list_skill_installations::<R>),
        )
        .with_state(state)
}

pub fn build_router_with_target_authorizer<R>(
    service: Arc<SkillsService<R>>,
    target_authorizer: Arc<dyn SkillInstallationTargetAuthorizer>,
) -> Router
where
    R: SkillsRepository + Clone + Send + Sync + 'static,
{
    router(AppState {
        service,
        target_authorizer,
    })
}

pub fn build_router<R>(service: Arc<SkillsService<R>>) -> Router
where
    R: SkillsRepository + Clone + Send + Sync + 'static,
{
    build_router_with_target_authorizer(service, Arc::new(DenyExternalInstallationTargets))
}
