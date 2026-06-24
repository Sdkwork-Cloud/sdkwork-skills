# Integrator Guide

## Overview

Integrators consuming SDKWork app-surface skills should use the **private implementation
monorepo** `sdkwork-skills-private`, not this legacy public pointer.

## Canonical Skill Packs

| Surface | Pack | External API target |
| --- | --- | --- |
| App | `sdkwork-skills-app` | `legacy-java-plus-app-api` |
| Backend/ops | `sdkwork-skills-ops-admin` | `legacy-java-plus-backend-api` |

Both packs share `sdkwork-skills-framework` for:

- profile and session JSON storage under `~/.sdkwork/`
- dual-token request headers and refresh recovery
- CLI commands for login, preview, and authenticated requests

## Runtime State Contract

App-scoped state:

```text
~/.sdkwork/app/<appId>/
  config.json
  profiles.json
  session.json
```

Backend-scoped state:

```text
~/.sdkwork/backend/
  config.json
  profiles.json
  session.json
```

Governed by `RUNTIME_DIRECTORY_SPEC.md`.

## Legacy Public Pointers

If integration docs still link to `sdkwork-skills/skills/*`, follow the redirect in each
`SKILL.md` to the canonical script under `sdkwork-skills-app` in the private monorepo.

## API Integration

- Use generated SDKs and framework HTTP adapters in the private monorepo; do not add raw HTTP
  wrappers in this repository
- Protected calls must follow `SECURITY_SPEC.md` and `IAM_LOGIN_INTEGRATION_SPEC.md` through
  the framework token resolver

## Not Applicable At This Root

- `sdkwork-web-framework` (no owned HTTP server)
- `sdkwork-database` (no persistence)
- `sdkwork-discovery` (no RPC; integrate in private monorepo when RPC skills are added)
- `@sdkwork/utils` (no authored TypeScript package here)

See [TECH_ARCHITECTURE.md](../../architecture/tech/TECH_ARCHITECTURE.md) section 8.
