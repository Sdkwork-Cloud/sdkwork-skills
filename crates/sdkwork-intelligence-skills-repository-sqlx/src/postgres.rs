use sdkwork_database_id::SnowflakeIdGenerator;
use sdkwork_intelligence_skills_service::{SkillsResult, SkillsServiceError};
use sdkwork_skills_contract::{
    SkillArtifactRecord, SkillCapabilityRecord, SkillCategoryRecord, SkillInstallationRecord,
    SkillLifecycleStatus, SkillPackageRecord, SkillRecord,
};
use sdkwork_utils_rust::{OffsetListPageParams, LIST_TOTAL_SQL_COLUMN};
use sqlx::{PgPool, Postgres, Row, Transaction};

use crate::json_util::{
    json_value_from_text, json_value_to_text, string_list_from_json, string_list_to_json,
    timestamp_to_rfc3339,
};
use crate::support::{
    artifact_status, capability_risk, int64_to_uint64, invocation, lifecycle, map_sqlx, new_uuid,
    next_id, optional_int64_to_uint64, optional_uint64_to_int64, search_pattern, subject_kind,
    uint64_to_int64, visibility,
};

fn row_to_package(row: &sqlx::postgres::PgRow) -> SkillsResult<SkillPackageRecord> {
    Ok(SkillPackageRecord {
        id: int64_to_uint64(row.try_get("id").map_err(map_sqlx)?, "id")?,
        uuid: row.try_get("uuid").map_err(map_sqlx)?,
        tenant_id: int64_to_uint64(row.try_get("tenant_id").map_err(map_sqlx)?, "tenant_id")?,
        organization_id: int64_to_uint64(
            row.try_get("organization_id").map_err(map_sqlx)?,
            "organization_id",
        )?,
        owner_user_id: int64_to_uint64(
            row.try_get("owner_user_id").map_err(map_sqlx)?,
            "owner_user_id",
        )?,
        skill_key: row.try_get("skill_key").map_err(map_sqlx)?,
        package_key: row.try_get("package_key").map_err(map_sqlx)?,
        code: row.try_get("code").map_err(map_sqlx)?,
        display_name: row.try_get("display_name").map_err(map_sqlx)?,
        summary: row.try_get("summary").map_err(map_sqlx)?,
        description: row.try_get("description").map_err(map_sqlx)?,
        categories: string_list_from_json(
            &row.try_get::<String, _>("category_codes_json")
                .map_err(map_sqlx)?,
            "category_codes_json",
        )?,
        tags: string_list_from_json(
            &row.try_get::<String, _>("tags_json").map_err(map_sqlx)?,
            "tags_json",
        )?,
        status: lifecycle(row.try_get("status").map_err(map_sqlx)?)?,
        visibility: visibility(row.try_get("visibility").map_err(map_sqlx)?)?,
        featured: row.try_get::<i16, _>("featured").map_err(map_sqlx)? != 0,
        sort_weight: row.try_get("sort_weight").map_err(map_sqlx)?,
        version: int64_to_uint64(row.try_get("version").map_err(map_sqlx)?, "version")?,
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
        id: int64_to_uint64(row.try_get("id").map_err(map_sqlx)?, "id")?,
        uuid: row.try_get("uuid").map_err(map_sqlx)?,
        tenant_id: int64_to_uint64(row.try_get("tenant_id").map_err(map_sqlx)?, "tenant_id")?,
        organization_id: int64_to_uint64(
            row.try_get("organization_id").map_err(map_sqlx)?,
            "organization_id",
        )?,
        skill_key: row.try_get("skill_key").map_err(map_sqlx)?,
        package_id: int64_to_uint64(row.try_get("package_id").map_err(map_sqlx)?, "package_id")?,
        name: row.try_get("display_name").map_err(map_sqlx)?,
        summary: row.try_get("summary").map_err(map_sqlx)?,
        description: row.try_get("description").map_err(map_sqlx)?,
        market_status: row.try_get("market_status").map_err(map_sqlx)?,
        visibility: visibility(row.try_get("visibility").map_err(map_sqlx)?)?,
        review_status: row.try_get("review_status").map_err(map_sqlx)?,
        categories: string_list_from_json(
            &row.try_get::<String, _>("category_codes_json")
                .map_err(map_sqlx)?,
            "category_codes_json",
        )?,
        enabled: row.try_get::<i16, _>("enabled").map_err(map_sqlx)? != 0,
        featured: row.try_get::<i16, _>("featured").map_err(map_sqlx)? != 0,
        install_count: int64_to_uint64(
            row.try_get("install_count").map_err(map_sqlx)?,
            "install_count",
        )?,
        tags: string_list_from_json(
            &row.try_get::<String, _>("tags_json").map_err(map_sqlx)?,
            "tags_json",
        )?,
        version: int64_to_uint64(row.try_get("version").map_err(map_sqlx)?, "version")?,
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
        id: int64_to_uint64(row.try_get("id").map_err(map_sqlx)?, "id")?,
        uuid: row.try_get("uuid").map_err(map_sqlx)?,
        tenant_id: int64_to_uint64(row.try_get("tenant_id").map_err(map_sqlx)?, "tenant_id")?,
        organization_id: int64_to_uint64(
            row.try_get("organization_id").map_err(map_sqlx)?,
            "organization_id",
        )?,
        category_type: row.try_get("category_type").map_err(map_sqlx)?,
        code: row.try_get("code").map_err(map_sqlx)?,
        name: row.try_get("name").map_err(map_sqlx)?,
        description: row.try_get("description").map_err(map_sqlx)?,
        parent_id: optional_int64_to_uint64(
            row.try_get("parent_id").map_err(map_sqlx)?,
            "parent_id",
        )?,
        sort_weight: row.try_get("sort_weight").map_err(map_sqlx)?,
        permission_code: row.try_get("permission_code").map_err(map_sqlx)?,
        visible: row.try_get::<i16, _>("visible").map_err(map_sqlx)? != 0,
        status: row.try_get("status").map_err(map_sqlx)?,
        version: int64_to_uint64(row.try_get("version").map_err(map_sqlx)?, "version")?,
        created_at: timestamp_to_rfc3339(row.try_get("created_at").map_err(map_sqlx)?),
        updated_at: timestamp_to_rfc3339(row.try_get("updated_at").map_err(map_sqlx)?),
    })
}

