use sdkwork_database_id::SnowflakeIdGenerator;
use sdkwork_intelligence_skills_service::{SkillsResult, SkillsServiceError};
use sdkwork_skills_contract::{
    SkillArtifactRecord, SkillCapabilityRecord, SkillCategoryRecord, SkillInstallationRecord,
    SkillLifecycleStatus, SkillPackageRecord, SkillRecord,
};
use sdkwork_utils_rust::{OffsetListPageParams, LIST_TOTAL_SQL_COLUMN};
use sqlx::{Row, Sqlite, SqlitePool, Transaction};

use crate::json_util::{
    json_value_from_text, json_value_to_text, string_list_from_json, string_list_to_json,
};
use crate::support::{
    artifact_status, capability_risk, invocation, lifecycle, map_sqlx, new_uuid, next_id,
    search_pattern, subject_kind, visibility,
};

fn row_to_package(row: &sqlx::sqlite::SqliteRow) -> SkillsResult<SkillPackageRecord> {
    Ok(SkillPackageRecord {
        id: row.try_get::<i64, _>("id").map_err(map_sqlx)? as u64,
        uuid: row.try_get("uuid").map_err(map_sqlx)?,
        tenant_id: row.try_get::<i64, _>("tenant_id").map_err(map_sqlx)? as u64,
        organization_id: row.try_get::<i64, _>("organization_id").map_err(map_sqlx)? as u64,
        owner_user_id: row.try_get::<i64, _>("owner_user_id").map_err(map_sqlx)? as u64,
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
        status: lifecycle(row.try_get::<i64, _>("status").map_err(map_sqlx)? as i16)?,
        visibility: visibility(row.try_get::<i64, _>("visibility").map_err(map_sqlx)? as i16)?,
        featured: row.try_get::<i64, _>("featured").map_err(map_sqlx)? != 0,
        sort_weight: row.try_get("sort_weight").map_err(map_sqlx)?,
        version: row.try_get::<i64, _>("version").map_err(map_sqlx)? as u64,
        created_at: row.try_get("created_at").map_err(map_sqlx)?,
        updated_at: row.try_get("updated_at").map_err(map_sqlx)?,
        deleted_at: row.try_get("deleted_at").map_err(map_sqlx)?,
    })
}

fn row_to_skill(row: &sqlx::sqlite::SqliteRow) -> SkillsResult<SkillRecord> {
    Ok(SkillRecord {
        id: row.try_get::<i64, _>("id").map_err(map_sqlx)? as u64,
        uuid: row.try_get("uuid").map_err(map_sqlx)?,
        tenant_id: row.try_get::<i64, _>("tenant_id").map_err(map_sqlx)? as u64,
        organization_id: row.try_get::<i64, _>("organization_id").map_err(map_sqlx)? as u64,
        skill_key: row.try_get("skill_key").map_err(map_sqlx)?,
        package_id: row.try_get::<i64, _>("package_id").map_err(map_sqlx)? as u64,
        name: row.try_get("display_name").map_err(map_sqlx)?,
        summary: row.try_get("summary").map_err(map_sqlx)?,
        description: row.try_get("description").map_err(map_sqlx)?,
        market_status: row.try_get("market_status").map_err(map_sqlx)?,
        visibility: visibility(row.try_get::<i64, _>("visibility").map_err(map_sqlx)? as i16)?,
        review_status: row.try_get("review_status").map_err(map_sqlx)?,
        categories: string_list_from_json(
            &row.try_get::<String, _>("category_codes_json")
                .map_err(map_sqlx)?,
            "category_codes_json",
        )?,
        enabled: row.try_get::<i64, _>("enabled").map_err(map_sqlx)? != 0,
        featured: row.try_get::<i64, _>("featured").map_err(map_sqlx)? != 0,
        install_count: row.try_get::<i64, _>("install_count").map_err(map_sqlx)? as u64,
        tags: string_list_from_json(
            &row.try_get::<String, _>("tags_json").map_err(map_sqlx)?,
            "tags_json",
        )?,
        version: row.try_get::<i64, _>("version").map_err(map_sqlx)? as u64,
        created_at: row.try_get("created_at").map_err(map_sqlx)?,
        updated_at: row.try_get("updated_at").map_err(map_sqlx)?,
        deleted_at: row.try_get("deleted_at").map_err(map_sqlx)?,
    })
}

fn row_to_category(row: &sqlx::sqlite::SqliteRow) -> SkillsResult<SkillCategoryRecord> {
    Ok(SkillCategoryRecord {
        id: row.try_get::<i64, _>("id").map_err(map_sqlx)? as u64,
        uuid: row.try_get("uuid").map_err(map_sqlx)?,
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
        permission_code: row.try_get("permission_code").map_err(map_sqlx)?,
        visible: row.try_get::<i64, _>("visible").map_err(map_sqlx)? != 0,
        status: row.try_get::<i64, _>("status").map_err(map_sqlx)? as i16,
        version: row.try_get::<i64, _>("version").map_err(map_sqlx)? as u64,
        created_at: row.try_get("created_at").map_err(map_sqlx)?,
        updated_at: row.try_get("updated_at").map_err(map_sqlx)?,
    })
}

fn row_to_capability(row: &sqlx::sqlite::SqliteRow) -> SkillsResult<SkillCapabilityRecord> {
    Ok(SkillCapabilityRecord {
        id: row.try_get::<i64, _>("id").map_err(map_sqlx)? as u64,
        uuid: row.try_get("uuid").map_err(map_sqlx)?,
        tenant_id: row.try_get::<i64, _>("tenant_id").map_err(map_sqlx)? as u64,
        organization_id: row.try_get::<i64, _>("organization_id").map_err(map_sqlx)? as u64,
        capability_key: row.try_get("capability_key").map_err(map_sqlx)?,
        display_name: row.try_get("display_name").map_err(map_sqlx)?,
        description: row.try_get("description").map_err(map_sqlx)?,
        risk_level: capability_risk(&row.try_get::<String, _>("risk_level").map_err(map_sqlx)?)?,
        status: row.try_get::<i64, _>("status").map_err(map_sqlx)? as i16,
        version: row.try_get::<i64, _>("version").map_err(map_sqlx)? as u64,
        created_at: row.try_get("created_at").map_err(map_sqlx)?,
        updated_at: row.try_get("updated_at").map_err(map_sqlx)?,
    })
}

