---
name: sdkwork-email
description: "Build and operate SDKWORK app-v3 email capability with OpenAPI 3.x driven endpoint resolution, reusable shared auth module, and full lifecycle operations (account, send, receive, read, delete, sync). Auth context is persisted in ~/.sdkwork/user/auth.json."
---

# SDKWORK Email

## Overview

Implement and use a complete email lifecycle for app v3:
- Send outbound email through API.
- Receive/read/manage inbound and outbound messages through API.
- Inbox sync endpoint integration via `/app/v3/api/email/sync`.
- Email channel account is managed by SaaS backend (`PlusChannelAccount`), not by client payload.
- Reuse shared auth from `sdkwork-auth-core` and persist auth state to `~/.sdkwork/user/auth.json`.
- Resolve endpoint paths progressively from OpenAPI 3.x (`/v3/api-docs/app`) with safe fallback.

## Workflow

1. Load endpoint contract progressively from OpenAPI 3.x:
   - `GET /v3/api-docs/app` (primary)
   - `GET /v3/api-docs/business` / `GET /v3/api-docs/all` (group fallback)
   - `GET /v3/api-docs?group=app|business|all` (query fallback)
   - `GET /api/v3/api-docs/app` / `GET /api/v3/api-docs/business` / `GET /api/v3/api-docs/all` (legacy fallback)
   - `GET /api/v3/api-docs?group=app|business|all` (legacy query fallback)
   - `GET /v3/api-docs` (global fallback)
   - `GET /openapi.json` (exported spec fallback)
2. Resolve email/auth endpoints dynamically for the current environment.
3. Reuse shared auth module under `skills/shared/sdkwork_skill_core`:
   - register/login/refresh
   - shared transparent request helper (`request_with_transparent_auth`)
   - persistent auth context in `~/.sdkwork/user/auth.json`
4. Use `scripts/sdkwork_email_client.py` for operational flow:
   - Register/login/refresh
   - Send/receive/sync/list/read/delete messages
   - OpenAPI diagnostics with scanned source trace (`domainCheckedOpenapiSources`)
5. All business requests use transparent auth:
   - If no auth context exists, auto register + login
   - Local auth file persists encrypted sensitive fields
   - If token is invalid/expired, auto refresh and retry once
6. Keep all downstream skills on the same auth contract (no duplicated auth logic per skill).

## Auth Persistence Contract

Persist auth data at:
- `~/.sdkwork/user/auth.json`

Store at least:
- `base_url`
- `username`
- `password`
- `authToken`
- `refreshToken`
- `tokenType`
- `expiresIn`
- `savedAt`

Do not change this location unless explicitly requested.

## Script Usage

Run from repository root:

```bash
python spring-ai-plus-app-api/skills/sdkwork-email/scripts/sdkwork_email_client.py login --base-url http://127.0.0.1:8080 --username demo --password 123456
python spring-ai-plus-app-api/skills/sdkwork-email/scripts/sdkwork_email_client.py refresh-auth
python spring-ai-plus-app-api/skills/sdkwork-email/scripts/sdkwork_email_client.py send --to a@example.com,b@example.com --subject "hello" --content "hi"
python spring-ai-plus-app-api/skills/sdkwork-email/scripts/sdkwork_email_client.py sync --folder INBOX --max-messages 50
python spring-ai-plus-app-api/skills/sdkwork-email/scripts/sdkwork_email_client.py list --folder INBOX --unread-only
python spring-ai-plus-app-api/skills/sdkwork-email/scripts/sdkwork_email_client.py read --message-id 1
python spring-ai-plus-app-api/skills/sdkwork-email/scripts/sdkwork_email_client.py check-openapi --base-url http://127.0.0.1:8080 --include-source
python spring-ai-plus-app-api/skills/sdkwork-email/scripts/sdkwork_email_client.py check-openapi --base-url http://127.0.0.1:8080 --refresh-openapi --include-source
python spring-ai-plus-app-api/skills/sdkwork-email/scripts/sdkwork_email_client.py check-openapi --base-url http://127.0.0.1:8080 --strict
```

## Quality Gate

After changes:

```bash
mvn -pl spring-ai-plus-app-api -DskipTests test-compile
```

Use targeted tests when test execution is enabled. If the build skips tests globally, still keep/maintain unit tests for behavior locking.

## References

- API fields and examples: `references/api-contract.md`
- OpenAPI 3.x view: `references/openapi3-auth-email.md`
- Operational CLI: `scripts/sdkwork_email_client.py`
- Shared auth/runtime module: `../shared/sdkwork_skill_core`
- Shared auth skill: `../sdkwork-auth-core/SKILL.md`

