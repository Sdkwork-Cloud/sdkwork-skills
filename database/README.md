# Skills Database

Owner: skills-platform

Skills Hub system-of-record uses intelligence-domain `ai_*` tables. Skill market
classification is owned by `ai_skill_category`; packages and skills bind categories
through `categories_json` category codes.

## Tables

| Table | Purpose |
| --- | --- |
| `ai_skill_category` | Skill market and collection taxonomy with IAM permission binding |
| `ai_agent_skill_package` | Skill package metadata and kernel runtime binding |
| `ai_agent_skill` | Skill marketplace entries |
| `ai_user_agent_skill` | Per-user install and enablement |
| `ai_skill_asset` | Icons, covers, media |
| `ai_skill_artifact` | Versioned release artifacts |
| `ai_skill_action` | Download, favorite, rating actions |

## Category Permissions

Each `ai_skill_category.permission_code` defines the IAM scope required to manage
packages in that category. Default pattern: `skills.packages.manage.<category_code>`.

Global scopes (see `specs/iam.module.manifest.json`):

- `skills.categories.manage` — manage category taxonomy
- `skills.packages.manage` — manage packages across all categories
- `skills.marketplace.read` — read marketplace catalog and admin list surfaces
- `skills.packages.install` — install skills for the current user

## Verification

```bash
pnpm db:validate
pnpm db:plan
```

## Initialization state

This module is in **initialization state** for greenfield deployments:

1. **Baseline** — `database/ddl/baseline/{engine}/0001_skills_baseline.sql` contains the full DDL snapshot.
2. **Migrations** — `database/migrations/{engine}/` is reserved for post-GA incremental schema changes only.
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