fn row_to_artifact(row: &sqlx::sqlite::SqliteRow) -> SkillsResult<SkillArtifactRecord> {
    Ok(SkillArtifactRecord {
        id: row.try_get::<i64, _>("id").map_err(map_sqlx)? as u64,
        uuid: row.try_get("uuid").map_err(map_sqlx)?,
        tenant_id: row.try_get::<i64, _>("tenant_id").map_err(map_sqlx)? as u64,
        package_id: row.try_get::<i64, _>("package_id").map_err(map_sqlx)? as u64,
        version_label: row.try_get("version_label").map_err(map_sqlx)?,
        artifact_ref: row.try_get("artifact_ref").map_err(map_sqlx)?,
        checksum_sha256: row.try_get("checksum_sha256").map_err(map_sqlx)?,
        size_bytes: row
            .try_get::<Option<i64>, _>("size_bytes")
            .map_err(map_sqlx)?
            .map(|value| value as u64),
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
        published_at: row.try_get("published_at").map_err(map_sqlx)?,
        yanked_at: row.try_get("yanked_at").map_err(map_sqlx)?,
        created_at: row.try_get("created_at").map_err(map_sqlx)?,
    })
}

fn row_to_installation(row: &sqlx::sqlite::SqliteRow) -> SkillsResult<SkillInstallationRecord> {
    Ok(SkillInstallationRecord {
        id: row.try_get::<i64, _>("id").map_err(map_sqlx)? as u64,
        uuid: row.try_get("uuid").map_err(map_sqlx)?,
        tenant_id: row.try_get::<i64, _>("tenant_id").map_err(map_sqlx)? as u64,
        organization_id: row.try_get::<i64, _>("organization_id").map_err(map_sqlx)? as u64,
        subject_kind: subject_kind(&row.try_get::<String, _>("subject_kind").map_err(map_sqlx)?)?,
        subject_id: row.try_get::<i64, _>("subject_id").map_err(map_sqlx)? as u64,
        skill_id: row.try_get::<i64, _>("skill_id").map_err(map_sqlx)? as u64,
        package_id: row.try_get::<i64, _>("package_id").map_err(map_sqlx)? as u64,
        artifact_id: row.try_get::<i64, _>("artifact_id").map_err(map_sqlx)? as u64,
        installed_by_user_id: row
            .try_get::<i64, _>("installed_by_user_id")
            .map_err(map_sqlx)? as u64,
        install_status: row.try_get("install_status").map_err(map_sqlx)?,
        enabled: row.try_get::<i64, _>("enabled").map_err(map_sqlx)? != 0,
        config: json_value_from_text(
            &row.try_get::<String, _>("config_json").map_err(map_sqlx)?,
            "config_json",
        )?,
        version: row.try_get::<i64, _>("version").map_err(map_sqlx)? as u64,
        installed_at: row.try_get("installed_at").map_err(map_sqlx)?,
        updated_at: row.try_get("updated_at").map_err(map_sqlx)?,
    })
}

const PACKAGE_SELECT: &str = r#"
    SELECT p.id, p.uuid, p.tenant_id, p.organization_id, p.owner_user_id,
           s.skill_key, p.package_key, p.code, p.display_name, p.summary, p.description,
           COALESCE((
               SELECT json_group_array(category_code)
               FROM (
                   SELECT c.code AS category_code
                   FROM ai_skill_category_binding b
                   JOIN ai_skill_category c ON c.id = b.category_id AND c.deleted_at IS NULL
                   WHERE b.skill_id = s.id
                   ORDER BY c.code
               ) category_codes
           ), '[]') AS category_codes_json,
           p.tags_json, p.status, p.visibility, p.featured, p.sort_weight, p.version,
           p.created_at, p.updated_at, p.deleted_at
    FROM ai_agent_skill_package p
    JOIN ai_agent_skill s ON s.package_id = p.id AND s.deleted_at IS NULL
"#;

const SKILL_SELECT: &str = r#"
    SELECT s.id, s.uuid, s.tenant_id, s.organization_id, s.skill_key, s.package_id,
           p.display_name, p.summary, p.description, s.market_status, p.visibility,
           s.review_status,
           COALESCE((
               SELECT json_group_array(category_code)
               FROM (
                   SELECT c.code AS category_code
                   FROM ai_skill_category_binding b
                   JOIN ai_skill_category c ON c.id = b.category_id AND c.deleted_at IS NULL
                   WHERE b.skill_id = s.id
                   ORDER BY c.code
               ) category_codes
           ), '[]') AS category_codes_json,
           s.enabled, s.featured, s.install_count, p.tags_json, s.version,
           s.created_at, s.updated_at, s.deleted_at
    FROM ai_agent_skill s
    JOIN ai_agent_skill_package p ON p.id = s.package_id AND p.deleted_at IS NULL
"#;

const ARTIFACT_SELECT: &str = r#"
    SELECT a.id, a.uuid, a.tenant_id, a.package_id, a.version_label, a.artifact_ref,
           a.checksum_sha256, a.size_bytes, a.invocation_kind, a.entrypoint,
           a.input_schema_json, a.output_schema_json, a.config_schema_json,
           a.default_config_json, a.security_profile_id, a.status,
           COALESCE((
               SELECT json_group_array(capability_key)
               FROM (
                   SELECT c.capability_key AS capability_key
                   FROM ai_skill_artifact_capability ac
                   JOIN ai_skill_capability c ON c.id = ac.capability_id AND c.deleted_at IS NULL
                   WHERE ac.artifact_id = a.id
                   ORDER BY c.capability_key
               ) capability_keys
           ), '[]') AS capability_keys_json,
           a.published_at, a.yanked_at, a.created_at
    FROM ai_skill_artifact a
