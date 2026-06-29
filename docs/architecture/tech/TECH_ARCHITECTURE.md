# SDKWork Skills Technical Architecture

Status: active
Owner: SDKWork maintainers
Updated: 2026-06-29
Specs: ARCHITECTURE_DECISION_SPEC.md, DOCUMENTATION_SPEC.md

## 1. Architecture Overview

```
┌─────────────────────┐     ┌──────────────────────────┐
│ sdkwork-skills-pc   │────▶│ sdkwork-skills-app-sdk   │
│ (Hub/Console/Admin) │     │ sdkwork-skills-backend-sdk│
└─────────────────────┘     └────────────┬─────────────┘
                                         │ HTTPS (SdkWorkApiResponse)
┌─────────────────────┐     ┌────────────▼─────────────┐
│ sdkwork-kernel      │────▶│ standalone gateway       │
│ (contract consumer) │     │ app-api :18090           │
└─────────────────────┘     │ backend-api :18091       │
                            └────────────┬─────────────┘
                                         │
                            ┌────────────▼─────────────┐
                            │ sdkwork-web-framework    │
                            │ + IAM web adapter        │
                            └────────────┬─────────────┘
                                         │
                            ┌────────────▼─────────────┐
                            │ intelligence-skills-*    │
                            │ service + SQLx repo      │
                            └────────────┬─────────────┘
                                         │
                            ┌────────────▼─────────────┐
                            │ PostgreSQL (ai_* tables) │
                            └──────────────────────────┘
```

## 2. Technology Choices

| Layer | Choice |
| --- | --- |
| HTTP | Axum + `sdkwork-web-axum` / `sdkwork-web-core` |
| Auth | `sdkwork-iam-web-adapter` dual-token |
| Persistence | PostgreSQL via `sdkwork-database-*` + SQLx |
| Shared HTTP types | `sdkwork-utils-rust` (`SdkWorkApiResponse`, `PageInfo`) |
| PC UI | React + Vite; `@sdkwork/utils` for string helpers |
| Contracts | OpenAPI 3.1.2, `sdkwork-v3` SDK profile |

## 3. System Boundaries And Modules

| Crate / package | Responsibility |
| --- | --- |
| `sdkwork-skills-contract` | Cross-repo Rust DTOs and enums |
| `sdkwork-intelligence-skills-service` | Validation and domain orchestration |
| `sdkwork-intelligence-skills-repository-sqlx` | `ai_*` SQL access |
| `sdkwork-routes-skills-common` | Shared API response + service helpers |
| `sdkwork-routes-skills-app-api` | App-api routes and manifest |
| `sdkwork-routes-skills-backend-api` | Backend-api routes and manifest |
| `sdkwork-skills-standalone-gateway` | Local/dev gateway binary |
| `apps/sdkwork-skills-pc` | Browser client surfaces |

## 4. Directory And Package Layout

See repository root `AGENTS.md` dictionary. Authoritative app config: `sdkwork.app.config.json`.

## 5. API, SDK, And Data Ownership

- OpenAPI authorities materialized by `tools/skills_openapi_materialize.mjs` (uses
  `sdkwork-specs/tools/lib/openapi-envelope-schemas.mjs`).
- SDK generation: `pnpm sdk:generate` via `@sdkwork/sdk-generator`.
- Tables: `ai_agent_skill`, `ai_agent_skill_package`, `ai_skill_category`, user install tables.

## 6. Security, Privacy, And Observability

- IAM route manifest operation IDs aligned with OpenAPI.
- Delete package route: `RateLimitTier::AuthCritical`.
- Health: `/livez`, `/readyz`, `/healthz` with DB readiness on standalone gateway.
- Errors: ProblemDetail with sanitized 5xx detail; trace via `traceId` / `x-sdkwork-trace-id`.

## 7. Deployment And Runtime Topology

- Dev: `pnpm dev` (PC) + standalone gateway or cloud topology per `specs/topology.spec.json`.
- Packaging: `sdkwork.workflow.json` / `.github/workflows/package.yml`.
- Container image id: `registry.sdkwork.com/apps/sdkwork-skills` (from app config).

## 8. Architecture Decision Index

- [ADR-20260624-skills-domain-extraction-and-ai-table-standard.md](../decisions/ADR-20260624-skills-domain-extraction-and-ai-table-standard.md) (accepted)

## 9. Verification

```bash
pnpm verify
node ../sdkwork-specs/tools/check-api-response-envelope.mjs --workspace .
pnpm topology:validate
```

## Standards Alignment Matrix

| Standard | Status |
| --- | --- |
| `sdkwork-specs` agent/docs scripts | Aligned |
| `sdkwork-web-framework` | Integrated on route crates |
| `sdkwork-database` | Integrated (`database/`, host crate) |
| `sdkwork-utils` (`@sdkwork/utils`, `sdkwork-utils-rust`) | Used in UI + handlers |
| `sdkwork-discovery` | N/A (no RPC yet) |
| API §15 `SdkWorkApiResponse` | Aligned (handlers + OpenAPI + SDK) |
