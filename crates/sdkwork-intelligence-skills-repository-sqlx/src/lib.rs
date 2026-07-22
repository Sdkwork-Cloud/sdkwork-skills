mod json_util;
mod postgres;
mod sqlite;
mod support;

#[cfg(test)]
mod json_util_tests;
#[cfg(test)]
mod sqlite_tests;

use async_trait::async_trait;
use sdkwork_database_id::SnowflakeIdGenerator;
use sdkwork_database_sqlx::DatabasePool;
use sdkwork_intelligence_skills_service::{SkillsRepository, SkillsResult};
use sdkwork_skills_contract::{
    SkillArtifactRecord, SkillCapabilityRecord, SkillCategoryRecord, SkillInstallationRecord,
    SkillPackageRecord, SkillRecord,
};
use sdkwork_utils_rust::OffsetListPageParams;
use sqlx::{PgPool, SqlitePool};

#[derive(Clone)]
enum SkillsPool {
    Postgres(PgPool),
    Sqlite(SqlitePool),
}

#[derive(Clone)]
pub struct SqlxSkillsRepository {
    pool: SkillsPool,
    id_generator: SnowflakeIdGenerator,
}

impl SqlxSkillsRepository {
    pub fn new(pool: DatabasePool, id_generator: SnowflakeIdGenerator) -> Self {
        let pool = match pool {
            DatabasePool::Postgres(pool, _) => SkillsPool::Postgres(pool),
            DatabasePool::Sqlite(pool, _) => SkillsPool::Sqlite(pool),
        };
        Self { pool, id_generator }
    }

    pub fn from_postgres(pool: PgPool, id_generator: SnowflakeIdGenerator) -> Self {
        Self {
            pool: SkillsPool::Postgres(pool),
            id_generator,
        }
    }

    pub fn from_sqlite(pool: SqlitePool, id_generator: SnowflakeIdGenerator) -> Self {
        Self {
            pool: SkillsPool::Sqlite(pool),
            id_generator,
        }
    }
}

#[async_trait]
impl SkillsRepository for SqlxSkillsRepository {
    async fn list_skill_packages_page(
        &self,
        tenant_id: u64,
        params: OffsetListPageParams,
        keyword: Option<&str>,
    ) -> SkillsResult<(Vec<SkillPackageRecord>, i64)> {
        match &self.pool {
            SkillsPool::Postgres(pool) => {
                postgres::list_skill_packages_page(pool, tenant_id, params, keyword).await
            }
            SkillsPool::Sqlite(pool) => {
                sqlite::list_skill_packages_page(pool, tenant_id, params, keyword).await
            }
        }
    }

    async fn get_skill_package(
        &self,
        tenant_id: u64,
        package_id: u64,
    ) -> SkillsResult<SkillPackageRecord> {
        match &self.pool {
            SkillsPool::Postgres(pool) => {
                postgres::get_skill_package(pool, tenant_id, package_id).await
            }
            SkillsPool::Sqlite(pool) => {
                sqlite::get_skill_package(pool, tenant_id, package_id).await
            }
        }
    }

    async fn list_marketplace_skill_packages_page(
        &self,
        tenant_id: u64,
        organization_id: u64,
        user_id: u64,
        params: OffsetListPageParams,
        keyword: Option<&str>,
    ) -> SkillsResult<(Vec<SkillPackageRecord>, i64)> {
        match &self.pool {
            SkillsPool::Postgres(pool) => {
                postgres::list_marketplace_skill_packages_page(
                    pool,
                    tenant_id,
                    organization_id,
                    user_id,
                    params,
                    keyword,
                )
                .await
            }
            SkillsPool::Sqlite(pool) => {
                sqlite::list_marketplace_skill_packages_page(
                    pool,
                    tenant_id,
                    organization_id,
                    user_id,
                    params,
                    keyword,
                )
                .await
            }
        }
    }

    async fn get_marketplace_skill_package(
        &self,
        tenant_id: u64,
        organization_id: u64,
        user_id: u64,
        package_id: u64,
    ) -> SkillsResult<SkillPackageRecord> {
        match &self.pool {
            SkillsPool::Postgres(pool) => {
                postgres::get_marketplace_skill_package(
                    pool,
                    tenant_id,
                    organization_id,
                    user_id,
                    package_id,
                )
                .await
            }
            SkillsPool::Sqlite(pool) => {
                sqlite::get_marketplace_skill_package(
                    pool,
                    tenant_id,
                    organization_id,
                    user_id,
                    package_id,
                )
                .await
            }
        }
    }

