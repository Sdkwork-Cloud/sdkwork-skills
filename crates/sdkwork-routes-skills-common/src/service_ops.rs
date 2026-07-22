use sdkwork_intelligence_skills_service::{SkillsRepository, SkillsService, SkillsServiceError};
use sdkwork_skills_contract::{
    SkillArtifactRecord, SkillCapabilityRecord, SkillCategoryRecord, SkillInstallationRecord,
    SkillPackageRecord, SkillRecord,
};
use sdkwork_utils_rust::{offset_list_page_data, SdkWorkPageData, SdkWorkResourceData};

use crate::list_query::SdkWorkListQuery;
use crate::response::{item_data, ApiProblem, ApiResult};

impl From<SkillsServiceError> for ApiProblem {
    fn from(error: SkillsServiceError) -> Self {
        match error {
            SkillsServiceError::NotFound(message) => ApiProblem::not_found(message),
            SkillsServiceError::InvalidArgument(message) => ApiProblem::bad_request(message),
            SkillsServiceError::Conflict(message) => ApiProblem::conflict(message),
            SkillsServiceError::Repository(message) => ApiProblem::internal_server_error(message),
        }
    }
}

pub fn parse_resource_id(value: &str, resource: &str) -> ApiResult<u64> {
    value
        .parse::<u64>()
        .ok()
        .filter(|id| *id > 0)
        .ok_or_else(|| {
            ApiProblem::bad_request(format!("{resource} must be a positive Snowflake id"))
        })
}

pub async fn list_hub_skills<R: SkillsRepository>(
    service: &SkillsService<R>,
    tenant_id: u64,
    organization_id: u64,
    user_id: u64,
    query: &SdkWorkListQuery,
) -> ApiResult<SdkWorkPageData<SkillRecord>> {
    query.validate()?;
    let params = query.offset_params()?;
    let (items, total) = service
        .list_hub_skills_page(
            tenant_id,
            organization_id,
            user_id,
            params,
            query.search_keyword(),
        )
        .await?;
    Ok(offset_list_page_data(items, total, params))
}

pub async fn get_skill<R: SkillsRepository>(
    service: &SkillsService<R>,
    tenant_id: u64,
    organization_id: u64,
    user_id: u64,
    skill_key: &str,
) -> ApiResult<SdkWorkResourceData<SkillRecord>> {
    Ok(item_data(
        service
            .get_skill(tenant_id, organization_id, user_id, skill_key)
            .await?,
    ))
}

pub async fn list_skill_packages<R: SkillsRepository>(
    service: &SkillsService<R>,
    tenant_id: u64,
    query: &SdkWorkListQuery,
) -> ApiResult<SdkWorkPageData<SkillPackageRecord>> {
    query.validate()?;
    let params = query.offset_params()?;
    let (items, total) = service
        .list_skill_packages_page(tenant_id, params, query.search_keyword())
        .await?;
    Ok(offset_list_page_data(items, total, params))
}

pub async fn get_skill_package<R: SkillsRepository>(
    service: &SkillsService<R>,
    tenant_id: u64,
    package_id: u64,
) -> ApiResult<SdkWorkResourceData<SkillPackageRecord>> {
    Ok(item_data(
        service.get_skill_package(tenant_id, package_id).await?,
    ))
}

pub async fn list_marketplace_skill_packages<R: SkillsRepository>(
    service: &SkillsService<R>,
    tenant_id: u64,
    organization_id: u64,
    user_id: u64,
    query: &SdkWorkListQuery,
) -> ApiResult<SdkWorkPageData<SkillPackageRecord>> {
    query.validate()?;
    let params = query.offset_params()?;
    let (items, total) = service
        .list_marketplace_skill_packages_page(
            tenant_id,
            organization_id,
            user_id,
            params,
            query.search_keyword(),
        )
        .await?;
    Ok(offset_list_page_data(items, total, params))
}

pub async fn get_marketplace_skill_package<R: SkillsRepository>(
    service: &SkillsService<R>,
    tenant_id: u64,
    organization_id: u64,
    user_id: u64,
    package_id: u64,
) -> ApiResult<SdkWorkResourceData<SkillPackageRecord>> {
    Ok(item_data(
        service
            .get_marketplace_skill_package(tenant_id, organization_id, user_id, package_id)
            .await?,
    ))
}

pub async fn create_skill_package<R: SkillsRepository>(
    service: &SkillsService<R>,
    package: SkillPackageRecord,
    initial_artifact: SkillArtifactRecord,
) -> ApiResult<SdkWorkResourceData<SkillPackageRecord>> {
    Ok(item_data(
        service
            .create_skill_package(package, initial_artifact)
            .await?,
    ))
}

pub async fn update_skill_package<R: SkillsRepository>(
    service: &SkillsService<R>,
    package: SkillPackageRecord,
) -> ApiResult<SdkWorkResourceData<SkillPackageRecord>> {
    Ok(item_data(service.update_skill_package(package).await?))
}

