use sdkwork_database_id::SnowflakeIdGenerator;
use sdkwork_intelligence_skills_service::{SkillsService, SkillsServiceError};
use sdkwork_skills_contract::{
    SkillArtifactRecord, SkillArtifactStatus, SkillCapabilityRecord, SkillCapabilityRiskLevel,
    SkillCategoryRecord, SkillInstallationRecord, SkillInstallationSubjectKind,
    SkillInvocationKind, SkillLifecycleStatus, SkillPackageRecord, SkillVisibility,
};
use sdkwork_utils_rust::OffsetListPageParams;
use sqlx::sqlite::SqlitePoolOptions;

use crate::SqlxSkillsRepository;

const SQLITE_BASELINE: &str =
    include_str!("../../../database/ddl/baseline/sqlite/0001_skills_baseline.sql");

async fn service() -> SkillsService<SqlxSkillsRepository> {
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .expect("connect in-memory SQLite");
    sqlx::raw_sql(SQLITE_BASELINE)
        .execute(&pool)
        .await
        .expect("apply Skills SQLite baseline");
    let generator = SnowflakeIdGenerator::new(731).expect("create test Snowflake generator");
    SkillsService::new(SqlxSkillsRepository::from_sqlite(pool, generator))
}

fn page(page: i64, page_size: i64) -> OffsetListPageParams {
    OffsetListPageParams {
        page,
        page_size,
        offset: (page - 1) * page_size,
    }
}

fn category(tenant_id: u64) -> SkillCategoryRecord {
    SkillCategoryRecord {
        id: 0,
        uuid: String::new(),
        tenant_id,
        organization_id: 0,
        category_type: "skill_market".to_string(),
        code: "coding".to_string(),
        name: "Coding".to_string(),
        description: Some("Coding automation skills".to_string()),
        parent_id: None,
        sort_weight: 10,
        permission_code: "skills.market.read".to_string(),
        visible: true,
        status: 1,
        version: 0,
        created_at: String::new(),
        updated_at: String::new(),
    }
}

fn capability(tenant_id: u64) -> SkillCapabilityRecord {
    SkillCapabilityRecord {
        id: 0,
        uuid: String::new(),
        tenant_id,
        organization_id: 0,
        capability_key: "workspace.read".to_string(),
        display_name: "Workspace Read".to_string(),
        description: Some("Read authorized workspace content".to_string()),
        risk_level: SkillCapabilityRiskLevel::Sensitive,
        status: 1,
        version: 0,
        created_at: String::new(),
        updated_at: String::new(),
    }
}

fn artifact(tenant_id: u64, package_id: u64, version_label: &str) -> SkillArtifactRecord {
    SkillArtifactRecord {
        id: 0,
        uuid: String::new(),
        tenant_id,
        package_id,
        version_label: version_label.to_string(),
        artifact_ref: format!("drive://spaces/skills/nodes/artifact-{version_label}"),
        checksum_sha256: "a".repeat(64),
        size_bytes: Some(4096),
        invocation_kind: SkillInvocationKind::LocalWorkflow,
        entrypoint: "workflows/main.yaml".to_string(),
        input_schema: serde_json::json!({}),
        output_schema: serde_json::json!({}),
        config_schema: serde_json::json!({}),
        default_config: serde_json::json!({}),
        security_profile_id: Some("skills.standard".to_string()),
        status: SkillArtifactStatus::Published,
        capability_keys: vec!["workspace.read".to_string()],
        published_at: None,
        yanked_at: None,
        created_at: String::new(),
    }
}

fn package(tenant_id: u64, suffix: &str) -> SkillPackageRecord {
    SkillPackageRecord {
        id: 0,
        uuid: String::new(),
        tenant_id,
        organization_id: 17,
        owner_user_id: 23,
        skill_key: format!("skill.{suffix}"),
        package_key: format!("com.sdkwork.skills.{suffix}"),
        code: suffix.to_string(),
        display_name: format!("{suffix} skill"),
        summary: Some("Production skill".to_string()),
        description: Some("A normalized Skills marketplace package".to_string()),
        categories: vec!["coding".to_string()],
        tags: vec!["automation".to_string(), "coding".to_string()],
        status: SkillLifecycleStatus::Active,
        visibility: SkillVisibility::Public,
        featured: true,
        sort_weight: 100,
        version: 0,
        created_at: String::new(),
        updated_at: String::new(),
        deleted_at: None,
    }
}

