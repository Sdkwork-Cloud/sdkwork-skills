use sdkwork_skills_contract::{
    SkillArtifactRecord, SkillArtifactStatus, SkillLifecycleStatus, SkillPackageRecord,
    SkillVisibility,
};

use crate::commands::{CreateSkillArtifactCommand, CreateSkillPackageCommand};

pub fn artifact_record(
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

/// Backend-api package aggregate: admin-created packages start as drafts and
/// are published through the marketplace management flow.
pub fn package_aggregate(
    tenant_id: u64,
    organization_id: u64,
    owner_user_id: u64,
    command: CreateSkillPackageCommand,
) -> (SkillPackageRecord, SkillArtifactRecord) {
    let package_key = command.resolved_package_key();
    let package = SkillPackageRecord {
        id: 0,
        uuid: String::new(),
        tenant_id,
        organization_id,
        owner_user_id,
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
    let artifact = artifact_record(tenant_id, 0, command.initial_artifact);
    (package, artifact)
}

/// App-api self-service package aggregate: user-created packages are active
/// within the owning tenant and ship a published initial artifact so they are
/// immediately installable. Marketplace publication scope (visibility,
/// featured, categories) remains admin-managed through the backend surface.
/// The command-level status is intentionally ignored: self-service creation
/// always produces an active tenant package.
pub fn self_service_package_aggregate(
    tenant_id: u64,
    organization_id: u64,
    owner_user_id: u64,
    command: CreateSkillPackageCommand,
) -> (SkillPackageRecord, SkillArtifactRecord) {
    let package_key = command.resolved_package_key();
    let package = SkillPackageRecord {
        id: 0,
        uuid: String::new(),
        tenant_id,
        organization_id,
        owner_user_id,
        skill_key: command.skill_key,
        package_key,
        code: command.code,
        display_name: command.display_name,
        summary: command.summary,
        description: command.description,
        categories: command.categories,
        tags: command.tags,
        status: SkillLifecycleStatus::Active,
        visibility: SkillVisibility::Tenant,
        featured: false,
        sort_weight: 0,
        version: 0,
        created_at: String::new(),
        updated_at: String::new(),
        deleted_at: None,
    };
    let mut artifact = artifact_record(tenant_id, 0, command.initial_artifact);
    // An active package requires a published initial artifact (service rule),
    // so the self-service path forces publication of the uploaded archive.
    artifact.status = SkillArtifactStatus::Published;
    (package, artifact)
}