fn row_to_capability(row: &sqlx::postgres::PgRow) -> SkillsResult<SkillCapabilityRecord> {
    Ok(SkillCapabilityRecord {
        id: int64_to_uint64(row.try_get("id").map_err(map_sqlx)?, "id")?,
        uuid: row.try_get("uuid").map_err(map_sqlx)?,
        tenant_id: int64_to_uint64(row.try_get("tenant_id").map_err(map_sqlx)?, "tenant_id")?,
        organization_id: int64_to_uint64(
            row.try_get("organization_id").map_err(map_sqlx)?,
            "organization_id",
        )?,
        capability_key: row.try_get("capability_key").map_err(map_sqlx)?,
        display_name: row.try_get("display_name").map_err(map_sqlx)?,
        description: row.try_get("description").map_err(map_sqlx)?,
        risk_level: capability_risk(&row.try_get::<String, _>("risk_level").map_err(map_sqlx)?)?,
        status: row.try_get("status").map_err(map_sqlx)?,
        version: int64_to_uint64(row.try_get("version").map_err(map_sqlx)?, "version")?,
        created_at: timestamp_to_rfc3339(row.try_get("created_at").map_err(map_sqlx)?),
        updated_at: timestamp_to_rfc3339(row.try_get("updated_at").map_err(map_sqlx)?),
    })
}

fn row_to_artifact(row: &sqlx::postgres::PgRow) -> SkillsResult<SkillArtifactRecord> {
    Ok(SkillArtifactRecord {
        id: int64_to_uint64(row.try_get("id").map_err(map_sqlx)?, "id")?,
        uuid: row.try_get("uuid").map_err(map_sqlx)?,
        tenant_id: int64_to_uint64(row.try_get("tenant_id").map_err(map_sqlx)?, "tenant_id")?,
        package_id: int64_to_uint64(row.try_get("package_id").map_err(map_sqlx)?, "package_id")?,
        version_label: row.try_get("version_label").map_err(map_sqlx)?,
        artifact_ref: row.try_get("artifact_ref").map_err(map_sqlx)?,
        checksum_sha256: row.try_get("checksum_sha256").map_err(map_sqlx)?,
        size_bytes: optional_int64_to_uint64(
            row.try_get("size_bytes").map_err(map_sqlx)?,
            "size_bytes",
        )?,
        invocation_kind: invocation(
            &row.try_get::<String, _>("invocation_kind")
                .map_err(map_sqlx)?,
        )?,
        entrypoint: row.try_get("entrypoint").map_err(map_sqlx)?,
        input_schema: json_value_from_text(
            &row.try_get::<String, _>("input_schema_json")
                .map_err(map_sqlx)?,
            "input_schema_json",
        )?,
        output_schema: json_value_from_text(
            &row.try_get::<String, _>("output_schema_json")
                .map_err(map_sqlx)?,
            "output_schema_json",
        )?,
        config_schema: json_value_from_text(
            &row.try_get::<String, _>("config_schema_json")
                .map_err(map_sqlx)?,
            "config_schema_json",
        )?,
        default_config: json_value_from_text(
            &row.try_get::<String, _>("default_config_json")
                .map_err(map_sqlx)?,
            "default_config_json",
        )?,
        security_profile_id: row.try_get("security_profile_id").map_err(map_sqlx)?,
        status: artifact_status(&row.try_get::<String, _>("status").map_err(map_sqlx)?)?,
        capability_keys: string_list_from_json(
            &row.try_get::<String, _>("capability_keys_json")
                .map_err(map_sqlx)?,
            "capability_keys_json",
        )?,
        published_at: row
            .try_get::<Option<chrono::DateTime<chrono::Utc>>, _>("published_at")
            .map_err(map_sqlx)?
            .map(timestamp_to_rfc3339),
        yanked_at: row
            .try_get::<Option<chrono::DateTime<chrono::Utc>>, _>("yanked_at")
            .map_err(map_sqlx)?
            .map(timestamp_to_rfc3339),
        created_at: timestamp_to_rfc3339(row.try_get("created_at").map_err(map_sqlx)?),
    })
}

fn row_to_installation(row: &sqlx::postgres::PgRow) -> SkillsResult<SkillInstallationRecord> {
    Ok(SkillInstallationRecord {
        id: int64_to_uint64(row.try_get("id").map_err(map_sqlx)?, "id")?,
        uuid: row.try_get("uuid").map_err(map_sqlx)?,
        tenant_id: int64_to_uint64(row.try_get("tenant_id").map_err(map_sqlx)?, "tenant_id")?,
        organization_id: int64_to_uint64(
            row.try_get("organization_id").map_err(map_sqlx)?,
            "organization_id",
        )?,
        subject_kind: subject_kind(&row.try_get::<String, _>("subject_kind").map_err(map_sqlx)?)?,
        subject_id: int64_to_uint64(row.try_get("subject_id").map_err(map_sqlx)?, "subject_id")?,
        skill_id: int64_to_uint64(row.try_get("skill_id").map_err(map_sqlx)?, "skill_id")?,
        package_id: int64_to_uint64(row.try_get("package_id").map_err(map_sqlx)?, "package_id")?,
        artifact_id: int64_to_uint64(row.try_get("artifact_id").map_err(map_sqlx)?, "artifact_id")?,
        installed_by_user_id: int64_to_uint64(
            row.try_get("installed_by_user_id").map_err(map_sqlx)?,
            "installed_by_user_id",
        )?,
        install_status: row.try_get("install_status").map_err(map_sqlx)?,
        enabled: row.try_get::<i16, _>("enabled").map_err(map_sqlx)? != 0,
        config: json_value_from_text(
            &row.try_get::<String, _>("config_json").map_err(map_sqlx)?,
            "config_json",
        )?,
        version: int64_to_uint64(row.try_get("version").map_err(map_sqlx)?, "version")?,
        installed_at: timestamp_to_rfc3339(row.try_get("installed_at").map_err(map_sqlx)?),
        updated_at: timestamp_to_rfc3339(row.try_get("updated_at").map_err(map_sqlx)?),
    })
}

const PACKAGE_SELECT: &str = r#"
    SELECT p.id, p.uuid, p.tenant_id, p.organization_id, p.owner_user_id,
           s.skill_key, p.package_key, p.code, p.display_name, p.summary, p.description,
           COALESCE((
               SELECT jsonb_agg(c.code ORDER BY c.code)::text
               FROM skills_category_binding b
               JOIN skills_category c ON c.id = b.category_id AND c.deleted_at IS NULL
               WHERE b.skill_id = s.id
           ), '[]') AS category_codes_json,
           p.tags_json, p.status, p.visibility, p.featured, p.sort_weight, p.version,
           p.created_at, p.updated_at, p.deleted_at
    FROM skills_package p
    JOIN skills_definition s ON s.package_id = p.id AND s.deleted_at IS NULL
"#;

const SKILL_SELECT: &str = r#"
    SELECT s.id, s.uuid, s.tenant_id, s.organization_id, s.skill_key, s.package_id,
           p.display_name, p.summary, p.description, s.market_status, p.visibility,
           s.review_status,
           COALESCE((
               SELECT jsonb_agg(c.code ORDER BY c.code)::text
               FROM skills_category_binding b
               JOIN skills_category c ON c.id = b.category_id AND c.deleted_at IS NULL
               WHERE b.skill_id = s.id
           ), '[]') AS category_codes_json,
           s.enabled, s.featured, s.install_count, p.tags_json, s.version,
           s.created_at, s.updated_at, s.deleted_at
    FROM skills_definition s
    JOIN skills_package p ON p.id = s.package_id AND p.deleted_at IS NULL
