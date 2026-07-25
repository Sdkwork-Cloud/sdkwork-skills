# ADR-20260722: Skills Domain Ownership And Artifact Model

Status: accepted
Date: 2026-07-22
Deciders: SDKWork maintainers

## Context

Skills packages, marketplace publication, versioned artifacts, capabilities,
and installations form one cohesive domain. Duplicating those concepts in
consumer applications creates competing write authorities, ambiguous version
selection, and authorization drift. The application is pre-launch, so the
correct boundary can be established without compatibility schemas or data
projection layers.

## Decision

1. `sdkwork-skills` is the sole write authority for Skill packages,
   marketplace entries, categories, capabilities, immutable artifacts,
   installations, assets, and actions.
2. The authoritative-server persistence contract contains exactly ten
   normalized `ai_*` tables on PostgreSQL. Relationships with authority
   semantics use binding tables, not JSON arrays. Skills owns no client-local
   database or alternate server store.
3. An artifact is an immutable release identified by `artifact_id`, version,
   checksum, Drive reference, schemas, entrypoint, and capability bindings.
   Packages do not carry version payloads or a latest-artifact projection.
4. Installation always selects an explicit published artifact and records an
   authorized IAM `user`/`organization` or Agents `project`/`agent` subject. Persistence
   keeps the package installation slot and selected artifact only; `skill_id`
   is derived through the canonical one-to-one package relation. The active
   subject/package key is the atomic concurrency and idempotency boundary.
5. Consumers use the generated `@sdkwork/skills-app-sdk` or
   `@sdkwork/skills-backend-sdk`. They do not copy Skills API paths, DTOs,
   tables, storage bindings, or runtime configuration authorities.
6. Agent sessions, conversations, business chat messages, and IM transport
   messages remain outside the Skills domain.
7. App-api and backend-api are the only current product surfaces. An open-api
   requires an independent public contract and review.

## Consequences

- There is one database and one API authority for each Skills capability.
- Marketplace reads and artifact installation apply tenant, organization,
  user, publication, lifecycle, and object authorization before SQL pagination.
- Database checks prevent cross-package artifact selection, ambiguous asset
  ownership, and inconsistent draft/published/yanked timestamps.
- Consumers retain only references needed for their own orchestration; they do
  not synchronize Skill state.
- No projection, double write, shadow table, compatibility facade, or default
  tenant path is permitted.

## Verification

```bash
pnpm db:materialize:contract
pnpm db:validate
pnpm sdk:generate
node tools/skills_openapi_materialize.mjs --check
node ../sdkwork-specs/tools/check-app-sdk-consumer-imports.mjs --workspace .
cargo test --workspace
```
