# Repository Guidelines

<!-- SDKWORK-AGENTS-GENERATED: v1 -->

## SDKWORK Soul

Read `../sdkwork-specs/SOUL.md` before executing tasks in this root. Follow specs before memory, dictionary before context, stop on ambiguity, and evidence before completion.

## SDKWORK Standards

Canonical SDKWORK specs path from this root:

- `../sdkwork-specs/README.md`
- `../sdkwork-specs/SOUL.md`
- `../sdkwork-specs/AGENTS_SPEC.md`
- `../sdkwork-specs/CODE_STYLE_SPEC.md`
- `../sdkwork-specs/NAMING_SPEC.md`

## Application Identity

This root is the **SDKWork Skills application** (`sdkwork.app.config.json`, `key: sdkwork-skills`).
PC surface: `apps/sdkwork-skills-pc/` (`sdkwork-skills-pc`).

Skills marketplace system-of-record uses `ai_*` tables under `database/`. Kernel must consume
`sdkwork-skills-contract` and skills APIs instead of local `a_agent_skill_package` persistence.

## Local Dictionary Structure

- `sdkwork.app.config.json`: application manifest.
- `apps/sdkwork-skills-pc/`: PC React Hub/Console/Admin client.
- `apis/`, `sdks/`, `crates/`, `database/`: backend contracts and runtime.
- `skills/`: optional static skill pack pointers (non-authoritative).
- `specs/component.spec.json`: application-root component contract.
- `.sdkwork/`: workspace metadata.

## Platform Framework Integration

| Framework | Required | Path |
| --- | --- | --- |
| `sdkwork-web-framework` | Yes | `crates/sdkwork-router-skills-*` |
| `sdkwork-database` | Yes | `database/` |
| `@sdkwork/utils` | Yes (PC) | `apps/sdkwork-skills-pc` |
| `sdkwork-discovery` | No (until RPC) | deferred |

See [ADR-20260624-skills-domain-extraction-and-ai-table-standard](docs/architecture/decisions/ADR-20260624-skills-domain-extraction-and-ai-table-standard.md).

## Documentation Canon

- [docs/README.md](docs/README.md)
- [docs/product/prd/PRD.md](docs/product/prd/PRD.md)
- [docs/architecture/tech/TECH_ARCHITECTURE.md](docs/architecture/tech/TECH_ARCHITECTURE.md)

## Build, Test, and Verification

```bash
pnpm verify
cargo test --workspace
node ../sdkwork-specs/tools/check-repository-docs-standard.mjs --root .
```

## Agent Execution Rules

Use the convention dictionary instead of broad context loading. Keep skill persistence and CRUD in
this repository; do not reintroduce skill tables in `sdkwork-kernel`.