"#;

pub async fn list_skill_packages_page(
    pool: &SqlitePool,
    tenant_id: u64,
    params: OffsetListPageParams,
    keyword: Option<&str>,
) -> SkillsResult<(Vec<SkillPackageRecord>, i64)> {
    let sql = format!(
        "SELECT package_rows.*, COUNT(*) OVER() AS {LIST_TOTAL_SQL_COLUMN}
         FROM ({PACKAGE_SELECT}
               WHERE p.tenant_id = ?1 AND p.deleted_at IS NULL AND p.status <> 4
                 AND (?2 = '%' OR LOWER(p.display_name) LIKE LOWER(?2) ESCAPE '\\'
                      OR LOWER(p.package_key) LIKE LOWER(?2) ESCAPE '\\'
                      OR LOWER(p.code) LIKE LOWER(?2) ESCAPE '\\')
         ) package_rows
         ORDER BY featured DESC, sort_weight DESC, updated_at DESC, code ASC LIMIT ?3 OFFSET ?4"
    );
    let rows = sqlx::query(&sql)
        .bind(tenant_id as i64)
        .bind(search_pattern(keyword))
        .bind(params.page_size)
        .bind(params.offset)
        .fetch_all(pool)
        .await
        .map_err(map_sqlx)?;
    page(rows, row_to_package)
}

pub async fn get_skill_package(
    pool: &SqlitePool,
    tenant_id: u64,
    package_id: u64,
) -> SkillsResult<SkillPackageRecord> {
    let sql = format!(
        "{PACKAGE_SELECT} WHERE p.tenant_id = ?1 AND p.id = ?2 AND p.deleted_at IS NULL LIMIT 1"
    );
    let row = sqlx::query(&sql)
        .bind(tenant_id as i64)
        .bind(package_id as i64)
        .fetch_optional(pool)
        .await
        .map_err(map_sqlx)?;
    row.as_ref()
        .map(row_to_package)
        .transpose()?
        .ok_or_else(|| SkillsServiceError::NotFound(format!("skill package {package_id}")))
}

pub async fn list_marketplace_skill_packages_page(
    pool: &SqlitePool,
    tenant_id: u64,
    organization_id: u64,
    user_id: u64,
    params: OffsetListPageParams,
    keyword: Option<&str>,
) -> SkillsResult<(Vec<SkillPackageRecord>, i64)> {
    let sql = format!(
        "SELECT package_rows.*, COUNT(*) OVER() AS {LIST_TOTAL_SQL_COLUMN}
         FROM ({PACKAGE_SELECT}
               WHERE p.tenant_id = ?1 AND p.deleted_at IS NULL AND p.status = 1
                 AND (p.visibility IN (1, 3)
                      OR (p.visibility = 2 AND p.organization_id = ?2)
                      OR (p.visibility = 0 AND p.owner_user_id = ?3))
                 AND (?4 = '%' OR LOWER(p.display_name) LIKE LOWER(?4) ESCAPE '\\'
                      OR LOWER(p.package_key) LIKE LOWER(?4) ESCAPE '\\'
                      OR LOWER(p.code) LIKE LOWER(?4) ESCAPE '\\')
         ) package_rows
         ORDER BY featured DESC, sort_weight DESC, updated_at DESC, code ASC LIMIT ?5 OFFSET ?6"
    );
    let rows = sqlx::query(&sql)
        .bind(tenant_id as i64)
        .bind(organization_id as i64)
        .bind(user_id as i64)
        .bind(search_pattern(keyword))
        .bind(params.page_size)
        .bind(params.offset)
        .fetch_all(pool)
        .await
        .map_err(map_sqlx)?;
    page(rows, row_to_package)
}

pub async fn get_marketplace_skill_package(
    pool: &SqlitePool,
    tenant_id: u64,
    organization_id: u64,
    user_id: u64,
    package_id: u64,
) -> SkillsResult<SkillPackageRecord> {
    let sql = format!(
        "{PACKAGE_SELECT}
         WHERE p.tenant_id = ?1 AND p.id = ?2 AND p.status = 1 AND p.deleted_at IS NULL
           AND (p.visibility IN (1, 3)
                OR (p.visibility = 2 AND p.organization_id = ?3)
                OR (p.visibility = 0 AND p.owner_user_id = ?4))
         LIMIT 1"
    );
    let row = sqlx::query(&sql)
        .bind(tenant_id as i64)
        .bind(package_id as i64)
        .bind(organization_id as i64)
        .bind(user_id as i64)
        .fetch_optional(pool)
        .await
        .map_err(map_sqlx)?;
    row.as_ref()
        .map(row_to_package)
        .transpose()?
        .ok_or_else(|| SkillsServiceError::NotFound(format!("skill package {package_id}")))
}

pub async fn list_skills_page(
    pool: &SqlitePool,
    tenant_id: u64,
    organization_id: u64,
    user_id: u64,
    params: OffsetListPageParams,
    keyword: Option<&str>,
) -> SkillsResult<(Vec<SkillRecord>, i64)> {
    let sql = format!(
        "SELECT skill_rows.*, COUNT(*) OVER() AS {LIST_TOTAL_SQL_COLUMN}
         FROM ({SKILL_SELECT}
               WHERE s.tenant_id = ?1 AND s.deleted_at IS NULL AND s.enabled = 1
                 AND s.market_status = 'published' AND s.review_status = 'approved'
                 AND (p.visibility IN (1, 3)
                      OR (p.visibility = 2 AND p.organization_id = ?2)
                      OR (p.visibility = 0 AND p.owner_user_id = ?3))
                 AND (?4 = '%' OR LOWER(p.display_name) LIKE LOWER(?4) ESCAPE '\\'
                      OR LOWER(s.skill_key) LIKE LOWER(?4) ESCAPE '\\'
                      OR LOWER(COALESCE(p.summary, '')) LIKE LOWER(?4) ESCAPE '\\')
         ) skill_rows
         ORDER BY featured DESC, updated_at DESC, skill_key ASC LIMIT ?5 OFFSET ?6"
    );
    let rows = sqlx::query(&sql)
        .bind(tenant_id as i64)
        .bind(organization_id as i64)
        .bind(user_id as i64)
        .bind(search_pattern(keyword))
        .bind(params.page_size)
        .bind(params.offset)
        .fetch_all(pool)
        .await
        .map_err(map_sqlx)?;
    page(rows, row_to_skill)
}

