---
name: sdkwork-auth-core
description: Use when Skills PC or backend surfaces need IAM login, session tokens, or dual-token HTTP auth through SDKWork standards.
---

# SDKWORK Auth Core (Skills)

Skills PC auth is composed in `apps/sdkwork-skills-pc/src/bootstrap/iamRuntime.ts` via:

- `@sdkwork/auth-runtime-pc-react` for session lifecycle
- `@sdkwork/iam-app-sdk` for IAM app API calls
- Generated Skills app/backend SDK clients with `AuthTokenManager`

Do not add ad-hoc auth headers or raw HTTP login flows in Skills packages. Follow `sdkwork-specs/IAM_LOGIN_INTEGRATION_SPEC.md` and `sdkwork-specs/APP_SDK_INTEGRATION_SPEC.md`.
