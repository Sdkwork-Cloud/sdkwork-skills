mod json_util;
mod postgres;

#[cfg(test)]
mod json_util_tests;

use async_trait::async_trait;
use sdkwork_intelligence_skills_service::{SkillsRepository, SkillsResult};
use sdkwork_skills_contract::{
    SkillCategoryRecord, SkillPackageRecord, SkillRecord, UserSkillInstallRecord,
};
use sdkwork_utils_rust::OffsetListPageParams;
use sqlx::PgPool;

#[derive(Clone)]
pub struct SqlxSkillsRepository {
    pool: PgPool,
}

impl SqlxSkillsRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
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
        skill_id: &str,
    ) -> SkillsResult<SkillPackageRecord> {
        postgres::get_skill_package(&self.pool, tenant_id, skill_id).await
    }

    async fn upsert_skill_package(
        &self,
        record: SkillPackageRecord,
    ) -> SkillsResult<SkillPackageRecord> {
        postgres::upsert_skill_package(&self.pool, record).await
    }

    async fn list_skills_page(
        &self,
        tenant_id: u64,
        params: OffsetListPageParams,
        keyword: Option<&str>,
    ) -> SkillsResult<(Vec<SkillRecord>, i64)> {
        postgres::list_skills_page(&self.pool, tenant_id, params, keyword).await
    }

    async fn get_skill(&self, tenant_id: u64, skill_key: &str) -> SkillsResult<SkillRecord> {
        postgres::get_skill(&self.pool, tenant_id, skill_key).await
    }

    async fn list_categories(
        &self,
        tenant_id: u64,
        category_type: &str,
    ) -> SkillsResult<Vec<SkillCategoryRecord>> {
        postgres::list_categories(&self.pool, tenant_id, category_type).await
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
        postgres::upsert_category(&self.pool, record).await
    }

    async fn install_skill_for_user(
        &self,
        record: UserSkillInstallRecord,
    ) -> SkillsResult<UserSkillInstallRecord> {
        postgres::install_skill_for_user(&self.pool, record).await
    }

    async fn delete_skill_package(
        &self,
        tenant_id: u64,
        skill_id: &str,
    ) -> SkillsResult<SkillPackageRecord> {
        postgres::delete_skill_package(&self.pool, tenant_id, skill_id).await
    }

    async fn sync_skill_from_package(
        &self,
        package: &SkillPackageRecord,
    ) -> SkillsResult<SkillRecord> {
        postgres::sync_skill_from_package(&self.pool, package).await
    }
}
