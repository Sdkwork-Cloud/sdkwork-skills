# App V3 Auth OpenAPI 3.x Contract

## OpenAPI Source

- Primary spec endpoint: `GET /v3/api-docs/app`
- Alternate spec endpoints:
  - `GET /api/v3/api-docs/app`
  - `GET /v3/api-docs`

## Base Path

- `/app/v3/api/auth`

## Public Endpoints (No Auth Token Required)

1. `POST /app/v3/api/auth/login`
2. `POST /app/v3/api/auth/register`
3. `POST /app/v3/api/auth/refresh`
4. `POST /app/v3/api/auth/sms/send`
5. `POST /app/v3/api/auth/sms/verify`
6. `POST /app/v3/api/auth/password/reset`
7. `POST /app/v3/api/auth/qr/generate`
8. `GET /app/v3/api/auth/qr/status/{qrKey}`
9. `POST /app/v3/api/auth/phone/login`
10. `POST /app/v3/api/auth/oauth/url`
11. `POST /app/v3/api/auth/oauth/login`

These routes are explicitly listed in `spring-ai-plus-security` permit rules.

## Common Protected Endpoint

1. `POST /app/v3/api/auth/logout`

## Security Header Compatibility

Token extraction in `spring-ai-plus-security` accepts:

- `Authorization: Bearer <token>`
- `Authorization: <token>`
- `T-Auth-Token: <token>`
- `Auth-Token: <token>`

`Bearer ` and `token ` prefixes are normalized by backend utility parsing.

## Canonical Request Bodies

### Login

```json
{
  "username": "demo",
  "password": "123456",
  "captcha": null
}
```

### Register

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

### Refresh

```json
{
  "refreshToken": "xxx"
}
```

## Canonical Auth File

`~/.sdkwork/user/auth.json`

```json
{
  "base_url": "http://127.0.0.1:8080",
  "tokenType": "Bearer",
  "expiresIn": 7200,
  "savedAt": "2026-03-09T03:00:00Z",
  "encrypted": true,
  "encVersion": 1,
  "sensitive": {
    "username": "<encrypted>",
    "password": "<encrypted>",
    "authToken": "<encrypted>",
    "refreshToken": "<encrypted>"
  }
}
```