    async fn create_skill_package(
        &self,
        package: SkillPackageRecord,
        initial_artifact: SkillArtifactRecord,
    ) -> SkillsResult<SkillPackageRecord> {
        match &self.pool {
            SkillsPool::Postgres(pool) => {
                postgres::create_skill_package(pool, &self.id_generator, package, initial_artifact)
                    .await
            }
            SkillsPool::Sqlite(pool) => {
                sqlite::create_skill_package(pool, &self.id_generator, package, initial_artifact)
                    .await
            }
        }
    }

    async fn update_skill_package(
        &self,
        package: SkillPackageRecord,
    ) -> SkillsResult<SkillPackageRecord> {
        match &self.pool {
            SkillsPool::Postgres(pool) => {
                postgres::update_skill_package(pool, &self.id_generator, package).await
            }
            SkillsPool::Sqlite(pool) => {
                sqlite::update_skill_package(pool, &self.id_generator, package).await
            }
        }
    }

    async fn delete_skill_package(&self, tenant_id: u64, package_id: u64) -> SkillsResult<()> {
        match &self.pool {
            SkillsPool::Postgres(pool) => {
                postgres::delete_skill_package(pool, tenant_id, package_id).await
            }
            SkillsPool::Sqlite(pool) => {
                sqlite::delete_skill_package(pool, tenant_id, package_id).await
            }
        }
    }

    async fn list_skills_page(
        &self,
        tenant_id: u64,
        organization_id: u64,
        user_id: u64,
        params: OffsetListPageParams,
        keyword: Option<&str>,
    ) -> SkillsResult<(Vec<SkillRecord>, i64)> {
        match &self.pool {
            SkillsPool::Postgres(pool) => {
                postgres::list_skills_page(
                    pool,
                    tenant_id,
                    organization_id,
                    user_id,
                    params,
                    keyword,
                )
                .await
            }
            SkillsPool::Sqlite(pool) => {
                sqlite::list_skills_page(pool, tenant_id, organization_id, user_id, params, keyword)
                    .await
            }
        }
    }

    async fn get_skill(
        &self,
        tenant_id: u64,
        organization_id: u64,
        user_id: u64,
        skill_key: &str,
    ) -> SkillsResult<SkillRecord> {
        match &self.pool {
            SkillsPool::Postgres(pool) => {
                postgres::get_skill(pool, tenant_id, organization_id, user_id, skill_key).await
            }
            SkillsPool::Sqlite(pool) => {
                sqlite::get_skill(pool, tenant_id, organization_id, user_id, skill_key).await
            }
        }
    }

    async fn get_category(
        &self,
        tenant_id: u64,
        category_id: u64,
    ) -> SkillsResult<SkillCategoryRecord> {
        match &self.pool {
            SkillsPool::Postgres(pool) => {
                postgres::get_category(pool, tenant_id, category_id).await
            }
            SkillsPool::Sqlite(pool) => sqlite::get_category(pool, tenant_id, category_id).await,
        }
    }

    async fn list_categories_page(
        &self,
        tenant_id: u64,
        category_type: &str,
        params: OffsetListPageParams,
        keyword: Option<&str>,
    ) -> SkillsResult<(Vec<SkillCategoryRecord>, i64)> {
        match &self.pool {
            SkillsPool::Postgres(pool) => {
                postgres::list_categories_page(pool, tenant_id, category_type, params, keyword)
                    .await
            }
            SkillsPool::Sqlite(pool) => {
                sqlite::list_categories_page(pool, tenant_id, category_type, params, keyword).await
            }
        }
    }

    async fn upsert_category(
        &self,
        record: SkillCategoryRecord,
    ) -> SkillsResult<SkillCategoryRecord> {
        match &self.pool {
            SkillsPool::Postgres(pool) => {
                postgres::upsert_category(pool, &self.id_generator, record).await
            }
            SkillsPool::Sqlite(pool) => {
                sqlite::upsert_category(pool, &self.id_generator, record).await
            }
        }
    }

