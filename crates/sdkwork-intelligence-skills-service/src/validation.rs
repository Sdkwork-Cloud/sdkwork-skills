use regex_lite::Regex;
use sdkwork_drive_contract::DriveUri;
use sdkwork_skills_contract::{
    SkillArtifactRecord, SkillCapabilityRecord, SkillCategoryRecord, SkillInstallationRecord,
    SkillInstallationSubjectKind, SkillPackageRecord,
};
use sdkwork_utils_rust::trim;

use crate::{SkillsResult, SkillsServiceError};

fn invalid(message: impl Into<String>) -> SkillsServiceError {
    SkillsServiceError::InvalidArgument(message.into())
}

fn skill_key_pattern() -> Regex {
    Regex::new(r"^skill\.[a-z0-9_-]+(?:\.[a-z0-9_-]+)*$").expect("valid skill key regex")
}

fn capability_key_pattern() -> Regex {
    Regex::new(r"^[a-z0-9_-]+(?:\.[a-z0-9_-]+)+$").expect("valid capability key regex")
}

fn sha256_pattern() -> Regex {
    Regex::new(r"^[0-9a-f]{64}$").expect("valid sha256 regex")
}

pub fn validate_skill_key(value: &str) -> SkillsResult<()> {
    let value = trim(value);
    if !skill_key_pattern().is_match(&value) {
        return Err(invalid(
            "skill_key must use skill.<segment>[.<segment>] lowercase format",
        ));
    }
    Ok(())
}

fn validate_json_object(value: &serde_json::Value, field: &str) -> SkillsResult<()> {
    if !value.is_object() {
        return Err(invalid(format!("{field} must be a JSON object")));
    }
    Ok(())
}

pub fn validate_artifact_record(record: &SkillArtifactRecord) -> SkillsResult<()> {
    if trim(&record.version_label).is_empty() || record.version_label.len() > 128 {
        return Err(invalid("version_label must contain 1 to 128 characters"));
    }
    DriveUri::parse(&record.artifact_ref).map_err(|error| {
        invalid(format!(
            "artifact_ref must be a canonical Drive URI: {error}"
        ))
    })?;
    if !sha256_pattern().is_match(&record.checksum_sha256) {
        return Err(invalid(
            "checksum_sha256 must be 64 lowercase hexadecimal characters",
        ));
    }
    if trim(&record.entrypoint).is_empty() || record.entrypoint.len() > 255 {
        return Err(invalid("entrypoint must contain 1 to 255 characters"));
    }
    validate_json_object(&record.input_schema, "input_schema")?;
    validate_json_object(&record.output_schema, "output_schema")?;
    validate_json_object(&record.config_schema, "config_schema")?;
    validate_json_object(&record.default_config, "default_config")?;
    if record.capability_keys.len() > 64 {
        return Err(invalid(
            "capability_keys cannot contain more than 64 entries",
        ));
    }
    for key in &record.capability_keys {
        if !capability_key_pattern().is_match(&trim(key)) {
            return Err(invalid(format!("invalid capability key: {key}")));
        }
    }
    Ok(())
}

pub fn validate_skill_package_record(record: &SkillPackageRecord) -> SkillsResult<()> {
    validate_skill_key(&record.skill_key)?;
    for (field, value, max) in [
        ("package_key", record.package_key.as_str(), 128_usize),
        ("code", record.code.as_str(), 128_usize),
        ("display_name", record.display_name.as_str(), 255_usize),
    ] {
        let value = trim(value);
        if value.is_empty() || value.len() > max {
            return Err(invalid(format!(
                "{field} must contain 1 to {max} characters"
            )));
        }
    }
    if record.categories.len() > 32 {
        return Err(invalid("categories cannot contain more than 32 entries"));
    }
    if record.tags.len() > 64 {
        return Err(invalid("tags cannot contain more than 64 entries"));
    }
    Ok(())
}

pub fn validate_category_record(record: &SkillCategoryRecord) -> SkillsResult<()> {
    if trim(&record.code).is_empty() || record.code.len() > 128 {
        return Err(invalid("category code must contain 1 to 128 characters"));
    }
    if trim(&record.name).is_empty() || record.name.len() > 255 {
        return Err(invalid("category name must contain 1 to 255 characters"));
    }
    if trim(&record.permission_code).is_empty() {
        return Err(invalid("category permission_code is required"));
    }
    Ok(())
}

pub fn validate_capability_record(record: &SkillCapabilityRecord) -> SkillsResult<()> {
    if !capability_key_pattern().is_match(&trim(&record.capability_key)) {
        return Err(invalid(
            "capability_key must contain at least two lowercase dotted segments",
        ));
    }
    if trim(&record.display_name).is_empty() || record.display_name.len() > 255 {
        return Err(invalid(
            "capability display_name must contain 1 to 255 characters",
        ));
    }
    Ok(())
}

pub fn validate_installation_subject(subject_kind: &str, subject_id: u64) -> SkillsResult<()> {
    if SkillInstallationSubjectKind::parse(subject_kind).is_none() {
        return Err(invalid(
            "subject_kind must be user, workspace, project, or agent",
        ));
    }
    if subject_id == 0 {
        return Err(invalid("subject_id must be a positive Snowflake id"));
    }
    Ok(())
}

pub fn validate_installation_record(record: &SkillInstallationRecord) -> SkillsResult<()> {
    validate_installation_subject(record.subject_kind.as_str(), record.subject_id)?;
    if record.package_id == 0 || record.artifact_id == 0 || record.installed_by_user_id == 0 {
        return Err(invalid(
            "package_id, artifact_id, and installed_by_user_id are required",
        ));
    }
    validate_json_object(&record.config, "config")
}
