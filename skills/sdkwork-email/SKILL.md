---
name: sdkwork-email
description: Use when an app workflow against legacy-java-plus-app-api needs SDKWORK email operations through the shared app auth runtime.
---

# SDKWORK Email

The active implementation is in `sdkwork-skills-private/sdkwork-skills-app`.

Use the shared script:

`sdkwork-skills-private/sdkwork-skills-app/.sdkwork-skills-app-shared/scripts/sdkwork_email_client.mjs`

Reuse the same app-scoped state under `~/.sdkwork/app/<appId>/` via
`sdkwork-skills-private/sdkwork-skills-framework`.

Do not add email client logic to this legacy public repository.
