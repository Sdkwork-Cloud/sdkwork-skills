use sdkwork_intelligence_skills_service::{
    SkillsRepository, SkillsService, SkillsServiceError,
};
use sdkwork_skills_contract::{
    SkillCategoryRecord, SkillPackageRecord, SkillRecord, UserSkillInstallRecord,
};
use sdkwork_utils_rust::{SdkWorkPageData, SdkWorkResourceData};

use crate::list_query::SdkWorkListQuery;
use crate::response::{item_data, paginate_items, ApiProblem, ApiResult};

pub fn resolve_tenant_id(headers: &axum::http::HeaderMap, default_tenant_id: u64) -> u64 {
    headers
        .get("x-sdkwork-tenant-id")
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.parse::<u64>().ok())
        .unwrap_or(default_tenant_id)
}

impl From<SkillsServiceError> for ApiProblem {
    fn from(error: SkillsServiceError) -> Self {
        match error {
            SkillsServiceError::NotFound(message) => ApiProblem::not_found(message),
            SkillsServiceError::InvalidArgument(message) => ApiProblem::bad_request(message),
            SkillsServiceError::Repository(message) => ApiProblem::internal_server_error(message),
        }
    }
}

fn filter_skill_records(items: Vec<SkillRecord>, query: &SdkWorkListQuery) -> Vec<SkillRecord> {
    let Some(keyword) = query.search_keyword() else {
        return items;
    };
    let keyword = keyword.to_lowercase();
    items
        .into_iter()
        .filter(|item| {
            item.name.to_lowercase().contains(&keyword)
                || item.skill_key.to_lowercase().contains(&keyword)
                || item
                    .summary
                    .as_deref()
                    .unwrap_or("")
                    .to_lowercase()
                    .contains(&keyword)
        })
        .collect()
}

fn filter_skill_packages(
    items: Vec<SkillPackageRecord>,
    query: &SdkWorkListQuery,
) -> Vec<SkillPackageRecord> {
    let Some(keyword) = query.search_keyword() else {
        return items;
    };
    let keyword = keyword.to_lowercase();
    items
        .into_iter()
        .filter(|item| {
            item.display_name.to_lowercase().contains(&keyword)
                || item.skill_id.to_lowercase().contains(&keyword)
                || item.code.to_lowercase().contains(&keyword)
        })
        .collect()
}

fn filter_categories(
    items: Vec<SkillCategoryRecord>,
    query: &SdkWorkListQuery,
) -> Vec<SkillCategoryRecord> {
    let Some(keyword) = query.search_keyword() else {
        return items;
    };
    let keyword = keyword.to_lowercase();
    items
        .into_iter()
        .filter(|item| {
            item.name.to_lowercase().contains(&keyword)
                || item.code.to_lowercase().contains(&keyword)
        })
        .collect()
}

pub async fn list_hub_skills<R: SkillsRepository>(
    service: &SkillsService<R>,
    tenant_id: u64,
    query: &SdkWorkListQuery,
) -> ApiResult<SdkWorkPageData<SkillRecord>> {
    query.validate()?;
    let items = service.list_hub_skills(tenant_id).await?;
    Ok(paginate_items(filter_skill_records(items, query), query))
}

pub async fn list_skill_packages<R: SkillsRepository>(
    service: &SkillsService<R>,
    tenant_id: u64,
    query: &SdkWorkListQuery,
) -> ApiResult<SdkWorkPageData<SkillPackageRecord>> {
    query.validate()?;
    let items = service.list_skill_packages(tenant_id).await?;
    Ok(paginate_items(filter_skill_packages(items, query), query))
}

pub async fn list_categories<R: SkillsRepository>(
    service: &SkillsService<R>,
    tenant_id: u64,
    category_type: &str,
    query: &SdkWorkListQuery,
) -> ApiResult<SdkWorkPageData<SkillCategoryRecord>> {
    query.validate()?;
    let items = service
        .list_categories(tenant_id, category_type)
        .await?;
    Ok(paginate_items(filter_categories(items, query), query))
}

pub async fn get_skill_package<R: SkillsRepository>(
    service: &SkillsService<R>,
    tenant_id: u64,
    skill_id: &str,
) -> ApiResult<SdkWorkResourceData<SkillPackageRecord>> {
    let record = service.get_skill_package(tenant_id, skill_id).await?;
    Ok(item_data(record))
}

pub async fn install_skill<R: SkillsRepository>(
    service: &SkillsService<R>,
    record: UserSkillInstallRecord,
) -> ApiResult<SdkWorkResourceData<UserSkillInstallRecord>> {
    let saved = service.install_skill(record).await?;
    Ok(item_data(saved))
}

pub async fn upsert_skill_package<R: SkillsRepository>(
    service: &SkillsService<R>,
    record: SkillPackageRecord,
) -> ApiResult<SdkWorkResourceData<SkillPackageRecord>> {
    let saved = service.upsert_skill_package(record).await?;
    Ok(item_data(saved))
}

pub async fn upsert_category<R: SkillsRepository>(
    service: &SkillsService<R>,
    record: SkillCategoryRecord,
) -> ApiResult<SdkWorkResourceData<SkillCategoryRecord>> {
    let saved = service.upsert_category(record).await?;
    Ok(item_data(saved))
}

pub async fn delete_skill_package<R: SkillsRepository>(
    service: &SkillsService<R>,
    tenant_id: u64,
    skill_id: &str,
) -> ApiResult<SdkWorkResourceData<SkillPackageRecord>> {
    let deleted = service.delete_skill_package(tenant_id, skill_id).await?;
    Ok(item_data(deleted))
}

pub async fn get_skill<R: SkillsRepository>(
    service: &SkillsService<R>,
    tenant_id: u64,
    skill_key: &str,
) -> ApiResult<SdkWorkResourceData<SkillRecord>> {
    let record = service.get_skill(tenant_id, skill_key).await?;
    Ok(item_data(record))
}
