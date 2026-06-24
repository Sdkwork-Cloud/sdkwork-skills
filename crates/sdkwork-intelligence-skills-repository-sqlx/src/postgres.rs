use sdkwork_intelligence_skills_service::{SkillsResult, SkillsServiceError};
use sdkwork_skills_contract::{
    SkillCategoryRecord, SkillInvocationKind, SkillLifecycleStatus, SkillPackageRecord,
    SkillRecord, SkillVisibility, UserSkillInstallRecord,
};
use sqlx::{PgPool, Row};

use crate::json_util::{string_list_from_json, string_list_to_json, timestamp_to_rfc3339};

fn map_invocation_kind(value: &str) -> SkillsResult<SkillInvocationKind> {
    SkillInvocationKind::parse(value).ok_or_else(|| {
        SkillsServiceError::Repository(format!("invalid invocation_kind: {value}"))
    })
}

fn map_lifecycle_status(value: i16) -> SkillsResult<SkillLifecycleStatus> {
    SkillLifecycleStatus::from_db_code(value).ok_or_else(|| {
        SkillsServiceError::Repository(format!("invalid skill package status: {value}"))
    })
}

fn map_visibility(value: i16) -> SkillsResult<SkillVisibility> {
    SkillVisibility::from_db_code(value).ok_or_else(|| {
        SkillsServiceError::Repository(format!("invalid skill visibility: {value}"))
    })
}

fn row_to_skill_package(row: &sqlx::postgres::PgRow) -> SkillsResult<SkillPackageRecord> {
    Ok(SkillPackageRecord {
        id: row.try_get::<i64, _>("id").map_err(map_sqlx)? as u64,
        tenant_id: row.try_get::<i64, _>("tenant_id").map_err(map_sqlx)? as u64,
        organization_id: row.try_get::<i64, _>("organization_id").map_err(map_sqlx)? as u64,
        owner_user_id: row.try_get::<i64, _>("owner_user_id").map_err(map_sqlx)? as u64,
        skill_id: row.try_get("skill_id").map_err(map_sqlx)?,
        package_key: row.try_get("package_key").map_err(map_sqlx)?,
        code: row.try_get("code").map_err(map_sqlx)?,
        display_name: row.try_get("display_name").map_err(map_sqlx)?,
        summary: row.try_get("summary").map_err(map_sqlx)?,
        description: row.try_get("description").map_err(map_sqlx)?,
        invocation_kind: map_invocation_kind(
            row.try_get::<String, _>("invocation_kind")
                .map_err(map_sqlx)?
                .as_str(),
        )?,
        package_ref: row.try_get("package_ref").map_err(map_sqlx)?,
        entrypoint: row.try_get("entrypoint").map_err(map_sqlx)?,
        input_schema_json: row.try_get("input_schema_json").map_err(map_sqlx)?,
        output_schema_json: row.try_get("output_schema_json").map_err(map_sqlx)?,
        capability_ids: string_list_from_json(
            row.try_get::<String, _>("capability_ids_json")
                .map_err(map_sqlx)?
                .as_str(),
            "capability_ids_json",
        )?,
        categories: string_list_from_json(
            row.try_get::<String, _>("categories_json")
                .map_err(map_sqlx)?
                .as_str(),
            "categories_json",
        )?,
        tags: string_list_from_json(
            row.try_get::<String, _>("tags_json")
                .map_err(map_sqlx)?
                .as_str(),
            "tags_json",
        )?,
        security_profile_id: row.try_get("security_profile_id").map_err(map_sqlx)?,
        category_id: row
            .try_get::<Option<i64>, _>("category_id")
            .map_err(map_sqlx)?
            .map(|value| value as u64),
        status: map_lifecycle_status(row.try_get("status").map_err(map_sqlx)?)?,
        visibility: map_visibility(row.try_get("visibility").map_err(map_sqlx)?)?,
        version: row.try_get::<i64, _>("version").map_err(map_sqlx)? as u64,
        created_at: timestamp_to_rfc3339(row.try_get("created_at").map_err(map_sqlx)?),
        updated_at: timestamp_to_rfc3339(row.try_get("updated_at").map_err(map_sqlx)?),
        deleted_at: row
            .try_get::<Option<chrono::DateTime<chrono::Utc>>, _>("deleted_at")
            .map_err(map_sqlx)?
            .map(timestamp_to_rfc3339),
    })
}