pub async fn get_skill(
    pool: &SqlitePool,
    tenant_id: u64,
    organization_id: u64,
    user_id: u64,
    skill_key: &str,
) -> SkillsResult<SkillRecord> {
    let sql = format!(
        "{SKILL_SELECT}
         WHERE s.tenant_id = ?1 AND s.skill_key = ?2 AND s.deleted_at IS NULL
           AND s.enabled = 1 AND s.market_status = 'published' AND s.review_status = 'approved'
           AND (p.visibility IN (1, 3)
                OR (p.visibility = 2 AND p.organization_id = ?3)
                OR (p.visibility = 0 AND p.owner_user_id = ?4))
         LIMIT 1"
    );
    let row = sqlx::query(&sql)
        .bind(tenant_id as i64)
        .bind(skill_key)
        .bind(organization_id as i64)
        .bind(user_id as i64)
        .fetch_optional(pool)
        .await
        .map_err(map_sqlx)?;
    row.as_ref()
        .map(row_to_skill)
        .transpose()?
        .ok_or_else(|| SkillsServiceError::NotFound(skill_key.to_string()))
}

fn page<T>(
    rows: Vec<sqlx::sqlite::SqliteRow>,
    mapper: fn(&sqlx::sqlite::SqliteRow) -> SkillsResult<T>,
) -> SkillsResult<(Vec<T>, i64)> {
    let total = rows
        .first()
        .and_then(|row| row.try_get::<i64, _>(LIST_TOTAL_SQL_COLUMN).ok())
        .unwrap_or(0);
    let items = rows.iter().map(mapper).collect::<SkillsResult<Vec<_>>>()?;
    Ok((items, total))
}

pub async fn get_category(
    pool: &SqlitePool,
    tenant_id: u64,
    category_id: u64,
) -> SkillsResult<SkillCategoryRecord> {
    let row = sqlx::query(
        "SELECT id, uuid, tenant_id, organization_id, category_type, code, name, description,
                parent_id, sort_weight, permission_code, visible, status, version,
                created_at, updated_at
         FROM ai_skill_category
         WHERE id=?1 AND tenant_id IN (0,?2) AND deleted_at IS NULL LIMIT 1",
    )
    .bind(category_id as i64)
    .bind(tenant_id as i64)
    .fetch_optional(pool)
    .await
    .map_err(map_sqlx)?;
    row.as_ref()
        .map(row_to_category)
        .transpose()?
        .ok_or_else(|| SkillsServiceError::NotFound(format!("skill category {category_id}")))
}

pub async fn list_categories_page(
    pool: &SqlitePool,
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
         FROM ai_skill_category
         WHERE category_type = ?1 AND tenant_id IN (0, ?2) AND deleted_at IS NULL
           AND (?3 = '%' OR LOWER(name) LIKE LOWER(?3) ESCAPE '\\'
                OR LOWER(code) LIKE LOWER(?3) ESCAPE '\\')
         ORDER BY sort_weight ASC, code ASC LIMIT ?4 OFFSET ?5"
    );
    let rows = sqlx::query(&sql)
        .bind(category_type)
        .bind(tenant_id as i64)
        .bind(search_pattern(keyword))
        .bind(params.page_size)
        .bind(params.offset)
        .fetch_all(pool)
        .await
        .map_err(map_sqlx)?;
    page(rows, row_to_category)
}

pub async fn upsert_category(
    pool: &SqlitePool,
    id_generator: &SnowflakeIdGenerator,
    record: SkillCategoryRecord,
) -> SkillsResult<SkillCategoryRecord> {
    let row = if record.id == 0 {
        sqlx::query(
            "INSERT INTO ai_skill_category (
                 id, uuid, tenant_id, organization_id, category_type, code, name, description,
                 parent_id, sort_weight, permission_code, visible, status, version
             ) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,1)
             RETURNING id, uuid, tenant_id, organization_id, category_type, code, name,
                       description, parent_id, sort_weight, permission_code, visible, status,
                       version, created_at, updated_at",
        )
        .bind(next_id(id_generator)?)
        .bind(new_uuid())
        .bind(record.tenant_id as i64)
        .bind(record.organization_id as i64)
        .bind(&record.category_type)
        .bind(&record.code)
        .bind(&record.name)
        .bind(&record.description)
        .bind(record.parent_id.map(|value| value as i64))
        .bind(record.sort_weight)
        .bind(&record.permission_code)
        .bind(i64::from(record.visible))
        .bind(record.status)
        .fetch_one(pool)
        .await
        .map_err(map_sqlx)?
    } else {
        sqlx::query(
            "UPDATE ai_skill_category SET name=?4, description=?5, parent_id=?6,
                    sort_weight=?7, permission_code=?8, visible=?9, status=?10,
                    version=version+1,
                    updated_at=strftime('%Y-%m-%dT%H:%M:%fZ', 'now')
             WHERE id=?1 AND tenant_id IN (0,?2) AND version=?3 AND deleted_at IS NULL
             RETURNING id, uuid, tenant_id, organization_id, category_type, code, name,
                       description, parent_id, sort_weight, permission_code, visible, status,
                       version, created_at, updated_at",
        )
        .bind(record.id as i64)
        .bind(record.tenant_id as i64)
        .bind(record.version as i64)
        .bind(&record.name)
        .bind(&record.description)
        .bind(record.parent_id.map(|value| value as i64))
        .bind(record.sort_weight)
        .bind(&record.permission_code)
        .bind(i64::from(record.visible))
        .bind(record.status)
        .fetch_optional(pool)
        .await
        .map_err(map_sqlx)?
        .ok_or_else(|| SkillsServiceError::Conflict("category version changed".to_string()))?
    };
    row_to_category(&row)
}

