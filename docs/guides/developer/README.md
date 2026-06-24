# Developer Guide

## Repository Role

`sdkwork-skills` is a legacy public pointer repository. Implement skills, CLI runtime, HTTP
clients, and tests in `sdkwork-skills-private`, not here.

## Where To Work

| Task | Repository / path |
| --- | --- |
| Add or change app auth/email clients | `sdkwork-skills-private/sdkwork-skills-app` |
| Shared profile/session/token runtime | `sdkwork-skills-private/sdkwork-skills-framework` |
| Development or conversion skills | `sdkwork-skills-private/sdkwork-skills-development` |
| Backend/ops admin skills | `sdkwork-skills-private/sdkwork-skills-ops-admin` |
| Update legacy public skill name only | `sdkwork-skills/skills/<skill-name>/SKILL.md` |

## Local Setup

No install step is required for this repository. Clone it for documentation and legacy skill
discovery only.

For runnable skill packs:

1. Open `sdkwork-skills-private`
2. Follow the README in the target pack (`sdkwork-skills-framework/INSTALL.md` for CLI usage)
3. Run pack tests with the pack's `package.json` scripts (typically `pnpm test`)

## Contribution Rules

- Do not add `package.json`, HTTP servers, database assets, or RPC services to this root
- Legacy `SKILL.md` files must name the canonical private implementation path
- Update [docs/architecture/tech/TECH_ARCHITECTURE.md](../architecture/tech/TECH_ARCHITECTURE.md) when
  the active directory layout or standards matrix changes
- Record boundary changes in `docs/architecture/decisions/`

## Verification

```bash
node ../../sdkwork-specs/tools/check-repository-docs-standard.mjs --root ../..
```

From repository root:

```bash
node ../sdkwork-specs/tools/check-repository-docs-standard.mjs --root .
```

## Related Specs

- `../sdkwork-specs/SDKWORK_WORKSPACE_SPEC.md`
- `../sdkwork-specs/DOCUMENTATION_SPEC.md`
- `../sdkwork-specs/TYPESCRIPT_CODE_SPEC.md` (private monorepo only)