"#;

const ARTIFACT_SELECT: &str = r#"
    SELECT a.id, a.uuid, a.tenant_id, a.package_id, a.version_label, a.artifact_ref,
           a.checksum_sha256, a.size_bytes, a.invocation_kind, a.entrypoint,
           a.input_schema_json, a.output_schema_json, a.config_schema_json,
           a.default_config_json, a.security_profile_id, a.status,
           COALESCE((
               SELECT jsonb_agg(c.capability_key ORDER BY c.capability_key)::text
               FROM skills_artifact_capability ac
               JOIN skills_capability c ON c.id = ac.capability_id AND c.deleted_at IS NULL
               WHERE ac.artifact_id = a.id
           ), '[]') AS capability_keys_json,
           a.published_at, a.yanked_at, a.created_at
    FROM skills_artifact a
"#;

const INSTALLATION_SELECT: &str = r#"
    SELECT i.id, i.uuid, i.tenant_id, i.organization_id, i.subject_kind, i.subject_id,
           s.id AS skill_id, i.package_id, i.artifact_id, i.installed_by_user_id,
           i.install_status, i.enabled, i.config_json, i.version, i.installed_at, i.updated_at
    FROM skills_installation i
    JOIN skills_definition s
      ON s.tenant_id=i.tenant_id AND s.package_id=i.package_id AND s.deleted_at IS NULL
"#;

pub async fn list_skill_packages_page(
    pool: &PgPool,
    tenant_id: u64,
    params: OffsetListPageParams,
    keyword: Option<&str>,
) -> SkillsResult<(Vec<SkillPackageRecord>, i64)> {
    let sql = format!(
        "SELECT package_rows.*, COUNT(*) OVER() AS {LIST_TOTAL_SQL_COLUMN}
         FROM ({PACKAGE_SELECT}
               WHERE p.tenant_id = $1 AND p.deleted_at IS NULL AND p.status <> 4
                 AND ($2 = '%' OR p.display_name ILIKE $2 OR p.package_key ILIKE $2 OR p.code ILIKE $2)
         ) package_rows
         ORDER BY featured DESC, sort_weight DESC, updated_at DESC, code ASC LIMIT $3 OFFSET $4"
    );
    let rows = sqlx::query(&sql)
        .bind(uint64_to_int64(tenant_id, "tenant_id")?)
        .bind(search_pattern(keyword))
        .bind(params.page_size)
        .bind(params.offset)
        .fetch_all(pool)
        .await
        .map_err(map_sqlx)?;
    page(rows, row_to_package)
}

pub async fn get_skill_package(
    pool: &PgPool,
    tenant_id: u64,
    package_id: u64,
) -> SkillsResult<SkillPackageRecord> {
    let sql = format!(
        "{PACKAGE_SELECT} WHERE p.tenant_id = $1 AND p.id = $2 AND p.deleted_at IS NULL LIMIT 1"
    );
    let row = sqlx::query(&sql)
        .bind(uint64_to_int64(tenant_id, "tenant_id")?)
        .bind(uint64_to_int64(package_id, "package_id")?)
        .fetch_optional(pool)
        .await
        .map_err(map_sqlx)?;
    row.as_ref()
        .map(row_to_package)
        .transpose()?
        .ok_or_else(|| SkillsServiceError::NotFound(format!("skill package {package_id}")))
}

pub async fn list_marketplace_skill_packages_page(
    pool: &PgPool,
    tenant_id: u64,
    organization_id: u64,
    user_id: u64,
    params: OffsetListPageParams,
    keyword: Option<&str>,
) -> SkillsResult<(Vec<SkillPackageRecord>, i64)> {
    let sql = format!(
        "SELECT package_rows.*, COUNT(*) OVER() AS {LIST_TOTAL_SQL_COLUMN}
         FROM ({PACKAGE_SELECT}
               WHERE p.tenant_id = $1 AND p.deleted_at IS NULL AND p.status = 1
                 AND (p.visibility IN (1, 3)
                      OR (p.visibility = 2 AND p.organization_id = $2)
                      OR (p.visibility = 0 AND p.owner_user_id = $3))
                 AND ($4 = '%' OR p.display_name ILIKE $4 OR p.package_key ILIKE $4 OR p.code ILIKE $4)
         ) package_rows
         ORDER BY featured DESC, sort_weight DESC, updated_at DESC, code ASC LIMIT $5 OFFSET $6"
    );
    let rows = sqlx::query(&sql)
        .bind(uint64_to_int64(tenant_id, "tenant_id")?)
        .bind(uint64_to_int64(organization_id, "organization_id")?)
        .bind(uint64_to_int64(user_id, "user_id")?)
        .bind(search_pattern(keyword))
        .bind(params.page_size)
        .bind(params.offset)
        .fetch_all(pool)
        .await
        .map_err(map_sqlx)?;
    page(rows, row_to_package)
}

pub async fn get_marketplace_skill_package(
    pool: &PgPool,
    tenant_id: u64,
    organization_id: u64,
    user_id: u64,
    package_id: u64,
) -> SkillsResult<SkillPackageRecord> {
    let sql = format!(
        "{PACKAGE_SELECT}
         WHERE p.tenant_id = $1 AND p.id = $2 AND p.status = 1 AND p.deleted_at IS NULL
           AND (p.visibility IN (1, 3)
                OR (p.visibility = 2 AND p.organization_id = $3)
                OR (p.visibility = 0 AND p.owner_user_id = $4))
         LIMIT 1"
    );
    let row = sqlx::query(&sql)
        .bind(uint64_to_int64(tenant_id, "tenant_id")?)
        .bind(uint64_to_int64(package_id, "package_id")?)
        .bind(uint64_to_int64(organization_id, "organization_id")?)
        .bind(uint64_to_int64(user_id, "user_id")?)
        .fetch_optional(pool)
        .await
        .map_err(map_sqlx)?;
    row.as_ref()
        .map(row_to_package)
        .transpose()?
        .ok_or_else(|| SkillsServiceError::NotFound(format!("skill package {package_id}")))
}

pub async fn list_skills_page(
    pool: &PgPool,
    tenant_id: u64,
    organization_id: u64,
    user_id: u64,
    params: OffsetListPageParams,
    keyword: Option<&str>,
) -> SkillsResult<(Vec<SkillRecord>, i64)> {
    let sql = format!(
        "SELECT skill_rows.*, COUNT(*) OVER() AS {LIST_TOTAL_SQL_COLUMN}
         FROM ({SKILL_SELECT}
               WHERE s.tenant_id = $1 AND s.deleted_at IS NULL AND s.enabled = 1
                 AND s.market_status = 'published' AND s.review_status = 'approved'
                 AND (p.visibility IN (1, 3)
                      OR (p.visibility = 2 AND p.organization_id = $2)
                      OR (p.visibility = 0 AND p.owner_user_id = $3))
                 AND ($4 = '%' OR p.display_name ILIKE $4 OR s.skill_key ILIKE $4 OR COALESCE(p.summary, '') ILIKE $4)
         ) skill_rows
         ORDER BY featured DESC, updated_at DESC, skill_key ASC LIMIT $5 OFFSET $6"
    );
    let rows = sqlx::query(&sql)
        .bind(uint64_to_int64(tenant_id, "tenant_id")?)
        .bind(uint64_to_int64(organization_id, "organization_id")?)
        .bind(uint64_to_int64(user_id, "user_id")?)
        .bind(search_pattern(keyword))
        .bind(params.page_size)
        .bind(params.offset)
        .fetch_all(pool)
        .await
        .map_err(map_sqlx)?;
    page(rows, row_to_skill)
}

