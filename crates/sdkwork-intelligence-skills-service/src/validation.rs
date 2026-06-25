use sdkwork_drive_contract::DriveUri;
use sdkwork_skills_contract::{
    SkillCategoryRecord, SkillInvocationKind, SkillPackageRecord,
};
use sdkwork_utils_rust::{is_blank, trim};

use crate::{SkillsResult, SkillsServiceError};

const SKILL_ID_PATTERN: &str = r"^skill\.[a-z0-9_-]+(\.[a-z0-9_-]+)*$";

pub fn validate_skill_id(skill_id: &str) -> SkillsResult<()> {
    let normalized = trim(skill_id);
    let valid = regex_lite::Regex::new(SKILL_ID_PATTERN)
        .map_err(|error| SkillsServiceError::Repository(error.to_string()))?
        .is_match(&normalized);
    if !valid {
        return Err(SkillsServiceError::InvalidArgument(format!(
            "skill_id must match {SKILL_ID_PATTERN}: {skill_id}"
        )));
    }
    Ok(())
}

pub fn validate_package_ref(package_ref: &str) -> SkillsResult<()> {
    let normalized = trim(package_ref);
    if is_blank(Some(&normalized)) {
        return Err(SkillsServiceError::InvalidArgument(
            "package_ref must not be empty".to_string(),
        ));
    }
    DriveUri::parse(&normalized).map_err(|error| {
        SkillsServiceError::InvalidArgument(format!(
            "package_ref must be a canonical sdkwork-drive URI: {error}"
        ))
    })?;
    Ok(())
}

pub fn validate_skill_package_record(record: &SkillPackageRecord) -> SkillsResult<()> {
    validate_skill_id(record.skill_id.as_str())?;
    if is_blank(Some(trim(record.code.as_str()).as_str())) {
        return Err(SkillsServiceError::InvalidArgument(
            "code must not be empty".to_string(),
        ));
    }
    if is_blank(Some(trim(record.display_name.as_str()).as_str())) {
        return Err(SkillsServiceError::InvalidArgument(
            "display_name must not be empty".to_string(),
        ));
    }
    validate_package_ref(record.package_ref.as_str())?;
    if is_blank(Some(trim(record.entrypoint.as_str()).as_str())) {
        return Err(SkillsServiceError::InvalidArgument(
            "entrypoint must not be empty".to_string(),
        ));
    }
    validate_invocation_kind(record.invocation_kind)?;
    validate_json_object(record.input_schema_json.as_str(), "input_schema_json")?;
    validate_json_object(record.output_schema_json.as_str(), "output_schema_json")?;
    Ok(())
}

pub fn validate_invocation_kind(kind: SkillInvocationKind) -> SkillsResult<()> {
    match kind {
        SkillInvocationKind::LocalWorkflow
        | SkillInvocationKind::ProcessAdapter
        | SkillInvocationKind::McpTool
        | SkillInvocationKind::KernelProvider => Ok(()),
    }
}

pub fn validate_category_record(record: &SkillCategoryRecord) -> SkillsResult<()> {
    if is_blank(Some(trim(record.code.as_str()).as_str())) {
        return Err(SkillsServiceError::InvalidArgument(
            "category code must not be empty".to_string(),
        ));
    }
    if is_blank(Some(trim(record.name.as_str()).as_str())) {
        return Err(SkillsServiceError::InvalidArgument(
            "category name must not be empty".to_string(),
        ));
    }
    if record.category_type != "skill_market" && record.category_type != "skills_collection" {
        return Err(SkillsServiceError::InvalidArgument(format!(
            "unsupported category_type: {}",
            record.category_type
        )));
    }
    Ok(())
}

fn validate_json_object(input: &str, field: &str) -> SkillsResult<()> {
    let value: serde_json::Value = serde_json::from_str(input).map_err(|error| {
        SkillsServiceError::InvalidArgument(format!("{field} must be valid json: {error}"))
    })?;
    if !value.is_object() && !value.is_array() {
        return Err(SkillsServiceError::InvalidArgument(format!(
            "{field} must be a json object or array"
        )));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use sdkwork_skills_contract::{
        SkillInvocationKind, SkillLifecycleStatus, SkillPackageRecord, SkillVisibility,
    };

    fn sample_package(package_ref: &str) -> SkillPackageRecord {
        SkillPackageRecord {
            id: 1,
            tenant_id: 1,
            organization_id: 0,
            owner_user_id: 0,
            skill_id: "skill.demo.sample".to_string(),
            package_key: "demo".to_string(),
            code: "demo".to_string(),
            display_name: "Demo".to_string(),
            summary: None,
            description: None,
            invocation_kind: SkillInvocationKind::LocalWorkflow,
            package_ref: package_ref.to_string(),
            entrypoint: "run".to_string(),
            input_schema_json: "{}".to_string(),
            output_schema_json: "{}".to_string(),
            capability_ids: vec![],
            categories: vec![],
            tags: vec![],
            security_profile_id: None,
            category_id: None,
            status: SkillLifecycleStatus::Active,
            visibility: SkillVisibility::Tenant,
            version: 1,
            created_at: "2026-01-01T00:00:00Z".to_string(),
            updated_at: "2026-01-01T00:00:00Z".to_string(),
            deleted_at: None,
        }
    }

    #[test]
    fn accepts_drive_package_ref() {
        assert!(validate_skill_package_record(&sample_package(
            "drive://spaces/skills-space/nodes/pkg-node-1"
        ))
        .is_ok());
    }

    #[test]
    fn rejects_invalid_package_ref() {
        assert!(validate_skill_package_record(&sample_package("invalid-ref")).is_err());
        assert!(validate_skill_package_record(&sample_package("file://demo")).is_err());
    }
}
