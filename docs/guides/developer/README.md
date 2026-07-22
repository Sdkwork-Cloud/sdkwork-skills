# Developer Guide

## Repository Role

`sdkwork-skills` is the application root and single write authority for Skills
marketplace persistence, HTTP APIs, generated SDKs, and the PC browser client.
Skills data lives in the module's ten `ai_*` PostgreSQL and SQLite tables.

Authority:
[ADR-20260722-skills-domain-ownership-and-artifact-model.md](../../architecture/decisions/ADR-20260722-skills-domain-ownership-and-artifact-model.md)

## Where To Work

| Task | Path |
| --- | --- |
| App identity and runtime | `sdkwork.app.config.json` |
| PC Hub, Console, and Admin UI | `apps/sdkwork-skills-pc/` |
| App and backend OpenAPI | `apis/app-api/`, `apis/backend-api/` |
| Generated SDK families | `sdks/sdkwork-skills-app-sdk/`, `sdks/sdkwork-skills-backend-sdk/` |
| Rust route crates | `crates/sdkwork-routes-skills-*` |
| Domain contract and service | `crates/sdkwork-skills-contract/`, `crates/sdkwork-intelligence-skills-service/` |
| SQLx repository | `crates/sdkwork-intelligence-skills-repository-sqlx/` |
| Database baseline and contracts | `database/ddl/baseline/`, `database/contract/` |
| Local agent skill pointers | `.sdkwork/skills/` (optional and non-authoritative) |

## Local Setup

1. Run `pnpm install` from the repository root.
2. Materialize and validate the contract with
   `pnpm db:materialize:contract && pnpm db:validate`.
3. Bootstrap the selected database profile with `pnpm db:bootstrap`.
4. Start the configured surface with `pnpm dev`.
5. Use `cargo run -p sdkwork-api-skills-standalone-gateway` when testing
   without the cloud gateway.

## API And SDK Conventions

- Success bodies use `SdkWorkApiResponse` with `code`, `data`, and `traceId`.
- Lists expose `data.items` and `data.pageInfo`; resources use `data.item`.
- Errors use `application/problem+json` with numeric `code` and `traceId`.
- Regenerate contracts through `pnpm api:materialize && pnpm sdk:generate`.
- UI and application consumers import composed SDK packages. Do not add raw
  HTTP wrappers, manual auth headers, generated transport imports, or local DTO
  forks.
- Installation flows list installable artifacts and submit an explicit
  `artifactId`; they never infer a latest release.
- Tenant, organization, and user identity come from the shared token-backed
  request context, never from consumer query parameters or identity headers.

## Verification

```bash
pnpm verify
node ../sdkwork-specs/tools/check-api-response-envelope.mjs --workspace .
node ../sdkwork-specs/tools/verify-repo.mjs --root .
node ../sdkwork-specs/tools/check-repository-docs-standard.mjs --root .
```

Root `tests/` use `tests/register-workspace-imports.mjs` to resolve workspace
packages for TypeScript source checks.

## Related Specs

- `../sdkwork-specs/SDKWORK_WORKSPACE_SPEC.md`
- `../sdkwork-specs/API_SPEC.md`
- `../sdkwork-specs/DATABASE_SPEC.md`
- `../sdkwork-specs/SDK_SPEC.md`
- `../sdkwork-specs/RUST_CODE_SPEC.md`
