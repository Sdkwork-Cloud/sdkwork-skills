# Skills Database

Owner: skills-platform

The Skills module is the single write authority for Skill packages, marketplace
entries, immutable artifacts, capabilities, installations, assets, and actions.
The pre-launch authoritative-server baseline targets PostgreSQL and contains
exactly the ten tables below. Skills does not own a client-local database.

## Tables

| Table | Purpose |
| --- | --- |
| `skills_category` | Tenant or organization taxonomy with IAM permission binding |
| `skills_package` | Package identity, ownership, lifecycle, and visibility |
| `skills_definition` | Marketplace publication and review state for one package |
| `skills_category_binding` | Normalized Skill-to-category relationship |
| `skills_capability` | Governed capability dictionary and risk level |
| `skills_artifact` | Immutable versioned release metadata and Drive artifact reference |
| `skills_artifact_capability` | Normalized artifact-to-capability relationship |
| `skills_installation` | Exact artifact installed for an IAM user/organization or Agents project/agent |
| `skills_asset` | Skill, package, or artifact media references |
| `skills_action` | User download, favorite, rating, and view events |

Packages never store a latest-artifact projection. Installation commands select
an explicit published `artifact_id`. Each installation stores the stable
`package_id` slot and selected immutable artifact; `skill_id` is derived through
the package's one-to-one marketplace entry and is not duplicated in the table.
The database enforces that the artifact belongs to the package. Categories and
capabilities use binding tables; there are no JSON authority lists, shadow
tables, synchronized copies, or latest-version projections.

Artifact lifecycle timestamps form a database-checked state machine: drafts
have no publication timestamps, published artifacts have `published_at`, and
yanked artifacts have ordered `published_at` and `yanked_at` values. Every
asset belongs to exactly one Skill, package, or artifact.

## Isolation And Concurrency

- Every repository query is tenant-scoped; organization and user visibility are
  applied before pagination.
- Identifiers come from the process-shared Snowflake allocator.
- Mutable aggregates use optimistic `version` checks and soft deletion.
- The active installation uniqueness boundary is tenant, organization,
  subject, and package. Atomic conflict handling serializes concurrent install
  requests and increments marketplace `install_count` only for the first row.
- Filtering, ordering, total counting, and pagination execute in SQL.
- PostgreSQL is the only authoritative Skills engine; server startup fails
  closed for any other configured engine.

## Category Permissions

Each `skills_category.permission_code` defines the IAM permission required to
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

1. `database/ddl/baseline/postgres/0001_skills_baseline.sql` is the full
   system-of-record DDL.
2. `database/migrations/postgres/` is reserved for post-GA incremental schema
   changes only.
3. No legacy Skills schema is bootstrapped or copied into this module.
4. Run `pnpm db:drift:check` before release.

## Initialization state

This module is in **initialization state** for greenfield deployments:

1. **Baseline** — `database/ddl/baseline/{engine}/0001_skills_baseline.sql` contains the full DDL snapshot.
2. **Migrations** — `database/migrations/{engine}/` is reserved for post-GA incremental schema changes only. It is intentionally empty at initialization.
3. **Drift** — run `pnpm db:drift:check` before release.

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