    async fn list_capabilities_page(
        &self,
        tenant_id: u64,
        params: OffsetListPageParams,
        keyword: Option<&str>,
    ) -> SkillsResult<(Vec<SkillCapabilityRecord>, i64)> {
        match &self.pool {
            SkillsPool::Postgres(pool) => {
                postgres::list_capabilities_page(pool, tenant_id, params, keyword).await
            }
            SkillsPool::Sqlite(pool) => {
                sqlite::list_capabilities_page(pool, tenant_id, params, keyword).await
            }
        }
    }

    async fn get_capability(
        &self,
        tenant_id: u64,
        capability_id: u64,
    ) -> SkillsResult<SkillCapabilityRecord> {
        match &self.pool {
            SkillsPool::Postgres(pool) => {
                postgres::get_capability(pool, tenant_id, capability_id).await
            }
            SkillsPool::Sqlite(pool) => {
                sqlite::get_capability(pool, tenant_id, capability_id).await
            }
        }
    }

    async fn upsert_capability(
        &self,
        record: SkillCapabilityRecord,
    ) -> SkillsResult<SkillCapabilityRecord> {
        match &self.pool {
            SkillsPool::Postgres(pool) => {
                postgres::upsert_capability(pool, &self.id_generator, record).await
            }
            SkillsPool::Sqlite(pool) => {
                sqlite::upsert_capability(pool, &self.id_generator, record).await
            }
        }
    }

    async fn list_artifacts_page(
        &self,
        tenant_id: u64,
        package_id: u64,
        params: OffsetListPageParams,
    ) -> SkillsResult<(Vec<SkillArtifactRecord>, i64)> {
        match &self.pool {
            SkillsPool::Postgres(pool) => {
                postgres::list_artifacts_page(pool, tenant_id, package_id, params).await
            }
            SkillsPool::Sqlite(pool) => {
                sqlite::list_artifacts_page(pool, tenant_id, package_id, params).await
            }
        }
    }

    async fn list_installable_artifacts_page(
        &self,
        tenant_id: u64,
        organization_id: u64,
        user_id: u64,
        package_id: u64,
        params: OffsetListPageParams,
    ) -> SkillsResult<(Vec<SkillArtifactRecord>, i64)> {
        match &self.pool {
            SkillsPool::Postgres(pool) => {
                postgres::list_installable_artifacts_page(
                    pool,
                    tenant_id,
                    organization_id,
                    user_id,
                    package_id,
                    params,
                )
                .await
            }
            SkillsPool::Sqlite(pool) => {
                sqlite::list_installable_artifacts_page(
                    pool,
                    tenant_id,
                    organization_id,
                    user_id,
                    package_id,
                    params,
                )
                .await
            }
        }
    }

    async fn create_artifact(
        &self,
        artifact: SkillArtifactRecord,
    ) -> SkillsResult<SkillArtifactRecord> {
        match &self.pool {
            SkillsPool::Postgres(pool) => {
                postgres::create_artifact(pool, &self.id_generator, artifact).await
            }
            SkillsPool::Sqlite(pool) => {
                sqlite::create_artifact(pool, &self.id_generator, artifact).await
            }
        }
    }

    async fn install_skill(
        &self,
        record: SkillInstallationRecord,
    ) -> SkillsResult<SkillInstallationRecord> {
        match &self.pool {
            SkillsPool::Postgres(pool) => {
                postgres::install_skill(pool, &self.id_generator, record).await
            }
            SkillsPool::Sqlite(pool) => {
                sqlite::install_skill(pool, &self.id_generator, record).await
            }
        }
    }

    async fn list_installations_page(
        &self,
        tenant_id: u64,
        organization_id: u64,
        subject_kind: &str,
        subject_id: u64,
        params: OffsetListPageParams,
    ) -> SkillsResult<(Vec<SkillInstallationRecord>, i64)> {
        match &self.pool {
            SkillsPool::Postgres(pool) => {
                postgres::list_installations_page(
                    pool,
                    tenant_id,
                    organization_id,
                    subject_kind,
                    subject_id,
                    params,
                )
                .await
            }
            SkillsPool::Sqlite(pool) => {
                sqlite::list_installations_page(
                    pool,
                    tenant_id,
                    organization_id,
                    subject_kind,
                    subject_id,
                    params,
                )
                .await
            }
        }
    }
}