fn row_to_skill(row: &sqlx::postgres::PgRow) -> SkillsResult<SkillRecord> {
    Ok(SkillRecord {
        id: row.try_get::<i64, _>("id").map_err(map_sqlx)? as u64,
        tenant_id: row.try_get::<i64, _>("tenant_id").map_err(map_sqlx)? as u64,
        organization_id: row.try_get::<i64, _>("organization_id").map_err(map_sqlx)? as u64,
        owner_user_id: row.try_get::<i64, _>("owner_user_id").map_err(map_sqlx)? as u64,
        skill_key: row.try_get("skill_key").map_err(map_sqlx)?,
        package_id: row
            .try_get::<Option<i64>, _>("package_id")
            .map_err(map_sqlx)?
            .map(|value| value as u64),
        name: row.try_get("name").map_err(map_sqlx)?,
        summary: row.try_get("summary").map_err(map_sqlx)?,
        description: row.try_get("description").map_err(map_sqlx)?,
        runtime: row.try_get("runtime").map_err(map_sqlx)?,
        entrypoint: row.try_get("entrypoint").map_err(map_sqlx)?,
        market_status: row.try_get("market_status").map_err(map_sqlx)?,
        visibility: row.try_get("visibility").map_err(map_sqlx)?,
        review_status: row.try_get("review_status").map_err(map_sqlx)?,
        category_id: row
            .try_get::<Option<i64>, _>("category_id")
            .map_err(map_sqlx)?
            .map(|value| value as u64),
        enabled: row.try_get::<i16, _>("enabled").map_err(map_sqlx)? != 0,
        featured: row.try_get::<i16, _>("featured").map_err(map_sqlx)? != 0,
        install_count: row.try_get::<i64, _>("install_count").map_err(map_sqlx)? as u64,
        tags: string_list_from_json(
            row.try_get::<String, _>("tags_json")
                .map_err(map_sqlx)?
                .as_str(),
            "tags_json",
        )?,
        capabilities: string_list_from_json(
            row.try_get::<String, _>("capabilities_json")
                .map_err(map_sqlx)?
                .as_str(),
            "capabilities_json",
        )?,
        version: row.try_get::<i64, _>("version").map_err(map_sqlx)? as u64,
        created_at: timestamp_to_rfc3339(row.try_get("created_at").map_err(map_sqlx)?),
        updated_at: timestamp_to_rfc3339(row.try_get("updated_at").map_err(map_sqlx)?),
        deleted_at: row
            .try_get::<Option<chrono::DateTime<chrono::Utc>>, _>("deleted_at")
            .map_err(map_sqlx)?
            .map(timestamp_to_rfc3339),
    })
}

fn row_to_category(row: &sqlx::postgres::PgRow) -> SkillsResult<SkillCategoryRecord> {
    Ok(SkillCategoryRecord {
        id: row.try_get::<i64, _>("id").map_err(map_sqlx)? as u64,
        tenant_id: row.try_get::<i64, _>("tenant_id").map_err(map_sqlx)? as u64,
        organization_id: row.try_get::<i64, _>("organization_id").map_err(map_sqlx)? as u64,
        category_type: row.try_get("category_type").map_err(map_sqlx)?,
        code: row.try_get("code").map_err(map_sqlx)?,
        name: row.try_get("name").map_err(map_sqlx)?,
        description: row.try_get("description").map_err(map_sqlx)?,
        parent_id: row
            .try_get::<Option<i64>, _>("parent_id")
            .map_err(map_sqlx)?
            .map(|value| value as u64),
        sort_weight: row.try_get("sort_weight").map_err(map_sqlx)?,
        visible: row.try_get::<i16, _>("visible").map_err(map_sqlx)? != 0,
        status: row.try_get("status").map_err(map_sqlx)?,
    })
}