pub async fn list_capabilities_page(
    pool: &SqlitePool,
    tenant_id: u64,
    params: OffsetListPageParams,
    keyword: Option<&str>,
) -> SkillsResult<(Vec<SkillCapabilityRecord>, i64)> {
    let sql = format!(
        "SELECT id, uuid, tenant_id, organization_id, capability_key, display_name,
                description, risk_level, status, version, created_at, updated_at,
                COUNT(*) OVER() AS {LIST_TOTAL_SQL_COLUMN}
         FROM ai_skill_capability
         WHERE tenant_id IN (0,?1) AND deleted_at IS NULL
           AND (?2='%' OR LOWER(capability_key) LIKE LOWER(?2) ESCAPE '\\'
                OR LOWER(display_name) LIKE LOWER(?2) ESCAPE '\\')
         ORDER BY capability_key ASC LIMIT ?3 OFFSET ?4"
    );
    let rows = sqlx::query(&sql)
        .bind(tenant_id as i64)
        .bind(search_pattern(keyword))
        .bind(params.page_size)
        .bind(params.offset)
        .fetch_all(pool)
        .await
        .map_err(map_sqlx)?;
    page(rows, row_to_capability)
}

pub async fn get_capability(
    pool: &SqlitePool,
    tenant_id: u64,
    capability_id: u64,
) -> SkillsResult<SkillCapabilityRecord> {
    let row = sqlx::query(
        "SELECT id, uuid, tenant_id, organization_id, capability_key, display_name,
                description, risk_level, status, version, created_at, updated_at
         FROM ai_skill_capability
         WHERE id=?1 AND tenant_id IN (0,?2) AND deleted_at IS NULL LIMIT 1",
    )
    .bind(capability_id as i64)
    .bind(tenant_id as i64)
    .fetch_optional(pool)
    .await
    .map_err(map_sqlx)?;
    row.as_ref()
        .map(row_to_capability)
        .transpose()?
        .ok_or_else(|| SkillsServiceError::NotFound(format!("skill capability {capability_id}")))
}

pub async fn upsert_capability(
    pool: &SqlitePool,
    id_generator: &SnowflakeIdGenerator,
    record: SkillCapabilityRecord,
) -> SkillsResult<SkillCapabilityRecord> {
    let row = if record.id == 0 {
        sqlx::query(
            "INSERT INTO ai_skill_capability (
                 id, uuid, tenant_id, organization_id, capability_key, display_name,
                 description, risk_level, status, version
             ) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,1)
             RETURNING id, uuid, tenant_id, organization_id, capability_key, display_name,
                       description, risk_level, status, version, created_at, updated_at",
        )
        .bind(next_id(id_generator)?)
        .bind(new_uuid())
        .bind(record.tenant_id as i64)
        .bind(record.organization_id as i64)
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
            "UPDATE ai_skill_capability SET display_name=?4, description=?5, risk_level=?6,
                    status=?7, version=version+1,
                    updated_at=strftime('%Y-%m-%dT%H:%M:%fZ', 'now')
             WHERE id=?1 AND tenant_id IN (0,?2) AND version=?3 AND deleted_at IS NULL
             RETURNING id, uuid, tenant_id, organization_id, capability_key, display_name,
                       description, risk_level, status, version, created_at, updated_at",
        )
        .bind(record.id as i64)
        .bind(record.tenant_id as i64)
        .bind(record.version as i64)
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
    pool: &SqlitePool,
    tenant_id: u64,
    package_id: u64,
    params: OffsetListPageParams,
) -> SkillsResult<(Vec<SkillArtifactRecord>, i64)> {
    let sql = format!(
        "SELECT artifact_rows.*, COUNT(*) OVER() AS {LIST_TOTAL_SQL_COLUMN}
         FROM ({ARTIFACT_SELECT} WHERE a.tenant_id=?1 AND a.package_id=?2) artifact_rows
         ORDER BY created_at DESC, id DESC LIMIT ?3 OFFSET ?4"
    );
    let rows = sqlx::query(&sql)
        .bind(tenant_id as i64)
        .bind(package_id as i64)
        .bind(params.page_size)
        .bind(params.offset)
        .fetch_all(pool)
        .await
        .map_err(map_sqlx)?;
    page(rows, row_to_artifact)
}

pub async fn list_installable_artifacts_page(
    pool: &SqlitePool,
    tenant_id: u64,
    organization_id: u64,
    user_id: u64,
    package_id: u64,
    params: OffsetListPageParams,
) -> SkillsResult<(Vec<SkillArtifactRecord>, i64)> {
    let sql = format!(
        "SELECT artifact_rows.*, COUNT(*) OVER() AS {LIST_TOTAL_SQL_COLUMN}
         FROM ({ARTIFACT_SELECT}
               JOIN ai_agent_skill_package p
                 ON p.id=a.package_id AND p.tenant_id=a.tenant_id AND p.deleted_at IS NULL
               JOIN ai_agent_skill s
                 ON s.package_id=p.id AND s.tenant_id=p.tenant_id AND s.deleted_at IS NULL
               WHERE a.tenant_id=?1 AND a.package_id=?2 AND a.status='published'
                 AND p.status=1
                 AND s.enabled=1 AND s.market_status='published' AND s.review_status='approved'
                 AND (p.visibility IN (1, 3)
                      OR (p.visibility=2 AND p.organization_id=?3)
                      OR (p.visibility=0 AND p.owner_user_id=?4))
         ) artifact_rows
         ORDER BY published_at DESC, created_at DESC, id DESC LIMIT ?5 OFFSET ?6"
    );
    let rows = sqlx::query(&sql)
        .bind(tenant_id as i64)
        .bind(package_id as i64)
        .bind(organization_id as i64)
        .bind(user_id as i64)
        .bind(params.page_size)
        .bind(params.offset)
        .fetch_all(pool)
        .await
        .map_err(map_sqlx)?;
    page(rows, row_to_artifact)
}

