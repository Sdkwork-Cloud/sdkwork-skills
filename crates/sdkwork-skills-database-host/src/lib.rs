use std::path::PathBuf;
use std::sync::Arc;

use sdkwork_database_config::{DatabaseConfig, DatabaseEngine};
use sdkwork_database_id::{
    NodeAllocatorConfig, NodeLease, SnowflakeIdGenerator, SnowflakeNodeAllocator,
};
use sdkwork_database_lifecycle::{lifecycle_options_from_env, LifecycleOrchestrator};
use sdkwork_database_spi::{DatabaseAssetProvider, DatabaseManifest, DefaultDatabaseModule};
use sdkwork_database_sqlx::{create_pool_from_config, DatabasePool};

pub const MODULE_ID: &str = "skills";
const PROCESS_SERVICE_NAME: &str = "sdkwork-skills";

#[derive(Clone)]
pub struct SkillsDatabaseHost {
    pool: DatabasePool,
    postgres_pool: sqlx::PgPool,
    module: Arc<DefaultDatabaseModule>,
    id_generator: SnowflakeIdGenerator,
    node_lease: NodeLease,
}

impl SkillsDatabaseHost {
    pub fn pool(&self) -> &DatabasePool {
        &self.pool
    }

    pub fn postgres_pool(&self) -> &sqlx::PgPool {
        &self.postgres_pool
    }

    pub fn module(&self) -> Arc<DefaultDatabaseModule> {
        self.module.clone()
    }

    pub fn id_generator(&self) -> &SnowflakeIdGenerator {
        &self.id_generator
    }

    pub fn node_lease(&self) -> &NodeLease {
        &self.node_lease
    }
}

pub async fn bootstrap_skills_database(pool: DatabasePool) -> Result<SkillsDatabaseHost, String> {
    let postgres_pool = pool
        .as_postgres()
        .cloned()
        .ok_or_else(|| unsupported_database_engine(pool.engine()))?;
    let app_root = resolve_app_root();
    let module = Arc::new(
        DefaultDatabaseModule::from_app_root(&app_root)
            .map_err(|error| format!("load skills database module failed: {error}"))?,
    );
    let manifest = DatabaseManifest::from_file(module.manifest_path())
        .map_err(|error| format!("read skills database manifest failed: {error}"))?;
    let options = lifecycle_options_from_env("SKILLS", &manifest);
    let orchestrator =
        LifecycleOrchestrator::new(pool.clone(), module.clone()).with_applied_by("sdkwork-skills");

    orchestrator
        .init()
        .await
        .map_err(|error| format!("skills database init failed: {error}"))?;

    if options.auto_migrate {
        orchestrator
            .migrate()
            .await
            .map_err(|error| format!("skills database migrate failed: {error}"))?;
    }

    let allocator_config = NodeAllocatorConfig::from_service_name(PROCESS_SERVICE_NAME);
    let (id_generator, node_lease) =
        SnowflakeNodeAllocator::allocate_process_generator(&pool, &allocator_config)
            .await
            .map_err(|error| format!("allocate Skills Snowflake node lease failed: {error}"))?;

    Ok(SkillsDatabaseHost {
        pool,
        postgres_pool,
        module,
        id_generator,
        node_lease,
    })
}

pub async fn bootstrap_skills_database_from_env() -> Result<SkillsDatabaseHost, String> {
    let _ = dotenvy::dotenv();
    let config = DatabaseConfig::from_env("SKILLS")
        .map_err(|error| format!("read skills database config failed: {error}"))?;
    require_authoritative_postgres_config(&config)?;
    let pool = create_pool_from_config(config)
        .await
        .map_err(|error| format!("create skills database pool failed: {error}"))?;
    bootstrap_skills_database(pool).await
}

fn require_authoritative_postgres_config(config: &DatabaseConfig) -> Result<(), String> {
    if config.engine == DatabaseEngine::Postgres {
        return Ok(());
    }
    Err(unsupported_database_engine(config.engine))
}

fn unsupported_database_engine(engine: DatabaseEngine) -> String {
    format!(
        "skills authoritative-server database requires PostgreSQL; configured engine is {}",
        engine
    )
}

fn resolve_app_root() -> PathBuf {
    std::env::var("SDKWORK_SKILLS_APP_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("../..")
                .canonicalize()
                .unwrap_or_else(|_| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../.."))
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn authoritative_config_accepts_postgres() {
        let config = DatabaseConfig {
            engine: DatabaseEngine::Postgres,
            ..Default::default()
        };

        assert_eq!(require_authoritative_postgres_config(&config), Ok(()));
    }

    #[test]
    fn authoritative_config_rejects_non_postgres() {
        let config = DatabaseConfig {
            engine: DatabaseEngine::Sqlite,
            ..Default::default()
        };

        let error = require_authoritative_postgres_config(&config)
            .expect_err("non-PostgreSQL config must fail closed");
        assert_eq!(
            error,
            "skills authoritative-server database requires PostgreSQL; configured engine is sqlite"
        );
    }
}
