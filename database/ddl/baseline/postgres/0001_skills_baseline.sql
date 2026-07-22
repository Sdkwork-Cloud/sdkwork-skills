-- SDKWork Skills PostgreSQL initialization baseline.
-- This repository is pre-launch: the baseline is the complete system-of-record contract.

CREATE TABLE IF NOT EXISTS ai_skill_category (
    id BIGINT NOT NULL PRIMARY KEY,
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
    version BIGINT NOT NULL DEFAULT 1,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMPTZ,
    CONSTRAINT uk_ai_skill_category_uuid UNIQUE (uuid),
    CONSTRAINT ck_ai_skill_category_type CHECK (category_type IN ('skill_market', 'skills_collection')),
    CONSTRAINT ck_ai_skill_category_visible CHECK (visible IN (0, 1)),
    CONSTRAINT ck_ai_skill_category_status CHECK (status IN (0, 1, 2)),
    CONSTRAINT ck_ai_skill_category_version CHECK (version > 0)
);

CREATE UNIQUE INDEX IF NOT EXISTS uk_ai_skill_category_scope_code_active
    ON ai_skill_category (tenant_id, organization_id, category_type, code)
    WHERE deleted_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_ai_skill_category_tree
    ON ai_skill_category (tenant_id, organization_id, category_type, parent_id, sort_weight)
    WHERE deleted_at IS NULL;

CREATE TABLE IF NOT EXISTS ai_agent_skill_package (
    id BIGINT NOT NULL PRIMARY KEY,
    uuid VARCHAR(96) NOT NULL,
    tenant_id BIGINT NOT NULL,
    organization_id BIGINT NOT NULL DEFAULT 0,
    owner_user_id BIGINT NOT NULL DEFAULT 0,
    package_key VARCHAR(128) NOT NULL,
    code VARCHAR(128) NOT NULL,
    display_name VARCHAR(255) NOT NULL,
    summary TEXT,
    description TEXT,
    tags_json TEXT NOT NULL DEFAULT '[]',
    status SMALLINT NOT NULL DEFAULT 0,
    visibility SMALLINT NOT NULL DEFAULT 0,
    featured SMALLINT NOT NULL DEFAULT 0,
    sort_weight INTEGER NOT NULL DEFAULT 0,
    version BIGINT NOT NULL DEFAULT 1,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMPTZ,
    CONSTRAINT uk_ai_agent_skill_package_uuid UNIQUE (uuid),
    CONSTRAINT ck_ai_agent_skill_package_tags CHECK (jsonb_typeof(tags_json::jsonb) = 'array'),
    CONSTRAINT ck_ai_agent_skill_package_status CHECK (status IN (0, 1, 2, 3, 4)),
    CONSTRAINT ck_ai_agent_skill_package_visibility CHECK (visibility IN (0, 1, 2, 3)),
    CONSTRAINT ck_ai_agent_skill_package_featured CHECK (featured IN (0, 1)),
    CONSTRAINT ck_ai_agent_skill_package_version CHECK (version > 0)
);

CREATE UNIQUE INDEX IF NOT EXISTS uk_ai_agent_skill_package_scope_key_active
    ON ai_agent_skill_package (tenant_id, organization_id, package_key)
    WHERE deleted_at IS NULL;
CREATE UNIQUE INDEX IF NOT EXISTS uk_ai_agent_skill_package_scope_code_active
    ON ai_agent_skill_package (tenant_id, organization_id, code)
    WHERE deleted_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_ai_agent_skill_package_market
    ON ai_agent_skill_package (tenant_id, visibility, status, featured, sort_weight, updated_at DESC)
    WHERE deleted_at IS NULL;

