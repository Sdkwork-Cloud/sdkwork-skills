---
name: sdkwork-auth-core
description: "Reusable app-v3 auth skill for login/register/refresh with dynamic OpenAPI 3.x endpoint discovery and persistent auth storage at ~/.sdkwork/user/auth.json. Use this skill when any other skill needs shared authentication/authorization."
---

# SDKWORK Auth Core

## Purpose

Provide one reusable authentication module for all app-v3 skills:
- Login and register against `/app/v3/api/auth/*`
- Persist auth context in `~/.sdkwork/user/auth.json`
- Encrypt sensitive auth fields in local storage
- Refresh tokens
- Output standard request headers for downstream skills
- Resolve auth endpoints progressively from OpenAPI 3.x at runtime

## Runtime Contract

- Auth file path: `~/.sdkwork/user/auth.json`
- OpenAPI discovery order:
  1. `/v3/api-docs/app`
  2. `/api/v3/api-docs/app`
  3. `/v3/api-docs`
- If OpenAPI is unavailable, fallback to stable default endpoint paths.

## Usage

```bash
python spring-ai-plus-app-api/skills/sdkwork-auth-core/scripts/sdkwork_auth_client.py login --base-url http://127.0.0.1:8080 --username demo --password 123456
python spring-ai-plus-app-api/skills/sdkwork-auth-core/scripts/sdkwork_auth_client.py refresh
python spring-ai-plus-app-api/skills/sdkwork-auth-core/scripts/sdkwork_auth_client.py headers
```

## Reuse From Other Skills

Other skill scripts should import shared modules from:
- `spring-ai-plus-app-api/skills/shared/sdkwork_skill_core`

Core reusable pieces:
- `AuthClient` (login/register/refresh/persist)
- `ProgressiveEndpointResolver` (OpenAPI 3.x dynamic progressive loading)
- `request_json` + `unwrap_plus_result` (uniform API invocation)
- `request_with_transparent_auth` (transparent login/register/refresh + retry)

## References

- OpenAPI contract: `references/openapi3-auth.md`
- Shared module: `../shared/sdkwork_skill_core`
