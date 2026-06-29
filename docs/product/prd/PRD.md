# SDKWork Skills PRD

Status: active
Owner: SDKWork maintainers
Application: sdkwork-skills
Updated: 2026-06-29
Specs: REQUIREMENTS_SPEC.md, DOCUMENTATION_SPEC.md

## 1. Background And Problem

Agent platforms need a governed Skills marketplace: discoverable packages, tenant-scoped install
state, category permissions, and admin lifecycle (create, update, soft-delete). Skill persistence
must not live in `sdkwork-kernel`; this application is the system-of-record.

## 2. Target Users

| Persona | Surface | Needs |
| --- | --- | --- |
| End user / agent operator | Hub (`/skills-hub`) | Browse and install published skills |
| Tenant member | Console (`/console/skills`) | View owned skill packages |
| Platform / tenant admin | Admin (`/admin/skills`, `/admin/categories`) | CRUD packages and categories with IAM |

## 3. Goals And Non-Goals

**Goals**

- Skills Hub with list, detail, and install flows backed by app-api.
- Admin backend-api for package and category CRUD with drive-backed `package_ref`.
- `ai_*` PostgreSQL tables as sole persistence; kernel reads contract types only.
- SDKWork-standard HTTP envelopes, ProblemDetail errors, generated SDKs, IAM route manifests.

**Non-Goals**

- In-repo RPC or service discovery (deferred until RPC surfaces are required).
- Skill CRUD or marketplace tables inside `sdkwork-kernel`.

## 4. Scope

- Rust standalone gateways (app + backend), PC React client, OpenAPI + SDK generation.
- Migration from kernel `a_agent_skill_package` via `database/migrations/postgres/`.

## 5. User Scenarios

1. Admin uploads a skill archive via sdkwork-drive, creates a package record, assigns categories.
2. User browses Hub, opens skill detail, installs to their profile.
3. Kernel agent references installed skills by contract IDs without local skill tables.

## 6. Success Metrics

- `pnpm verify` green; envelope checker passes.
- No raw HTTP in PC production surfaces; SDK unwrap matches handler envelopes.
- IAM operation IDs match OpenAPI and route manifests.

## 7. Phases

| Phase | Status |
| --- | --- |
| Application root + `ai_*` schema | Done |
| App/backend APIs + web-framework | Done |
| PC Hub / Console / Admin | Done |
| Kernel decoupling + migration | In progress (external kernel repo) |
| Production topology / cloud bundle | Planned |

## 8. Linked Requirements

- [ADR-20260624-skills-domain-extraction-and-ai-table-standard.md](../architecture/decisions/ADR-20260624-skills-domain-extraction-and-ai-table-standard.md)
- [TECH_ARCHITECTURE.md](../architecture/tech/TECH_ARCHITECTURE.md)

## 9. Open Questions

- Multi-region read replicas for Hub list latency (post-MVP).
- Category delete semantics when packages still reference a category code.
