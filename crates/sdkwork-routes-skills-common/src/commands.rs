use sdkwork_skills_contract::{
    SkillArtifactStatus, SkillCapabilityRiskLevel, SkillCategoryType, SkillInstallationSubjectKind,
    SkillInvocationKind, SkillLifecycleStatus, SkillVisibility,
};
use serde::{de::Error as _, Deserialize, Deserializer};
use serde_json::{Map, Value};

#[derive(Debug, Default, PartialEq, Eq)]
pub enum NullablePatch<T> {
    #[default]
    Unset,
    Null,
    Value(T),
}

impl<'de, T> Deserialize<'de> for NullablePatch<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(match Option::<T>::deserialize(deserializer)? {
            Some(value) => Self::Value(value),
            None => Self::Null,
        })
    }
}

impl<T> NullablePatch<T> {
    pub fn apply_to(self, target: &mut Option<T>) {
        match self {
            Self::Unset => {}
            Self::Null => *target = None,
            Self::Value(value) => *target = Some(value),
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub enum NullableSnowflakeIdPatch {
    #[default]
    Unset,
    Null,
    Value(u64),
}

impl<'de> Deserialize<'de> for NullableSnowflakeIdPatch {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Option::<String>::deserialize(deserializer)?;
        match value {
            None => Ok(Self::Null),
            Some(value) => value
                .parse::<u64>()
                .ok()
                .filter(|id| *id > 0)
                .map(Self::Value)
                .ok_or_else(|| D::Error::custom("Snowflake id must be a positive decimal string")),
        }
    }
}

impl NullableSnowflakeIdPatch {
    pub fn apply_to(self, target: &mut Option<u64>) {
        match self {
            Self::Unset => {}
            Self::Null => *target = None,
            Self::Value(value) => *target = Some(value),
        }
    }
}

fn empty_object() -> Value {
    Value::Object(Map::new())
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SkillInstallationTargetCommand {
    pub kind: SkillInstallationSubjectKind,
    #[serde(with = "sdkwork_utils_rust::serde_uint64")]
    pub id: u64,
}

/// App-api installation command. Omitting `target` always means the authenticated user.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CreateSkillInstallationCommand {
    #[serde(with = "sdkwork_utils_rust::serde_uint64")]
    pub artifact_id: u64,
    #[serde(default)]
    pub target: Option<SkillInstallationTargetCommand>,
    #[serde(default = "empty_object")]
    pub config: Value,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CreateSkillArtifactCommand {
    pub version_label: String,
    pub artifact_ref: String,
    pub checksum_sha256: String,
    #[serde(default, with = "sdkwork_utils_rust::serde_uint64::option")]
    pub size_bytes: Option<u64>,
    pub invocation_kind: SkillInvocationKind,
    pub entrypoint: String,
    #[serde(default = "empty_object")]
    pub input_schema: Value,
    #[serde(default = "empty_object")]
    pub output_schema: Value,
    #[serde(default = "empty_object")]
    pub config_schema: Value,
    #[serde(default = "empty_object")]
    pub default_config: Value,
    #[serde(default)]
    pub security_profile_id: Option<String>,
    #[serde(default)]
    pub status: Option<SkillArtifactStatus>,
    #[serde(default)]
    pub capability_keys: Vec<String>,
}

/// Backend-api package aggregate creation command.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CreateSkillPackageCommand {
    pub skill_key: String,
    #[serde(default)]
    pub package_key: Option<String>,
    pub code: String,
    pub display_name: String,
    #[serde(default)]
    pub summary: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub categories: Vec<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub status: Option<SkillLifecycleStatus>,
    #[serde(default)]
    pub visibility: Option<SkillVisibility>,
    #[serde(default)]
    pub featured: bool,
    #[serde(default)]
    pub sort_weight: i32,
    pub initial_artifact: CreateSkillArtifactCommand,
}

impl CreateSkillPackageCommand {
    pub fn resolved_package_key(&self) -> String {
        self.package_key
            .clone()
            .filter(|value| !value.trim().is_empty())
            .unwrap_or_else(|| self.code.clone())
    }
}

/// Backend-api partial package update. Immutable keys are intentionally absent.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct UpdateSkillPackageCommand {
    #[serde(with = "sdkwork_utils_rust::serde_uint64")]
    pub version: u64,
    #[serde(default)]
    pub display_name: Option<String>,
    #[serde(default)]
    pub summary: NullablePatch<String>,
    #[serde(default)]
    pub description: NullablePatch<String>,
    #[serde(default)]
    pub categories: Option<Vec<String>>,
    #[serde(default)]
    pub tags: Option<Vec<String>>,
    #[serde(default)]
    pub status: Option<SkillLifecycleStatus>,
    #[serde(default)]
    pub visibility: Option<SkillVisibility>,
    #[serde(default)]
    pub featured: Option<bool>,
    #[serde(default)]
    pub sort_weight: Option<i32>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CreateSkillCategoryCommand {
    #[serde(default)]
    pub category_type: Option<SkillCategoryType>,
    pub code: String,
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default, with = "sdkwork_utils_rust::serde_uint64::option")]
    pub parent_id: Option<u64>,
    #[serde(default)]
    pub sort_weight: i32,
    #[serde(default)]
    pub permission_code: Option<String>,
    #[serde(default)]
    pub visible: Option<bool>,
    #[serde(default)]
    pub status: Option<i16>,
}

/// Backend-api partial category update. Category type and code are immutable.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct UpdateSkillCategoryCommand {
    #[serde(with = "sdkwork_utils_rust::serde_uint64")]
    pub version: u64,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: NullablePatch<String>,
    #[serde(default)]
    pub parent_id: NullableSnowflakeIdPatch,
    #[serde(default)]
    pub sort_weight: Option<i32>,
    #[serde(default)]
    pub permission_code: Option<String>,
    #[serde(default)]
    pub visible: Option<bool>,
    #[serde(default)]
    pub status: Option<i16>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CreateSkillCapabilityCommand {
    pub capability_key: String,
    pub display_name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub risk_level: Option<SkillCapabilityRiskLevel>,
    #[serde(default)]
    pub status: Option<i16>,
}

/// Backend-api partial capability update. Capability key is immutable.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct UpdateSkillCapabilityCommand {
    #[serde(with = "sdkwork_utils_rust::serde_uint64")]
    pub version: u64,
    #[serde(default)]
    pub display_name: Option<String>,
    #[serde(default)]
    pub description: NullablePatch<String>,
    #[serde(default)]
    pub risk_level: Option<SkillCapabilityRiskLevel>,
    #[serde(default)]
    pub status: Option<i16>,
}
