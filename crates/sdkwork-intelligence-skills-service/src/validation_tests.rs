use sdkwork_skills_contract::{
    SkillArtifactRecord, SkillArtifactStatus, SkillInvocationKind, SkillLifecycleStatus,
    SkillPackageRecord, SkillVisibility,
};

use super::validation::{
    validate_artifact_record, validate_installation_subject, validate_skill_package_record,
};

fn artifact(artifact_ref: &str, checksum: &str) -> SkillArtifactRecord {
    SkillArtifactRecord {
        id: 0,
        uuid: String::new(),
        tenant_id: 1,
        package_id: 0,
        version_label: "1.0.0".to_string(),
        artifact_ref: artifact_ref.to_string(),
        checksum_sha256: checksum.to_string(),
        size_bytes: Some(42),
        invocation_kind: SkillInvocationKind::LocalWorkflow,
        entrypoint: "run".to_string(),
        input_schema: serde_json::json!({}),
        output_schema: serde_json::json!({}),
        config_schema: serde_json::json!({}),
        default_config: serde_json::json!({}),
        security_profile_id: None,
        status: SkillArtifactStatus::Published,
        capability_keys: Vec::new(),
        published_at: None,
        yanked_at: None,
        created_at: String::new(),
    }
}

fn package() -> SkillPackageRecord {
    SkillPackageRecord {
        id: 0,
        uuid: String::new(),
        tenant_id: 1,
        organization_id: 0,
        owner_user_id: 1,
        skill_key: "skill.demo.run".to_string(),
        package_key: "demo-run".to_string(),
        code: "demo-run".to_string(),
        display_name: "Demo Run".to_string(),
        summary: None,
        description: None,
        categories: Vec::new(),
        tags: Vec::new(),
        status: SkillLifecycleStatus::Active,
        visibility: SkillVisibility::Tenant,
        featured: false,
        sort_weight: 0,
        version: 1,
        created_at: String::new(),
        updated_at: String::new(),
        deleted_at: None,
    }
}

#[test]
fn accepts_drive_artifact_with_sha256() {
    let record = artifact(
        "drive://spaces/skills-dev/nodes/demo-package",
        "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
    );
    validate_artifact_record(&record).expect("valid artifact");
    validate_skill_package_record(&package()).expect("valid package");
}

#[test]
fn rejects_non_drive_artifact_and_invalid_checksum() {
    let record = artifact("https://example.com/demo.zip", "bad");
    assert!(validate_artifact_record(&record).is_err());
}

#[test]
fn rejects_artifact_created_directly_in_yanked_state() {
    let mut record = artifact(
        "drive://spaces/skills-dev/nodes/yanked-package",
        "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
    );
    record.status = SkillArtifactStatus::Yanked;

    assert!(validate_artifact_record(&record).is_err());
}

#[test]
fn installation_subjects_use_iam_and_agents_authorities() {
    for subject_kind in ["user", "organization", "project", "agent"] {
        validate_installation_subject(subject_kind, 1).expect("canonical installation subject");
    }
    assert!(validate_installation_subject("workspace", 1).is_err());
    assert!(validate_installation_subject("project", 0).is_err());
}
