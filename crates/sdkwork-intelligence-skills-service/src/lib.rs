mod validation;

#[cfg(test)]
mod validation_tests;

use async_trait::async_trait;
use sdkwork_skills_contract::{
    SkillCategoryRecord, SkillCategoryType, SkillPackageRecord, SkillRecord, UserSkillInstallRecord,
};
use sdkwork_utils_rust::{trim, OffsetListPageParams};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SkillsServiceError {
    #[error("not found: {0}")]
    NotFound(String),
    #[error("invalid argument: {0}")]
    InvalidArgument(String),
    #[error("repository error: {0}")]
    Repository(String),
}

pub type SkillsResult<T> = Result<T, SkillsServiceError>;

#[async_trait]
pub trait SkillsRepository: Send + Sync {
    async fn list_skill_packages_page(
        &self,
        tenant_id: u64,
        params: OffsetListPageParams,
        keyword: Option<&str>,
    ) -> SkillsResult<(Vec<SkillPackageRecord>, i64)>;
    async fn get_skill_package(
        &self,
        tenant_id: u64,
        skill_id: &str,
    ) -> SkillsResult<SkillPackageRecord>;
    async fn upsert_skill_package(
        &self,
        record: SkillPackageRecord,
    ) -> SkillsResult<SkillPackageRecord>;
    async fn list_skills_page(
        &self,
        tenant_id: u64,
        params: OffsetListPageParams,
        keyword: Option<&str>,
    ) -> SkillsResult<(Vec<SkillRecord>, i64)>;
    async fn get_skill(&self, tenant_id: u64, skill_key: &str) -> SkillsResult<SkillRecord>;
    async fn list_categories(
        &self,
        tenant_id: u64,
        category_type: &str,
    ) -> SkillsResult<Vec<SkillCategoryRecord>>;
    async fn list_categories_page(
        &self,
        tenant_id: u64,
        category_type: &str,
        params: OffsetListPageParams,
        keyword: Option<&str>,
    ) -> SkillsResult<(Vec<SkillCategoryRecord>, i64)>;
    async fn upsert_category(
        &self,
        record: SkillCategoryRecord,
    ) -> SkillsResult<SkillCategoryRecord>;
    async fn install_skill_for_user(
        &self,
        record: UserSkillInstallRecord,
    ) -> SkillsResult<UserSkillInstallRecord>;
    async fn delete_skill_package(
        &self,
        tenant_id: u64,
        skill_id: &str,
    ) -> SkillsResult<SkillPackageRecord>;
    async fn sync_skill_from_package(
        &self,
        package: &SkillPackageRecord,
    ) -> SkillsResult<SkillRecord>;
}

#[derive(Clone)]
pub struct SkillsService<R: SkillsRepository> {
    repository: R,
}

impl<R: SkillsRepository> SkillsService<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn list_hub_skills_page(
        &self,
        tenant_id: u64,
        params: OffsetListPageParams,
        keyword: Option<&str>,
    ) -> SkillsResult<(Vec<SkillRecord>, i64)> {
        self.repository
            .list_skills_page(tenant_id, params, keyword)
            .await
    }

    pub async fn list_skill_packages_page(
        &self,
        tenant_id: u64,
        params: OffsetListPageParams,
        keyword: Option<&str>,
    ) -> SkillsResult<(Vec<SkillPackageRecord>, i64)> {
        self.repository
            .list_skill_packages_page(tenant_id, params, keyword)
            .await
    }

    pub async fn list_categories_page(
        &self,
        tenant_id: u64,
        category_type: &str,
        params: OffsetListPageParams,
        keyword: Option<&str>,
    ) -> SkillsResult<(Vec<SkillCategoryRecord>, i64)> {
        self.repository
            .list_categories_page(tenant_id, category_type, params, keyword)
            .await
    }

    pub async fn get_skill(&self, tenant_id: u64, skill_key: &str) -> SkillsResult<SkillRecord> {
        self.repository.get_skill(tenant_id, skill_key).await
    }

    pub async fn get_skill_package(
        &self,
        tenant_id: u64,
        skill_id: &str,
    ) -> SkillsResult<SkillPackageRecord> {
        self.repository.get_skill_package(tenant_id, skill_id).await
    }

    pub async fn upsert_skill_package(
        &self,
        record: SkillPackageRecord,
    ) -> SkillsResult<SkillPackageRecord> {
        validation::validate_skill_package_record(&record)?;
        self.validate_package_categories(record.tenant_id, &record.categories)
            .await?;
        let saved = self.repository.upsert_skill_package(record).await?;
        self.repository.sync_skill_from_package(&saved).await?;
        Ok(saved)
    }

    pub async fn delete_skill_package(
        &self,
        tenant_id: u64,
        skill_id: &str,
    ) -> SkillsResult<SkillPackageRecord> {
        validation::validate_skill_id(skill_id)?;
        self.repository
            .delete_skill_package(tenant_id, skill_id)
            .await
    }

    pub async fn list_categories(
        &self,
        tenant_id: u64,
        category_type: &str,
    ) -> SkillsResult<Vec<SkillCategoryRecord>> {
        self.repository
            .list_categories(tenant_id, category_type)
            .await
    }

    pub async fn upsert_category(
        &self,
        record: SkillCategoryRecord,
    ) -> SkillsResult<SkillCategoryRecord> {
        validation::validate_category_record(&record)?;
        self.repository.upsert_category(record).await
    }

    async fn validate_package_categories(
        &self,
        tenant_id: u64,
        categories: &[String],
    ) -> SkillsResult<()> {
        if categories.is_empty() {
            return Ok(());
        }
        let known = self
            .repository
            .list_categories(tenant_id, SkillCategoryType::SkillMarket.as_str())
            .await?;
        let known_codes: std::collections::HashSet<String> =
            known.into_iter().map(|item| item.code).collect();
        for code in categories {
            let normalized = trim(code);
            if normalized.is_empty() {
                continue;
            }
            if !known_codes.contains(&normalized) {
                return Err(SkillsServiceError::InvalidArgument(format!(
                    "unknown skill category code: {normalized}"
                )));
            }
        }
        Ok(())
    }

    pub async fn install_skill(
        &self,
        record: UserSkillInstallRecord,
    ) -> SkillsResult<UserSkillInstallRecord> {
        self.repository.install_skill_for_user(record).await
    }
}
