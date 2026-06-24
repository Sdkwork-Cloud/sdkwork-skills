use std::sync::Arc;

use sdkwork_intelligence_skills_service::{SkillsRepository, SkillsService, SkillsServiceError};
use sdkwork_skills_contract::UserSkillInstallRecord;

pub type SharedSkillsService<R> = Arc<SkillsService<R>>;

pub fn resolve_tenant_id(headers: &axum::http::HeaderMap, default_tenant_id: u64) -> u64 {
    headers
        .get("x-sdkwork-tenant-id")
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.parse::<u64>().ok())
        .unwrap_or(default_tenant_id)
}

pub fn service_error_response(error: SkillsServiceError) -> (axum::http::StatusCode, String) {
    match error {
        SkillsServiceError::NotFound(message) => (axum::http::StatusCode::NOT_FOUND, message),
        SkillsServiceError::InvalidArgument(message) => {
            (axum::http::StatusCode::BAD_REQUEST, message)
        }
        SkillsServiceError::Repository(message) => {
            (axum::http::StatusCode::INTERNAL_SERVER_ERROR, message)
        }
    }
}

pub fn items_response<T: serde::Serialize>(items: Vec<T>) -> serde_json::Value {
    serde_json::json!({ "items": items })
}

pub fn record_response<T: serde::Serialize>(record: T) -> serde_json::Value {
    serde_json::json!({ "data": record })
}

pub async fn list_hub_skills<R: SkillsRepository>(
    service: &SkillsService<R>,
    tenant_id: u64,
) -> Result<serde_json::Value, (axum::http::StatusCode, String)> {
    let items = service
        .list_hub_skills(tenant_id)
        .await
        .map_err(service_error_response)?;
    Ok(items_response(items))
}

pub async fn list_skill_packages<R: SkillsRepository>(
    service: &SkillsService<R>,
    tenant_id: u64,
) -> Result<serde_json::Value, (axum::http::StatusCode, String)> {
    let items = service
        .list_skill_packages(tenant_id)
        .await
        .map_err(service_error_response)?;
    Ok(items_response(items))
}

pub async fn list_categories<R: SkillsRepository>(
    service: &SkillsService<R>,
    tenant_id: u64,
    category_type: &str,
) -> Result<serde_json::Value, (axum::http::StatusCode, String)> {
    let items = service
        .list_categories(tenant_id, category_type)
        .await
        .map_err(service_error_response)?;
    Ok(items_response(items))
}

pub async fn get_skill_package<R: SkillsRepository>(
    service: &SkillsService<R>,
    tenant_id: u64,
    skill_id: &str,
) -> Result<serde_json::Value, (axum::http::StatusCode, String)> {
    let record = service
        .get_skill_package(tenant_id, skill_id)
        .await
        .map_err(service_error_response)?;
    Ok(record_response(record))
}

pub async fn install_skill<R: SkillsRepository>(
    service: &SkillsService<R>,
    record: UserSkillInstallRecord,
) -> Result<serde_json::Value, (axum::http::StatusCode, String)> {
    let saved = service
        .install_skill(record)
        .await
        .map_err(service_error_response)?;
    Ok(record_response(saved))
}
