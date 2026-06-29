# Integrator Guide

## Overview

Integrate with Skills marketplace APIs through generated SDK families in `sdks/`. Wire contracts
from `apis/app-api/skills/` (user-facing Hub) and `apis/backend-api/skills/` (admin Console).

Application authority: `sdkwork-skills.app` (app-api), `sdkwork-skills.backend` (backend-api).

## SDK Families

| Surface | SDK | OpenAPI |
| --- | --- | --- |
| App (Hub, user install) | `sdks/sdkwork-skills-app-sdk/` | `apis/app-api/skills/skills-app-api.openapi.json` |
| Backend (Admin CRUD) | `sdks/sdkwork-skills-backend-sdk/` | `apis/backend-api/skills/skills-backend-api.openapi.json` |
| Cross-domain contract | `crates/sdkwork-skills-contract/` | Rust types shared with kernel |

Generate TypeScript SDKs:

```bash
pnpm sdk:generate
```

Standard profile: `sdkwork-v3` (success envelope unwrap enabled in HTTP clients).

## Authentication

Dual-token model (`AuthToken` bearer + `Access-Token` header) per SDKWork IAM. Route manifests
in `crates/sdkwork-routes-skills-*/src/http_route_manifest.rs` declare IAM operation IDs aligned
with OpenAPI `operationId` values.

## Response Envelope

HTTP 2xx JSON:

```json
{
  "code": 0,
  "data": { "items": [], "pageInfo": { "mode": "offset", "page": 1 } },
  "traceId": "<uuid>"
}
```

Single resource:

```json
{
  "code": 0,
  "data": { "item": { } },
  "traceId": "<uuid>"
}
```

Errors: HTTP 4xx/5xx with `application/problem+json` including numeric `code` and `traceId`.

## Kernel Integration

`sdkwork-kernel` references skills by ID through `sdkwork-skills-contract`. Skill package
persistence and CRUD remain in this repository only.

## Verification

```bash
pnpm api:check
pnpm verify
node ../sdkwork-specs/tools/check-api-response-envelope.mjs --workspace .
node ../sdkwork-specs/tools/check-dependency-composition.mjs --workspace ..
```

`pnpm api:check` materializes OpenAPI, runs `tools/skills_schema_quality_gate.mjs` (envelope,
list query params, OpenAPI ↔ route manifest parity), and validates generated SDK drift.
