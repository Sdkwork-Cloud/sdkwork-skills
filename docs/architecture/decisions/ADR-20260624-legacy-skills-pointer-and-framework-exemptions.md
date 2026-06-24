# ADR-20260624: Legacy Skills Pointer Repository And Platform Framework Exemptions

Status: superseded
Superseded-by: ADR-20260624-skills-domain-extraction-and-ai-table-standard.md
Date: 2026-06-24
Deciders: SDKWork maintainers
Specs: ARCHITECTURE_DECISION_SPEC.md, SDKWORK_WORKSPACE_SPEC.md, WEB_FRAMEWORK_SPEC.md, DATABASE_FRAMEWORK_SPEC.md, DISCOVERY_SPEC.md

## Context

`sdkwork-skills` is a public legacy repository that historically hosted app-surface agent
skills and Node.js helper scripts. Implementation has moved to `sdkwork-skills-private`, which
contains `sdkwork-skills-framework`, `sdkwork-skills-app`, `sdkwork-skills-development`, and
`sdkwork-skills-ops-admin`.

A standards alignment review asked whether this repository must integrate
`sdkwork-web-framework`, `sdkwork-database`, `@sdkwork/utils`, and `sdkwork-discovery`.

## Decision

1. **Keep `sdkwork-skills` as a narrow-purpose legacy pointer repository.** It retains
   `skills/<name>/SKILL.md` entrypoints that redirect to the canonical private implementation.
   Do not add new runtime logic, HTTP servers, database assets, or RPC services here.

2. **Do not integrate `sdkwork-web-framework` at this root.** The repository does not own,
   serve, develop, proxy, or compose any SDKWork HTTP `*-api` surface. HTTP client behavior
   belongs to `sdkwork-skills-private/sdkwork-skills-framework`.

3. **Do not integrate `sdkwork-database` at this root.** There is no owned persistence
   lifecycle, migrations, or seeds.

4. **Do not integrate `sdkwork-discovery` at this root.** There are no RPC/gRPC services and
   no dynamic endpoint resolution requirements.

5. **Do not add `@sdkwork/utils` at this root.** There is no authored TypeScript/JavaScript
   package or duplicate utility layer to optimize. Shared helpers live in the private
   framework and skill packs. Revisit only if this repository regains authored runtime code.

6. **Record exemptions in Canon architecture docs** and keep agent entrypoints accurate about
   the active directory layout and implementation authority.

## Consequences

Positive:

- Eliminates duplicate runtime code and drift between public and private repositories
- Makes standards applicability explicit for agents and reviewers
- Preserves backward-compatible skill names for existing links

Negative:

- Contributors must know to implement in `sdkwork-skills-private`, not this repository
- Public repository appears "empty" relative to full application roots

## Alternatives Considered

| Alternative | Why rejected |
| --- | --- |
| Move all runtime back into `sdkwork-skills` | Duplicates private monorepo; increases secret and release risk in a public repo |
| Convert this root into a full application root | No product surface, manifest, or deployable artifact justifies `sdkwork.app.config.json` |
| Integrate platform frameworks preemptively | Violates spec scope: frameworks apply when the capability exists |

## Verification

- [docs/architecture/tech/TECH_ARCHITECTURE.md](../TECH_ARCHITECTURE.md) standards matrix documents exemptions
- [docs/product/prd/PRD.md](../../product/prd/PRD.md) non-goals list matches this ADR
- `node ../sdkwork-specs/tools/check-repository-docs-standard.mjs --root .` passes

## Follow-up

- When adding RPC-backed skills in the private monorepo, integrate `sdkwork-discovery` there,
  not in this legacy pointer repository.
- When retiring legacy pointers, add a migration note under `docs/migrations/` and update
  `docs/archive/`.
