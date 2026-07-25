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

pub(crate) fn uint64_to_int64(value: u64, field: &str) -> SkillsResult<i64> {
    i64::try_from(value).map_err(|_| {
        SkillsServiceError::InvalidArgument(format!(
            "{field} exceeds the PostgreSQL signed-int64 range"
        ))
    })
}

pub(crate) fn optional_uint64_to_int64(
    value: Option<u64>,
    field: &str,
) -> SkillsResult<Option<i64>> {
    value.map(|value| uint64_to_int64(value, field)).transpose()
}

pub(crate) fn int64_to_uint64(value: i64, field: &str) -> SkillsResult<u64> {
    u64::try_from(value).map_err(|_| {
        SkillsServiceError::Repository(format!(
            "database field {field} must not contain a negative signed-int64 value"
        ))
    })
}

pub(crate) fn optional_int64_to_uint64(
    value: Option<i64>,
    field: &str,
) -> SkillsResult<Option<u64>> {
    value.map(|value| int64_to_uint64(value, field)).transpose()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sql_int64_conversion_accepts_boundary_and_rejects_overflow() {
        let signed_max = u64::try_from(i64::MAX).expect("i64::MAX fits in u64");
        assert_eq!(uint64_to_int64(signed_max, "id").unwrap(), i64::MAX);
        assert!(matches!(
            uint64_to_int64(signed_max + 1, "id"),
            Err(SkillsServiceError::InvalidArgument(message))
                if message == "id exceeds the PostgreSQL signed-int64 range"
        ));
    }

    #[test]
    fn domain_uint64_conversion_rejects_negative_database_values() {
        assert_eq!(int64_to_uint64(0, "version").unwrap(), 0);
        assert!(matches!(
            int64_to_uint64(-1, "id"),
            Err(SkillsServiceError::Repository(message))
                if message == "database field id must not contain a negative signed-int64 value"
        ));
    }

    #[test]
    fn postgres_repository_contains_no_lossy_business_id_casts() {
        let source = include_str!("postgres.rs");
        for forbidden in [" as i64", " as u64"] {
            assert!(
                !source.contains(forbidden),
                "PostgreSQL repository must use checked conversion instead of {forbidden}"
            );
        }
    }
}