CREATE TABLE IF NOT EXISTS ai_agent_skill (
    id BIGINT NOT NULL PRIMARY KEY,
    uuid VARCHAR(96) NOT NULL,
    tenant_id BIGINT NOT NULL,
    organization_id BIGINT NOT NULL DEFAULT 0,
    skill_key VARCHAR(128) NOT NULL,
    package_id BIGINT NOT NULL REFERENCES ai_agent_skill_package(id),
    market_status VARCHAR(32) NOT NULL DEFAULT 'draft',
    review_status VARCHAR(32) NOT NULL DEFAULT 'pending',
    enabled SMALLINT NOT NULL DEFAULT 0,
    featured SMALLINT NOT NULL DEFAULT 0,
    recommend_weight INTEGER NOT NULL DEFAULT 0,
    install_count BIGINT NOT NULL DEFAULT 0,
    rating_avg NUMERIC(4, 2) NOT NULL DEFAULT 0,
    rating_count BIGINT NOT NULL DEFAULT 0,
    version BIGINT NOT NULL DEFAULT 1,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMPTZ,
    CONSTRAINT uk_ai_agent_skill_uuid UNIQUE (uuid),
    CONSTRAINT ck_ai_agent_skill_key CHECK (skill_key ~ '^skill\.[a-z0-9_-]+(\.[a-z0-9_-]+)*$'),
    CONSTRAINT ck_ai_agent_skill_market_status CHECK (market_status IN ('draft', 'published', 'disabled', 'archived', 'removed')),
    CONSTRAINT ck_ai_agent_skill_review_status CHECK (review_status IN ('pending', 'approved', 'rejected')),
    CONSTRAINT ck_ai_agent_skill_enabled CHECK (enabled IN (0, 1)),
    CONSTRAINT ck_ai_agent_skill_featured CHECK (featured IN (0, 1)),
    CONSTRAINT ck_ai_agent_skill_rating CHECK (rating_avg >= 0 AND rating_avg <= 5 AND rating_count >= 0),
    CONSTRAINT ck_ai_agent_skill_version CHECK (version > 0)
);

CREATE UNIQUE INDEX IF NOT EXISTS uk_ai_agent_skill_scope_key_active
    ON ai_agent_skill (tenant_id, organization_id, skill_key)
    WHERE deleted_at IS NULL;
CREATE UNIQUE INDEX IF NOT EXISTS uk_ai_agent_skill_package_active
    ON ai_agent_skill (package_id)
    WHERE deleted_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_ai_agent_skill_market
    ON ai_agent_skill (tenant_id, enabled, market_status, review_status, featured, recommend_weight, updated_at DESC)
    WHERE deleted_at IS NULL;