fn row_to_user_install(row: &sqlx::postgres::PgRow) -> SkillsResult<UserSkillInstallRecord> {
    Ok(UserSkillInstallRecord {
        id: row.try_get::<i64, _>("id").map_err(map_sqlx)? as u64,
        tenant_id: row.try_get::<i64, _>("tenant_id").map_err(map_sqlx)? as u64,
        organization_id: row.try_get::<i64, _>("organization_id").map_err(map_sqlx)? as u64,
        user_id: row.try_get::<i64, _>("user_id").map_err(map_sqlx)? as u64,
        skill_id: row.try_get::<i64, _>("skill_id").map_err(map_sqlx)? as u64,
        package_id: row
            .try_get::<Option<i64>, _>("package_id")
            .map_err(map_sqlx)?
            .map(|value| value as u64),
        install_status: row.try_get("install_status").map_err(map_sqlx)?,
        enabled: row.try_get::<i16, _>("enabled").map_err(map_sqlx)? != 0,
        config_json: row.try_get("config_json").map_err(map_sqlx)?,
        installed_at: timestamp_to_rfc3339(row.try_get("installed_at").map_err(map_sqlx)?),
        updated_at: timestamp_to_rfc3339(row.try_get("updated_at").map_err(map_sqlx)?),
    })
}

fn map_sqlx(error: sqlx::Error) -> SkillsServiceError {
    SkillsServiceError::Repository(error.to_string())
}

pub async fn list_skill_packages(pool: &PgPool, tenant_id: u64) -> SkillsResult<Vec<SkillPackageRecord>> {
    let rows = sqlx::query(
        r#"
        SELECT id, tenant_id, organization_id, owner_user_id, skill_id, package_key, code,
               display_name, summary, description, invocation_kind, package_ref, entrypoint,
               input_schema_json, output_schema_json, capability_ids_json, categories_json,
               tags_json, security_profile_id, category_id, status, visibility, version,
               created_at, updated_at, deleted_at
        FROM ai_agent_skill_package
        WHERE tenant_id = $1 AND deleted_at IS NULL AND status <> 4
        ORDER BY sort_weight DESC, updated_at DESC, code ASC
        "#,
    )
    .bind(tenant_id as i64)
    .fetch_all(pool)
    .await
    .map_err(map_sqlx)?;

    rows.iter().map(row_to_skill_package).collect()
}

pub async fn get_skill_package(
    pool: &PgPool,
    tenant_id: u64,
    skill_id: &str,
) -> SkillsResult<SkillPackageRecord> {
    let row = sqlx::query(
        r#"
        SELECT id, tenant_id, organization_id, owner_user_id, skill_id, package_key, code,
               display_name, summary, description, invocation_kind, package_ref, entrypoint,
               input_schema_json, output_schema_json, capability_ids_json, categories_json,
               tags_json, security_profile_id, category_id, status, visibility, version,
               created_at, updated_at, deleted_at
        FROM ai_agent_skill_package
        WHERE tenant_id = $1 AND skill_id = $2 AND deleted_at IS NULL
        LIMIT 1
        "#,
    )
    .bind(tenant_id as i64)
    .bind(skill_id)
    .fetch_optional(pool)
    .await
    .map_err(map_sqlx)?;

    row.as_ref()
        .map(row_to_skill_package)
        .transpose()?
        .ok_or_else(|| SkillsServiceError::NotFound(skill_id.to_string()))
}

