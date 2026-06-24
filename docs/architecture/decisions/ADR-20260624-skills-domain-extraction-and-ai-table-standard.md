# ADR-20260624: Skills Domain Extraction And ai_ Table Standard

Status: accepted
Date: 2026-06-24
Deciders: SDKWork maintainers
Supersedes: ADR-20260624-legacy-skills-pointer-and-framework-exemptions.md

## Context

Skills marketplace persistence incorrectly lived in `sdkwork-kernel` as `a_agent_skill_package`.
SDKWork intelligence-domain tables must use the `ai_` prefix. Skills Hub requires a dedicated
application root with PC Hub/Console/Admin surfaces.

## Decision

1. **`sdkwork-skills` is the application root** for skills marketplace system-of-record.
2. **All skills tables use `ai_` prefix** (`ai_agent_skill`, `ai_agent_skill_package`, etc.).
3. **Categories** use shared `c_category` with `skill_market` / `skills_collection` types.
4. **`sdkwork-kernel` depends on `sdkwork-skills-contract`** and must not own skill persistence.
5. **`sdkwork-skills-pc`** delivers Hub (`/skills-hub`), Console (`/console/skills`), and Admin
   (`/admin/skills`, `/admin/categories`) surfaces.

## Consequences

- Kernel `a_agent_skill_package` is deprecated; data migrates via
  `database/migrations/postgres/0002_migrate_kernel_a_agent_skill_package.sql`.
- Platform frameworks (`sdkwork-web-framework`, `sdkwork-database`, `@sdkwork/utils`) are required
  at the skills application root.

## Verification

```bash
cargo test --workspace
pnpm db:materialize:contract
node ../sdkwork-specs/tools/check-repository-docs-standard.mjs --root .
```
