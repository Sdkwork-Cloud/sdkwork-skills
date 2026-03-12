# OpenAPI 3.x Endpoint Matrix (Auth + Email)

## Spec Endpoints

1. `GET /v3/api-docs/app` (preferred)
2. `GET /v3/api-docs/business` (group fallback)
3. `GET /v3/api-docs/all` (group fallback)
4. `GET /v3/api-docs?group=app|business|all` (query fallback)
5. `GET /api/v3/api-docs/app` (legacy fallback)
6. `GET /api/v3/api-docs/business` (legacy fallback)
7. `GET /api/v3/api-docs/all` (legacy fallback)
8. `GET /api/v3/api-docs?group=app|business|all` (legacy query fallback)
9. `GET /v3/api-docs` (global fallback)
10. `GET /openapi.json` (server exported OpenAPI file fallback)

## Server Base

- `http://<host>:<port>`

## Paths

### Auth (`/app/v3/api/auth`)

| Method | Path | Auth Required |
|---|---|---|
| POST | `/login` | No |
| POST | `/register` | No |
| POST | `/logout` | Yes |
| POST | `/refresh` | No |
| POST | `/sms/send` | No |
| POST | `/verify/send` | No |
| POST | `/sms/verify` | No |
| POST | `/verify/check` | No |
| POST | `/password/reset/request` | No |
| POST | `/password/reset` | No |
| POST | `/qr/generate` | No |
| GET | `/qr/status/{qrKey}` | No |
| GET | `/qr/entry/{qrKey}` | No |
| POST | `/qr/confirm` | Yes |
| POST | `/phone/login` | No |
| POST | `/oauth/url` | No |
| POST | `/oauth/login` | No |

### Email (`/app/v3/api/email`)

| Method | Path | Auth Required |
|---|---|---|
| GET | `/account` | Yes |
| POST | `/send` | Yes |
| POST | `/receive` | Yes |
| GET | `/messages` | Yes |
| GET | `/messages/{messageId}` | Yes |
| POST | `/messages/{messageId}/read` | Yes |
| DELETE | `/messages/{messageId}` | Yes |
| POST | `/sync` | Yes |

Note: skill runtime treats email account configuration as SaaS managed (`PlusChannelAccount`) and does not send account config payload from client side.

## Header Policy

Accepted token headers by backend parser:

1. `Authorization`
2. `T-Auth-Token`
3. `Auth-Token`
4. `AuthorizationT`

Compatibility headers commonly sent by skill:

1. `Authorization: Bearer <token>`
2. `AuthorizationT: Bearer <token>`
3. `T-Auth-Token: Bearer <token>`
4. `Auth-Token: Bearer <token>`
5. `Access-Token: <token>`
6. `access-token: <token>`
7. `access_token: <token>`
8. `X-ACCESS-TOKEN: <token>`

## OpenAPI Verification

```bash
python spring-ai-plus-app-api/skills/sdkwork-email/scripts/sdkwork_email_client.py check-openapi --base-url http://127.0.0.1:8080
```

Include source controller comparison (`src/main/java/com/sdkwork/ai/gateway/api/app/v3/{auth,email}`):

```bash
python spring-ai-plus-app-api/skills/sdkwork-email/scripts/sdkwork_email_client.py check-openapi --base-url http://127.0.0.1:8080 --include-source
```

Strict mode (fails when required endpoints are missing from OpenAPI discovery and only available by fallback):

```bash
python spring-ai-plus-app-api/skills/sdkwork-email/scripts/sdkwork_email_client.py check-openapi --base-url http://127.0.0.1:8080 --strict
```

Force refresh runtime OpenAPI (clear local cache first):

```bash
python spring-ai-plus-app-api/skills/sdkwork-email/scripts/sdkwork_email_client.py check-openapi --base-url http://127.0.0.1:8080 --refresh-openapi --include-source
```

Diagnostic fields in report:
- `openapiSource`: first successfully loaded OpenAPI URL.
- `domainOpenapiSource`: source that actually provided domain mappings.
- `domainCheckedOpenapiSources`: all OpenAPI candidate URLs scanned in diagnostic mode.
