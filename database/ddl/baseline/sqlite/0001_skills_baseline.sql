-- SDKWork Skills SQLite initialization baseline.
-- Logical tables and constraints mirror the PostgreSQL baseline.

PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS ai_skill_category (
    id BIGINT NOT NULL PRIMARY KEY,
    uuid TEXT NOT NULL UNIQUE,
    tenant_id BIGINT NOT NULL DEFAULT 0,
    organization_id BIGINT NOT NULL DEFAULT 0,
    category_type TEXT NOT NULL CHECK (category_type IN ('skill_market', 'skills_collection')),
    code TEXT NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    parent_id BIGINT REFERENCES ai_skill_category(id),
    path TEXT,
    sort_weight INTEGER NOT NULL DEFAULT 0,
    permission_code TEXT NOT NULL,
    visible INTEGER NOT NULL DEFAULT 1 CHECK (visible IN (0, 1)),
    status INTEGER NOT NULL DEFAULT 1 CHECK (status IN (0, 1, 2)),
    version BIGINT NOT NULL DEFAULT 1 CHECK (version > 0),
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    deleted_at TEXT
);
CREATE UNIQUE INDEX IF NOT EXISTS uk_ai_skill_category_scope_code_active
    ON ai_skill_category (tenant_id, organization_id, category_type, code) WHERE deleted_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_ai_skill_category_tree
    ON ai_skill_category (tenant_id, organization_id, category_type, parent_id, sort_weight) WHERE deleted_at IS NULL;

CREATE TABLE IF NOT EXISTS ai_agent_skill_package (
    id BIGINT NOT NULL PRIMARY KEY,
    uuid TEXT NOT NULL UNIQUE,
    tenant_id BIGINT NOT NULL,
    organization_id BIGINT NOT NULL DEFAULT 0,
    owner_user_id BIGINT NOT NULL DEFAULT 0,
    package_key TEXT NOT NULL,
    code TEXT NOT NULL,
    display_name TEXT NOT NULL,
    summary TEXT,
    description TEXT,
    tags_json TEXT NOT NULL DEFAULT '[]' CHECK (json_valid(tags_json) AND json_type(tags_json) = 'array'),
    status INTEGER NOT NULL DEFAULT 0 CHECK (status IN (0, 1, 2, 3, 4)),
    visibility INTEGER NOT NULL DEFAULT 0 CHECK (visibility IN (0, 1, 2, 3)),
    featured INTEGER NOT NULL DEFAULT 0 CHECK (featured IN (0, 1)),
    sort_weight INTEGER NOT NULL DEFAULT 0,
    version BIGINT NOT NULL DEFAULT 1 CHECK (version > 0),
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    deleted_at TEXT
);
CREATE UNIQUE INDEX IF NOT EXISTS uk_ai_agent_skill_package_scope_key_active
    ON ai_agent_skill_package (tenant_id, organization_id, package_key) WHERE deleted_at IS NULL;
CREATE UNIQUE INDEX IF NOT EXISTS uk_ai_agent_skill_package_scope_code_active
    ON ai_agent_skill_package (tenant_id, organization_id, code) WHERE deleted_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_ai_agent_skill_package_market
    ON ai_agent_skill_package (tenant_id, visibility, status, featured, sort_weight, updated_at DESC) WHERE deleted_at IS NULL;

CREATE TABLE IF NOT EXISTS ai_agent_skill (
    id BIGINT NOT NULL PRIMARY KEY,
    uuid TEXT NOT NULL UNIQUE,
    tenant_id BIGINT NOT NULL,
    organization_id BIGINT NOT NULL DEFAULT 0,
    skill_key TEXT NOT NULL,
    package_id BIGINT NOT NULL REFERENCES ai_agent_skill_package(id),
    market_status TEXT NOT NULL DEFAULT 'draft' CHECK (market_status IN ('draft', 'published', 'disabled', 'archived', 'removed')),
    review_status TEXT NOT NULL DEFAULT 'pending' CHECK (review_status IN ('pending', 'approved', 'rejected')),
    enabled INTEGER NOT NULL DEFAULT 0 CHECK (enabled IN (0, 1)),
    featured INTEGER NOT NULL DEFAULT 0 CHECK (featured IN (0, 1)),
    recommend_weight INTEGER NOT NULL DEFAULT 0,
    install_count BIGINT NOT NULL DEFAULT 0,
    rating_avg NUMERIC NOT NULL DEFAULT 0 CHECK (rating_avg >= 0 AND rating_avg <= 5),
    rating_count BIGINT NOT NULL DEFAULT 0 CHECK (rating_count >= 0),
    version BIGINT NOT NULL DEFAULT 1 CHECK (version > 0),
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    deleted_at TEXT
);
CREATE UNIQUE INDEX IF NOT EXISTS uk_ai_agent_skill_scope_key_active
    ON ai_agent_skill (tenant_id, organization_id, skill_key) WHERE deleted_at IS NULL;
