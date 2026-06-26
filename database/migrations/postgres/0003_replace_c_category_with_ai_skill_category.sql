-- Replace legacy c_category with ai_skill_category and unify package/skill category binding.

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

INSERT INTO ai_skill_category (
    uuid,
    tenant_id,
    organization_id,
    category_type,
    code,
    name,
    description,
    parent_id,
    path,
    sort_weight,
    permission_code,
    visible,
    status,
    tags_json,
    created_at,
    updated_at,
    deleted_at
)
SELECT
    legacy.uuid,
    legacy.tenant_id,
    legacy.organization_id,
    legacy.category_type,
    legacy.code,
    legacy.name,
    legacy.description,
    legacy.parent_id,
    legacy.path,
    legacy.sort_weight,
    'skills.admin.package.manage.' || legacy.code AS permission_code,
    legacy.visible,
    legacy.status,
    legacy.tags_json,
    legacy.created_at,
    legacy.updated_at,
    legacy.deleted_at
FROM c_category AS legacy
WHERE legacy.category_type IN ('skill_market', 'skills_collection')
ON CONFLICT (tenant_id, organization_id, category_type, code) DO NOTHING;

UPDATE ai_agent_skill_package AS package
SET categories_json = COALESCE(
    NULLIF(package.categories_json, '[]'),
    (
        SELECT jsonb_build_array(category.code)::text
        FROM c_category AS category
        WHERE category.id = package.category_id
    ),
    package.categories_json
)
WHERE package.category_id IS NOT NULL;

ALTER TABLE ai_agent_skill_package
    DROP CONSTRAINT IF EXISTS ai_agent_skill_package_category_id_fkey;

ALTER TABLE ai_agent_skill_package
    DROP COLUMN IF EXISTS category_id;

ALTER TABLE ai_agent_skill
    ADD COLUMN IF NOT EXISTS categories_json TEXT NOT NULL DEFAULT '[]';

UPDATE ai_agent_skill AS skill
SET categories_json = package.categories_json
FROM ai_agent_skill_package AS package
WHERE skill.package_id = package.id
  AND (skill.categories_json = '[]' OR skill.categories_json IS NULL);

ALTER TABLE ai_agent_skill
    DROP CONSTRAINT IF EXISTS ai_agent_skill_category_id_fkey;

ALTER TABLE ai_agent_skill
    DROP COLUMN IF EXISTS category_id;

DROP TABLE IF EXISTS c_category;
