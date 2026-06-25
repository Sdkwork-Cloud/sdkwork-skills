mod validation;

#[cfg(test)]
mod validation_tests;

use async_trait::async_trait;
use sdkwork_skills_contract::{
    SkillCategoryRecord, SkillPackageRecord, SkillRecord, UserSkillInstallRecord,
};
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
    async fn list_skill_packages(
        &self,
        tenant_id: u64,
    ) -> SkillsResult<Vec<SkillPackageRecord>>;
    async fn get_skill_package(
        &self,
        tenant_id: u64,
        skill_id: &str,
    ) -> SkillsResult<SkillPackageRecord>;
    async fn upsert_skill_package(
        &self,
        record: SkillPackageRecord,
    ) -> SkillsResult<SkillPackageRecord>;
    async fn list_skills(&self, tenant_id: u64) -> SkillsResult<Vec<SkillRecord>>;
    async fn get_skill(&self, tenant_id: u64, skill_key: &str) -> SkillsResult<SkillRecord>;
    async fn list_categories(
        &self,
        tenant_id: u64,
        category_type: &str,
    ) -> SkillsResult<Vec<SkillCategoryRecord>>;
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

    pub async fn list_hub_skills(&self, tenant_id: u64) -> SkillsResult<Vec<SkillRecord>> {
        self.repository.list_skills(tenant_id).await
    }

    pub async fn get_skill(&self, tenant_id: u64, skill_key: &str) -> SkillsResult<SkillRecord> {
        self.repository.get_skill(tenant_id, skill_key).await
    }

    pub async fn list_skill_packages(
        &self,
        tenant_id: u64,
    ) -> SkillsResult<Vec<SkillPackageRecord>> {
        self.repository.list_skill_packages(tenant_id).await
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

    pub async fn install_skill(
        &self,
        record: UserSkillInstallRecord,
    ) -> SkillsResult<UserSkillInstallRecord> {
        self.repository.install_skill_for_user(record).await
    }
}
