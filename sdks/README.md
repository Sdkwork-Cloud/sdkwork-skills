# SDKWork Skills SDK Workspace

HTTP SDK families owned by the Skills application root.

| SDK Family | Authority | Consumer | Operations |
| --- | --- | --- | ---: |
| `sdkwork-skills-app-sdk` | `sdkwork-skills-app-api` | Authenticated app clients | 8 |
| `sdkwork-skills-backend-sdk` | `sdkwork-skills-backend-api` | Operator and admin clients | 16 |

## Commands

```bash
pnpm api:materialize
pnpm sdk:generate
pnpm api:check
```

OpenAPI authority lives in `apis/app-api/skills/` and
`apis/backend-api/skills/`. Generated TypeScript transport output lives under
each family's `*-typescript/generated/server-openapi/` directory.

Do not hand-edit generated SDK output. Change route handlers, contracts, or
`tools/skills_openapi_materialize.mjs`, then regenerate. A public/open SDK
family is intentionally absent because no Skills open-api product authority has
been approved.