pub async fn create_artifact(
    pool: &SqlitePool,
    id_generator: &SnowflakeIdGenerator,
    artifact: SkillArtifactRecord,
) -> SkillsResult<SkillArtifactRecord> {
    let mut tx = pool.begin().await.map_err(map_sqlx)?;
    ensure_package(&mut tx, artifact.tenant_id, artifact.package_id).await?;
    let artifact_id = insert_artifact(&mut tx, id_generator, artifact).await?;
    tx.commit().await.map_err(map_sqlx)?;
    get_artifact(pool, artifact_id as u64).await
}

async fn get_artifact(pool: &SqlitePool, artifact_id: u64) -> SkillsResult<SkillArtifactRecord> {
    let sql = format!("{ARTIFACT_SELECT} WHERE a.id=?1 LIMIT 1");
    let row = sqlx::query(&sql)
        .bind(artifact_id as i64)
        .fetch_optional(pool)
        .await
        .map_err(map_sqlx)?;
    row.as_ref()
        .map(row_to_artifact)
        .transpose()?
        .ok_or_else(|| SkillsServiceError::NotFound(format!("skill artifact {artifact_id}")))
}

pub async fn create_skill_package(
    pool: &SqlitePool,
    id_generator: &SnowflakeIdGenerator,
    record: SkillPackageRecord,
    mut initial_artifact: SkillArtifactRecord,
) -> SkillsResult<SkillPackageRecord> {
    let mut tx = pool.begin().await.map_err(map_sqlx)?;
    let tags_json = string_list_to_json(&record.tags, "tags")?;
    let package_id = next_id(id_generator)?;
    let skill_id = next_id(id_generator)?;
    sqlx::query(
        "INSERT INTO ai_agent_skill_package (
             id, uuid, tenant_id, organization_id, owner_user_id, package_key, code,
             display_name, summary, description, tags_json, status, visibility,
             featured, sort_weight, version
         ) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,?15,1)",
    )
    .bind(package_id)
    .bind(new_uuid())
    .bind(record.tenant_id as i64)
    .bind(record.organization_id as i64)
    .bind(record.owner_user_id as i64)
    .bind(&record.package_key)
    .bind(&record.code)
    .bind(&record.display_name)
    .bind(&record.summary)
    .bind(&record.description)
    .bind(tags_json)
    .bind(record.status.as_db_code())
    .bind(record.visibility.as_db_code())
    .bind(i64::from(record.featured))
    .bind(record.sort_weight)
    .execute(&mut *tx)
    .await
    .map_err(map_sqlx)?;
    sqlx::query(
        "INSERT INTO ai_agent_skill (
             id, uuid, tenant_id, organization_id, skill_key, package_id, market_status,
             review_status, enabled, featured, version
         ) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,1)",
    )
    .bind(skill_id)
    .bind(new_uuid())
    .bind(record.tenant_id as i64)
    .bind(record.organization_id as i64)
    .bind(&record.skill_key)
    .bind(package_id)
    .bind(package_market_status(record.status))
    .bind(package_review_status(record.status))
    .bind(i64::from(record.status == SkillLifecycleStatus::Active))
    .bind(i64::from(record.featured))
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
    initial_artifact.package_id = package_id as u64;
    insert_artifact(&mut tx, id_generator, initial_artifact).await?;
    tx.commit().await.map_err(map_sqlx)?;
    get_skill_package(pool, record.tenant_id, package_id as u64).await
}

pub async fn update_skill_package(
    pool: &SqlitePool,
    id_generator: &SnowflakeIdGenerator,
    record: SkillPackageRecord,
) -> SkillsResult<SkillPackageRecord> {
    let mut tx = pool.begin().await.map_err(map_sqlx)?;
    let tags_json = string_list_to_json(&record.tags, "tags")?;
    let updated = sqlx::query(
        "UPDATE ai_agent_skill_package SET display_name=?4, summary=?5, description=?6,
                tags_json=?7, status=?8, visibility=?9, featured=?10, sort_weight=?11,
                version=version+1, updated_at=strftime('%Y-%m-%dT%H:%M:%fZ', 'now')
         WHERE id=?1 AND tenant_id=?2 AND version=?3 AND deleted_at IS NULL",
    )
    .bind(record.id as i64)
    .bind(record.tenant_id as i64)
    .bind(record.version as i64)
    .bind(&record.display_name)
    .bind(&record.summary)
    .bind(&record.description)
    .bind(tags_json)
    .bind(record.status.as_db_code())
    .bind(record.visibility.as_db_code())
    .bind(i64::from(record.featured))
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
        "UPDATE ai_agent_skill SET market_status=?3, review_status=?4, enabled=?5, featured=?6,
                version=version+1, updated_at=strftime('%Y-%m-%dT%H:%M:%fZ', 'now')
         WHERE package_id=?1 AND tenant_id=?2 AND deleted_at IS NULL RETURNING id",
    )
    .bind(record.id as i64)
    .bind(record.tenant_id as i64)
    .bind(package_market_status(record.status))
    .bind(package_review_status(record.status))
    .bind(i64::from(record.status == SkillLifecycleStatus::Active))
    .bind(i64::from(record.featured))
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
    tx: &mut Transaction<'_, Sqlite>,
    id_generator: &SnowflakeIdGenerator,
    tenant_id: u64,
    skill_id: i64,
    category_codes: &[String],
) -> SkillsResult<()> {
    sqlx::query("DELETE FROM ai_skill_category_binding WHERE skill_id=?1")
        .bind(skill_id)
        .execute(&mut **tx)
        .await
        .map_err(map_sqlx)?;
    for code in category_codes {
        let category_id = sqlx::query_scalar::<_, i64>(
            "SELECT id FROM ai_skill_category
             WHERE tenant_id IN (0,?1) AND code=?2 AND category_type='skill_market'
               AND status=1 AND deleted_at IS NULL
             ORDER BY tenant_id DESC LIMIT 1",
        )
        .bind(tenant_id as i64)
        .bind(code)
        .fetch_optional(&mut **tx)
        .await
        .map_err(map_sqlx)?
        .ok_or_else(|| SkillsServiceError::InvalidArgument(format!("unknown category: {code}")))?;
        sqlx::query(
            "INSERT INTO ai_skill_category_binding (id, tenant_id, skill_id, category_id)
             VALUES (?1,?2,?3,?4)",
        )
        .bind(next_id(id_generator)?)
        .bind(tenant_id as i64)
        .bind(skill_id)
        .bind(category_id)
        .execute(&mut **tx)
        .await
        .map_err(map_sqlx)?;
    }
    Ok(())
}

