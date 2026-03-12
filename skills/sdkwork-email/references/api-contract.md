# SDKWORK Email + Auth OpenAPI 3.x Contract

## OpenAPI Source

Primary discovery endpoint:
- `GET /v3/api-docs/app`

Fallback endpoints:
- `GET /v3/api-docs/business`
- `GET /v3/api-docs/all`
- `GET /v3/api-docs?group=app|business|all`
- `GET /api/v3/api-docs/app`
- `GET /api/v3/api-docs/business`
- `GET /api/v3/api-docs/all`
- `GET /api/v3/api-docs?group=app|business|all`
- `GET /v3/api-docs`
- `GET /openapi.json`

This skill resolves endpoints dynamically from OpenAPI and falls back to stable defaults when docs are unavailable.

## Security Model (spring-ai-plus-security aligned)

Header extraction accepts these auth keys:
- `Authorization`
- `T-Auth-Token`
- `Auth-Token`
- `AuthorizationT`

For compatibility, client also sends:
- `Access-Token`
- `access-token`
- `access_token`
- `X-ACCESS-TOKEN`
- `AuthorizationT`
- `T-Auth-Token`
- `Auth-Token`

Token values may be either raw token or prefixed with `Bearer ` / `token `.

## Auth API Surface (`/app/v3/api/auth`)

Public endpoints:
1. `POST /login`
2. `POST /register`
3. `POST /refresh`
4. `POST /sms/send` and `POST /verify/send`
5. `POST /sms/verify` and `POST /verify/check`
6. `POST /password/reset`
7. `POST /qr/generate`
8. `GET /qr/status/{qrKey}`
9. `POST /phone/login`
10. `POST /oauth/url`
11. `POST /oauth/login`

Authenticated endpoint:
1. `POST /logout`

Canonical login request:

```json
{
  "username": "demo",
  "password": "123456",
  "captcha": null
}
```

Canonical register request:

```json
{
  "username": "demo",
  "password": "123456",
  "confirmPassword": "123456",
  "email": "demo@example.com",
  "phone": null,
  "type": "EMAIL",
  "verificationCode": null
}
```

## Email API Surface (`/app/v3/api/email`)

Authenticated endpoints:
1. `GET /account` (read-only, SaaS-managed channel account summary)
2. `POST /send`
3. `POST /receive`
4. `GET /messages`
5. `GET /messages/{messageId}`
6. `POST /messages/{messageId}/read`
7. `DELETE /messages/{messageId}`
8. `POST /sync`

Canonical send request:

```json
{
  "to": ["a@example.com", "b@example.com"],
  "cc": [],
  "bcc": [],
  "subject": "hello",
  "content": "email content",
  "contentType": "text/plain"
}
```

## Auth Persistence Contract

Path:
- `~/.sdkwork/user/auth.json`

Sensitive fields (`username/password/authToken/refreshToken`) are encrypted in storage.

Shape:

```json
{
  "base_url": "http://127.0.0.1:8080",
  "username": "demo",
  "password": "123456",
  "authToken": "xxx",
  "refreshToken": "yyy",
  "tokenType": "Bearer",
  "expiresIn": 7200,
  "savedAt": "2026-03-09T03:00:00Z"
}
```

## Progressive Loading Notes

- Endpoint domains (`auth`, `email`) are resolved lazily at command execution time.
- Resolver first maps by `operationId` (`auth__*` / `email__*`), then falls back to canonical path mapping.
- OpenAPI spec is cached under `~/.sdkwork/cache/openapi-app-v3.json` with TTL.
- Resolver report includes `openapiSource` to indicate which runtime OpenAPI endpoint was used.
- `check-openapi` enables cross-source scan and outputs `domainCheckedOpenapiSources` for full OpenAPI candidate trace.
- Use `--refresh-openapi` to clear local cache and force fresh fetch when diagnosing runtime mismatch.
- Missing OpenAPI does not block execution; static fallback paths stay available.
- All business requests should use shared `request_with_transparent_auth(...)` for reusable cross-skill auth handling.

Verification command:

```bash
python spring-ai-plus-app-api/skills/sdkwork-email/scripts/sdkwork_email_client.py check-openapi --base-url http://127.0.0.1:8080 --strict
```

Source + OpenAPI comparison:

```bash
python spring-ai-plus-app-api/skills/sdkwork-email/scripts/sdkwork_email_client.py check-openapi --base-url http://127.0.0.1:8080 --include-source
```

Force refresh OpenAPI cache before check:

```bash
python spring-ai-plus-app-api/skills/sdkwork-email/scripts/sdkwork_email_client.py check-openapi --base-url http://127.0.0.1:8080 --refresh-openapi --include-source
```
