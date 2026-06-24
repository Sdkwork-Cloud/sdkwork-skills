use serde::{Deserialize, Serialize};

use crate::enums::{SkillInvocationKind, SkillLifecycleStatus, SkillVisibility};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SkillPackageRecord {
    pub id: u64,
    pub tenant_id: u64,
    pub organization_id: u64,
    pub owner_user_id: u64,
    pub skill_id: String,
    pub package_key: String,
    pub code: String,
    pub display_name: String,
    pub summary: Option<String>,
    pub description: Option<String>,
    pub invocation_kind: SkillInvocationKind,
    pub package_ref: String,
    pub entrypoint: String,
    pub input_schema_json: String,
    pub output_schema_json: String,
    pub capability_ids: Vec<String>,
    pub categories: Vec<String>,
    pub tags: Vec<String>,
    pub security_profile_id: Option<String>,
    pub category_id: Option<u64>,
    pub status: SkillLifecycleStatus,
    pub visibility: SkillVisibility,
    pub version: u64,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SkillRecord {
    pub id: u64,
    pub tenant_id: u64,
    pub organization_id: u64,
    pub owner_user_id: u64,
    pub skill_key: String,
    pub package_id: Option<u64>,
    pub name: String,
    pub summary: Option<String>,
    pub description: Option<String>,
    pub runtime: Option<String>,
    pub entrypoint: Option<String>,
    pub market_status: String,
    pub visibility: String,
    pub review_status: String,
    pub category_id: Option<u64>,
    pub enabled: bool,
    pub featured: bool,
    pub install_count: u64,
    pub tags: Vec<String>,
    pub capabilities: Vec<String>,
    pub version: u64,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SkillCategoryRecord {
    pub id: u64,
    pub tenant_id: u64,
    pub organization_id: u64,
    pub category_type: String,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub parent_id: Option<u64>,
    pub sort_weight: i32,
    pub visible: bool,
    pub status: i16,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserSkillInstallRecord {
    pub id: u64,
    pub tenant_id: u64,
    pub organization_id: u64,
    pub user_id: u64,
    pub skill_id: u64,
    pub package_id: Option<u64>,
    pub install_status: String,
    pub enabled: bool,
    pub config_json: String,
    pub installed_at: String,
    pub updated_at: String,
}
