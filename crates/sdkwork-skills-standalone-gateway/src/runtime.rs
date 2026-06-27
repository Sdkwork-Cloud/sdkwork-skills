use std::sync::Arc;

use sdkwork_intelligence_skills_repository_sqlx::SqlxSkillsRepository;
use sdkwork_intelligence_skills_service::SkillsService;
use sdkwork_skills_database_host::bootstrap_skills_database_from_env;
use sqlx::PgPool;

pub struct SkillsRuntime {
    service: Arc<SkillsService<SqlxSkillsRepository>>,
    default_tenant_id: u64,
    pool: PgPool,
}

impl SkillsRuntime {
    pub async fn bootstrap_from_env() -> Result<Self, String> {
        let host = bootstrap_skills_database_from_env().await?;
        let pool = host
            .postgres_pool()
            .ok_or_else(|| "skills runtime requires postgres database pool".to_string())?
            .clone();
        let repository = SqlxSkillsRepository::new(pool.clone());
        let service = Arc::new(SkillsService::new(repository));
        let default_tenant_id = std::env::var("SDKWORK_SKILLS_TENANT_ID")
            .ok()
            .and_then(|value| value.parse::<u64>().ok())
            .unwrap_or(100_001);
        Ok(Self {
            service,
            default_tenant_id,
            pool,
        })
    }

    pub fn service(&self) -> Arc<SkillsService<SqlxSkillsRepository>> {
        self.service.clone()
    }

    pub fn default_tenant_id(&self) -> u64 {
        self.default_tenant_id
    }

    pub fn postgres_pool(&self) -> PgPool {
        self.pool.clone()
    }
}
