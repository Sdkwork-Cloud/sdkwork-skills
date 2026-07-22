# Skills Database

Owner: skills-platform

The Skills module is the single write authority for Skill packages, marketplace
entries, immutable artifacts, capabilities, installations, assets, and actions.
The pre-launch baseline is portable across PostgreSQL and SQLite and contains
exactly the ten tables below.

## Tables

| Table | Purpose |
| --- | --- |
| `ai_skill_category` | Tenant or organization taxonomy with IAM permission binding |
| `ai_agent_skill_package` | Package identity, ownership, lifecycle, and visibility |
| `ai_agent_skill` | Marketplace publication and review state for one package |
| `ai_skill_category_binding` | Normalized Skill-to-category relationship |
| `ai_skill_capability` | Governed capability dictionary and risk level |
| `ai_skill_artifact` | Immutable versioned release metadata and Drive artifact reference |
| `ai_skill_artifact_capability` | Normalized artifact-to-capability relationship |
| `ai_skill_installation` | Exact artifact installed for a user, workspace, project, or agent |
| `ai_skill_asset` | Skill, package, or artifact media references |
| `ai_skill_action` | User download, favorite, rating, and view events |

Packages never store a latest-artifact projection. Installation commands select
an explicit published `artifact_id`, and the installation row preserves that
immutable release identity. Categories and capabilities use binding tables;
there are no JSON authority lists, shadow tables, or synchronized copies.

## Isolation And Concurrency

- Every repository query is tenant-scoped; organization and user visibility are
  applied before pagination.
- Identifiers come from the process-shared Snowflake allocator.
- Mutable aggregates use optimistic `version` checks and soft deletion.
- Filtering, ordering, total counting, and pagination execute in SQL.
- PostgreSQL and SQLite baselines express the same logical contract.

## Category Permissions

Each `ai_skill_category.permission_code` defines the IAM permission required to
manage packages in that category. Global permissions are declared in
`specs/iam.module.manifest.json`:

- `skills.categories.manage`: manage category taxonomy.
- `skills.packages.manage`: manage packages and artifacts.
- `skills.marketplace.read`: read the marketplace and installable artifacts.
- `skills.packages.install`: install an exact published artifact for an
  authorized subject.

## Verification

```bash
pnpm db:materialize:contract
pnpm db:validate
pnpm db:plan
```

## Initialization State

This module is in initialization state for greenfield deployments:

1. `database/ddl/baseline/{engine}/0001_skills_baseline.sql` is the full
   system-of-record DDL.
2. `database/migrations/{engine}/` is reserved for post-GA incremental schema
   changes only.
3. No legacy Skills schema is bootstrapped or copied into this module.
4. Run `pnpm db:drift:check` before release.

## Commands

```bash
pnpm run db:validate
pnpm run db:materialize:contract
pnpm run db:plan
pnpm run db:init
pnpm run db:migrate
pnpm run db:seed
pnpm run db:status
pnpm run db:drift:check
```
