# SDKWork Skills Technical Architecture

Status: active
Owner: SDKWork maintainers
Updated: 2026-07-22
Specs: `ARCHITECTURE_DECISION_SPEC.md`, `APPLICATION_GATEWAY_SPEC.md`, `API_ASSEMBLY_SPEC.md`, `API_SPEC.md`, `SDK_SPEC.md`, `DATABASE_FRAMEWORK_SPEC.md`

## 1. Architecture Overview

SDKWork Skills exposes one host-neutral API assembly with two business
surfaces:

```text
sdkwork-skills-pc
  -> sdkwork-skills-app-sdk / sdkwork-skills-backend-sdk
  -> application.public-ingress or platform.api-gateway
  -> sdkwork-api-skills-assembly
  -> sdkwork-routes-skills-app-api / sdkwork-routes-skills-backend-api
  -> sdkwork-routes-skills-common
  -> sdkwork-intelligence-skills-service
  -> sdkwork-intelligence-skills-repository-sqlx
  -> process-shared PostgreSQL or SQLite pool
```

The standalone gateway and platform cloud gateway are sibling hosts. Both
consume `sdkwork-api-skills-assembly`; neither duplicates Skills routes,
services, repositories, or database pools. App-api and backend-api are surfaces
on one ingress, not separate business servers.

## 2. Technology Choices

| Layer | Choice |
| --- | --- |
| HTTP | Axum with `sdkwork-web-axum` and `sdkwork-web-core` |
| Authentication | `sdkwork-iam-web-adapter` dual-token request context |
| Persistence | PostgreSQL and SQLite through `sdkwork-database-*` and SQLx |
| Shared HTTP types | `sdkwork-utils-rust` envelopes and pagination types |
| PC UI | React and Vite |
| Contracts | OpenAPI 3.1.2 with the `sdkwork-v3` profile |

## 3. Component Boundaries

| Component | Responsibility |
| --- | --- |
| `sdkwork-skills-contract` | Surface-neutral domain records, enums, operations, and permissions |
| `sdkwork-intelligence-skills-service` | Validation, use cases, and repository port |
| `sdkwork-intelligence-skills-repository-sqlx` | Tenant-scoped PostgreSQL and SQLite queries |
| `sdkwork-routes-skills-common` | Commands, list queries, envelopes, errors, and service calls |
| `sdkwork-routes-skills-app-api` | App-api handlers and route manifest only |
| `sdkwork-routes-skills-backend-api` | Backend-api handlers and route manifest only |
| `sdkwork-api-skills-assembly` | Host-neutral router composition and readiness capability |
| `sdkwork-api-skills-standalone-gateway` | Standalone listener and host-owned operations endpoints |
| `sdkwork-skills-database-host` | Skills lifecycle on a caller-provided process pool |
| `apps/sdkwork-skills-pc` | Browser client composition through generated SDK families |

Route crates do not register `/healthz`, `/livez`, `/readyz`, or `/metrics`.
Those paths belong to the active gateway host.

## 4. API And SDK Ownership

| Surface | API Authority | SDK Family | Operations |
| --- | --- | --- | ---: |
| App | `sdkwork-skills-app-api` | `sdkwork-skills-app-sdk` | 8 |
| Backend | `sdkwork-skills-backend-api` | `sdkwork-skills-backend-sdk` | 16 |

`tools/skills_openapi_materialize.mjs` materializes owner-only OpenAPI from the
route manifests. `pnpm sdk:generate` invokes `@sdkwork/sdk-generator`; generated
transport stays under each SDK family's `generated/server-openapi` directory.
Generated files are never edited by hand.

There is no Skills open-api surface. Public exposure requires a separately
approved product contract rather than reusing authenticated app routes.

Create operations return HTTP `201` with `SdkWorkApiResponse.data.item`.
Updates return HTTP `200` with `data.item`; lists return `data.items` and
`data.pageInfo`; deletes return HTTP `204`; errors use
`application/problem+json` with numeric `code` and `traceId`.

## 5. Persistence Model

The database module owns exactly ten `ai_*` tables. Category and capability
relationships are normalized through binding tables. Artifacts are immutable
releases; package rows do not carry release payloads or a latest-artifact
projection. Installations reference one exact artifact and support `user`,
`workspace`, `project`, and `agent` subjects.

The module owns lifecycle assets but not the process pool. The gateway supplies
one shared `DatabasePool`; the assembly constructs one `SqlxSkillsRepository`
and one `SkillsService` for both surfaces. PostgreSQL and SQLite select
engine-specific SQL inside the repository while preserving one logical
contract.

## 6. Security And Performance

- Route manifests require typed `WebRequestContext` and explicit Skills
  permissions.
- Tenant, organization, user, and operator identity come only from the request
  context. Handlers do not parse identity headers or accept a client tenant
  selector.
- Marketplace package and artifact visibility is checked for tenant,
  organization, user ownership, publication, approval, and lifecycle state.
- Installation applies subject-level authorization and requires an explicit
  published `artifactId`.
- Filtering, ordering, total counting, authorization, and pagination execute in
  SQL with bounded page sizes.
- Mutable aggregates use Snowflake IDs, optimistic versions, and soft deletion.
- Package bytes are owned by Drive; Skills artifacts store canonical
  `drive://` references, checksums, invocation metadata, and schemas.

## 7. Deployment Topology

- `standalone`: `sdkwork-api-skills-standalone-gateway` owns one listener,
  default bind `127.0.0.1:18092`.
- `cloud`: `sdkwork-api-cloud-gateway` embeds
  `sdkwork-api-skills-assembly` through the `foundation-skills` feature.
- Both profiles expose identical app/backend contracts and SDK families.

## 8. Verification

```bash
pnpm db:materialize:contract
pnpm db:validate
pnpm api:check
pnpm api:assembly:validate
pnpm sdk:generate
cargo test --workspace
cargo clippy --workspace --tests -- -D warnings
node ../sdkwork-specs/tools/check-api-operation-patterns.mjs --workspace .
node ../sdkwork-specs/tools/check-api-response-envelope.mjs --workspace .
node ../sdkwork-specs/tools/check-pagination.mjs --workspace .
node ../sdkwork-specs/tools/check-app-sdk-consumer-imports.mjs --workspace .
```

SDK generation is complete only when a second `pnpm sdk:generate` reports no
created, updated, or deleted generated files.

## 9. Architecture Decision

- [ADR-20260722-skills-domain-ownership-and-artifact-model.md](../decisions/ADR-20260722-skills-domain-ownership-and-artifact-model.md)
