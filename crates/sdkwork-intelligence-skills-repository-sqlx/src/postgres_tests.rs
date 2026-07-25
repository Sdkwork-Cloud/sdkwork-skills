use std::time::{SystemTime, UNIX_EPOCH};

use sdkwork_database_id::SnowflakeIdGenerator;
use sdkwork_intelligence_skills_service::{SkillsService, SkillsServiceError};
use sdkwork_skills_contract::{
    SkillArtifactRecord, SkillArtifactStatus, SkillCapabilityRecord, SkillCapabilityRiskLevel,
    SkillCategoryRecord, SkillInstallationRecord, SkillInstallationSubjectKind,
    SkillInvocationKind, SkillLifecycleStatus, SkillPackageRecord, SkillVisibility,
};
use sdkwork_utils_rust::OffsetListPageParams;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

use crate::SqlxSkillsRepository;

const POSTGRES_BASELINE: &str =
    include_str!("../../../database/ddl/baseline/postgres/0001_skills_baseline.sql");
const POSTGRES_TEST_URL_ENV: &str = "SDKWORK_SKILLS_POSTGRES_URL";

struct PostgresTestContext {
    service: SkillsService<SqlxSkillsRepository>,
    pool: PgPool,
    admin_pool: PgPool,
    schema: String,
}

impl PostgresTestContext {
    async fn from_env() -> Option<Self> {
        let database_url = match std::env::var(POSTGRES_TEST_URL_ENV) {
            Ok(value) if !value.trim().is_empty() => value,
            _ => {
                eprintln!(
                    "skip Skills PostgreSQL repository contract: {POSTGRES_TEST_URL_ENV} is not set"
                );
                return None;
            }
        };
        let admin_pool = PgPoolOptions::new()
            .max_connections(1)
            .connect(&database_url)
            .await
            .expect("connect Skills PostgreSQL test database");
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time after Unix epoch")
            .as_nanos();
        let schema = format!("skills_test_{}_{}", std::process::id(), nonce);
        sqlx::query(&format!("CREATE SCHEMA \"{schema}\""))
            .execute(&admin_pool)
            .await
            .expect("create isolated Skills PostgreSQL test schema");

        let connection_schema = schema.clone();
        let pool = PgPoolOptions::new()
            .max_connections(4)
            .after_connect(move |connection, _metadata| {
                let statement = format!("SET search_path TO \"{connection_schema}\"");
                Box::pin(async move {
                    sqlx::query(&statement).execute(connection).await?;
                    Ok(())
                })
            })
            .connect(&database_url)
            .await
            .expect("connect isolated Skills PostgreSQL test pool");
        sqlx::raw_sql(POSTGRES_BASELINE)
            .execute(&pool)
            .await
            .expect("apply Skills PostgreSQL baseline");
        sqlx::raw_sql(POSTGRES_BASELINE)
            .execute(&pool)
            .await
            .expect("reapply Skills PostgreSQL baseline idempotently");

        let generator = SnowflakeIdGenerator::new(731).expect("create test Snowflake generator");
        let service = SkillsService::new(SqlxSkillsRepository::new(pool.clone(), generator));
        Some(Self {
            service,
            pool,
            admin_pool,
            schema,
        })
    }

