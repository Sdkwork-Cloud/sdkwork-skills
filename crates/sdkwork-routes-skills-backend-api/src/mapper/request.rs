use sdkwork_routes_skills_common::{
    CreateSkillArtifactCommand, CreateSkillCapabilityCommand, CreateSkillCategoryCommand,
    CreateSkillPackageCommand,
};
use sdkwork_skills_contract::{
    resolve_category_package_permission, SkillArtifactRecord, SkillArtifactStatus,
    SkillCapabilityRecord, SkillCapabilityRiskLevel, SkillCategoryRecord, SkillCategoryType,
    SkillLifecycleStatus, SkillPackageRecord, SkillVisibility,
};

use crate::SkillsBackendRequestContext;

pub(crate) fn artifact_record(
    tenant_id: u64,
    package_id: u64,
    command: CreateSkillArtifactCommand,
) -> SkillArtifactRecord {
    SkillArtifactRecord {
        id: 0,
        uuid: String::new(),
        tenant_id,
        package_id,
        version_label: command.version_label,
        artifact_ref: command.artifact_ref,
        checksum_sha256: command.checksum_sha256,
        size_bytes: command.size_bytes,
        invocation_kind: command.invocation_kind,
        entrypoint: command.entrypoint,
        input_schema: command.input_schema,
        output_schema: command.output_schema,
        config_schema: command.config_schema,
        default_config: command.default_config,
        security_profile_id: command.security_profile_id,
        status: command.status.unwrap_or(SkillArtifactStatus::Draft),
        capability_keys: command.capability_keys,
        published_at: None,
        yanked_at: None,
        created_at: String::new(),
    }
}

pub(crate) fn package_aggregate(
    context: &SkillsBackendRequestContext,
    command: CreateSkillPackageCommand,
) -> (SkillPackageRecord, SkillArtifactRecord) {
    let package_key = command.resolved_package_key();
    let package = SkillPackageRecord {
        id: 0,
        uuid: String::new(),
        tenant_id: context.tenant_id,
        organization_id: context.organization_id,
        owner_user_id: context.operator_id,
        skill_key: command.skill_key,
        package_key,
        code: command.code,
        display_name: command.display_name,
        summary: command.summary,
        description: command.description,
        categories: command.categories,
        tags: command.tags,
        status: command.status.unwrap_or(SkillLifecycleStatus::Draft),
        visibility: command.visibility.unwrap_or(SkillVisibility::Tenant),
        featured: command.featured,
        sort_weight: command.sort_weight,
        version: 0,
        created_at: String::new(),
        updated_at: String::new(),
        deleted_at: None,
    };
    let artifact = artifact_record(context.tenant_id, 0, command.initial_artifact);
    (package, artifact)
}

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
