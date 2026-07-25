# SDKWork Skills PRD

Status: active
Owner: SDKWork maintainers
Application: sdkwork-skills
Updated: 2026-07-24
Specs: REQUIREMENTS_SPEC.md, DOCUMENTATION_SPEC.md

## 1. Background And Problem

Agent products need a governed Skills marketplace with discoverable packages,
reviewed marketplace entries, immutable release artifacts, explicit capability
declarations, subject-scoped installations, and an auditable admin lifecycle.
`sdkwork-skills` is the sole system of record for that domain.

## 2. Target Users

| Persona | Surface | Needs |
| --- | --- | --- |
| End user or agent operator | Hub (`/skills-hub`) | Browse and install an exact published artifact |
| Tenant member | Console (`/console/skills`) | View owned Skill packages |
| Platform or tenant admin | Admin (`/admin/skills`, `/admin/categories`) | Govern packages, artifacts, categories, and capabilities with IAM |

## 3. Goals And Non-Goals

### Goals

- App-api discovery, detail, installable-artifact listing, installation, and
  installation inventory flows.
- Backend-api governance for packages, immutable artifacts, categories, and
  capabilities. Artifact bytes remain in Drive and are referenced by canonical
  `drive://` URIs.
- Ten normalized `ai_*` tables as the sole Skills persistence authority on
  PostgreSQL.
- SDKWork-standard inputs, envelopes, ProblemDetail errors, generated SDKs,
  dual-token authentication, route permissions, and object authorization.
- Direct consumption of `@sdkwork/skills-app-sdk` and
  `@sdkwork/skills-backend-sdk`; consumers must not fork Skills DTOs or routes.

### Non-Goals

- A public open-api surface without an approved public-product requirement.
- Agent session, conversation, chat-message, or IM transport persistence.
- Consumer-owned Skills tables, projections, double writes, compatibility
  facades, or locally generated copies of Skills APIs.

## 4. Scope

- Host-neutral Rust assembly with app-api and backend-api route surfaces.
- PC React client and generated TypeScript App/Backend SDK families.
- Authoritative PostgreSQL baseline, shared process pool integration, and
  store-level pagination.

## 5. User Scenarios

1. An operator uploads an archive through Drive, then creates a package and its
   initial immutable artifact with checksum and schema metadata.
2. An operator publishes and approves the marketplace entry and assigns
   normalized categories and capabilities.
3. A user browses visible packages, lists installable published artifacts, and
   installs one exact artifact for an authorized user, organization, project, or
   agent subject.
4. Agent runtimes reference canonical installation and artifact identities
   without owning a second Skills database.

## 6. Success Metrics

- `pnpm verify` is green and includes API, SDK, topology, assembly, and database
  contract gates.
- App SDK regeneration is idempotent with 8 owner operations; Backend SDK
  regeneration is idempotent with 16 owner operations.
- Repository verification covers tenant, user, organization, publication,
  package lifecycle, optimistic concurrency, and PostgreSQL transaction
  boundaries.

## 7. Delivery State

| Capability | Status |
| --- | --- |
| Ten-table Skills system of record | Done |
| App/backend APIs and generated SDKs | Done |
| Dual-token context and object authorization | Done |
| PC Hub, Console, and Admin integration | Active |
| Production topology and operational validation | Active |

## 8. Linked Documents

- [ADR-20260722-skills-domain-ownership-and-artifact-model.md](../../architecture/decisions/ADR-20260722-skills-domain-ownership-and-artifact-model.md)
- [TECH_ARCHITECTURE.md](../../architecture/tech/TECH_ARCHITECTURE.md)
- [MIG-2026-0010-skills-greenfield-boundary-cutover.md](../../migrations/MIG-2026-0010-skills-greenfield-boundary-cutover.md)

## 9. Release Gate

Production release requires the PostgreSQL database drift check, SDK
idempotence check, repository tests, permission composition checks, and cloud
and standalone readiness probes to pass from a clean checkout.
