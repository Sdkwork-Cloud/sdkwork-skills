-- SDKWork Skills PostgreSQL baseline
-- Domain: intelligence (ai_* tables)
-- Migrated from sdkwork-kernel a_agent_skill_package (deprecated)

CREATE OR REPLACE FUNCTION sdkwork_skills_capabilities_json_is_standard(input TEXT)
RETURNS BOOLEAN
LANGUAGE plpgsql
IMMUTABLE
AS $$
DECLARE
    payload JSONB;
BEGIN
    payload := input::jsonb;
    IF jsonb_typeof(payload) <> 'array' THEN
        RETURN FALSE;
    END IF;
    RETURN NOT EXISTS (
        SELECT 1
        FROM jsonb_array_elements(payload) AS capability_values(value)
        WHERE NOT (
            jsonb_typeof(capability_values.value) = 'string'
            AND char_length(capability_values.value #>> '{}') <= 128
            AND (capability_values.value #>> '{}') ~ '^[a-z0-9_-]+(\.[a-z0-9_-]+)+$'
        )
    );
EXCEPTION WHEN others THEN
    RETURN FALSE;
END;
$$;

CREATE TABLE IF NOT EXISTS ai_skill_category (
    id BIGSERIAL PRIMARY KEY,
    uuid VARCHAR(96) NOT NULL,
    tenant_id BIGINT NOT NULL DEFAULT 0,
    organization_id BIGINT NOT NULL DEFAULT 0,
    category_type VARCHAR(64) NOT NULL,
    code VARCHAR(128) NOT NULL,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    parent_id BIGINT REFERENCES ai_skill_category(id),
    path TEXT,
    sort_weight INTEGER NOT NULL DEFAULT 0,
    permission_code VARCHAR(255) NOT NULL,
    visible SMALLINT NOT NULL DEFAULT 1,
    status SMALLINT NOT NULL DEFAULT 1,
    tags_json TEXT NOT NULL DEFAULT '[]',
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMPTZ,
    CONSTRAINT uk_ai_skill_category_uuid UNIQUE (uuid),
    CONSTRAINT uk_ai_skill_category_scope_code UNIQUE (tenant_id, organization_id, category_type, code),
    CONSTRAINT ck_ai_skill_category_type CHECK (
        category_type IN ('skill_market', 'skills_collection')
    )
);

CREATE INDEX IF NOT EXISTS idx_ai_skill_category_tree
    ON ai_skill_category (tenant_id, category_type, parent_id, sort_weight);

CREATE TABLE IF NOT EXISTS ai_agent_skill_package (
    id BIGSERIAL PRIMARY KEY,
    uuid VARCHAR(96) NOT NULL,
    tenant_id BIGINT NOT NULL,
    organization_id BIGINT NOT NULL DEFAULT 0,
    owner_user_id BIGINT NOT NULL DEFAULT 0,
    skill_id VARCHAR(128) NOT NULL,
    package_key VARCHAR(128) NOT NULL,
    code VARCHAR(128) NOT NULL,
    display_name VARCHAR(255) NOT NULL,
    summary TEXT,
    description TEXT,
    invocation_kind VARCHAR(64) NOT NULL,
    package_ref TEXT NOT NULL,
    entrypoint VARCHAR(255) NOT NULL,
    input_schema_json TEXT NOT NULL DEFAULT '{}',
    output_schema_json TEXT NOT NULL DEFAULT '{}',
    capability_ids_json TEXT NOT NULL DEFAULT '[]',
    categories_json TEXT NOT NULL DEFAULT '[]',
    tags_json TEXT NOT NULL DEFAULT '[]',
    security_profile_id VARCHAR(128),
    status SMALLINT NOT NULL DEFAULT 1,
    visibility SMALLINT NOT NULL DEFAULT 0,
    featured SMALLINT NOT NULL DEFAULT 0,
    sort_weight INTEGER NOT NULL DEFAULT 0,
    enabled SMALLINT NOT NULL DEFAULT 1,
    version BIGINT NOT NULL DEFAULT 1,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMPTZ,
    CONSTRAINT uk_ai_agent_skill_package_uuid UNIQUE (uuid),
    CONSTRAINT uk_ai_agent_skill_package_tenant_skill UNIQUE (tenant_id, skill_id),
    CONSTRAINT uk_ai_agent_skill_package_tenant_code UNIQUE (tenant_id, code),
    CONSTRAINT uk_ai_agent_skill_package_tenant_package_key UNIQUE (tenant_id, organization_id, package_key),
    CONSTRAINT ck_ai_agent_skill_package_skill_id CHECK (
        skill_id ~ '^skill\.[a-z0-9_-]+(\.[a-z0-9_-]+)*$'
    ),
    CONSTRAINT ck_ai_agent_skill_package_invocation_kind CHECK (
        invocation_kind IN ('local-workflow', 'process-adapter', 'mcp-tool', 'kernel-provider')
    ),
    CONSTRAINT ck_ai_agent_skill_package_capabilities CHECK (
        sdkwork_skills_capabilities_json_is_standard(capability_ids_json)
    ),
    CONSTRAINT ck_ai_agent_skill_package_status CHECK (status IN (0, 1, 2, 3, 4)),
    CONSTRAINT ck_ai_agent_skill_package_visibility CHECK (visibility IN (0, 1, 2, 3))
);

CREATE INDEX IF NOT EXISTS idx_ai_agent_skill_package_market
    ON ai_agent_skill_package (tenant_id, enabled, featured, sort_weight, updated_at DESC);

CREATE TABLE IF NOT EXISTS ai_agent_skill (
    id BIGSERIAL PRIMARY KEY,
    uuid VARCHAR(96) NOT NULL,
    tenant_id BIGINT NOT NULL,
    organization_id BIGINT NOT NULL DEFAULT 0,
    owner_user_id BIGINT NOT NULL DEFAULT 0,
    skill_key VARCHAR(128) NOT NULL,
    package_id BIGINT REFERENCES ai_agent_skill_package(id),
    name VARCHAR(255) NOT NULL,
    summary TEXT,
    description TEXT,
    provider TEXT,
    runtime TEXT,
    entrypoint VARCHAR(255),
    manifest_url TEXT,
    repository_url TEXT,
    homepage_url TEXT,
    documentation_url TEXT,
    license_name TEXT,
    source_type VARCHAR(64) NOT NULL DEFAULT 'package',
    market_status VARCHAR(64) NOT NULL DEFAULT 'published',
    visibility VARCHAR(64) NOT NULL DEFAULT 'public',
    review_status VARCHAR(64) NOT NULL DEFAULT 'approved',
    categories_json TEXT NOT NULL DEFAULT '[]',
    enabled SMALLINT NOT NULL DEFAULT 1,
    featured SMALLINT NOT NULL DEFAULT 0,
    recommend_weight INTEGER NOT NULL DEFAULT 0,
    install_count BIGINT NOT NULL DEFAULT 0,
    rating_avg NUMERIC(4, 2) NOT NULL DEFAULT 0,
    rating_count BIGINT NOT NULL DEFAULT 0,
    tags_json TEXT NOT NULL DEFAULT '[]',
    capabilities_json TEXT NOT NULL DEFAULT '[]',
    config_schema_json TEXT NOT NULL DEFAULT '{}',
    default_config_json TEXT NOT NULL DEFAULT '{}',
    version BIGINT NOT NULL DEFAULT 1,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMPTZ,
    CONSTRAINT uk_ai_agent_skill_uuid UNIQUE (uuid),
    CONSTRAINT uk_ai_agent_skill_scope_key UNIQUE (tenant_id, organization_id, skill_key),
    CONSTRAINT ck_ai_agent_skill_key CHECK (
        skill_key ~ '^skill\.[a-z0-9_-]+(\.[a-z0-9_-]+)*$'
    )
);

CREATE INDEX IF NOT EXISTS idx_ai_agent_skill_market
    ON ai_agent_skill (tenant_id, enabled, visibility, review_status, market_status, featured, recommend_weight);

CREATE TABLE IF NOT EXISTS ai_user_agent_skill (
    id BIGSERIAL PRIMARY KEY,
    uuid VARCHAR(96) NOT NULL,
    tenant_id BIGINT NOT NULL,
    organization_id BIGINT NOT NULL DEFAULT 0,
    user_id BIGINT NOT NULL,
    skill_id BIGINT NOT NULL REFERENCES ai_agent_skill(id),
    package_id BIGINT REFERENCES ai_agent_skill_package(id),
    install_status VARCHAR(32) NOT NULL DEFAULT 'installed',
    enabled SMALLINT NOT NULL DEFAULT 1,
    config_json TEXT NOT NULL DEFAULT '{}',
    installed_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMPTZ,
    CONSTRAINT uk_ai_user_agent_skill_uuid UNIQUE (uuid),
    CONSTRAINT uk_ai_user_agent_skill_scope UNIQUE (tenant_id, user_id, skill_id),
    CONSTRAINT ck_ai_user_agent_skill_status CHECK (
        install_status IN ('installed', 'disabled', 'removed')
    )
);

CREATE INDEX IF NOT EXISTS idx_ai_user_agent_skill_user
    ON ai_user_agent_skill (tenant_id, user_id, enabled, updated_at DESC);

CREATE TABLE IF NOT EXISTS ai_skill_asset (
    id BIGSERIAL PRIMARY KEY,
    uuid VARCHAR(96) NOT NULL,
    tenant_id BIGINT NOT NULL,
    skill_id BIGINT REFERENCES ai_agent_skill(id),
    package_id BIGINT REFERENCES ai_agent_skill_package(id),
    asset_type VARCHAR(64) NOT NULL,
    purpose VARCHAR(64) NOT NULL,
    url TEXT,
    media_resource_id TEXT,
    sort_weight INTEGER NOT NULL DEFAULT 0,
    metadata_json TEXT NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT uk_ai_skill_asset_uuid UNIQUE (uuid)
);

CREATE TABLE IF NOT EXISTS ai_skill_artifact (
    id BIGSERIAL PRIMARY KEY,
    uuid VARCHAR(96) NOT NULL,
    tenant_id BIGINT NOT NULL,
    package_id BIGINT NOT NULL REFERENCES ai_agent_skill_package(id),
    version_label VARCHAR(128) NOT NULL,
    artifact_ref TEXT NOT NULL,
    checksum_sha256 VARCHAR(64),
    size_bytes BIGINT,
    metadata_json TEXT NOT NULL DEFAULT '{}',
    published_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT uk_ai_skill_artifact_uuid UNIQUE (uuid),
    CONSTRAINT uk_ai_skill_artifact_version UNIQUE (package_id, version_label)
);

CREATE TABLE IF NOT EXISTS ai_skill_action (
    id BIGSERIAL PRIMARY KEY,
    uuid VARCHAR(96) NOT NULL,
    tenant_id BIGINT NOT NULL,
    user_id BIGINT,
    skill_id BIGINT REFERENCES ai_agent_skill(id),
    package_id BIGINT REFERENCES ai_agent_skill_package(id),
    action_type VARCHAR(64) NOT NULL,
    action_value TEXT,
    metadata_json TEXT NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT uk_ai_skill_action_uuid UNIQUE (uuid),
    CONSTRAINT ck_ai_skill_action_type CHECK (
        action_type IN ('download', 'favorite', 'unfavorite', 'rate', 'view')
    )
);

CREATE INDEX IF NOT EXISTS idx_ai_skill_action_skill
    ON ai_skill_action (tenant_id, skill_id, action_type, created_at DESC);
