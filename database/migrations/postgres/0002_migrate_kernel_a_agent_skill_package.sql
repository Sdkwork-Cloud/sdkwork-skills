-- Migrate legacy sdkwork-kernel a_agent_skill_package into ai_agent_skill_package
-- Run only when source table exists in the same database during platform cutover.

INSERT INTO ai_agent_skill_package (
    uuid,
    tenant_id,
    organization_id,
    owner_user_id,
    skill_id,
    package_key,
    code,
    display_name,
    summary,
    description,
    invocation_kind,
    package_ref,
    entrypoint,
    input_schema_json,
    output_schema_json,
    capability_ids_json,
    categories_json,
    tags_json,
    security_profile_id,
    status,
    visibility,
    version,
    created_at,
    updated_at,
    deleted_at
)
SELECT
    legacy.uuid,
    legacy.tenant_id,
    legacy.organization_id,
    legacy.owner_user_id,
    legacy.skill_id,
    legacy.code AS package_key,
    legacy.code,
    legacy.display_name,
    NULL AS summary,
    legacy.description,
    legacy.invocation_kind,
    legacy.package_ref,
    legacy.entrypoint,
    legacy.input_schema_json,
    legacy.output_schema_json,
    legacy.capability_ids_json,
    legacy.categories_json,
    legacy.tags_json,
    legacy.security_profile_id,
    legacy.status,
    legacy.visibility,
    legacy.version,
    legacy.created_at,
    legacy.updated_at,
    legacy.deleted_at
FROM a_agent_skill_package AS legacy
WHERE NOT EXISTS (
    SELECT 1
    FROM ai_agent_skill_package AS target
    WHERE target.tenant_id = legacy.tenant_id
      AND target.skill_id = legacy.skill_id
);

INSERT INTO ai_agent_skill (
    uuid,
    tenant_id,
    organization_id,
    owner_user_id,
    skill_key,
    package_id,
    name,
    summary,
    description,
    runtime,
    entrypoint,
    source_type,
    market_status,
    visibility,
    review_status,
    tags_json,
    capabilities_json,
    config_schema_json,
    default_config_json,
    version,
    created_at,
    updated_at,
    deleted_at
)
SELECT
    pkg.uuid || ':skill' AS uuid,
    pkg.tenant_id,
    pkg.organization_id,
    pkg.owner_user_id,
    pkg.skill_id AS skill_key,
    pkg.id AS package_id,
    pkg.display_name AS name,
    pkg.summary,
    pkg.description,
    pkg.invocation_kind AS runtime,
    pkg.entrypoint,
    'package' AS source_type,
    CASE WHEN pkg.deleted_at IS NULL THEN 'published' ELSE 'archived' END AS market_status,
    CASE pkg.visibility
        WHEN 0 THEN 'private'
        WHEN 1 THEN 'tenant'
        WHEN 2 THEN 'organization'
        ELSE 'public'
    END AS visibility,
    'approved' AS review_status,
    pkg.tags_json,
    pkg.capability_ids_json,
    pkg.input_schema_json,
    '{}'::text,
    pkg.version,
    pkg.created_at,
    pkg.updated_at,
    pkg.deleted_at
FROM ai_agent_skill_package AS pkg
WHERE NOT EXISTS (
    SELECT 1
    FROM ai_agent_skill AS skill
    WHERE skill.tenant_id = pkg.tenant_id
      AND skill.skill_key = pkg.skill_id
);
