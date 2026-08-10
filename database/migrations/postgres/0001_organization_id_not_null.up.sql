-- sdkwork:migration
-- id: 0001_organization_id_not_null
-- engine: postgres
-- module: sdkwork-skills
-- purpose: Enforce organization_id NOT NULL DEFAULT on all tables in the
--   consolidated baseline. NULL rows (pre-standard data anomalies) are
--   backfilled with the platform sentinel before NOT NULL is set, and
--   NOT NULL columns without an explicit default receive the sentinel
--   default, keeping existing deployments consistent with fresh baseline
--   installs.
-- reversible: false
-- rollback: forward-fix (sentinel backfill is the canonical fix; NULL
--   organization rows are data anomalies)
-- transactional: true
-- lock: lightweight
-- lock_timeout: 2s
-- statement_timeout: 30s

BEGIN;

UPDATE skills_category SET organization_id = 0 WHERE organization_id IS NULL;
ALTER TABLE skills_category ALTER COLUMN organization_id SET DEFAULT 0;
ALTER TABLE skills_category ALTER COLUMN organization_id SET NOT NULL;

UPDATE skills_package SET organization_id = 0 WHERE organization_id IS NULL;
ALTER TABLE skills_package ALTER COLUMN organization_id SET DEFAULT 0;
ALTER TABLE skills_package ALTER COLUMN organization_id SET NOT NULL;

UPDATE skills_definition SET organization_id = 0 WHERE organization_id IS NULL;
ALTER TABLE skills_definition ALTER COLUMN organization_id SET DEFAULT 0;
ALTER TABLE skills_definition ALTER COLUMN organization_id SET NOT NULL;

UPDATE skills_capability SET organization_id = 0 WHERE organization_id IS NULL;
ALTER TABLE skills_capability ALTER COLUMN organization_id SET DEFAULT 0;
ALTER TABLE skills_capability ALTER COLUMN organization_id SET NOT NULL;

UPDATE skills_installation SET organization_id = 0 WHERE organization_id IS NULL;
ALTER TABLE skills_installation ALTER COLUMN organization_id SET DEFAULT 0;
ALTER TABLE skills_installation ALTER COLUMN organization_id SET NOT NULL;

COMMIT;