CREATE UNIQUE INDEX IF NOT EXISTS uk_ai_agent_skill_package_active
    ON ai_agent_skill (package_id) WHERE deleted_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_ai_agent_skill_market
    ON ai_agent_skill (tenant_id, enabled, market_status, review_status, featured, recommend_weight, updated_at DESC) WHERE deleted_at IS NULL;

CREATE TABLE IF NOT EXISTS ai_skill_category_binding (
    id BIGINT NOT NULL PRIMARY KEY,
    tenant_id BIGINT NOT NULL,
    skill_id BIGINT NOT NULL REFERENCES ai_agent_skill(id),
    category_id BIGINT NOT NULL REFERENCES ai_skill_category(id),
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    UNIQUE (skill_id, category_id)
);
CREATE INDEX IF NOT EXISTS idx_ai_skill_category_binding_category
    ON ai_skill_category_binding (tenant_id, category_id, skill_id);

CREATE TABLE IF NOT EXISTS ai_skill_capability (
    id BIGINT NOT NULL PRIMARY KEY,
    uuid TEXT NOT NULL UNIQUE,
    tenant_id BIGINT NOT NULL,
    organization_id BIGINT NOT NULL DEFAULT 0,
    capability_key TEXT NOT NULL,
    display_name TEXT NOT NULL,
    description TEXT,
    risk_level TEXT NOT NULL DEFAULT 'standard' CHECK (risk_level IN ('standard', 'sensitive', 'privileged')),
    status INTEGER NOT NULL DEFAULT 1 CHECK (status IN (0, 1, 2)),
    version BIGINT NOT NULL DEFAULT 1 CHECK (version > 0),
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    deleted_at TEXT
);
CREATE UNIQUE INDEX IF NOT EXISTS uk_ai_skill_capability_scope_key_active
    ON ai_skill_capability (tenant_id, organization_id, capability_key) WHERE deleted_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_ai_skill_capability_lookup
    ON ai_skill_capability (tenant_id, status, capability_key) WHERE deleted_at IS NULL;

CREATE TABLE IF NOT EXISTS ai_skill_artifact (
    id BIGINT NOT NULL PRIMARY KEY,
    uuid TEXT NOT NULL UNIQUE,
    tenant_id BIGINT NOT NULL,
    package_id BIGINT NOT NULL REFERENCES ai_agent_skill_package(id),
    version_label TEXT NOT NULL,
    artifact_ref TEXT NOT NULL CHECK (artifact_ref LIKE 'drive://spaces/%/nodes/%'),
    checksum_sha256 TEXT NOT NULL CHECK (length(checksum_sha256) = 64 AND checksum_sha256 NOT GLOB '*[^0-9a-f]*'),
    size_bytes BIGINT CHECK (size_bytes IS NULL OR size_bytes >= 0),
    invocation_kind TEXT NOT NULL CHECK (invocation_kind IN ('local-workflow', 'process-adapter', 'mcp-tool', 'kernel-provider')),
    entrypoint TEXT NOT NULL,
    input_schema_json TEXT NOT NULL DEFAULT '{}' CHECK (json_valid(input_schema_json) AND json_type(input_schema_json) = 'object'),
    output_schema_json TEXT NOT NULL DEFAULT '{}' CHECK (json_valid(output_schema_json) AND json_type(output_schema_json) = 'object'),
    config_schema_json TEXT NOT NULL DEFAULT '{}' CHECK (json_valid(config_schema_json) AND json_type(config_schema_json) = 'object'),
    default_config_json TEXT NOT NULL DEFAULT '{}' CHECK (json_valid(default_config_json) AND json_type(default_config_json) = 'object'),
    security_profile_id TEXT,
    status TEXT NOT NULL DEFAULT 'draft' CHECK (status IN ('draft', 'published', 'yanked')),
    published_at TEXT,
    yanked_at TEXT,
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    UNIQUE (package_id, version_label)
);
CREATE INDEX IF NOT EXISTS idx_ai_skill_artifact_release
    ON ai_skill_artifact (tenant_id, package_id, status, published_at DESC, created_at DESC);

