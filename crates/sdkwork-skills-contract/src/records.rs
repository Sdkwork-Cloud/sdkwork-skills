use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::enums::{
    SkillArtifactStatus, SkillCapabilityRiskLevel, SkillInstallationSubjectKind,
    SkillInvocationKind, SkillLifecycleStatus, SkillVisibility,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkillArtifactRecord {
    #[serde(with = "sdkwork_utils_rust::serde_uint64")]
    pub id: u64,
    pub uuid: String,
    #[serde(with = "sdkwork_utils_rust::serde_uint64")]
    pub tenant_id: u64,
    #[serde(with = "sdkwork_utils_rust::serde_uint64")]
    pub package_id: u64,
    pub version_label: String,
    pub artifact_ref: String,
    pub checksum_sha256: String,
    #[serde(with = "sdkwork_utils_rust::serde_uint64::option")]
    pub size_bytes: Option<u64>,
    pub invocation_kind: SkillInvocationKind,
    pub entrypoint: String,
    pub input_schema: Value,
    pub output_schema: Value,
    pub config_schema: Value,
    pub default_config: Value,
    pub security_profile_id: Option<String>,
    pub status: SkillArtifactStatus,
    pub capability_keys: Vec<String>,
    pub published_at: Option<String>,
    pub yanked_at: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkillPackageRecord {
    #[serde(with = "sdkwork_utils_rust::serde_uint64")]
    pub id: u64,
    pub uuid: String,
    #[serde(with = "sdkwork_utils_rust::serde_uint64")]
    pub tenant_id: u64,
    #[serde(with = "sdkwork_utils_rust::serde_uint64")]
    pub organization_id: u64,
    #[serde(with = "sdkwork_utils_rust::serde_uint64")]
    pub owner_user_id: u64,
    pub skill_key: String,
    pub package_key: String,
    pub code: String,
    pub display_name: String,
    pub summary: Option<String>,
    pub description: Option<String>,
    pub categories: Vec<String>,
    pub tags: Vec<String>,
    pub status: SkillLifecycleStatus,
    pub visibility: SkillVisibility,
    pub featured: bool,
    pub sort_weight: i32,
    #[serde(with = "sdkwork_utils_rust::serde_uint64")]
    pub version: u64,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkillRecord {
    #[serde(with = "sdkwork_utils_rust::serde_uint64")]
    pub id: u64,
    pub uuid: String,
    #[serde(with = "sdkwork_utils_rust::serde_uint64")]
    pub tenant_id: u64,
    #[serde(with = "sdkwork_utils_rust::serde_uint64")]
    pub organization_id: u64,
    pub skill_key: String,
    #[serde(with = "sdkwork_utils_rust::serde_uint64")]
    pub package_id: u64,
    pub name: String,
    pub summary: Option<String>,
    pub description: Option<String>,
    pub market_status: String,
    pub visibility: SkillVisibility,
    pub review_status: String,
    pub categories: Vec<String>,
    pub enabled: bool,
    pub featured: bool,
    #[serde(with = "sdkwork_utils_rust::serde_uint64")]
    pub install_count: u64,
    pub tags: Vec<String>,
    #[serde(with = "sdkwork_utils_rust::serde_uint64")]
    pub version: u64,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkillCategoryRecord {
    #[serde(with = "sdkwork_utils_rust::serde_uint64")]
    pub id: u64,
    pub uuid: String,
    #[serde(with = "sdkwork_utils_rust::serde_uint64")]
    pub tenant_id: u64,
    #[serde(with = "sdkwork_utils_rust::serde_uint64")]
    pub organization_id: u64,
    pub category_type: String,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    #[serde(with = "sdkwork_utils_rust::serde_uint64::option")]
    pub parent_id: Option<u64>,
    pub sort_weight: i32,
    pub permission_code: String,
    pub visible: bool,
    pub status: i16,
    #[serde(with = "sdkwork_utils_rust::serde_uint64")]
    pub version: u64,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkillCapabilityRecord {
    #[serde(with = "sdkwork_utils_rust::serde_uint64")]
    pub id: u64,
    pub uuid: String,
    #[serde(with = "sdkwork_utils_rust::serde_uint64")]
    pub tenant_id: u64,
    #[serde(with = "sdkwork_utils_rust::serde_uint64")]
    pub organization_id: u64,
    pub capability_key: String,
    pub display_name: String,
    pub description: Option<String>,
    pub risk_level: SkillCapabilityRiskLevel,
    pub status: i16,
    #[serde(with = "sdkwork_utils_rust::serde_uint64")]
    pub version: u64,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkillInstallationRecord {
    #[serde(with = "sdkwork_utils_rust::serde_uint64")]
    pub id: u64,
    pub uuid: String,
    #[serde(with = "sdkwork_utils_rust::serde_uint64")]
    pub tenant_id: u64,
    #[serde(with = "sdkwork_utils_rust::serde_uint64")]
    pub organization_id: u64,
    pub subject_kind: SkillInstallationSubjectKind,
    #[serde(with = "sdkwork_utils_rust::serde_uint64")]
    pub subject_id: u64,
    #[serde(with = "sdkwork_utils_rust::serde_uint64")]
    pub skill_id: u64,
    #[serde(with = "sdkwork_utils_rust::serde_uint64")]
    pub package_id: u64,
    #[serde(with = "sdkwork_utils_rust::serde_uint64")]
    pub artifact_id: u64,
    #[serde(with = "sdkwork_utils_rust::serde_uint64")]
    pub installed_by_user_id: u64,
    pub install_status: String,
    pub enabled: bool,
    pub config: Value,
    #[serde(with = "sdkwork_utils_rust::serde_uint64")]
    pub version: u64,
    pub installed_at: String,
    pub updated_at: String,
}