    async fn cleanup(self) {
        let Self {
            service,
            pool,
            admin_pool,
            schema,
        } = self;
        drop(service);
        pool.close().await;
        sqlx::query(&format!("DROP SCHEMA \"{schema}\" CASCADE"))
            .execute(&admin_pool)
            .await
            .expect("drop isolated Skills PostgreSQL test schema");
        admin_pool.close().await;
    }
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
        capability_key: "project.read".to_string(),
        display_name: "Project Read".to_string(),
        description: Some("Read authorized project content".to_string()),
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
        capability_keys: vec!["project.read".to_string()],
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

async fn verify_marketplace_and_subject_installations(
    service: &SkillsService<SqlxSkillsRepository>,
) {
    let tenant_id = 7;
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
    assert_eq!(artifacts[0].capability_keys, vec!["project.read"]);
    let artifact_id = artifacts[0].id;

    for (kind, subject_id) in [
        (SkillInstallationSubjectKind::User, 23),
        (SkillInstallationSubjectKind::Organization, 17),
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

async fn verify_optimistic_updates_and_soft_delete(service: &SkillsService<SqlxSkillsRepository>) {
    let tenant_id = 11;
    service
        .create_category(category(tenant_id))
        .await
        .expect("create category");
    let original = service
        .create_capability(capability(tenant_id))
        .await
        .expect("create capability");

    let mut first_update = original.clone();
    first_update.display_name = "Project Read Access".to_string();
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

async fn verify_installable_artifact_visibility(service: &SkillsService<SqlxSkillsRepository>) {
    let tenant_id = 13;
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

async fn verify_database_integrity(service: &SkillsService<SqlxSkillsRepository>, pool: &PgPool) {
    let tenant_id = 19;
    service
        .create_category(category(tenant_id))
        .await
        .expect("create category");
    service
        .create_capability(capability(tenant_id))
        .await
        .expect("create capability");
    let first = service
        .create_skill_package(
            package(tenant_id, "integrity-first"),
            artifact(tenant_id, 0, "1.0.0"),
        )
        .await
        .expect("create first package");
    let second = service
        .create_skill_package(
            package(tenant_id, "integrity-second"),
            artifact(tenant_id, 0, "1.0.0"),
        )
        .await
        .expect("create second package");
    let first_artifact_id = service
        .list_artifacts_page(tenant_id, first.id, page(1, 10))
        .await
        .expect("list first package artifacts")
        .0[0]
        .id;
    let second_artifact_id = service
        .list_artifacts_page(tenant_id, second.id, page(1, 10))
        .await
        .expect("list second package artifacts")
        .0[0]
        .id;
    let first_skill_id = sqlx::query_scalar::<_, i64>(
        "SELECT id FROM ai_agent_skill WHERE tenant_id=$1 AND package_id=$2",
    )
    .bind(tenant_id as i64)
    .bind(first.id as i64)
    .fetch_one(pool)
    .await
    .expect("resolve first skill id");

    let multi_owner = sqlx::query(
        "INSERT INTO ai_skill_asset (
             id, uuid, tenant_id, skill_id, package_id, asset_type, purpose, media_resource_id
         ) VALUES (91, 'multi-owner', $1, $2, $3, 'image', 'icon', 'media-91')",
    )
    .bind(tenant_id as i64)
    .bind(first_skill_id)
    .bind(first.id as i64)
    .execute(pool)
    .await;
    assert!(multi_owner.is_err(), "an asset must have exactly one owner");

    let invalid_lifecycle = sqlx::query("UPDATE ai_skill_artifact SET status='draft' WHERE id=$1")
        .bind(first_artifact_id as i64)
        .execute(pool)
        .await;
    assert!(
        invalid_lifecycle.is_err(),
        "artifact status and lifecycle timestamps must remain consistent"
    );

    let installed = service
        .install_skill(installation(
            tenant_id,
            first.id,
            first_artifact_id,
            SkillInstallationSubjectKind::User,
            23,
        ))
        .await
        .expect("install first package artifact");
    let cross_package_artifact =
        sqlx::query("UPDATE ai_skill_installation SET artifact_id=$1 WHERE id=$2")
            .bind(second_artifact_id as i64)
            .bind(installed.id as i64)
            .execute(pool)
            .await;
    assert!(
        cross_package_artifact.is_err(),
        "an installation artifact must belong to its package"
    );

    let has_redundant_skill_id: bool = sqlx::query_scalar(
        "SELECT EXISTS (
            SELECT 1 FROM information_schema.columns
            WHERE table_schema=current_schema()
              AND table_name='ai_skill_installation'
              AND column_name='skill_id'
        )",
    )
    .fetch_one(pool)
    .await
    .expect("inspect installation columns");
    assert!(!has_redundant_skill_id);
}

async fn verify_concurrent_installation(
    service: &SkillsService<SqlxSkillsRepository>,
    pool: &PgPool,
) {
    let tenant_id = 23;
    service
        .create_category(category(tenant_id))
        .await
        .expect("create category");
    service
        .create_capability(capability(tenant_id))
        .await
        .expect("create capability");
    let created = service
        .create_skill_package(
            package(tenant_id, "concurrent-install"),
            artifact(tenant_id, 0, "1.0.0"),
        )
        .await
        .expect("create package");
    let artifact_id = service
        .list_artifacts_page(tenant_id, created.id, page(1, 10))
        .await
        .expect("list package artifacts")
        .0[0]
        .id;
    let first_service = service.clone();
    let second_service = service.clone();
    let first_record = installation(
        tenant_id,
        created.id,
        artifact_id,
        SkillInstallationSubjectKind::Organization,
        17,
    );
    let second_record = first_record.clone();

    let (first_result, second_result) = tokio::join!(
        first_service.install_skill(first_record),
        second_service.install_skill(second_record)
    );
    let first_installation = first_result.expect("first concurrent installation succeeds");
    let second_installation = second_result.expect("second concurrent installation succeeds");
    assert_eq!(first_installation.id, second_installation.id);

    let installation_count = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM ai_skill_installation
         WHERE tenant_id=$1 AND organization_id=17 AND subject_kind='organization'
           AND subject_id=17 AND package_id=$2 AND deleted_at IS NULL",
    )
    .bind(tenant_id as i64)
    .bind(created.id as i64)
    .fetch_one(pool)
    .await
    .expect("count active installations");
    assert_eq!(installation_count, 1);

    let install_count = sqlx::query_scalar::<_, i64>(
        "SELECT install_count FROM ai_agent_skill WHERE tenant_id=$1 AND package_id=$2",
    )
    .bind(tenant_id as i64)
    .bind(created.id as i64)
    .fetch_one(pool)
    .await
    .expect("read aggregate install count");
    assert_eq!(install_count, 1);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn postgres_enforces_skills_repository_contracts() {
    let Some(context) = PostgresTestContext::from_env().await else {
        return;
    };

    verify_marketplace_and_subject_installations(&context.service).await;
    verify_optimistic_updates_and_soft_delete(&context.service).await;
    verify_installable_artifact_visibility(&context.service).await;
    verify_database_integrity(&context.service, &context.pool).await;
    verify_concurrent_installation(&context.service, &context.pool).await;

    context.cleanup().await;
}