CREATE TABLE IF NOT EXISTS ai_skill_artifact_capability (
    id BIGINT NOT NULL PRIMARY KEY,
    tenant_id BIGINT NOT NULL,
    artifact_id BIGINT NOT NULL REFERENCES ai_skill_artifact(id),
    capability_id BIGINT NOT NULL REFERENCES ai_skill_capability(id),
    required INTEGER NOT NULL DEFAULT 1 CHECK (required IN (0, 1)),
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    UNIQUE (artifact_id, capability_id)
);
CREATE INDEX IF NOT EXISTS idx_ai_skill_artifact_capability_capability
    ON ai_skill_artifact_capability (tenant_id, capability_id, artifact_id);

CREATE TABLE IF NOT EXISTS ai_skill_installation (
    id BIGINT NOT NULL PRIMARY KEY,
    uuid TEXT NOT NULL UNIQUE,
    tenant_id BIGINT NOT NULL,
    organization_id BIGINT NOT NULL DEFAULT 0,
    subject_kind TEXT NOT NULL CHECK (subject_kind IN ('user', 'workspace', 'project', 'agent')),
    subject_id BIGINT NOT NULL CHECK (subject_id > 0),
    skill_id BIGINT NOT NULL REFERENCES ai_agent_skill(id),
    package_id BIGINT NOT NULL REFERENCES ai_agent_skill_package(id),
    artifact_id BIGINT NOT NULL REFERENCES ai_skill_artifact(id),
    installed_by_user_id BIGINT NOT NULL,
    install_status TEXT NOT NULL DEFAULT 'installed' CHECK (install_status IN ('installed', 'disabled', 'removed')),
    enabled INTEGER NOT NULL DEFAULT 1 CHECK (enabled IN (0, 1)),
    config_json TEXT NOT NULL DEFAULT '{}' CHECK (json_valid(config_json) AND json_type(config_json) = 'object'),
    version BIGINT NOT NULL DEFAULT 1 CHECK (version > 0),
    installed_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    deleted_at TEXT
);
CREATE UNIQUE INDEX IF NOT EXISTS uk_ai_skill_installation_subject_skill_active
    ON ai_skill_installation (tenant_id, organization_id, subject_kind, subject_id, skill_id) WHERE deleted_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_ai_skill_installation_subject
    ON ai_skill_installation (tenant_id, organization_id, subject_kind, subject_id, enabled, updated_at DESC) WHERE deleted_at IS NULL;
CREATE INDEX IF NOT EXISTS idx_ai_skill_installation_artifact
    ON ai_skill_installation (tenant_id, artifact_id) WHERE deleted_at IS NULL;

CREATE TABLE IF NOT EXISTS ai_skill_asset (
    id BIGINT NOT NULL PRIMARY KEY,
    uuid TEXT NOT NULL UNIQUE,
    tenant_id BIGINT NOT NULL,
    skill_id BIGINT REFERENCES ai_agent_skill(id),
    package_id BIGINT REFERENCES ai_agent_skill_package(id),
    artifact_id BIGINT REFERENCES ai_skill_artifact(id),
    asset_type TEXT NOT NULL,
    purpose TEXT NOT NULL,
    media_resource_id TEXT NOT NULL,
    sort_weight INTEGER NOT NULL DEFAULT 0,
    metadata_json TEXT NOT NULL DEFAULT '{}' CHECK (json_valid(metadata_json) AND json_type(metadata_json) = 'object'),
    version BIGINT NOT NULL DEFAULT 1 CHECK (version > 0),
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    CHECK (skill_id IS NOT NULL OR package_id IS NOT NULL OR artifact_id IS NOT NULL)
);
CREATE INDEX IF NOT EXISTS idx_ai_skill_asset_owner
    ON ai_skill_asset (tenant_id, skill_id, package_id, artifact_id, purpose, sort_weight);

CREATE TABLE IF NOT EXISTS ai_skill_action (
    id BIGINT NOT NULL PRIMARY KEY,
    uuid TEXT NOT NULL UNIQUE,
    tenant_id BIGINT NOT NULL,
    user_id BIGINT NOT NULL,
    skill_id BIGINT NOT NULL REFERENCES ai_agent_skill(id),
    action_type TEXT NOT NULL CHECK (action_type IN ('download', 'favorite', 'unfavorite', 'rate', 'view')),
    action_value TEXT,
    context_json TEXT NOT NULL DEFAULT '{}' CHECK (json_valid(context_json) AND json_type(context_json) = 'object'),
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);
CREATE INDEX IF NOT EXISTS idx_ai_skill_action_skill
    ON ai_skill_action (tenant_id, skill_id, action_type, created_at DESC);
CREATE INDEX IF NOT EXISTS idx_ai_skill_action_user
    ON ai_skill_action (tenant_id, user_id, action_type, created_at DESC);
