use sdkwork_skills_contract::{SkillInvocationKind, SkillVisibility};
use serde::Deserialize;

/// App-api `InstallSkillCommand` (`userSkills.install`).
#[derive(Debug, Deserialize)]
pub struct InstallSkillCommand {
    #[serde(with = "sdkwork_utils_rust::serde_int64")]
    pub skill_id: i64,
    #[serde(default, with = "sdkwork_utils_rust::serde_int64::option")]
    pub package_id: Option<i64>,
}

/// Backend-api `CreateSkillPackageCommand` (`skillPackages.create`).
#[derive(Debug, Deserialize)]
pub struct CreateSkillPackageCommand {
    pub skill_id: String,
    #[serde(default)]
    pub package_key: Option<String>,
    pub code: String,
    pub display_name: String,
    #[serde(default)]
    pub summary: Option<String>,
    pub invocation_kind: SkillInvocationKind,
    pub package_ref: String,
    pub entrypoint: String,
    #[serde(default)]
    pub capability_ids: Option<Vec<String>>,
    #[serde(default)]
    pub categories: Option<Vec<String>>,
    #[serde(default)]
    pub tags: Option<Vec<String>>,
}

impl CreateSkillPackageCommand {
    pub fn resolved_package_key(&self) -> String {
        self.package_key
            .clone()
            .filter(|value| !value.trim().is_empty())
            .unwrap_or_else(|| self.code.clone())
    }

    pub fn resolved_capability_ids(&self) -> Vec<String> {
        self.capability_ids.clone().unwrap_or_default()
    }

    pub fn resolved_categories(&self) -> Vec<String> {
        self.categories.clone().unwrap_or_default()
    }

    pub fn resolved_tags(&self) -> Vec<String> {
        self.tags.clone().unwrap_or_default()
    }
}

/// Backend-api partial update body (`skillPackages.update`).
#[derive(Debug, Deserialize)]
pub struct UpdateSkillPackageCommand {
    #[serde(default)]
    pub package_key: Option<String>,
    #[serde(default)]
    pub code: Option<String>,
    #[serde(default)]
    pub display_name: Option<String>,
    #[serde(default)]
    pub summary: Option<String>,
    #[serde(default)]
    pub invocation_kind: Option<SkillInvocationKind>,
    #[serde(default)]
    pub package_ref: Option<String>,
    #[serde(default)]
    pub entrypoint: Option<String>,
    #[serde(default)]
    pub capability_ids: Option<Vec<String>>,
    #[serde(default)]
    pub categories: Option<Vec<String>>,
    #[serde(default)]
    pub tags: Option<Vec<String>>,
    #[serde(default)]
    pub visibility: Option<SkillVisibility>,
}

/// Backend-api partial category update (`categories.update`).
#[derive(Debug, Deserialize)]
pub struct UpdateSkillCategoryCommand {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub sort_weight: Option<i32>,
    #[serde(default)]
    pub permission_code: Option<String>,
}

/// Backend-api `CreateSkillCategoryCommand` (`categories.create`).
#[derive(Debug, Deserialize)]
pub struct CreateSkillCategoryCommand {
    pub code: String,
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub sort_weight: Option<i32>,
    #[serde(default)]
    pub permission_code: Option<String>,
}