async fn ensure_package(
    tx: &mut Transaction<'_, Sqlite>,
    tenant_id: u64,
    package_id: u64,
) -> SkillsResult<()> {
    let exists = sqlx::query_scalar::<_, i64>(
        "SELECT EXISTS(SELECT 1 FROM ai_agent_skill_package
         WHERE id=?1 AND tenant_id=?2 AND deleted_at IS NULL)",
    )
    .bind(package_id as i64)
    .bind(tenant_id as i64)
    .fetch_one(&mut **tx)
    .await
    .map_err(map_sqlx)?;
    if exists == 0 {
        return Err(SkillsServiceError::NotFound(format!(
            "skill package {package_id}"
        )));
    }
    Ok(())
}

async fn insert_artifact(
    tx: &mut Transaction<'_, Sqlite>,
    id_generator: &SnowflakeIdGenerator,
    artifact: SkillArtifactRecord,
) -> SkillsResult<i64> {
    let artifact_id = next_id(id_generator)?;
    let input_schema_json = json_value_to_text(&artifact.input_schema, "input_schema")?;
    let output_schema_json = json_value_to_text(&artifact.output_schema, "output_schema")?;
    let config_schema_json = json_value_to_text(&artifact.config_schema, "config_schema")?;
    let default_config_json = json_value_to_text(&artifact.default_config, "default_config")?;
    sqlx::query(
        "INSERT INTO ai_skill_artifact (
             id, uuid, tenant_id, package_id, version_label, artifact_ref, checksum_sha256,
             size_bytes, invocation_kind, entrypoint, input_schema_json, output_schema_json,
             config_schema_json, default_config_json, security_profile_id, status, published_at
         ) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,?15,?16,
                   CASE WHEN ?16='published'
                        THEN strftime('%Y-%m-%dT%H:%M:%fZ', 'now') ELSE NULL END)",
    )
    .bind(artifact_id)
    .bind(new_uuid())
    .bind(artifact.tenant_id as i64)
    .bind(artifact.package_id as i64)
    .bind(&artifact.version_label)
    .bind(&artifact.artifact_ref)
    .bind(&artifact.checksum_sha256)
    .bind(artifact.size_bytes.map(|value| value as i64))
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
            "SELECT id FROM ai_skill_capability
             WHERE tenant_id IN (0,?1) AND capability_key=?2 AND status=1 AND deleted_at IS NULL
             ORDER BY tenant_id DESC LIMIT 1",
        )
        .bind(artifact.tenant_id as i64)
        .bind(&key)
        .fetch_optional(&mut **tx)
        .await
        .map_err(map_sqlx)?
        .ok_or_else(|| SkillsServiceError::InvalidArgument(format!("unknown capability: {key}")))?;
        sqlx::query(
            "INSERT INTO ai_skill_artifact_capability
             (id, tenant_id, artifact_id, capability_id, required) VALUES (?1,?2,?3,?4,1)",
        )
        .bind(next_id(id_generator)?)
        .bind(artifact.tenant_id as i64)
        .bind(artifact_id)
        .bind(capability_id)
        .execute(&mut **tx)
        .await
        .map_err(map_sqlx)?;
    }
    Ok(artifact_id)
}

pub async fn delete_skill_package(
    pool: &SqlitePool,
    tenant_id: u64,
    package_id: u64,
) -> SkillsResult<()> {
    let mut tx = pool.begin().await.map_err(map_sqlx)?;
    let result = sqlx::query(
        "UPDATE ai_agent_skill_package SET status=4, version=version+1,
                deleted_at=strftime('%Y-%m-%dT%H:%M:%fZ', 'now'),
                updated_at=strftime('%Y-%m-%dT%H:%M:%fZ', 'now')
         WHERE id=?1 AND tenant_id=?2 AND deleted_at IS NULL",
    )
    .bind(package_id as i64)
    .bind(tenant_id as i64)
    .execute(&mut *tx)
    .await
    .map_err(map_sqlx)?;
    if result.rows_affected() != 1 {
        return Err(SkillsServiceError::NotFound(format!(
            "skill package {package_id}"
        )));
    }
    sqlx::query(
        "UPDATE ai_agent_skill SET market_status='removed', enabled=0, version=version+1,
                deleted_at=strftime('%Y-%m-%dT%H:%M:%fZ', 'now'),
                updated_at=strftime('%Y-%m-%dT%H:%M:%fZ', 'now')
         WHERE package_id=?1 AND tenant_id=?2 AND deleted_at IS NULL",
    )
    .bind(package_id as i64)
    .bind(tenant_id as i64)
    .execute(&mut *tx)
    .await
    .map_err(map_sqlx)?;
    sqlx::query(
        "UPDATE ai_skill_installation SET install_status='removed', enabled=0,
                version=version+1,
                deleted_at=strftime('%Y-%m-%dT%H:%M:%fZ', 'now'),
                updated_at=strftime('%Y-%m-%dT%H:%M:%fZ', 'now')
         WHERE package_id=?1 AND tenant_id=?2 AND deleted_at IS NULL",
    )
    .bind(package_id as i64)
    .bind(tenant_id as i64)
    .execute(&mut *tx)
    .await
    .map_err(map_sqlx)?;
    tx.commit().await.map_err(map_sqlx)
}