pub async fn upsert_skill_package(
    pool: &PgPool,
    record: SkillPackageRecord,
) -> SkillsResult<SkillPackageRecord> {
    let uuid = format!("skill_package_{}_{}", record.tenant_id, record.skill_id);
    let capability_ids_json = string_list_to_json(&record.capability_ids, "capability_ids")?;
    let categories_json = string_list_to_json(&record.categories, "categories")?;
    let tags_json = string_list_to_json(&record.tags, "tags")?;

    let row = sqlx::query(
        r#"
        INSERT INTO ai_agent_skill_package (
            uuid, tenant_id, organization_id, owner_user_id, skill_id, package_key, code,
            display_name, summary, description, invocation_kind, package_ref, entrypoint,
            input_schema_json, output_schema_json, capability_ids_json, categories_json,
            tags_json, security_profile_id, category_id, status, visibility, version
        ) VALUES (
            $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18,
            $19, $20, $21, $22, $23
        )
        ON CONFLICT (tenant_id, skill_id) DO UPDATE SET
            organization_id = EXCLUDED.organization_id,
            owner_user_id = EXCLUDED.owner_user_id,
            package_key = EXCLUDED.package_key,
            code = EXCLUDED.code,
            display_name = EXCLUDED.display_name,
            summary = EXCLUDED.summary,
            description = EXCLUDED.description,
            invocation_kind = EXCLUDED.invocation_kind,
            package_ref = EXCLUDED.package_ref,
            entrypoint = EXCLUDED.entrypoint,
            input_schema_json = EXCLUDED.input_schema_json,
            output_schema_json = EXCLUDED.output_schema_json,
            capability_ids_json = EXCLUDED.capability_ids_json,
            categories_json = EXCLUDED.categories_json,
            tags_json = EXCLUDED.tags_json,
            security_profile_id = EXCLUDED.security_profile_id,
            category_id = EXCLUDED.category_id,
            status = EXCLUDED.status,
            visibility = EXCLUDED.visibility,
            version = ai_agent_skill_package.version + 1,
            updated_at = CURRENT_TIMESTAMP,
            deleted_at = NULL
        RETURNING id, tenant_id, organization_id, owner_user_id, skill_id, package_key, code,
                  display_name, summary, description, invocation_kind, package_ref, entrypoint,
                  input_schema_json, output_schema_json, capability_ids_json, categories_json,
                  tags_json, security_profile_id, category_id, status, visibility, version,
                  created_at, updated_at, deleted_at
        "#,
    )
    .bind(uuid)
    .bind(record.tenant_id as i64)
    .bind(record.organization_id as i64)
    .bind(record.owner_user_id as i64)
    .bind(&record.skill_id)
    .bind(&record.package_key)
    .bind(&record.code)
    .bind(&record.display_name)
    .bind(&record.summary)
    .bind(&record.description)
    .bind(record.invocation_kind.as_str())
    .bind(&record.package_ref)
    .bind(&record.entrypoint)
    .bind(&record.input_schema_json)
    .bind(&record.output_schema_json)
    .bind(capability_ids_json)
    .bind(categories_json)
    .bind(tags_json)
    .bind(&record.security_profile_id)
    .bind(record.category_id.map(|value| value as i64))
    .bind(record.status.as_db_code())
    .bind(record.visibility.as_db_code())
    .bind(record.version as i64)
    .fetch_one(pool)
    .await
    .map_err(map_sqlx)?;

    row_to_skill_package(&row)
}

pub async fn list_skills(pool: &PgPool, tenant_id: u64) -> SkillsResult<Vec<SkillRecord>> {
    let rows = sqlx::query(
        r#"
        SELECT id, tenant_id, organization_id, owner_user_id, skill_key, package_id, name,
               summary, description, runtime, entrypoint, market_status, visibility,
               review_status, category_id, enabled, featured, install_count, tags_json,
               capabilities_json, version, created_at, updated_at, deleted_at
        FROM ai_agent_skill
        WHERE tenant_id = $1 AND deleted_at IS NULL AND enabled = 1
        ORDER BY featured DESC, recommend_weight DESC, updated_at DESC, skill_key ASC
        "#,
    )
    .bind(tenant_id as i64)
    .fetch_all(pool)
    .await
    .map_err(map_sqlx)?;

    rows.iter().map(row_to_skill).collect()
}

pub async fn get_skill(
    pool: &PgPool,
    tenant_id: u64,
    skill_key: &str,
) -> SkillsResult<SkillRecord> {
    let row = sqlx::query(
        r#"
        SELECT id, tenant_id, organization_id, owner_user_id, skill_key, package_id, name,
               summary, description, runtime, entrypoint, market_status, visibility,
               review_status, category_id, enabled, featured, install_count, tags_json,
               capabilities_json, version, created_at, updated_at, deleted_at
        FROM ai_agent_skill
        WHERE tenant_id = $1 AND skill_key = $2 AND deleted_at IS NULL
        LIMIT 1
        "#,
    )
    .bind(tenant_id as i64)
    .bind(skill_key)
    .fetch_optional(pool)
    .await
    .map_err(map_sqlx)?;

    row.as_ref()
        .map(row_to_skill)
        .transpose()?
        .ok_or_else(|| SkillsServiceError::NotFound(skill_key.to_string()))
}

pub async fn list_categories(
    pool: &PgPool,
    tenant_id: u64,
    category_type: &str,
) -> SkillsResult<Vec<SkillCategoryRecord>> {
    let rows = sqlx::query(
        r#"
        SELECT id, tenant_id, organization_id, category_type, code, name, description,
               parent_id, sort_weight, visible, status
        FROM c_category
        WHERE category_type = $1 AND tenant_id IN (0, $2) AND deleted_at IS NULL AND status = 1
        ORDER BY sort_weight ASC, code ASC
        "#,
    )
    .bind(category_type)
    .bind(tenant_id as i64)
    .fetch_all(pool)
    .await
    .map_err(map_sqlx)?;

    rows.iter().map(row_to_category).collect()
}

