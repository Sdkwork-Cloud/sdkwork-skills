# SDKWork Skills SDK Workspace

HTTP SDK families for the Skills application root.

| SDK family | Authority | Consumer |
| --- | --- | --- |
| `sdkwork-skills-app-sdk` | `sdkwork-skills.app` | Hub, Console (user-facing) |
| `sdkwork-skills-backend-sdk` | `sdkwork-skills.backend` | Admin (operator) |

## Commands

```bash
pnpm api:materialize   # Regenerate OpenAPI authority from tools/skills_openapi_materialize.mjs
pnpm sdk:generate      # Materialize OpenAPI + run canonical sdkgen
pnpm api:check         # Contract + schema quality gate + sdkgen dry check
```

OpenAPI authority lives in `apis/app-api/skills/` and `apis/backend-api/skills/`.
Generated TypeScript transport output lives under each family’s `*-typescript/generated/server-openapi/`.

Do not hand-edit generated SDK output. Change `tools/skills_openapi_materialize.mjs`, route handlers, or contracts, then regenerate.
