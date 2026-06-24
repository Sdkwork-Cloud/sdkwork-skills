use sdkwork_skills_contract::{SkillInvocationKind, SkillPackageRecord};

use crate::{SkillsResult, SkillsServiceError};

const SKILL_ID_PATTERN: &str = r"^skill\.[a-z0-9_-]+(\.[a-z0-9_-]+)*$";

pub fn validate_skill_id(skill_id: &str) -> SkillsResult<()> {
    let valid = regex_lite::Regex::new(SKILL_ID_PATTERN)
        .map_err(|error| SkillsServiceError::Repository(error.to_string()))?
        .is_match(skill_id);
    if !valid {
        return Err(SkillsServiceError::InvalidArgument(format!(
            "skill_id must match {SKILL_ID_PATTERN}: {skill_id}"
        )));
    }
    Ok(())
}

pub fn validate_skill_package_record(record: &SkillPackageRecord) -> SkillsResult<()> {
    validate_skill_id(record.skill_id.as_str())?;
    if record.code.trim().is_empty() {
        return Err(SkillsServiceError::InvalidArgument(
            "code must not be empty".to_string(),
        ));
    }
    if record.display_name.trim().is_empty() {
        return Err(SkillsServiceError::InvalidArgument(
            "display_name must not be empty".to_string(),
        ));
    }
    if record.package_ref.trim().is_empty() {
        return Err(SkillsServiceError::InvalidArgument(
            "package_ref must not be empty".to_string(),
        ));
    }
    if record.entrypoint.trim().is_empty() {
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
