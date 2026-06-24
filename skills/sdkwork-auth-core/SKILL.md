---
name: sdkwork-auth-core
description: Use when an app skill against legacy-java-plus-app-api needs reusable login, refresh, per-app session storage, or backend-bound access-token headers.
---

# SDKWORK Auth Core

The active implementation is in `sdkwork-skills-private/sdkwork-skills-app`.

Use the shared script:

`sdkwork-skills-private/sdkwork-skills-app/.sdkwork-skills-app-shared/scripts/sdkwork_auth_client.mjs`

Runtime is provided by `sdkwork-skills-private/sdkwork-skills-framework`.

Persist state only under:

- `~/.sdkwork/app/<appId>/config.json`
- `~/.sdkwork/app/<appId>/profiles.json`
- `~/.sdkwork/app/<appId>/session.json`

Do not add auth client logic to this legacy public repository.