pub async fn upsert_category(
    pool: &PgPool,
    record: SkillCategoryRecord,
) -> SkillsResult<SkillCategoryRecord> {
    let uuid = format!(
        "skill_category_{}_{}_{}",
        record.tenant_id, record.category_type, record.code
    );
    let visible = if record.visible { 1_i16 } else { 0_i16 };

    let row = if record.id == 0 {
        sqlx::query(
            r#"
            INSERT INTO c_category (
                uuid, tenant_id, organization_id, category_type, code, name, description,
                parent_id, sort_weight, visible, status
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            RETURNING id, tenant_id, organization_id, category_type, code, name, description,
                      parent_id, sort_weight, visible, status
            "#,
        )
        .bind(uuid)
        .bind(record.tenant_id as i64)
        .bind(record.organization_id as i64)
        .bind(&record.category_type)
        .bind(&record.code)
        .bind(&record.name)
        .bind(&record.description)
        .bind(record.parent_id.map(|value| value as i64))
        .bind(record.sort_weight)
        .bind(visible)
        .bind(record.status)
        .fetch_one(pool)
        .await
        .map_err(map_sqlx)?
    } else {
        sqlx::query(
            r#"
            UPDATE c_category SET
                name = $3,
                description = $4,
                parent_id = $5,
                sort_weight = $6,
                visible = $7,
                status = $8,
                updated_at = CURRENT_TIMESTAMP
            WHERE id = $1 AND tenant_id = $2
            RETURNING id, tenant_id, organization_id, category_type, code, name, description,
                      parent_id, sort_weight, visible, status
            "#,
        )
        .bind(record.id as i64)
        .bind(record.tenant_id as i64)
        .bind(&record.name)
        .bind(&record.description)
        .bind(record.parent_id.map(|value| value as i64))
        .bind(record.sort_weight)
        .bind(visible)
        .bind(record.status)
        .fetch_one(pool)
        .await
        .map_err(map_sqlx)?
    };

    row_to_category(&row)
}

pub async fn install_skill_for_user(
    pool: &PgPool,
    record: UserSkillInstallRecord,
) -> SkillsResult<UserSkillInstallRecord> {
    let uuid = format!(
        "user_skill_{}_{}_{}",
        record.tenant_id, record.user_id, record.skill_id
    );
    let enabled = if record.enabled { 1_i16 } else { 0_i16 };

    let row = sqlx::query(
        r#"
        INSERT INTO ai_user_agent_skill (
            uuid, tenant_id, organization_id, user_id, skill_id, package_id,
            install_status, enabled, config_json
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        ON CONFLICT (tenant_id, user_id, skill_id) DO UPDATE SET
            package_id = EXCLUDED.package_id,
            install_status = EXCLUDED.install_status,
            enabled = EXCLUDED.enabled,
            config_json = EXCLUDED.config_json,
            updated_at = CURRENT_TIMESTAMP,
            deleted_at = NULL
        RETURNING id, tenant_id, organization_id, user_id, skill_id, package_id,
                  install_status, enabled, config_json, installed_at, updated_at
        "#,
    )
    .bind(uuid)
    .bind(record.tenant_id as i64)
    .bind(record.organization_id as i64)
    .bind(record.user_id as i64)
    .bind(record.skill_id as i64)
    .bind(record.package_id.map(|value| value as i64))
    .bind(&record.install_status)
    .bind(enabled)
    .bind(&record.config_json)
    .fetch_one(pool)
    .await
    .map_err(map_sqlx)?;

    row_to_user_install(&row)
}

pub async fn delete_skill_package(
    pool: &PgPool,
    tenant_id: u64,
    skill_id: &str,
) -> SkillsResult<SkillPackageRecord> {
    let row = sqlx::query(
        r#"
        UPDATE ai_agent_skill_package
        SET status = 4,
            deleted_at = CURRENT_TIMESTAMP,
            updated_at = CURRENT_TIMESTAMP
        WHERE tenant_id = $1 AND skill_id = $2 AND deleted_at IS NULL
        RETURNING id, tenant_id, organization_id, owner_user_id, skill_id, package_key, code,
                  display_name, summary, description, invocation_kind, package_ref, entrypoint,
                  input_schema_json, output_schema_json, capability_ids_json, categories_json,
                  tags_json, security_profile_id, category_id, status, visibility, version,
                  created_at, updated_at, deleted_at
        "#,
    )
    .bind(tenant_id as i64)
    .bind(skill_id)
    .fetch_optional(pool)
    .await
    .map_err(map_sqlx)?;

    let package = row
        .as_ref()
        .map(row_to_skill_package)
        .transpose()?
        .ok_or_else(|| SkillsServiceError::NotFound(skill_id.to_string()))?;

    sqlx::query(
        r#"
        UPDATE ai_agent_skill
        SET enabled = 0,
            market_status = 'removed',
            deleted_at = CURRENT_TIMESTAMP,
            updated_at = CURRENT_TIMESTAMP
        WHERE tenant_id = $1 AND skill_key = $2 AND deleted_at IS NULL
        "#,
    )
    .bind(tenant_id as i64)
    .bind(skill_id)
    .execute(pool)
    .await
    .map_err(map_sqlx)?;

    Ok(package)
}

fn package_visibility_to_market(value: SkillVisibility) -> &'static str {
    match value {
        SkillVisibility::Private => "private",
        SkillVisibility::Tenant => "tenant",
        SkillVisibility::Organization => "organization",
        SkillVisibility::Public => "public",
    }
}

fn package_status_to_market(value: SkillLifecycleStatus) -> &'static str {
    match value {
        SkillLifecycleStatus::Draft => "draft",
        SkillLifecycleStatus::Active => "published",
        SkillLifecycleStatus::Disabled => "disabled",
        SkillLifecycleStatus::Archived => "archived",
        SkillLifecycleStatus::Deleted => "removed",
    }
}