pub async fn get_skill(
    pool: &PgPool,
    tenant_id: u64,
    organization_id: u64,
    user_id: u64,
    skill_key: &str,
) -> SkillsResult<SkillRecord> {
    let sql = format!(
        "{SKILL_SELECT}
         WHERE s.tenant_id = $1 AND s.skill_key = $2 AND s.deleted_at IS NULL
           AND s.enabled = 1 AND s.market_status = 'published' AND s.review_status = 'approved'
           AND (p.visibility IN (1, 3)
                OR (p.visibility = 2 AND p.organization_id = $3)
                OR (p.visibility = 0 AND p.owner_user_id = $4))
         LIMIT 1"
    );
    let row = sqlx::query(&sql)
        .bind(uint64_to_int64(tenant_id, "tenant_id")?)
        .bind(skill_key)
        .bind(uint64_to_int64(organization_id, "organization_id")?)
        .bind(uint64_to_int64(user_id, "user_id")?)
        .fetch_optional(pool)
        .await
        .map_err(map_sqlx)?;
    row.as_ref()
        .map(row_to_skill)
        .transpose()?
        .ok_or_else(|| SkillsServiceError::NotFound(skill_key.to_string()))
}

fn page<T>(
    rows: Vec<sqlx::postgres::PgRow>,
    mapper: fn(&sqlx::postgres::PgRow) -> SkillsResult<T>,
) -> SkillsResult<(Vec<T>, i64)> {
    let total = rows
        .first()
        .and_then(|row| row.try_get::<i64, _>(LIST_TOTAL_SQL_COLUMN).ok())
        .unwrap_or(0);
    let items = rows.iter().map(mapper).collect::<SkillsResult<Vec<_>>>()?;
    Ok((items, total))
}

pub async fn get_category(
    pool: &PgPool,
    tenant_id: u64,
    category_id: u64,
) -> SkillsResult<SkillCategoryRecord> {
    let row = sqlx::query(
        "SELECT id, uuid, tenant_id, organization_id, category_type, code, name, description,
                parent_id, sort_weight, permission_code, visible, status, version,
                created_at, updated_at
         FROM skills_category
         WHERE id=$1 AND tenant_id IN (0,$2) AND deleted_at IS NULL LIMIT 1",
    )
    .bind(uint64_to_int64(category_id, "category_id")?)
    .bind(uint64_to_int64(tenant_id, "tenant_id")?)
    .fetch_optional(pool)
    .await
    .map_err(map_sqlx)?;
    row.as_ref()
        .map(row_to_category)
        .transpose()?
        .ok_or_else(|| SkillsServiceError::NotFound(format!("skill category {category_id}")))
}

pub async fn list_categories_page(
    pool: &PgPool,
    tenant_id: u64,
    category_type: &str,
    params: OffsetListPageParams,
    keyword: Option<&str>,
) -> SkillsResult<(Vec<SkillCategoryRecord>, i64)> {
    let sql = format!(
        "SELECT id, uuid, tenant_id, organization_id, category_type, code, name, description,
                parent_id, sort_weight, permission_code, visible, status, version,
                created_at, updated_at,
                COUNT(*) OVER() AS {LIST_TOTAL_SQL_COLUMN}
         FROM skills_category
         WHERE category_type = $1 AND tenant_id IN (0, $2) AND deleted_at IS NULL
           AND ($3 = '%' OR name ILIKE $3 OR code ILIKE $3)
         ORDER BY sort_weight ASC, code ASC LIMIT $4 OFFSET $5"
    );
    let rows = sqlx::query(&sql)
        .bind(category_type)
        .bind(uint64_to_int64(tenant_id, "tenant_id")?)
        .bind(search_pattern(keyword))
        .bind(params.page_size)
        .bind(params.offset)
        .fetch_all(pool)
        .await
        .map_err(map_sqlx)?;
    page(rows, row_to_category)
}

pub async fn upsert_category(
    pool: &PgPool,
    id_generator: &SnowflakeIdGenerator,
    record: SkillCategoryRecord,
) -> SkillsResult<SkillCategoryRecord> {
    let row = if record.id == 0 {
        sqlx::query(
            "INSERT INTO skills_category (
                 id, uuid, tenant_id, organization_id, category_type, code, name, description,
                 parent_id, sort_weight, permission_code, visible, status, version
             ) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,1)
             RETURNING id, uuid, tenant_id, organization_id, category_type, code, name,
                       description, parent_id, sort_weight, permission_code, visible, status,
                       version, created_at, updated_at",
        )
        .bind(next_id(id_generator)?)
        .bind(new_uuid())
        .bind(uint64_to_int64(record.tenant_id, "tenant_id")?)
        .bind(uint64_to_int64(record.organization_id, "organization_id")?)
        .bind(&record.category_type)
        .bind(&record.code)
        .bind(&record.name)
        .bind(&record.description)
        .bind(optional_uint64_to_int64(record.parent_id, "parent_id")?)
        .bind(record.sort_weight)
        .bind(&record.permission_code)
        .bind(i16::from(record.visible))
        .bind(record.status)
        .fetch_one(pool)
        .await
        .map_err(map_sqlx)?
    } else {
        sqlx::query(
            "UPDATE skills_category SET name=$4, description=$5, parent_id=$6,
                    sort_weight=$7, permission_code=$8, visible=$9, status=$10,
                    version=version+1, updated_at=CURRENT_TIMESTAMP
             WHERE id=$1 AND tenant_id IN (0,$2) AND version=$3 AND deleted_at IS NULL
             RETURNING id, uuid, tenant_id, organization_id, category_type, code, name,
                       description, parent_id, sort_weight, permission_code, visible, status,
                       version, created_at, updated_at",
        )
        .bind(uint64_to_int64(record.id, "id")?)
        .bind(uint64_to_int64(record.tenant_id, "tenant_id")?)
        .bind(uint64_to_int64(record.version, "version")?)
        .bind(&record.name)
        .bind(&record.description)
        .bind(optional_uint64_to_int64(record.parent_id, "parent_id")?)
        .bind(record.sort_weight)
        .bind(&record.permission_code)
        .bind(i16::from(record.visible))
        .bind(record.status)
        .fetch_optional(pool)
        .await
        .map_err(map_sqlx)?
        .ok_or_else(|| SkillsServiceError::Conflict("category version changed".to_string()))?
    };
    row_to_category(&row)
}