fn installation(
    tenant_id: u64,
    package_id: u64,
    artifact_id: u64,
    subject_kind: SkillInstallationSubjectKind,
    subject_id: u64,
) -> SkillInstallationRecord {
    SkillInstallationRecord {
        id: 0,
        uuid: String::new(),
        tenant_id,
        organization_id: 17,
        subject_kind,
        subject_id,
        skill_id: 0,
        package_id,
        artifact_id,
        installed_by_user_id: 23,
        install_status: "installed".to_string(),
        enabled: true,
        config: serde_json::json!({"mode": "safe"}),
        version: 0,
        installed_at: String::new(),
        updated_at: String::new(),
    }
}

#[tokio::test]
async fn sqlite_supports_normalized_marketplace_and_subject_installations() {
    let tenant_id = 7;
    let service = service().await;

    let category = service
        .create_category(category(tenant_id))
        .await
        .expect("create category");
    assert_ne!(category.id, 0);

    let capability = service
        .create_capability(capability(tenant_id))
        .await
        .expect("create capability");
    assert_eq!(capability.version, 1);

    let first = service
        .create_skill_package(
            package(tenant_id, "code-review"),
            artifact(tenant_id, 0, "1.0.0"),
        )
        .await
        .expect("create first package aggregate");
    let second = service
        .create_skill_package(
            package(tenant_id, "test-authoring"),
            artifact(tenant_id, 0, "1.0.0"),
        )
        .await
        .expect("create second package aggregate");
    assert_ne!(first.id, second.id);
    assert_eq!(first.categories, vec!["coding"]);

    let (packages, package_total) = service
        .list_skill_packages_page(tenant_id, page(1, 1), None)
        .await
        .expect("list package page");
    assert_eq!(packages.len(), 1);
    assert_eq!(package_total, 2);

    let (skills, skill_total) = service
        .list_hub_skills_page(tenant_id, 17, 23, page(1, 10), Some("skill"))
        .await
        .expect("list marketplace skills");
    assert_eq!(skills.len(), 2);
    assert_eq!(skill_total, 2);

    let (artifacts, artifact_total) = service
        .list_artifacts_page(tenant_id, first.id, page(1, 10))
        .await
        .expect("list immutable artifacts");
    assert_eq!(artifact_total, 1);
    assert_eq!(artifacts[0].capability_keys, vec!["workspace.read"]);
    let artifact_id = artifacts[0].id;

    for (kind, subject_id) in [
        (SkillInstallationSubjectKind::User, 23),
        (SkillInstallationSubjectKind::Workspace, 41),
        (SkillInstallationSubjectKind::Project, 59),
        (SkillInstallationSubjectKind::Agent, 61),
    ] {
        let installed = service
            .install_skill(installation(
                tenant_id,
                first.id,
                artifact_id,
                kind,
                subject_id,
            ))
            .await
            .expect("install exact published artifact");
        assert_eq!(installed.subject_kind, kind);
        assert_eq!(installed.artifact_id, artifact_id);

        let (items, total) = service
            .list_installations_page(tenant_id, 17, kind.as_str(), subject_id, page(1, 10))
            .await
            .expect("list subject installation");
        assert_eq!(total, 1);
        assert_eq!(items, vec![installed]);
    }

    let reinstalled = service
        .install_skill(installation(
            tenant_id,
            first.id,
            artifact_id,
            SkillInstallationSubjectKind::User,
            23,
        ))
        .await
        .expect("idempotently update existing subject installation");
    assert_eq!(reinstalled.version, 2);

    let skill = service
        .get_skill(tenant_id, 17, 23, "skill.code-review")
        .await
        .expect("retrieve marketplace skill");
    assert_eq!(skill.install_count, 4);

    let duplicate = service
        .create_artifact(artifact(tenant_id, first.id, "1.0.0"))
        .await
        .expect_err("artifact versions are immutable and unique per package");
    assert!(matches!(duplicate, SkillsServiceError::Conflict(_)));

    let wrong_tenant = service
        .get_skill_package(tenant_id + 1, first.id)
        .await
        .expect_err("tenant isolation must hide another tenant package");
    assert!(matches!(wrong_tenant, SkillsServiceError::NotFound(_)));
}

