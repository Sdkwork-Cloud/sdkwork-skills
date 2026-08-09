mod validation;

#[cfg(test)]
mod validation_tests;

use async_trait::async_trait;
use sdkwork_skills_contract::{
    SkillArtifactRecord, SkillArtifactStatus, SkillCapabilityRecord, SkillCategoryRecord,
    SkillInstallationRecord, SkillLifecycleStatus, SkillPackageRecord, SkillRecord,
};
use sdkwork_utils_rust::OffsetListPageParams;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SkillsServiceError {
    #[error("not found: {0}")]
    NotFound(String),
    #[error("invalid argument: {0}")]
    InvalidArgument(String),
    #[error("conflict: {0}")]
    Conflict(String),
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
    async fn list_owned_skill_packages_page(
        &self,
        tenant_id: u64,
        owner_user_id: u64,
        params: OffsetListPageParams,
        keyword: Option<&str>,
    ) -> SkillsResult<(Vec<SkillPackageRecord>, i64)>;
    async fn get_skill_package(
        &self,
        tenant_id: u64,
        package_id: u64,
    ) -> SkillsResult<SkillPackageRecord>;
    async fn list_marketplace_skill_packages_page(
        &self,
        tenant_id: u64,
        organization_id: u64,
        user_id: u64,
        params: OffsetListPageParams,
        keyword: Option<&str>,
    ) -> SkillsResult<(Vec<SkillPackageRecord>, i64)>;
    async fn get_marketplace_skill_package(
        &self,
        tenant_id: u64,
        organization_id: u64,
        user_id: u64,
        package_id: u64,
    ) -> SkillsResult<SkillPackageRecord>;
    async fn create_skill_package(
        &self,
        package: SkillPackageRecord,
        initial_artifact: SkillArtifactRecord,
    ) -> SkillsResult<SkillPackageRecord>;
    async fn update_skill_package(
        &self,
        package: SkillPackageRecord,
    ) -> SkillsResult<SkillPackageRecord>;
    async fn delete_skill_package(&self, tenant_id: u64, package_id: u64) -> SkillsResult<()>;

    async fn list_skills_page(
        &self,
        tenant_id: u64,
        organization_id: u64,
        user_id: u64,
        params: OffsetListPageParams,
        keyword: Option<&str>,
    ) -> SkillsResult<(Vec<SkillRecord>, i64)>;
    async fn get_skill(
        &self,
        tenant_id: u64,
        organization_id: u64,
        user_id: u64,
        skill_key: &str,
    ) -> SkillsResult<SkillRecord>;

    async fn get_category(
        &self,
        tenant_id: u64,
        category_id: u64,
    ) -> SkillsResult<SkillCategoryRecord>;
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

    async fn list_capabilities_page(
        &self,
        tenant_id: u64,
        params: OffsetListPageParams,
        keyword: Option<&str>,
    ) -> SkillsResult<(Vec<SkillCapabilityRecord>, i64)>;
    async fn get_capability(
        &self,
        tenant_id: u64,
        capability_id: u64,
    ) -> SkillsResult<SkillCapabilityRecord>;
    async fn upsert_capability(
        &self,
        record: SkillCapabilityRecord,
    ) -> SkillsResult<SkillCapabilityRecord>;

    async fn list_artifacts_page(
        &self,
        tenant_id: u64,
        package_id: u64,
        params: OffsetListPageParams,
    ) -> SkillsResult<(Vec<SkillArtifactRecord>, i64)>;
    async fn list_installable_artifacts_page(
        &self,
        tenant_id: u64,
        organization_id: u64,
        user_id: u64,
        package_id: u64,
        params: OffsetListPageParams,
    ) -> SkillsResult<(Vec<SkillArtifactRecord>, i64)>;
    async fn create_artifact(
        &self,
        artifact: SkillArtifactRecord,
    ) -> SkillsResult<SkillArtifactRecord>;

    async fn install_skill(
        &self,
        record: SkillInstallationRecord,
    ) -> SkillsResult<SkillInstallationRecord>;
    async fn list_installations_page(
        &self,
        tenant_id: u64,
        organization_id: u64,
        subject_kind: &str,
        subject_id: u64,
        params: OffsetListPageParams,
    ) -> SkillsResult<(Vec<SkillInstallationRecord>, i64)>;
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
        organization_id: u64,
        user_id: u64,
        params: OffsetListPageParams,
        keyword: Option<&str>,
    ) -> SkillsResult<(Vec<SkillRecord>, i64)> {
        self.repository
            .list_skills_page(tenant_id, organization_id, user_id, params, keyword)
            .await
    }

    pub async fn get_skill(
        &self,
        tenant_id: u64,
        organization_id: u64,
        user_id: u64,
        skill_key: &str,
    ) -> SkillsResult<SkillRecord> {
        validation::validate_skill_key(skill_key)?;
        self.repository
            .get_skill(tenant_id, organization_id, user_id, skill_key)
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

    pub async fn list_owned_skill_packages_page(
        &self,
        tenant_id: u64,
        owner_user_id: u64,
        params: OffsetListPageParams,
        keyword: Option<&str>,
    ) -> SkillsResult<(Vec<SkillPackageRecord>, i64)> {
        self.repository
            .list_owned_skill_packages_page(tenant_id, owner_user_id, params, keyword)
            .await
    }

    pub async fn get_skill_package(
        &self,
        tenant_id: u64,
        package_id: u64,
    ) -> SkillsResult<SkillPackageRecord> {
        self.repository
            .get_skill_package(tenant_id, package_id)
            .await
    }

    pub async fn list_marketplace_skill_packages_page(
        &self,
        tenant_id: u64,
        organization_id: u64,
        user_id: u64,
        params: OffsetListPageParams,
        keyword: Option<&str>,
    ) -> SkillsResult<(Vec<SkillPackageRecord>, i64)> {
        self.repository
            .list_marketplace_skill_packages_page(
                tenant_id,
                organization_id,
                user_id,
                params,
                keyword,
            )
            .await
    }

    pub async fn get_marketplace_skill_package(
        &self,
        tenant_id: u64,
        organization_id: u64,
        user_id: u64,
        package_id: u64,
    ) -> SkillsResult<SkillPackageRecord> {
        self.repository
            .get_marketplace_skill_package(tenant_id, organization_id, user_id, package_id)
            .await
    }

    pub async fn create_skill_package(
        &self,
        package: SkillPackageRecord,
        initial_artifact: SkillArtifactRecord,
    ) -> SkillsResult<SkillPackageRecord> {
        validation::validate_skill_package_record(&package)?;
        validation::validate_artifact_record(&initial_artifact)?;
        if package.id != 0 {
            return Err(SkillsServiceError::InvalidArgument(
                "new skill package id must be zero".to_string(),
            ));
        }
        if initial_artifact.id != 0 || initial_artifact.package_id != 0 {
            return Err(SkillsServiceError::InvalidArgument(
                "new initial artifact id and package_id must be zero".to_string(),
            ));
        }
        if package.tenant_id != initial_artifact.tenant_id {
            return Err(SkillsServiceError::InvalidArgument(
                "package and initial artifact tenant_id must match".to_string(),
            ));
        }
        if package.status == SkillLifecycleStatus::Active
            && initial_artifact.status != SkillArtifactStatus::Published
        {
            return Err(SkillsServiceError::InvalidArgument(
                "an active package requires a published initial artifact".to_string(),
            ));
        }
        self.repository
            .create_skill_package(package, initial_artifact)
            .await
    }

    pub async fn update_skill_package(
        &self,
        package: SkillPackageRecord,
    ) -> SkillsResult<SkillPackageRecord> {
        validation::validate_skill_package_record(&package)?;
        if package.id == 0 || package.version == 0 {
            return Err(SkillsServiceError::InvalidArgument(
                "existing skill package id and version are required".to_string(),
            ));
        }
        self.repository.update_skill_package(package).await
    }

    pub async fn delete_skill_package(&self, tenant_id: u64, package_id: u64) -> SkillsResult<()> {
        self.repository
            .delete_skill_package(tenant_id, package_id)
            .await
    }

    pub async fn get_category(
        &self,
        tenant_id: u64,
        category_id: u64,
    ) -> SkillsResult<SkillCategoryRecord> {
        self.repository.get_category(tenant_id, category_id).await
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

    pub async fn create_category(
        &self,
        record: SkillCategoryRecord,
    ) -> SkillsResult<SkillCategoryRecord> {
        validation::validate_category_record(&record)?;
        if record.id != 0 {
            return Err(SkillsServiceError::InvalidArgument(
                "new category id must be zero".to_string(),
            ));
        }
        self.repository.upsert_category(record).await
    }

    pub async fn update_category(
        &self,
        record: SkillCategoryRecord,
    ) -> SkillsResult<SkillCategoryRecord> {
        validation::validate_category_record(&record)?;
        if record.id == 0 || record.version == 0 {
            return Err(SkillsServiceError::InvalidArgument(
                "existing category id and version are required".to_string(),
            ));
        }
        self.repository.upsert_category(record).await
    }

    pub async fn list_capabilities_page(
        &self,
        tenant_id: u64,
        params: OffsetListPageParams,
        keyword: Option<&str>,
    ) -> SkillsResult<(Vec<SkillCapabilityRecord>, i64)> {
        self.repository
            .list_capabilities_page(tenant_id, params, keyword)
            .await
    }

    pub async fn get_capability(
        &self,
        tenant_id: u64,
        capability_id: u64,
    ) -> SkillsResult<SkillCapabilityRecord> {
        self.repository
            .get_capability(tenant_id, capability_id)
            .await
    }

    pub async fn create_capability(
        &self,
        record: SkillCapabilityRecord,
    ) -> SkillsResult<SkillCapabilityRecord> {
        validation::validate_capability_record(&record)?;
        if record.id != 0 {
            return Err(SkillsServiceError::InvalidArgument(
                "new capability id must be zero".to_string(),
            ));
        }
        self.repository.upsert_capability(record).await
    }

    pub async fn update_capability(
        &self,
        record: SkillCapabilityRecord,
    ) -> SkillsResult<SkillCapabilityRecord> {
        validation::validate_capability_record(&record)?;
        if record.id == 0 || record.version == 0 {
            return Err(SkillsServiceError::InvalidArgument(
                "existing capability id and version are required".to_string(),
            ));
        }
        self.repository.upsert_capability(record).await
    }

    pub async fn list_artifacts_page(
        &self,
        tenant_id: u64,
        package_id: u64,
        params: OffsetListPageParams,
    ) -> SkillsResult<(Vec<SkillArtifactRecord>, i64)> {
        self.repository
            .list_artifacts_page(tenant_id, package_id, params)
            .await
    }

    pub async fn list_installable_artifacts_page(
        &self,
        tenant_id: u64,
        organization_id: u64,
        user_id: u64,
        package_id: u64,
        params: OffsetListPageParams,
    ) -> SkillsResult<(Vec<SkillArtifactRecord>, i64)> {
        self.repository
            .list_installable_artifacts_page(
                tenant_id,
                organization_id,
                user_id,
                package_id,
                params,
            )
            .await
    }

    pub async fn create_artifact(
        &self,
        artifact: SkillArtifactRecord,
    ) -> SkillsResult<SkillArtifactRecord> {
        validation::validate_artifact_record(&artifact)?;
        self.repository.create_artifact(artifact).await
    }

    pub async fn install_skill(
        &self,
        record: SkillInstallationRecord,
    ) -> SkillsResult<SkillInstallationRecord> {
        validation::validate_installation_record(&record)?;
        self.repository.install_skill(record).await
    }

    pub async fn list_installations_page(
        &self,
        tenant_id: u64,
        organization_id: u64,
        subject_kind: &str,
        subject_id: u64,
        params: OffsetListPageParams,
    ) -> SkillsResult<(Vec<SkillInstallationRecord>, i64)> {
        validation::validate_installation_subject(subject_kind, subject_id)?;
        self.repository
            .list_installations_page(tenant_id, organization_id, subject_kind, subject_id, params)
            .await
    }
}