pub async fn list_capabilities_page(
    pool: &PgPool,
    tenant_id: u64,
    params: OffsetListPageParams,
    keyword: Option<&str>,
) -> SkillsResult<(Vec<SkillCapabilityRecord>, i64)> {
    let sql = format!(
        "SELECT id, uuid, tenant_id, organization_id, capability_key, display_name,
                description, risk_level, status, version, created_at, updated_at,
                COUNT(*) OVER() AS {LIST_TOTAL_SQL_COLUMN}
         FROM skills_capability
         WHERE tenant_id IN (0,$1) AND deleted_at IS NULL
           AND ($2='%' OR capability_key ILIKE $2 OR display_name ILIKE $2)
         ORDER BY capability_key ASC LIMIT $3 OFFSET $4"
    );
    let rows = sqlx::query(&sql)
        .bind(uint64_to_int64(tenant_id, "tenant_id")?)
        .bind(search_pattern(keyword))
        .bind(params.page_size)
        .bind(params.offset)
        .fetch_all(pool)
        .await
        .map_err(map_sqlx)?;
    page(rows, row_to_capability)
}

pub async fn get_capability(
    pool: &PgPool,
    tenant_id: u64,
    capability_id: u64,
) -> SkillsResult<SkillCapabilityRecord> {
    let row = sqlx::query(
        "SELECT id, uuid, tenant_id, organization_id, capability_key, display_name,
                description, risk_level, status, version, created_at, updated_at
         FROM skills_capability
         WHERE id=$1 AND tenant_id IN (0,$2) AND deleted_at IS NULL LIMIT 1",
    )
    .bind(uint64_to_int64(capability_id, "capability_id")?)
    .bind(uint64_to_int64(tenant_id, "tenant_id")?)
    .fetch_optional(pool)
    .await
    .map_err(map_sqlx)?;
    row.as_ref()
        .map(row_to_capability)
        .transpose()?
        .ok_or_else(|| SkillsServiceError::NotFound(format!("skill capability {capability_id}")))
}

pub async fn upsert_capability(
    pool: &PgPool,
    id_generator: &SnowflakeIdGenerator,
    record: SkillCapabilityRecord,
) -> SkillsResult<SkillCapabilityRecord> {
    let row = if record.id == 0 {
        sqlx::query(
            "INSERT INTO skills_capability (
                 id, uuid, tenant_id, organization_id, capability_key, display_name,
                 description, risk_level, status, version
             ) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,1)
             RETURNING id, uuid, tenant_id, organization_id, capability_key, display_name,
                       description, risk_level, status, version, created_at, updated_at",
        )
        .bind(next_id(id_generator)?)
        .bind(new_uuid())
        .bind(uint64_to_int64(record.tenant_id, "tenant_id")?)
        .bind(uint64_to_int64(record.organization_id, "organization_id")?)
        .bind(&record.capability_key)
        .bind(&record.display_name)
        .bind(&record.description)
        .bind(record.risk_level.as_str())
        .bind(record.status)
        .fetch_one(pool)
        .await
        .map_err(map_sqlx)?
    } else {
        sqlx::query(
            "UPDATE skills_capability SET display_name=$4, description=$5, risk_level=$6,
                    status=$7, version=version+1, updated_at=CURRENT_TIMESTAMP
             WHERE id=$1 AND tenant_id IN (0,$2) AND version=$3 AND deleted_at IS NULL
             RETURNING id, uuid, tenant_id, organization_id, capability_key, display_name,
                       description, risk_level, status, version, created_at, updated_at",
        )
        .bind(uint64_to_int64(record.id, "id")?)
        .bind(uint64_to_int64(record.tenant_id, "tenant_id")?)
        .bind(uint64_to_int64(record.version, "version")?)
        .bind(&record.display_name)
        .bind(&record.description)
        .bind(record.risk_level.as_str())
        .bind(record.status)
        .fetch_optional(pool)
        .await
        .map_err(map_sqlx)?
        .ok_or_else(|| SkillsServiceError::Conflict("capability version changed".to_string()))?
    };
    row_to_capability(&row)
}

pub async fn list_artifacts_page(
    pool: &PgPool,
    tenant_id: u64,
    package_id: u64,
    params: OffsetListPageParams,
) -> SkillsResult<(Vec<SkillArtifactRecord>, i64)> {
    let sql = format!(
        "SELECT artifact_rows.*, COUNT(*) OVER() AS {LIST_TOTAL_SQL_COLUMN}
         FROM ({ARTIFACT_SELECT} WHERE a.tenant_id=$1 AND a.package_id=$2) artifact_rows
         ORDER BY created_at DESC, id DESC LIMIT $3 OFFSET $4"
    );
    let rows = sqlx::query(&sql)
        .bind(uint64_to_int64(tenant_id, "tenant_id")?)
        .bind(uint64_to_int64(package_id, "package_id")?)
        .bind(params.page_size)
        .bind(params.offset)
        .fetch_all(pool)
        .await
        .map_err(map_sqlx)?;
    page(rows, row_to_artifact)
}

pub async fn list_installable_artifacts_page(
    pool: &PgPool,
    tenant_id: u64,
    organization_id: u64,
    user_id: u64,
    package_id: u64,
    params: OffsetListPageParams,
) -> SkillsResult<(Vec<SkillArtifactRecord>, i64)> {
    let sql = format!(
        "SELECT artifact_rows.*, COUNT(*) OVER() AS {LIST_TOTAL_SQL_COLUMN}
         FROM ({ARTIFACT_SELECT}
               JOIN skills_package p
                 ON p.id=a.package_id AND p.tenant_id=a.tenant_id AND p.deleted_at IS NULL
               JOIN skills_definition s
                 ON s.package_id=p.id AND s.tenant_id=p.tenant_id AND s.deleted_at IS NULL
               WHERE a.tenant_id=$1 AND a.package_id=$2 AND a.status='published'
                 AND p.status=1
                 AND s.enabled=1 AND s.market_status='published' AND s.review_status='approved'
                 AND (p.visibility IN (1, 3)
                      OR (p.visibility=2 AND p.organization_id=$3)
                      OR (p.visibility=0 AND p.owner_user_id=$4))
         ) artifact_rows
         ORDER BY published_at DESC, created_at DESC, id DESC LIMIT $5 OFFSET $6"
    );
    let rows = sqlx::query(&sql)
        .bind(uint64_to_int64(tenant_id, "tenant_id")?)
        .bind(uint64_to_int64(package_id, "package_id")?)
        .bind(uint64_to_int64(organization_id, "organization_id")?)
        .bind(uint64_to_int64(user_id, "user_id")?)
        .bind(params.page_size)
        .bind(params.offset)
        .fetch_all(pool)
        .await
        .map_err(map_sqlx)?;
    page(rows, row_to_artifact)
}

