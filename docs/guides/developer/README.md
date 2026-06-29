# Developer Guide

## Repository Role

`sdkwork-skills` is the SDKWork application root for Skills marketplace persistence, HTTP APIs,
and the PC browser client (`apps/sdkwork-skills-pc`). Skills data lives in `ai_*` PostgreSQL tables;
`sdkwork-kernel` consumes `sdkwork-skills-contract` and must not own skill CRUD.

Authority: [ADR-20260624-skills-domain-extraction-and-ai-table-standard.md](../architecture/decisions/ADR-20260624-skills-domain-extraction-and-ai-table-standard.md)

## Where To Work

| Task | Path |
| --- | --- |
| App identity and runtime | `sdkwork.app.config.json` |
| PC Hub / Console / Admin UI | `apps/sdkwork-skills-pc/` |
| App and backend OpenAPI | `apis/app-api/`, `apis/backend-api/` |
| Generated SDK families | `sdks/sdkwork-skills-app-sdk/`, `sdks/sdkwork-skills-backend-sdk/` |
| Rust route crates | `crates/sdkwork-routes-skills-*` |
| Domain service and SQLx repository | `crates/sdkwork-intelligence-skills-*` |
| Database migrations | `database/migrations/postgres/` |
| Local agent skill pointers | `.sdkwork/skills/` (optional; non-authoritative) |

## Local Setup

1. Install workspace dependencies from repository root:
   `pnpm install --filter sdkwork-skills-pc...` (requires sibling repos in
   `sdkwork-space`, including `@sdkwork/iam-credential-entry` via `pnpm-workspace.yaml`).
2. Bootstrap database: `pnpm db:bootstrap` (requires PostgreSQL per `database/` config).
3. Start PC dev server (proxies to local gateways): `pnpm dev`.
4. Run standalone gateways on ports `18090` (app-api) and `18091` (backend-api) via
   `cargo run -p sdkwork-skills-standalone-gateway` when testing APIs without cloud topology.

## API And SDK Conventions

- Success bodies use `SdkWorkApiResponse` (`code: 0`, `data`, `traceId`) per `API_SPEC.md` §15.
- Lists expose `data.items` + `data.pageInfo`; single resources use `data.item`.
- Errors use `application/problem+json` with numeric `code` and `traceId`.
- Regenerate contracts: `pnpm api:materialize && pnpm sdk:generate`.
- PC surfaces must consume generated SDKs; do not add raw HTTP wrappers in production UI.

## Verification

```bash
pnpm verify
node ../sdkwork-specs/tools/check-api-response-envelope.mjs --workspace .
node ../sdkwork-specs/tools/check-dependency-composition.mjs --workspace ..
node ../sdkwork-specs/tools/check-repository-docs-standard.mjs --root .
```

Root `tests/` use `tests/register-workspace-imports.mjs` to resolve `@sdkwork/utils` for
TypeScript source checks without requiring a full `pnpm install` at the repository root.

## Related Specs

- `../sdkwork-specs/SDKWORK_WORKSPACE_SPEC.md`
- `../sdkwork-specs/API_SPEC.md`
- `../sdkwork-specs/FRONTEND_SPEC.md`
- `../sdkwork-specs/RUST_CODE_SPEC.md`
