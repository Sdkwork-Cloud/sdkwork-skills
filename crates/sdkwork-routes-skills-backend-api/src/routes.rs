use std::sync::Arc;

use axum::{routing::get, Router};
use sdkwork_intelligence_skills_service::{SkillsRepository, SkillsService};

use crate::handlers::{
    create_package, create_package_artifact, create_skill_capability, create_skill_category,
    delete_package, list_package_artifacts, list_packages, list_skill_capabilities,
    list_skill_categories, list_skills, retrieve_capability, retrieve_category, retrieve_package,
    update_package, update_skill_capability, update_skill_category, BackendState,
};
use crate::paths;

pub fn router<R>(state: BackendState<R>) -> Router
where
    R: SkillsRepository + Clone + Send + Sync + 'static,
{
    Router::new()
        .route(paths::SKILLS, get(list_skills::<R>))
        .route(
            paths::SKILL_PACKAGES,
            get(list_packages::<R>).post(create_package::<R>),
        )
        .route(
            paths::SKILL_PACKAGE,
            get(retrieve_package::<R>)
                .patch(update_package::<R>)
                .delete(delete_package::<R>),
        )
        .route(
            paths::PACKAGE_ARTIFACTS,
            get(list_package_artifacts::<R>).post(create_package_artifact::<R>),
        )
        .route(
            paths::SKILL_CAPABILITIES,
            get(list_skill_capabilities::<R>).post(create_skill_capability::<R>),
        )
        .route(
            paths::SKILL_CAPABILITY,
            get(retrieve_capability::<R>).patch(update_skill_capability::<R>),
        )
        .route(
            paths::SKILL_CATEGORIES,
            get(list_skill_categories::<R>).post(create_skill_category::<R>),
        )
        .route(
            paths::SKILL_CATEGORY,
            get(retrieve_category::<R>).patch(update_skill_category::<R>),
        )
        .with_state(state)
}

pub fn build_router<R>(service: Arc<SkillsService<R>>) -> Router
where
    R: SkillsRepository + Clone + Send + Sync + 'static,
{
    router(BackendState { service })
}