pub async fn create_artifact(
    pool: &PgPool,
    id_generator: &SnowflakeIdGenerator,
    artifact: SkillArtifactRecord,
) -> SkillsResult<SkillArtifactRecord> {
    let mut tx = pool.begin().await.map_err(map_sqlx)?;
    ensure_package(&mut tx, artifact.tenant_id, artifact.package_id).await?;
    let artifact_id = insert_artifact(&mut tx, id_generator, artifact).await?;
    tx.commit().await.map_err(map_sqlx)?;
    get_artifact(pool, int64_to_uint64(artifact_id, "artifact_id")?).await
}

async fn get_artifact(pool: &PgPool, artifact_id: u64) -> SkillsResult<SkillArtifactRecord> {
    let sql = format!("{ARTIFACT_SELECT} WHERE a.id=$1 LIMIT 1");
    let row = sqlx::query(&sql)
        .bind(uint64_to_int64(artifact_id, "artifact_id")?)
        .fetch_optional(pool)
        .await
        .map_err(map_sqlx)?;
    row.as_ref()
        .map(row_to_artifact)
        .transpose()?
        .ok_or_else(|| SkillsServiceError::NotFound(format!("skill artifact {artifact_id}")))
}

pub async fn create_skill_package(
    pool: &PgPool,
    id_generator: &SnowflakeIdGenerator,
    record: SkillPackageRecord,
    mut initial_artifact: SkillArtifactRecord,
) -> SkillsResult<SkillPackageRecord> {
    let mut tx = pool.begin().await.map_err(map_sqlx)?;
    let tags_json = string_list_to_json(&record.tags, "tags")?;
    let package_id = next_id(id_generator)?;
    let skill_id = next_id(id_generator)?;
    sqlx::query(
        "INSERT INTO skills_package (
             id, uuid, tenant_id, organization_id, owner_user_id, package_key, code,
             display_name, summary, description, tags_json, status, visibility,
             featured, sort_weight, version
         ) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,$15,1)",
    )
    .bind(package_id)
    .bind(new_uuid())
    .bind(uint64_to_int64(record.tenant_id, "tenant_id")?)
    .bind(uint64_to_int64(record.organization_id, "organization_id")?)
    .bind(uint64_to_int64(record.owner_user_id, "owner_user_id")?)
    .bind(&record.package_key)
    .bind(&record.code)
    .bind(&record.display_name)
    .bind(&record.summary)
    .bind(&record.description)
    .bind(tags_json)
    .bind(record.status.as_db_code())
    .bind(record.visibility.as_db_code())
    .bind(i16::from(record.featured))
    .bind(record.sort_weight)
    .execute(&mut *tx)
    .await
    .map_err(map_sqlx)?;
    sqlx::query(
        "INSERT INTO skills_definition (
             id, uuid, tenant_id, organization_id, skill_key, package_id, market_status,
             review_status, enabled, featured, version
         ) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,1)",
    )
    .bind(skill_id)
    .bind(new_uuid())
    .bind(uint64_to_int64(record.tenant_id, "tenant_id")?)
    .bind(uint64_to_int64(record.organization_id, "organization_id")?)
    .bind(&record.skill_key)
    .bind(package_id)
    .bind(package_market_status(record.status))
    .bind(package_review_status(record.status))
    .bind(i16::from(record.status == SkillLifecycleStatus::Active))
    .bind(i16::from(record.featured))
    .execute(&mut *tx)
    .await
    .map_err(map_sqlx)?;
    replace_category_bindings(
        &mut tx,
        id_generator,
        record.tenant_id,
        skill_id,
        &record.categories,
    )
    .await?;
    initial_artifact.tenant_id = record.tenant_id;
    initial_artifact.package_id = int64_to_uint64(package_id, "package_id")?;
    insert_artifact(&mut tx, id_generator, initial_artifact).await?;
    tx.commit().await.map_err(map_sqlx)?;
    get_skill_package(
        pool,
        record.tenant_id,
        int64_to_uint64(package_id, "package_id")?,
    )
    .await
}

pub async fn update_skill_package(
    pool: &PgPool,
    id_generator: &SnowflakeIdGenerator,
    record: SkillPackageRecord,
) -> SkillsResult<SkillPackageRecord> {
    let mut tx = pool.begin().await.map_err(map_sqlx)?;
    let tags_json = string_list_to_json(&record.tags, "tags")?;
    let updated = sqlx::query(
        "UPDATE skills_package SET display_name=$4, summary=$5, description=$6,
                tags_json=$7, status=$8, visibility=$9, featured=$10, sort_weight=$11,
                version=version+1, updated_at=CURRENT_TIMESTAMP
         WHERE id=$1 AND tenant_id=$2 AND version=$3 AND deleted_at IS NULL",
    )
    .bind(uint64_to_int64(record.id, "id")?)
    .bind(uint64_to_int64(record.tenant_id, "tenant_id")?)
    .bind(uint64_to_int64(record.version, "version")?)
    .bind(&record.display_name)
    .bind(&record.summary)
    .bind(&record.description)
    .bind(tags_json)
    .bind(record.status.as_db_code())
    .bind(record.visibility.as_db_code())
    .bind(i16::from(record.featured))
    .bind(record.sort_weight)
    .execute(&mut *tx)
    .await
    .map_err(map_sqlx)?;
    if updated.rows_affected() != 1 {
        return Err(SkillsServiceError::Conflict(
            "skill package version changed".to_string(),
        ));
    }
    let skill_id = sqlx::query_scalar::<_, i64>(
        "UPDATE skills_definition SET market_status=$3, review_status=$4, enabled=$5, featured=$6,
                version=version+1, updated_at=CURRENT_TIMESTAMP
         WHERE package_id=$1 AND tenant_id=$2 AND deleted_at IS NULL RETURNING id",
    )
    .bind(uint64_to_int64(record.id, "id")?)
    .bind(uint64_to_int64(record.tenant_id, "tenant_id")?)
    .bind(package_market_status(record.status))
    .bind(package_review_status(record.status))
    .bind(i16::from(record.status == SkillLifecycleStatus::Active))
    .bind(i16::from(record.featured))
    .fetch_one(&mut *tx)
    .await
    .map_err(map_sqlx)?;
    replace_category_bindings(
        &mut tx,
        id_generator,
        record.tenant_id,
        skill_id,
        &record.categories,
    )
    .await?;
    tx.commit().await.map_err(map_sqlx)?;
    get_skill_package(pool, record.tenant_id, record.id).await
}