pub async fn sync_skill_from_package(
    pool: &PgPool,
    package: &SkillPackageRecord,
) -> SkillsResult<SkillRecord> {
    let uuid = format!("agent_skill_{}_{}", package.tenant_id, package.skill_id);
    let tags_json = string_list_to_json(&package.tags, "tags")?;
    let capabilities_json = string_list_to_json(&package.capability_ids, "capabilities")?;
    let enabled = if package.status == SkillLifecycleStatus::Active {
        1_i16
    } else {
        0_i16
    };
    let market_status = package_status_to_market(package.status);
    let visibility = package_visibility_to_market(package.visibility);

    let row = sqlx::query(
        r#"
        INSERT INTO ai_agent_skill (
            uuid, tenant_id, organization_id, owner_user_id, skill_key, package_id, name,
            summary, description, runtime, entrypoint, market_status, visibility, review_status,
            category_id, enabled, tags_json, capabilities_json
        ) VALUES (
            $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, 'approved', $14, $15, $16, $17
        )
        ON CONFLICT (tenant_id, organization_id, skill_key) DO UPDATE SET
            package_id = EXCLUDED.package_id,
            name = EXCLUDED.name,
            summary = EXCLUDED.summary,
            description = EXCLUDED.description,
            runtime = EXCLUDED.runtime,
            entrypoint = EXCLUDED.entrypoint,
            market_status = EXCLUDED.market_status,
            visibility = EXCLUDED.visibility,
            category_id = EXCLUDED.category_id,
            enabled = EXCLUDED.enabled,
            tags_json = EXCLUDED.tags_json,
            capabilities_json = EXCLUDED.capabilities_json,
            version = ai_agent_skill.version + 1,
            updated_at = CURRENT_TIMESTAMP,
            deleted_at = NULL
        RETURNING id, tenant_id, organization_id, owner_user_id, skill_key, package_id, name,
                  summary, description, runtime, entrypoint, market_status, visibility,
                  review_status, category_id, enabled, featured, install_count, tags_json,
                  capabilities_json, version, created_at, updated_at, deleted_at
        "#,
    )
    .bind(uuid)
    .bind(package.tenant_id as i64)
    .bind(package.organization_id as i64)
    .bind(package.owner_user_id as i64)
    .bind(&package.skill_id)
    .bind(package.id as i64)
    .bind(&package.display_name)
    .bind(&package.summary)
    .bind(&package.description)
    .bind(package.invocation_kind.as_str())
    .bind(&package.entrypoint)
    .bind(market_status)
    .bind(visibility)
    .bind(package.category_id.map(|value| value as i64))
    .bind(enabled)
    .bind(tags_json)
    .bind(capabilities_json)
    .fetch_one(pool)
    .await
    .map_err(map_sqlx)?;

    row_to_skill(&row)
}
