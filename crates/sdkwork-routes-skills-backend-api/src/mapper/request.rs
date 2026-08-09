use sdkwork_routes_skills_common::{CreateSkillCapabilityCommand, CreateSkillCategoryCommand};
use sdkwork_skills_contract::{
    resolve_category_package_permission, SkillCapabilityRecord, SkillCapabilityRiskLevel,
    SkillCategoryRecord, SkillCategoryType,
};

use crate::SkillsBackendRequestContext;

pub(crate) fn category_record(
    context: &SkillsBackendRequestContext,
    command: CreateSkillCategoryCommand,
) -> SkillCategoryRecord {
    let permission_code = resolve_category_package_permission(
        command.code.as_str(),
        command.permission_code.as_deref(),
    );
    SkillCategoryRecord {
        id: 0,
        uuid: String::new(),
        tenant_id: context.tenant_id,
        organization_id: context.organization_id,
        category_type: command
            .category_type
            .unwrap_or(SkillCategoryType::SkillMarket)
            .as_str()
            .to_string(),
        code: command.code,
        name: command.name,
        description: command.description,
        parent_id: command.parent_id,
        sort_weight: command.sort_weight,
        permission_code,
        visible: command.visible.unwrap_or(true),
        status: command.status.unwrap_or(1),
        version: 0,
        created_at: String::new(),
        updated_at: String::new(),
    }
}

pub(crate) fn capability_record(
    context: &SkillsBackendRequestContext,
    command: CreateSkillCapabilityCommand,
) -> SkillCapabilityRecord {
    SkillCapabilityRecord {
        id: 0,
        uuid: String::new(),
        tenant_id: context.tenant_id,
        organization_id: context.organization_id,
        capability_key: command.capability_key,
        display_name: command.display_name,
        description: command.description,
        risk_level: command
            .risk_level
            .unwrap_or(SkillCapabilityRiskLevel::Standard),
        status: command.status.unwrap_or(1),
        version: 0,
        created_at: String::new(),
        updated_at: String::new(),
    }
}
