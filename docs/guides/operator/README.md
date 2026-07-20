# Operator Guide

Deployment, monitoring, and incident response for the Skills application root.

## Runtime Components

| Component | Role | Default local port |
| --- | --- | --- |
| `sdkwork-api-skills-standalone-gateway` | App + backend HTTP APIs | `18090` (app-api), `18091` (backend-api) |
| `apps/sdkwork-skills-pc` | Browser Hub / Console / Admin | Vite dev server (via `pnpm dev`) |
| PostgreSQL | `ai_*` persistence | Per `database/` config |

## Bootstrap And Migrate

```bash
pnpm db:bootstrap    # init + migrate + seed
pnpm db:status
pnpm db:drift-check
```

## Health And Readiness

Standalone gateway exposes:

- `/livez` — process alive
- `/readyz` — database readiness (when pool configured)
- `/healthz` — combined health summary

## Verification Before Release

```bash
pnpm verify
pnpm api:assembly:validate
pnpm topology:validate
node ../sdkwork-specs/tools/check-api-response-envelope.mjs --workspace .
```

## Packaging

Release workflow: repository `sdkwork.workflow.json` and `.github/workflows/package.yml`.
Container image id: `registry.sdkwork.com/apps/sdkwork-skills` (from `sdkwork.app.config.json`).

Authority: `DOCUMENTATION_SPEC.md` section 2, `DEPLOYMENT_SPEC.md`, `RELEASE_SPEC.md`.
