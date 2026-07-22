//! Gateway bootstrap for sdkwork-skills.
//! Split-surface hosts call surface-specific assemblers; unified ingress can merge both surfaces later.

use std::sync::Arc;

use axum::Router;
use sdkwork_intelligence_skills_repository_sqlx::SqlxSkillsRepository;
use sdkwork_intelligence_skills_service::SkillsService;
use sdkwork_skills_database_host::SkillsDatabaseHost;

pub struct ApiAssembly {
    pub router: Router,
    pub readiness: SkillsReadiness,
    _database_host: Arc<SkillsDatabaseHost>,
}

#[derive(Clone)]
pub struct SkillsReadiness {
    database_host: Arc<SkillsDatabaseHost>,
}

impl SkillsReadiness {
    fn new(database_host: Arc<SkillsDatabaseHost>) -> Self {
        Self { database_host }
    }

    pub async fn check(&self) -> Result<(), String> {
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

pub async fn assemble_app_surface_router(
    service: Arc<SkillsService<SqlxSkillsRepository>>,
) -> Router {
    sdkwork_routes_skills_app_api::build_router_with_web_framework_from_env(service).await
}

pub async fn assemble_backend_surface_router(
    service: Arc<SkillsService<SqlxSkillsRepository>>,
) -> Router {
    sdkwork_routes_skills_backend_api::build_router_with_web_framework_from_env(service).await
}

pub async fn assemble_api_router(
    service: Arc<SkillsService<SqlxSkillsRepository>>,
    database_host: Arc<SkillsDatabaseHost>,
) -> ApiAssembly {
    let app_router = assemble_app_surface_router(service.clone()).await;
    let backend_router = assemble_backend_surface_router(service).await;
    ApiAssembly {
        router: Router::new().merge(app_router).merge(backend_router),
        readiness: SkillsReadiness::new(database_host.clone()),
        _database_host: database_host,
    }
}

pub async fn assemble_api_router_from_env() -> Result<ApiAssembly, String> {
    let database_host =
        Arc::new(sdkwork_skills_database_host::bootstrap_skills_database_from_env().await?);
    let repository = SqlxSkillsRepository::new(
        database_host.pool().clone(),
        database_host.id_generator().clone(),
    );
    let service = Arc::new(SkillsService::new(repository));
    Ok(assemble_api_router(service, database_host).await)
}