CREATE TABLE IF NOT EXISTS ai_skill_category_binding (
    id BIGINT NOT NULL PRIMARY KEY,
    tenant_id BIGINT NOT NULL,
    skill_id BIGINT NOT NULL REFERENCES ai_agent_skill(id),
    category_id BIGINT NOT NULL REFERENCES ai_skill_category(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT uk_ai_skill_category_binding UNIQUE (skill_id, category_id)
);

CREATE INDEX IF NOT EXISTS idx_ai_skill_category_binding_category
    ON ai_skill_category_binding (tenant_id, category_id, skill_id);

CREATE TABLE IF NOT EXISTS ai_skill_capability (
    id BIGINT NOT NULL PRIMARY KEY,
    uuid VARCHAR(96) NOT NULL,
    tenant_id BIGINT NOT NULL,
    organization_id BIGINT NOT NULL DEFAULT 0,
    capability_key VARCHAR(128) NOT NULL,
    display_name VARCHAR(255) NOT NULL,
    description TEXT,
    risk_level VARCHAR(16) NOT NULL DEFAULT 'standard',
    status SMALLINT NOT NULL DEFAULT 1,
    version BIGINT NOT NULL DEFAULT 1,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMPTZ,
    CONSTRAINT uk_ai_skill_capability_uuid UNIQUE (uuid),
    CONSTRAINT ck_ai_skill_capability_key CHECK (capability_key ~ '^[a-z0-9_-]+(\.[a-z0-9_-]+)+$'),
    CONSTRAINT ck_ai_skill_capability_risk CHECK (risk_level IN ('standard', 'sensitive', 'privileged')),
    CONSTRAINT ck_ai_skill_capability_status CHECK (status IN (0, 1, 2)),
    CONSTRAINT ck_ai_skill_capability_version CHECK (version > 0)
);

CREATE UNIQUE INDEX IF NOT EXISTS uk_ai_skill_capability_scope_key_active
    ON ai_skill_capability (tenant_id, organization_id, capability_key)
    WHERE deleted_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_ai_skill_capability_lookup
    ON ai_skill_capability (tenant_id, status, capability_key)
    WHERE deleted_at IS NULL;

CREATE TABLE IF NOT EXISTS ai_skill_artifact (
    id BIGINT NOT NULL PRIMARY KEY,
    uuid VARCHAR(96) NOT NULL,
    tenant_id BIGINT NOT NULL,
    package_id BIGINT NOT NULL REFERENCES ai_agent_skill_package(id),
    version_label VARCHAR(128) NOT NULL,
    artifact_ref TEXT NOT NULL,
    checksum_sha256 VARCHAR(64) NOT NULL,
    size_bytes BIGINT,
    invocation_kind VARCHAR(64) NOT NULL,
    entrypoint VARCHAR(255) NOT NULL,
    input_schema_json TEXT NOT NULL DEFAULT '{}',
    output_schema_json TEXT NOT NULL DEFAULT '{}',
    config_schema_json TEXT NOT NULL DEFAULT '{}',
    default_config_json TEXT NOT NULL DEFAULT '{}',
    security_profile_id VARCHAR(128),
    status VARCHAR(16) NOT NULL DEFAULT 'draft',
    published_at TIMESTAMPTZ,
    yanked_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT uk_ai_skill_artifact_uuid UNIQUE (uuid),
    CONSTRAINT uk_ai_skill_artifact_version UNIQUE (package_id, version_label),
    CONSTRAINT ck_ai_skill_artifact_ref CHECK (artifact_ref ~ '^drive://spaces/[^/]+/nodes/[^/]+$'),
    CONSTRAINT ck_ai_skill_artifact_checksum CHECK (checksum_sha256 ~ '^[0-9a-f]{64}$'),
    CONSTRAINT ck_ai_skill_artifact_size CHECK (size_bytes IS NULL OR size_bytes >= 0),
    CONSTRAINT ck_ai_skill_artifact_invocation CHECK (invocation_kind IN ('local-workflow', 'process-adapter', 'mcp-tool', 'kernel-provider')),
    CONSTRAINT ck_ai_skill_artifact_input_schema CHECK (jsonb_typeof(input_schema_json::jsonb) = 'object'),
    CONSTRAINT ck_ai_skill_artifact_output_schema CHECK (jsonb_typeof(output_schema_json::jsonb) = 'object'),
    CONSTRAINT ck_ai_skill_artifact_config_schema CHECK (jsonb_typeof(config_schema_json::jsonb) = 'object'),
    CONSTRAINT ck_ai_skill_artifact_default_config CHECK (jsonb_typeof(default_config_json::jsonb) = 'object'),
    CONSTRAINT ck_ai_skill_artifact_status CHECK (status IN ('draft', 'published', 'yanked'))
);

CREATE INDEX IF NOT EXISTS idx_ai_skill_artifact_release
    ON ai_skill_artifact (tenant_id, package_id, status, published_at DESC, created_at DESC);

CREATE TABLE IF NOT EXISTS ai_skill_artifact_capability (
    id BIGINT NOT NULL PRIMARY KEY,
    tenant_id BIGINT NOT NULL,
    artifact_id BIGINT NOT NULL REFERENCES ai_skill_artifact(id),
    capability_id BIGINT NOT NULL REFERENCES ai_skill_capability(id),
    required SMALLINT NOT NULL DEFAULT 1,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT uk_ai_skill_artifact_capability UNIQUE (artifact_id, capability_id),
    CONSTRAINT ck_ai_skill_artifact_capability_required CHECK (required IN (0, 1))
);

CREATE INDEX IF NOT EXISTS idx_ai_skill_artifact_capability_capability
    ON ai_skill_artifact_capability (tenant_id, capability_id, artifact_id);

CREATE TABLE IF NOT EXISTS ai_skill_installation (
    id BIGINT NOT NULL PRIMARY KEY,
    uuid VARCHAR(96) NOT NULL,
    tenant_id BIGINT NOT NULL,
    organization_id BIGINT NOT NULL DEFAULT 0,
    subject_kind VARCHAR(32) NOT NULL,
    subject_id BIGINT NOT NULL,
    skill_id BIGINT NOT NULL REFERENCES ai_agent_skill(id),
    package_id BIGINT NOT NULL REFERENCES ai_agent_skill_package(id),
    artifact_id BIGINT NOT NULL REFERENCES ai_skill_artifact(id),
    installed_by_user_id BIGINT NOT NULL,
    install_status VARCHAR(32) NOT NULL DEFAULT 'installed',
    enabled SMALLINT NOT NULL DEFAULT 1,
    config_json TEXT NOT NULL DEFAULT '{}',
    version BIGINT NOT NULL DEFAULT 1,
    installed_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMPTZ,
    CONSTRAINT uk_ai_skill_installation_uuid UNIQUE (uuid),
    CONSTRAINT ck_ai_skill_installation_subject CHECK (subject_kind IN ('user', 'workspace', 'project', 'agent')),
    CONSTRAINT ck_ai_skill_installation_subject_id CHECK (subject_id > 0),
    CONSTRAINT ck_ai_skill_installation_status CHECK (install_status IN ('installed', 'disabled', 'removed')),
    CONSTRAINT ck_ai_skill_installation_enabled CHECK (enabled IN (0, 1)),
    CONSTRAINT ck_ai_skill_installation_config CHECK (jsonb_typeof(config_json::jsonb) = 'object'),
    CONSTRAINT ck_ai_skill_installation_version CHECK (version > 0)
);

CREATE UNIQUE INDEX IF NOT EXISTS uk_ai_skill_installation_subject_skill_active
    ON ai_skill_installation (tenant_id, organization_id, subject_kind, subject_id, skill_id)
    WHERE deleted_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_ai_skill_installation_subject
    ON ai_skill_installation (tenant_id, organization_id, subject_kind, subject_id, enabled, updated_at DESC)
    WHERE deleted_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_ai_skill_installation_artifact
    ON ai_skill_installation (tenant_id, artifact_id)
    WHERE deleted_at IS NULL;

CREATE TABLE IF NOT EXISTS ai_skill_asset (
    id BIGINT NOT NULL PRIMARY KEY,
    uuid VARCHAR(96) NOT NULL,
    tenant_id BIGINT NOT NULL,
    skill_id BIGINT REFERENCES ai_agent_skill(id),
    package_id BIGINT REFERENCES ai_agent_skill_package(id),
    artifact_id BIGINT REFERENCES ai_skill_artifact(id),
    asset_type VARCHAR(64) NOT NULL,
    purpose VARCHAR(64) NOT NULL,
    media_resource_id VARCHAR(128) NOT NULL,
    sort_weight INTEGER NOT NULL DEFAULT 0,
    metadata_json TEXT NOT NULL DEFAULT '{}',
    version BIGINT NOT NULL DEFAULT 1,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT uk_ai_skill_asset_uuid UNIQUE (uuid),
    CONSTRAINT ck_ai_skill_asset_owner CHECK (skill_id IS NOT NULL OR package_id IS NOT NULL OR artifact_id IS NOT NULL),
    CONSTRAINT ck_ai_skill_asset_metadata CHECK (jsonb_typeof(metadata_json::jsonb) = 'object'),
    CONSTRAINT ck_ai_skill_asset_version CHECK (version > 0)
);

CREATE INDEX IF NOT EXISTS idx_ai_skill_asset_owner
    ON ai_skill_asset (tenant_id, skill_id, package_id, artifact_id, purpose, sort_weight);

CREATE TABLE IF NOT EXISTS ai_skill_action (
    id BIGINT NOT NULL PRIMARY KEY,
    uuid VARCHAR(96) NOT NULL,
    tenant_id BIGINT NOT NULL,
    user_id BIGINT NOT NULL,
    skill_id BIGINT NOT NULL REFERENCES ai_agent_skill(id),
    action_type VARCHAR(64) NOT NULL,
    action_value TEXT,
    context_json TEXT NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT uk_ai_skill_action_uuid UNIQUE (uuid),
    CONSTRAINT ck_ai_skill_action_type CHECK (action_type IN ('download', 'favorite', 'unfavorite', 'rate', 'view')),
    CONSTRAINT ck_ai_skill_action_context CHECK (jsonb_typeof(context_json::jsonb) = 'object')
);

CREATE INDEX IF NOT EXISTS idx_ai_skill_action_skill
    ON ai_skill_action (tenant_id, skill_id, action_type, created_at DESC);
CREATE INDEX IF NOT EXISTS idx_ai_skill_action_user
    ON ai_skill_action (tenant_id, user_id, action_type, created_at DESC);