#[tokio::test]
async fn sqlite_enforces_optimistic_updates_and_soft_delete_cascade() {
    let tenant_id = 11;
    let service = service().await;
    service
        .create_category(category(tenant_id))
        .await
        .expect("create category");
    let original = service
        .create_capability(capability(tenant_id))
        .await
        .expect("create capability");

    let mut first_update = original.clone();
    first_update.display_name = "Workspace Read Access".to_string();
    let updated = service
        .update_capability(first_update)
        .await
        .expect("update current capability version");
    assert_eq!(updated.version, original.version + 1);

    let mut stale_update = original;
    stale_update.display_name = "Stale Update".to_string();
    let conflict = service
        .update_capability(stale_update)
        .await
        .expect_err("reject stale capability update");
    assert!(matches!(conflict, SkillsServiceError::Conflict(_)));

    let package = service
        .create_skill_package(
            package(tenant_id, "cleanup"),
            artifact(tenant_id, 0, "1.0.0"),
        )
        .await
        .expect("create package");
    let (artifacts, _) = service
        .list_artifacts_page(tenant_id, package.id, page(1, 10))
        .await
        .expect("list package artifact");
    service
        .install_skill(installation(
            tenant_id,
            package.id,
            artifacts[0].id,
            SkillInstallationSubjectKind::Agent,
            73,
        ))
        .await
        .expect("install before package deletion");

    service
        .delete_skill_package(tenant_id, package.id)
        .await
        .expect("soft delete package aggregate");
    let deleted = service
        .get_skill_package(tenant_id, package.id)
        .await
        .expect_err("soft-deleted package is not retrievable");
    assert!(matches!(deleted, SkillsServiceError::NotFound(_)));
    let (installations, total) = service
        .list_installations_page(
            tenant_id,
            17,
            SkillInstallationSubjectKind::Agent.as_str(),
            73,
            page(1, 10),
        )
        .await
        .expect("list installations after aggregate deletion");
    assert!(installations.is_empty());
    assert_eq!(total, 0);
}

#[tokio::test]
async fn sqlite_lists_only_installable_artifacts_visible_to_the_current_subject() {
    let tenant_id = 13;
    let service = service().await;
    service
        .create_category(category(tenant_id))
        .await
        .expect("create category");
    service
        .create_capability(capability(tenant_id))
        .await
        .expect("create capability");

    let mut private_package = package(tenant_id, "private-review");
    private_package.visibility = SkillVisibility::Private;
    let created = service
        .create_skill_package(private_package, artifact(tenant_id, 0, "1.0.0"))
        .await
        .expect("create private active package");

    let mut draft = artifact(tenant_id, created.id, "1.1.0");
    draft.status = SkillArtifactStatus::Draft;
    service
        .create_artifact(draft)
        .await
        .expect("create unpublished artifact");

    let (hidden, hidden_total) = service
        .list_installable_artifacts_page(tenant_id, 17, 24, created.id, page(1, 10))
        .await
        .expect("hide private package from another user");
    assert!(hidden.is_empty());
    assert_eq!(hidden_total, 0);

    let (visible, visible_total) = service
        .list_installable_artifacts_page(tenant_id, 17, 23, created.id, page(1, 10))
        .await
        .expect("list published artifact for package owner");
    assert_eq!(visible_total, 1);
    assert_eq!(visible.len(), 1);
    assert_eq!(visible[0].status, SkillArtifactStatus::Published);

    let mut organization_package = created;
    organization_package.visibility = SkillVisibility::Organization;
    let organization_package = service
        .update_skill_package(organization_package)
        .await
        .expect("publish package to its organization");
    let (_, same_organization_total) = service
        .list_installable_artifacts_page(tenant_id, 17, 24, organization_package.id, page(1, 10))
        .await
        .expect("list artifact for same organization");
    assert_eq!(same_organization_total, 1);
    let (_, other_organization_total) = service
        .list_installable_artifacts_page(tenant_id, 18, 24, organization_package.id, page(1, 10))
        .await
        .expect("hide artifact from another organization");
    assert_eq!(other_organization_total, 0);

    let mut disabled_package = organization_package;
    disabled_package.status = SkillLifecycleStatus::Disabled;
    let disabled_package = service
        .update_skill_package(disabled_package)
        .await
        .expect("disable package");
    let (_, disabled_total) = service
        .list_installable_artifacts_page(tenant_id, 17, 23, disabled_package.id, page(1, 10))
        .await
        .expect("hide artifacts from disabled package");
    assert_eq!(disabled_total, 0);
}