async fn replace_category_bindings(
    tx: &mut Transaction<'_, Postgres>,
    id_generator: &SnowflakeIdGenerator,
    tenant_id: u64,
    skill_id: i64,
    category_codes: &[String],
) -> SkillsResult<()> {
    sqlx::query("DELETE FROM skills_category_binding WHERE skill_id=$1")
        .bind(skill_id)
        .execute(&mut **tx)
        .await
        .map_err(map_sqlx)?;
    for code in category_codes {
        let category_id = sqlx::query_scalar::<_, i64>(
            "SELECT id FROM skills_category
             WHERE tenant_id IN (0,$1) AND code=$2 AND category_type='skill_market'
               AND status=1 AND deleted_at IS NULL
             ORDER BY tenant_id DESC LIMIT 1",
        )
        .bind(uint64_to_int64(tenant_id, "tenant_id")?)
        .bind(code)
        .fetch_optional(&mut **tx)
        .await
        .map_err(map_sqlx)?
        .ok_or_else(|| SkillsServiceError::InvalidArgument(format!("unknown category: {code}")))?;
        sqlx::query(
            "INSERT INTO skills_category_binding (id, tenant_id, skill_id, category_id)
             VALUES ($1,$2,$3,$4)",
        )
        .bind(next_id(id_generator)?)
        .bind(uint64_to_int64(tenant_id, "tenant_id")?)
        .bind(skill_id)
        .bind(category_id)
        .execute(&mut **tx)
        .await
        .map_err(map_sqlx)?;
    }
    Ok(())
}

async fn ensure_package(
    tx: &mut Transaction<'_, Postgres>,
    tenant_id: u64,
    package_id: u64,
) -> SkillsResult<()> {
    let exists = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM skills_package
         WHERE id=$1 AND tenant_id=$2 AND deleted_at IS NULL)",
    )
    .bind(uint64_to_int64(package_id, "package_id")?)
    .bind(uint64_to_int64(tenant_id, "tenant_id")?)
    .fetch_one(&mut **tx)
    .await
    .map_err(map_sqlx)?;
    if !exists {
        return Err(SkillsServiceError::NotFound(format!(
            "skill package {package_id}"
        )));
    }
    Ok(())
}

async fn insert_artifact(
    tx: &mut Transaction<'_, Postgres>,
    id_generator: &SnowflakeIdGenerator,
    artifact: SkillArtifactRecord,
) -> SkillsResult<i64> {
    let artifact_id = next_id(id_generator)?;
    let input_schema_json = json_value_to_text(&artifact.input_schema, "input_schema")?;
    let output_schema_json = json_value_to_text(&artifact.output_schema, "output_schema")?;
    let config_schema_json = json_value_to_text(&artifact.config_schema, "config_schema")?;
    let default_config_json = json_value_to_text(&artifact.default_config, "default_config")?;
    sqlx::query(
        "INSERT INTO skills_artifact (
             id, uuid, tenant_id, package_id, version_label, artifact_ref, checksum_sha256,
             size_bytes, invocation_kind, entrypoint, input_schema_json, output_schema_json,
             config_schema_json, default_config_json, security_profile_id, status, published_at
         ) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,$15,$16,
                   CASE WHEN $16='published' THEN CURRENT_TIMESTAMP ELSE NULL END)",
    )
    .bind(artifact_id)
    .bind(new_uuid())
    .bind(uint64_to_int64(artifact.tenant_id, "tenant_id")?)
    .bind(uint64_to_int64(artifact.package_id, "package_id")?)
    .bind(&artifact.version_label)
    .bind(&artifact.artifact_ref)
    .bind(&artifact.checksum_sha256)
    .bind(optional_uint64_to_int64(artifact.size_bytes, "size_bytes")?)
    .bind(artifact.invocation_kind.as_str())
    .bind(&artifact.entrypoint)
    .bind(input_schema_json)
    .bind(output_schema_json)
    .bind(config_schema_json)
    .bind(default_config_json)
    .bind(&artifact.security_profile_id)
    .bind(artifact.status.as_str())
    .execute(&mut **tx)
    .await
    .map_err(map_sqlx)?;
    for key in artifact.capability_keys {
        let capability_id = sqlx::query_scalar::<_, i64>(
            "SELECT id FROM skills_capability
             WHERE tenant_id IN (0,$1) AND capability_key=$2 AND status=1 AND deleted_at IS NULL
             ORDER BY tenant_id DESC LIMIT 1",
        )
        .bind(uint64_to_int64(artifact.tenant_id, "tenant_id")?)
        .bind(&key)
        .fetch_optional(&mut **tx)
        .await
        .map_err(map_sqlx)?
        .ok_or_else(|| SkillsServiceError::InvalidArgument(format!("unknown capability: {key}")))?;
        sqlx::query(
            "INSERT INTO skills_artifact_capability
             (id, tenant_id, artifact_id, capability_id, required) VALUES ($1,$2,$3,$4,1)",
        )
        .bind(next_id(id_generator)?)
        .bind(uint64_to_int64(artifact.tenant_id, "tenant_id")?)
        .bind(artifact_id)
        .bind(capability_id)
        .execute(&mut **tx)
        .await
        .map_err(map_sqlx)?;
    }
    Ok(artifact_id)
}

pub async fn delete_skill_package(
    pool: &PgPool,
    tenant_id: u64,
    package_id: u64,
) -> SkillsResult<()> {
    let mut tx = pool.begin().await.map_err(map_sqlx)?;
    let result = sqlx::query(
        "UPDATE skills_package SET status=4, version=version+1,
                deleted_at=CURRENT_TIMESTAMP, updated_at=CURRENT_TIMESTAMP
         WHERE id=$1 AND tenant_id=$2 AND deleted_at IS NULL",
    )
    .bind(uint64_to_int64(package_id, "package_id")?)
    .bind(uint64_to_int64(tenant_id, "tenant_id")?)
    .execute(&mut *tx)
    .await
    .map_err(map_sqlx)?;
    if result.rows_affected() != 1 {
        return Err(SkillsServiceError::NotFound(format!(
            "skill package {package_id}"
        )));
    }
    sqlx::query(
        "UPDATE skills_definition SET market_status='removed', enabled=0, version=version+1,
                deleted_at=CURRENT_TIMESTAMP, updated_at=CURRENT_TIMESTAMP
         WHERE package_id=$1 AND tenant_id=$2 AND deleted_at IS NULL",
    )
    .bind(uint64_to_int64(package_id, "package_id")?)
    .bind(uint64_to_int64(tenant_id, "tenant_id")?)
    .execute(&mut *tx)
    .await
    .map_err(map_sqlx)?;
    sqlx::query(
        "UPDATE skills_installation SET install_status='removed', enabled=0,
                version=version+1, deleted_at=CURRENT_TIMESTAMP, updated_at=CURRENT_TIMESTAMP
         WHERE package_id=$1 AND tenant_id=$2 AND deleted_at IS NULL",
    )
    .bind(uint64_to_int64(package_id, "package_id")?)
    .bind(uint64_to_int64(tenant_id, "tenant_id")?)
    .execute(&mut *tx)
    .await
    .map_err(map_sqlx)?;
    tx.commit().await.map_err(map_sqlx)
}