pub async fn delete_skill_package<R: SkillsRepository>(
    service: &SkillsService<R>,
    tenant_id: u64,
    package_id: u64,
) -> ApiResult<()> {
    service.delete_skill_package(tenant_id, package_id).await?;
    Ok(())
}

pub async fn list_categories<R: SkillsRepository>(
    service: &SkillsService<R>,
    tenant_id: u64,
    category_type: &str,
    query: &SdkWorkListQuery,
) -> ApiResult<SdkWorkPageData<SkillCategoryRecord>> {
    query.validate()?;
    let params = query.offset_params()?;
    let (items, total) = service
        .list_categories_page(tenant_id, category_type, params, query.search_keyword())
        .await?;
    Ok(offset_list_page_data(items, total, params))
}

pub async fn get_category<R: SkillsRepository>(
    service: &SkillsService<R>,
    tenant_id: u64,
    category_id: u64,
) -> ApiResult<SdkWorkResourceData<SkillCategoryRecord>> {
    Ok(item_data(
        service.get_category(tenant_id, category_id).await?,
    ))
}

pub async fn create_category<R: SkillsRepository>(
    service: &SkillsService<R>,
    record: SkillCategoryRecord,
) -> ApiResult<SdkWorkResourceData<SkillCategoryRecord>> {
    Ok(item_data(service.create_category(record).await?))
}

pub async fn update_category<R: SkillsRepository>(
    service: &SkillsService<R>,
    record: SkillCategoryRecord,
) -> ApiResult<SdkWorkResourceData<SkillCategoryRecord>> {
    Ok(item_data(service.update_category(record).await?))
}

pub async fn list_capabilities<R: SkillsRepository>(
    service: &SkillsService<R>,
    tenant_id: u64,
    query: &SdkWorkListQuery,
) -> ApiResult<SdkWorkPageData<SkillCapabilityRecord>> {
    query.validate()?;
    let params = query.offset_params()?;
    let (items, total) = service
        .list_capabilities_page(tenant_id, params, query.search_keyword())
        .await?;
    Ok(offset_list_page_data(items, total, params))
}

pub async fn get_capability<R: SkillsRepository>(
    service: &SkillsService<R>,
    tenant_id: u64,
    capability_id: u64,
) -> ApiResult<SdkWorkResourceData<SkillCapabilityRecord>> {
    Ok(item_data(
        service.get_capability(tenant_id, capability_id).await?,
    ))
}

pub async fn create_capability<R: SkillsRepository>(
    service: &SkillsService<R>,
    record: SkillCapabilityRecord,
) -> ApiResult<SdkWorkResourceData<SkillCapabilityRecord>> {
    Ok(item_data(service.create_capability(record).await?))
}

pub async fn update_capability<R: SkillsRepository>(
    service: &SkillsService<R>,
    record: SkillCapabilityRecord,
) -> ApiResult<SdkWorkResourceData<SkillCapabilityRecord>> {
    Ok(item_data(service.update_capability(record).await?))
}

pub async fn list_artifacts<R: SkillsRepository>(
    service: &SkillsService<R>,
    tenant_id: u64,
    package_id: u64,
    query: &SdkWorkListQuery,
) -> ApiResult<SdkWorkPageData<SkillArtifactRecord>> {
    query.validate()?;
    let params = query.offset_params()?;
    let (items, total) = service
        .list_artifacts_page(tenant_id, package_id, params)
        .await?;
    Ok(offset_list_page_data(items, total, params))
}

pub async fn list_installable_artifacts<R: SkillsRepository>(
    service: &SkillsService<R>,
    tenant_id: u64,
    organization_id: u64,
    user_id: u64,
    package_id: u64,
    query: &SdkWorkListQuery,
) -> ApiResult<SdkWorkPageData<SkillArtifactRecord>> {
    query.validate()?;
    let params = query.offset_params()?;
    let (items, total) = service
        .list_installable_artifacts_page(tenant_id, organization_id, user_id, package_id, params)
        .await?;
    Ok(offset_list_page_data(items, total, params))
}

pub async fn create_artifact<R: SkillsRepository>(
    service: &SkillsService<R>,
    artifact: SkillArtifactRecord,
) -> ApiResult<SdkWorkResourceData<SkillArtifactRecord>> {
    Ok(item_data(service.create_artifact(artifact).await?))
}

pub async fn install_skill<R: SkillsRepository>(
    service: &SkillsService<R>,
    record: SkillInstallationRecord,
) -> ApiResult<SdkWorkResourceData<SkillInstallationRecord>> {
    Ok(item_data(service.install_skill(record).await?))
}

pub async fn list_installations<R: SkillsRepository>(
    service: &SkillsService<R>,
    tenant_id: u64,
    organization_id: u64,
    subject_kind: &str,
    subject_id: u64,
    query: &SdkWorkListQuery,
) -> ApiResult<SdkWorkPageData<SkillInstallationRecord>> {
    query.validate()?;
    let params = query.offset_params()?;
    let (items, total) = service
        .list_installations_page(tenant_id, organization_id, subject_kind, subject_id, params)
        .await?;
    Ok(offset_list_page_data(items, total, params))
}
