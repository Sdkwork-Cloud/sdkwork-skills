# Integrator Guide

## Overview

Integrate with Skills through the composed SDK families under `sdks/`. The
owner-only contracts are under `apis/app-api/skills/` for authenticated
user-facing capabilities and `apis/backend-api/skills/` for operator
capabilities.

| Surface | API Authority | SDK Family | OpenAPI |
| --- | --- | --- | --- |
| App | `sdkwork-skills-app-api` | `sdkwork-skills-app-sdk` | `apis/app-api/skills/skills-app-api.openapi.json` |
| Backend | `sdkwork-skills-backend-api` | `sdkwork-skills-backend-sdk` | `apis/backend-api/skills/skills-backend-api.openapi.json` |

No Skills open-api is currently declared. Do not expose app-api routes as a
public contract without a separate product requirement and API authority.

Use `@sdkwork/skills-app-sdk` or `@sdkwork/skills-backend-sdk`. Consumers must
not import generated transport package names, construct raw HTTP requests,
assemble authentication headers, or redefine Skills DTOs.

## Runtime Topology

Cloud deployment consumes the host-neutral `sdkwork-api-skills-assembly`
through the platform gateway. Standalone deployment uses
`sdkwork-api-skills-standalone-gateway`. App-api and backend-api are surfaces on
one listener, not independent business servers.

## Authentication

Protected routes use the SDKWork dual-token model through the shared IAM
runtime. Route manifests declare typed `WebRequestContext`, surface,
operation IDs, and permissions. Consumers inject the shared token manager.

The server derives tenant, organization, user, and operator identity from
`WebRequestContext`. Consumers must not add tenant selectors, default tenants,
or identity headers.

## Installation Flow

```ts
const artifacts = await client.skills.skillPackages.artifacts.list(packageId, {
  page: 1,
  pageSize: 20,
});

const installation = await client.skills.skillPackages.installations.create(
  packageId,
  {
    artifactId: artifacts.items[0].id,
    target: { kind: 'workspace', id: workspaceId },
    config: {},
  },
);
```

The artifact endpoint returns only releases installable in the authenticated
context. The client still chooses an exact artifact; there is no
latest-artifact projection.

## Response Contract

- Create returns HTTP `201` with `data.item`.
- Retrieve and update return HTTP `200` with `data.item`.
- List returns HTTP `200` with `data.items` and `data.pageInfo`.
- Delete returns HTTP `204` with no body.
- Errors return HTTP 4xx or 5xx `application/problem+json` with numeric `code`
  and `traceId`.

## Cross-Domain Ownership

Skills owns package, marketplace, artifact, capability, installation, asset,
and action metadata. Drive owns uploaded file bytes; IAM owns login, tokens,
identity, and permissions; Agents owns agent sessions; messaging domains own
conversation semantics and IM transport. Dependency-owned APIs and data are
not copied into Skills or consumer authorities.

## SDK Generation And Verification

```bash
pnpm sdk:generate
pnpm api:check
pnpm api:assembly:validate
node ../sdkwork-specs/tools/check-api-operation-patterns.mjs --workspace .
node ../sdkwork-specs/tools/check-api-response-envelope.mjs --workspace .
node ../sdkwork-specs/tools/check-pagination.mjs --workspace .
node ../sdkwork-specs/tools/check-app-sdk-consumer-imports.mjs --workspace .
```

A second `pnpm sdk:generate` must produce no generated changes.