pub async fn install_skill(
    pool: &SqlitePool,
    id_generator: &SnowflakeIdGenerator,
    record: SkillInstallationRecord,
) -> SkillsResult<SkillInstallationRecord> {
    let mut tx = pool.begin().await.map_err(map_sqlx)?;
    let config_json = json_value_to_text(&record.config, "config")?;
    let skill_id = sqlx::query_scalar::<_, i64>(
        "SELECT s.id FROM ai_agent_skill s
         JOIN ai_agent_skill_package p ON p.id=s.package_id AND p.deleted_at IS NULL
         JOIN ai_skill_artifact a ON a.package_id=p.id AND a.tenant_id=p.tenant_id
         WHERE p.id=?1 AND p.tenant_id=?2 AND p.status=1
           AND a.id=?3 AND a.status='published'
           AND s.enabled=1 AND s.market_status='published' AND s.review_status='approved'
           AND s.deleted_at IS NULL LIMIT 1",
    )
    .bind(record.package_id as i64)
    .bind(record.tenant_id as i64)
    .bind(record.artifact_id as i64)
    .fetch_optional(&mut *tx)
    .await
    .map_err(map_sqlx)?
    .ok_or_else(|| {
        SkillsServiceError::InvalidArgument("artifact is not installable".to_string())
    })?;
    let existing_id = sqlx::query_scalar::<_, i64>(
        "SELECT id FROM ai_skill_installation
         WHERE tenant_id=?1 AND organization_id=?2 AND subject_kind=?3 AND subject_id=?4
           AND skill_id=?5 AND deleted_at IS NULL LIMIT 1",
    )
    .bind(record.tenant_id as i64)
    .bind(record.organization_id as i64)
    .bind(record.subject_kind.as_str())
    .bind(record.subject_id as i64)
    .bind(skill_id)
    .fetch_optional(&mut *tx)
    .await
    .map_err(map_sqlx)?;
    let installation_id = if let Some(id) = existing_id {
        sqlx::query(
            "UPDATE ai_skill_installation SET artifact_id=?2, installed_by_user_id=?3,
                    install_status='installed', enabled=1, config_json=?4, version=version+1,
                    updated_at=strftime('%Y-%m-%dT%H:%M:%fZ', 'now')
             WHERE id=?1",
        )
        .bind(id)
        .bind(record.artifact_id as i64)
        .bind(record.installed_by_user_id as i64)
        .bind(&config_json)
        .execute(&mut *tx)
        .await
        .map_err(map_sqlx)?;
        id
    } else {
        let id = next_id(id_generator)?;
        sqlx::query(
            "INSERT INTO ai_skill_installation (
                 id, uuid, tenant_id, organization_id, subject_kind, subject_id, skill_id,
                 package_id, artifact_id, installed_by_user_id, install_status, enabled,
                 config_json, version
             ) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,'installed',1,?11,1)",
        )
        .bind(id)
        .bind(new_uuid())
        .bind(record.tenant_id as i64)
        .bind(record.organization_id as i64)
        .bind(record.subject_kind.as_str())
        .bind(record.subject_id as i64)
        .bind(skill_id)
        .bind(record.package_id as i64)
        .bind(record.artifact_id as i64)
        .bind(record.installed_by_user_id as i64)
        .bind(&config_json)
        .execute(&mut *tx)
        .await
        .map_err(map_sqlx)?;
        sqlx::query("UPDATE ai_agent_skill SET install_count=install_count+1 WHERE id=?1")
            .bind(skill_id)
            .execute(&mut *tx)
            .await
            .map_err(map_sqlx)?;
        id
    };
    tx.commit().await.map_err(map_sqlx)?;
    get_installation(pool, installation_id as u64).await
}

async fn get_installation(
    pool: &SqlitePool,
    installation_id: u64,
) -> SkillsResult<SkillInstallationRecord> {
    let row = sqlx::query(
        "SELECT id, uuid, tenant_id, organization_id, subject_kind, subject_id, skill_id,
                package_id, artifact_id, installed_by_user_id, install_status, enabled,
                config_json, version, installed_at, updated_at
         FROM ai_skill_installation WHERE id=?1 AND deleted_at IS NULL",
    )
    .bind(installation_id as i64)
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
    pool: &SqlitePool,
    tenant_id: u64,
    organization_id: u64,
    subject_kind_value: &str,
    subject_id: u64,
    params: OffsetListPageParams,
) -> SkillsResult<(Vec<SkillInstallationRecord>, i64)> {
    let sql = format!(
        "SELECT id, uuid, tenant_id, organization_id, subject_kind, subject_id, skill_id,
                package_id, artifact_id, installed_by_user_id, install_status, enabled,
                config_json, version, installed_at, updated_at,
                COUNT(*) OVER() AS {LIST_TOTAL_SQL_COLUMN}
         FROM ai_skill_installation
         WHERE tenant_id=?1 AND organization_id=?2 AND subject_kind=?3 AND subject_id=?4
           AND deleted_at IS NULL
         ORDER BY updated_at DESC, id DESC LIMIT ?5 OFFSET ?6"
    );
    let rows = sqlx::query(&sql)
        .bind(tenant_id as i64)
        .bind(organization_id as i64)
        .bind(subject_kind_value)
        .bind(subject_id as i64)
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
