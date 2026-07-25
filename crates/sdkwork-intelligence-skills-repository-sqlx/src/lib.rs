mod json_util;
mod postgres;
mod support;

#[cfg(test)]
mod json_util_tests;
#[cfg(test)]
mod postgres_tests;

use async_trait::async_trait;
use sdkwork_database_id::SnowflakeIdGenerator;
use sdkwork_intelligence_skills_service::{SkillsRepository, SkillsResult};
use sdkwork_skills_contract::{
    SkillArtifactRecord, SkillCapabilityRecord, SkillCategoryRecord, SkillInstallationRecord,
    SkillPackageRecord, SkillRecord,
};
use sdkwork_utils_rust::OffsetListPageParams;
use sqlx::PgPool;

#[derive(Clone)]
pub struct SqlxSkillsRepository {
    pool: PgPool,
    id_generator: SnowflakeIdGenerator,
}

impl SqlxSkillsRepository {
    pub fn new(pool: PgPool, id_generator: SnowflakeIdGenerator) -> Self {
        Self { pool, id_generator }
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
        postgres::list_skill_packages_page(&self.pool, tenant_id, params, keyword).await
    }

    async fn get_skill_package(
        &self,
        tenant_id: u64,
        package_id: u64,
    ) -> SkillsResult<SkillPackageRecord> {
        postgres::get_skill_package(&self.pool, tenant_id, package_id).await
    }

    async fn list_marketplace_skill_packages_page(
        &self,
        tenant_id: u64,
        organization_id: u64,
        user_id: u64,
        params: OffsetListPageParams,
        keyword: Option<&str>,
    ) -> SkillsResult<(Vec<SkillPackageRecord>, i64)> {
        postgres::list_marketplace_skill_packages_page(
            &self.pool,
            tenant_id,
            organization_id,
            user_id,
            params,
            keyword,
        )
        .await
    }

    async fn get_marketplace_skill_package(
        &self,
        tenant_id: u64,
        organization_id: u64,
        user_id: u64,
        package_id: u64,
    ) -> SkillsResult<SkillPackageRecord> {
        postgres::get_marketplace_skill_package(
            &self.pool,
            tenant_id,
            organization_id,
            user_id,
            package_id,
        )
        .await
    }

    async fn create_skill_package(
        &self,
        package: SkillPackageRecord,
        initial_artifact: SkillArtifactRecord,
    ) -> SkillsResult<SkillPackageRecord> {
        postgres::create_skill_package(&self.pool, &self.id_generator, package, initial_artifact)
            .await
    }

    async fn update_skill_package(
        &self,
        package: SkillPackageRecord,
    ) -> SkillsResult<SkillPackageRecord> {
        postgres::update_skill_package(&self.pool, &self.id_generator, package).await
    }

    async fn delete_skill_package(&self, tenant_id: u64, package_id: u64) -> SkillsResult<()> {
        postgres::delete_skill_package(&self.pool, tenant_id, package_id).await
    }

    async fn list_skills_page(
        &self,
        tenant_id: u64,
        organization_id: u64,
        user_id: u64,
        params: OffsetListPageParams,
        keyword: Option<&str>,
    ) -> SkillsResult<(Vec<SkillRecord>, i64)> {
        postgres::list_skills_page(
            &self.pool,
            tenant_id,
            organization_id,
            user_id,
            params,
            keyword,
        )
        .await
    }

    async fn get_skill(
        &self,
        tenant_id: u64,
        organization_id: u64,
        user_id: u64,
        skill_key: &str,
    ) -> SkillsResult<SkillRecord> {
        postgres::get_skill(&self.pool, tenant_id, organization_id, user_id, skill_key).await
    }

    async fn get_category(
        &self,
        tenant_id: u64,
        category_id: u64,
    ) -> SkillsResult<SkillCategoryRecord> {
        postgres::get_category(&self.pool, tenant_id, category_id).await
    }

    async fn list_categories_page(
        &self,
        tenant_id: u64,
        category_type: &str,
        params: OffsetListPageParams,
        keyword: Option<&str>,
    ) -> SkillsResult<(Vec<SkillCategoryRecord>, i64)> {
        postgres::list_categories_page(&self.pool, tenant_id, category_type, params, keyword).await
    }

    async fn upsert_category(
        &self,
        record: SkillCategoryRecord,
    ) -> SkillsResult<SkillCategoryRecord> {
        postgres::upsert_category(&self.pool, &self.id_generator, record).await
    }

    async fn list_capabilities_page(
        &self,
        tenant_id: u64,
        params: OffsetListPageParams,
        keyword: Option<&str>,
    ) -> SkillsResult<(Vec<SkillCapabilityRecord>, i64)> {
        postgres::list_capabilities_page(&self.pool, tenant_id, params, keyword).await
    }

    async fn get_capability(
        &self,
        tenant_id: u64,
        capability_id: u64,
    ) -> SkillsResult<SkillCapabilityRecord> {
        postgres::get_capability(&self.pool, tenant_id, capability_id).await
    }

    async fn upsert_capability(
        &self,
        record: SkillCapabilityRecord,
    ) -> SkillsResult<SkillCapabilityRecord> {
        postgres::upsert_capability(&self.pool, &self.id_generator, record).await
    }

    async fn list_artifacts_page(
        &self,
        tenant_id: u64,
        package_id: u64,
        params: OffsetListPageParams,
    ) -> SkillsResult<(Vec<SkillArtifactRecord>, i64)> {
        postgres::list_artifacts_page(&self.pool, tenant_id, package_id, params).await
    }

    async fn list_installable_artifacts_page(
        &self,
        tenant_id: u64,
        organization_id: u64,
        user_id: u64,
        package_id: u64,
        params: OffsetListPageParams,
    ) -> SkillsResult<(Vec<SkillArtifactRecord>, i64)> {
        postgres::list_installable_artifacts_page(
            &self.pool,
            tenant_id,
            organization_id,
            user_id,
            package_id,
            params,
        )
        .await
    }

    async fn create_artifact(
        &self,
        artifact: SkillArtifactRecord,
    ) -> SkillsResult<SkillArtifactRecord> {
        postgres::create_artifact(&self.pool, &self.id_generator, artifact).await
    }

    async fn install_skill(
        &self,
        record: SkillInstallationRecord,
    ) -> SkillsResult<SkillInstallationRecord> {
        postgres::install_skill(&self.pool, &self.id_generator, record).await
    }

    async fn list_installations_page(
        &self,
        tenant_id: u64,
        organization_id: u64,
        subject_kind: &str,
        subject_id: u64,
        params: OffsetListPageParams,
    ) -> SkillsResult<(Vec<SkillInstallationRecord>, i64)> {
        postgres::list_installations_page(
            &self.pool,
            tenant_id,
            organization_id,
            subject_kind,
            subject_id,
            params,
        )
        .await
    }
}
