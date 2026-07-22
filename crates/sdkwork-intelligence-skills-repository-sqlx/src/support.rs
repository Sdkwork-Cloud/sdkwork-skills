use sdkwork_database_id::SnowflakeIdGenerator;
use sdkwork_intelligence_skills_service::{SkillsResult, SkillsServiceError};
use sdkwork_skills_contract::{
    SkillArtifactStatus, SkillCapabilityRiskLevel, SkillInstallationSubjectKind,
    SkillInvocationKind, SkillLifecycleStatus, SkillVisibility,
};

pub(crate) fn next_id(generator: &SnowflakeIdGenerator) -> SkillsResult<i64> {
    generator
        .generate()
        .map_err(|error| SkillsServiceError::Repository(error.to_string()))
}

pub(crate) fn new_uuid() -> String {
    sdkwork_database_id::uuid_v4()
}

pub(crate) fn search_pattern(keyword: Option<&str>) -> String {
    keyword
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(|value| format!("%{}%", value.replace('%', "\\%").replace('_', "\\_")))
        .unwrap_or_else(|| "%".to_string())
}

pub(crate) fn map_sqlx(error: sqlx::Error) -> SkillsServiceError {
    match &error {
        sqlx::Error::Database(database) if database.is_unique_violation() => {
            SkillsServiceError::Conflict(database.message().to_string())
        }
        _ => SkillsServiceError::Repository(error.to_string()),
    }
}

pub(crate) fn invocation(value: &str) -> SkillsResult<SkillInvocationKind> {
    SkillInvocationKind::parse(value).ok_or_else(|| {
        SkillsServiceError::Repository(format!("invalid invocation_kind in database: {value}"))
    })
}

pub(crate) fn lifecycle(value: i16) -> SkillsResult<SkillLifecycleStatus> {
    SkillLifecycleStatus::from_db_code(value).ok_or_else(|| {
        SkillsServiceError::Repository(format!("invalid skill lifecycle status: {value}"))
    })
}

pub(crate) fn visibility(value: i16) -> SkillsResult<SkillVisibility> {
    SkillVisibility::from_db_code(value)
        .ok_or_else(|| SkillsServiceError::Repository(format!("invalid skill visibility: {value}")))
}

pub(crate) fn artifact_status(value: &str) -> SkillsResult<SkillArtifactStatus> {
    SkillArtifactStatus::parse(value)
        .ok_or_else(|| SkillsServiceError::Repository(format!("invalid artifact status: {value}")))
}

pub(crate) fn subject_kind(value: &str) -> SkillsResult<SkillInstallationSubjectKind> {
    SkillInstallationSubjectKind::parse(value).ok_or_else(|| {
        SkillsServiceError::Repository(format!("invalid installation subject kind: {value}"))
    })
}

pub(crate) fn capability_risk(value: &str) -> SkillsResult<SkillCapabilityRiskLevel> {
    SkillCapabilityRiskLevel::parse(value).ok_or_else(|| {
        SkillsServiceError::Repository(format!("invalid capability risk level: {value}"))
    })
}
