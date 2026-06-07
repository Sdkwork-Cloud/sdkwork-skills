---
name: sdkwork-auth-core
description: Use when an app skill against legacy-java-plus-app-api needs reusable login, refresh, per-app session storage, or backend-bound access-token headers.
---

# SDKWORK Auth Core

The active implementation moved to the top-level `sdkwork-skills-app` pack.

Use:

`installedSkillDir/../.sdkwork-skills-app-shared/scripts/sdkwork_auth_client.mjs`

and persist state only under:

- `~/.sdkwork/app/<appId>/config.json`
- `~/.sdkwork/app/<appId>/profiles.json`
- `~/.sdkwork/app/<appId>/session.json`