pub async fn install_skill(
    pool: &PgPool,
    id_generator: &SnowflakeIdGenerator,
    record: SkillInstallationRecord,
) -> SkillsResult<SkillInstallationRecord> {
    let mut tx = pool.begin().await.map_err(map_sqlx)?;
    let config_json = json_value_to_text(&record.config, "config")?;
    let skill_id = sqlx::query_scalar::<_, i64>(
        "SELECT s.id FROM skills_definition s
         JOIN skills_package p ON p.id=s.package_id AND p.deleted_at IS NULL
         JOIN skills_artifact a ON a.package_id=p.id AND a.tenant_id=p.tenant_id
         WHERE p.id=$1 AND p.tenant_id=$2 AND p.status=1
           AND a.id=$3 AND a.status='published'
           AND s.enabled=1 AND s.market_status='published' AND s.review_status='approved'
           AND s.deleted_at IS NULL LIMIT 1 FOR SHARE OF p, a",
    )
    .bind(uint64_to_int64(record.package_id, "package_id")?)
    .bind(uint64_to_int64(record.tenant_id, "tenant_id")?)
    .bind(uint64_to_int64(record.artifact_id, "artifact_id")?)
    .fetch_optional(&mut *tx)
    .await
    .map_err(map_sqlx)?
    .ok_or_else(|| {
        SkillsServiceError::InvalidArgument("artifact is not installable".to_string())
    })?;
    let candidate_id = next_id(id_generator)?;
    let insert_result = sqlx::query(
        "INSERT INTO skills_installation (
             id, uuid, tenant_id, organization_id, subject_kind, subject_id,
             package_id, artifact_id, installed_by_user_id, install_status, enabled,
             config_json, version
         ) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,'installed',1,$10,1)
         ON CONFLICT (tenant_id, organization_id, subject_kind, subject_id, package_id)
         WHERE deleted_at IS NULL DO NOTHING",
    )
    .bind(candidate_id)
    .bind(new_uuid())
    .bind(uint64_to_int64(record.tenant_id, "tenant_id")?)
    .bind(uint64_to_int64(record.organization_id, "organization_id")?)
    .bind(record.subject_kind.as_str())
    .bind(uint64_to_int64(record.subject_id, "subject_id")?)
    .bind(uint64_to_int64(record.package_id, "package_id")?)
    .bind(uint64_to_int64(record.artifact_id, "artifact_id")?)
    .bind(uint64_to_int64(
        record.installed_by_user_id,
        "installed_by_user_id",
    )?)
    .bind(&config_json)
    .execute(&mut *tx)
    .await
    .map_err(map_sqlx)?;
    let installation_id = if insert_result.rows_affected() == 1 {
        let count_result = sqlx::query(
            "UPDATE skills_definition SET install_count=install_count+1
             WHERE id=$1 AND enabled=1 AND market_status='published'
               AND review_status='approved' AND deleted_at IS NULL",
        )
        .bind(skill_id)
        .execute(&mut *tx)
        .await
        .map_err(map_sqlx)?;
        if count_result.rows_affected() != 1 {
            return Err(SkillsServiceError::Repository(
                "installed skill aggregate disappeared during the transaction".to_string(),
            ));
        }
        candidate_id
    } else {
        sqlx::query_scalar::<_, i64>(
            "UPDATE skills_installation SET artifact_id=$6, installed_by_user_id=$7,
                    install_status='installed', enabled=1, config_json=$8, version=version+1,
                    updated_at=CURRENT_TIMESTAMP
             WHERE tenant_id=$1 AND organization_id=$2 AND subject_kind=$3 AND subject_id=$4
               AND package_id=$5 AND deleted_at IS NULL
               AND EXISTS (
                   SELECT 1 FROM skills_definition s
                   WHERE s.id=$9 AND s.enabled=1 AND s.market_status='published'
                     AND s.review_status='approved' AND s.deleted_at IS NULL
               )
             RETURNING id",
        )
        .bind(uint64_to_int64(record.tenant_id, "tenant_id")?)
        .bind(uint64_to_int64(record.organization_id, "organization_id")?)
        .bind(record.subject_kind.as_str())
        .bind(uint64_to_int64(record.subject_id, "subject_id")?)
        .bind(uint64_to_int64(record.package_id, "package_id")?)
        .bind(uint64_to_int64(record.artifact_id, "artifact_id")?)
        .bind(uint64_to_int64(
            record.installed_by_user_id,
            "installed_by_user_id",
        )?)
        .bind(&config_json)
        .bind(skill_id)
        .fetch_optional(&mut *tx)
        .await
        .map_err(map_sqlx)?
        .ok_or_else(|| {
            SkillsServiceError::Conflict(
                "skill installation changed during concurrent installation".to_string(),
            )
        })?
    };
    tx.commit().await.map_err(map_sqlx)?;
    get_installation(pool, int64_to_uint64(installation_id, "installation_id")?).await
}

async fn get_installation(
    pool: &PgPool,
    installation_id: u64,
) -> SkillsResult<SkillInstallationRecord> {
    let sql = format!("{INSTALLATION_SELECT} WHERE i.id=$1 AND i.deleted_at IS NULL LIMIT 1");
    let row = sqlx::query(&sql)
        .bind(uint64_to_int64(installation_id, "installation_id")?)
        .fetch_optional(pool)
        .await
        .map_err(map_sqlx)?;
    row.as_ref()
        .map(row_to_installation)
        .transpose()?
        .ok_or_else(|| {
            SkillsServiceError::NotFound(format!("skill installation {installation_id}"))
        })
}

pub async fn list_installations_page(
    pool: &PgPool,
    tenant_id: u64,
    organization_id: u64,
    subject_kind_value: &str,
    subject_id: u64,
    params: OffsetListPageParams,
) -> SkillsResult<(Vec<SkillInstallationRecord>, i64)> {
    let sql = format!(
        "SELECT installation_rows.*, COUNT(*) OVER() AS {LIST_TOTAL_SQL_COLUMN}
         FROM ({INSTALLATION_SELECT}
               WHERE i.tenant_id=$1 AND i.organization_id=$2
                 AND i.subject_kind=$3 AND i.subject_id=$4 AND i.deleted_at IS NULL
         ) installation_rows
         ORDER BY updated_at DESC, id DESC LIMIT $5 OFFSET $6"
    );
    let rows = sqlx::query(&sql)
        .bind(uint64_to_int64(tenant_id, "tenant_id")?)
        .bind(uint64_to_int64(organization_id, "organization_id")?)
        .bind(subject_kind_value)
        .bind(uint64_to_int64(subject_id, "subject_id")?)
        .bind(params.page_size)
        .bind(params.offset)
        .fetch_all(pool)
        .await
        .map_err(map_sqlx)?;
    page(rows, row_to_installation)
}

fn package_market_status(status: SkillLifecycleStatus) -> &'static str {
    match status {
        SkillLifecycleStatus::Draft => "draft",
        SkillLifecycleStatus::Active => "published",
        SkillLifecycleStatus::Disabled => "disabled",
        SkillLifecycleStatus::Archived => "archived",
        SkillLifecycleStatus::Deleted => "removed",
    }
}

fn package_review_status(status: SkillLifecycleStatus) -> &'static str {
    if status == SkillLifecycleStatus::Active {
        "approved"
    } else {
        "pending"
    }
}
