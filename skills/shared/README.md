# SDKWORK Skill Shared Core

Shared runtime modules for app-v3 skills:

- `sdkwork_skill_core/auth.py`
  - Unified auth persistence and login/register/refresh lifecycle
  - Auth file: `~/.sdkwork/user/auth.json`
  - Sensitive values are encrypted at rest
  - Supports transparent auto-register/login when auth context is missing
  - Security-aligned auth header aliases (`Authorization`, `AuthorizationT`, `T-Auth-Token`, `Auth-Token`, `Access-Token`, etc.)
  - Exposes `request_with_transparent_auth(...)` for cross-skill request reuse
- `sdkwork_skill_core/openapi.py`
  - Progressive OpenAPI 3.x endpoint loading
  - OpenAPI cache: `~/.sdkwork/cache/openapi-app-v3.json`
  - Supports cache clear/forced refresh via `clear_cache()`
  - Reports selected runtime OpenAPI URL in `openapiSource`
  - `check-openapi` mode can scan all candidates and emit `domainCheckedOpenapiSources`
  - Provides `domain_report(...)` to audit discovered vs fallback endpoints
- `sdkwork_skill_core/http.py`
  - Common HTTP + `PlusApiResult` unwrap helpers

## Import Pattern

```python
from pathlib import Path
import sys

SKILLS_ROOT = Path(__file__).resolve().parents[2]
SHARED_ROOT = SKILLS_ROOT / "shared"
if str(SHARED_ROOT) not in sys.path:
    sys.path.insert(0, str(SHARED_ROOT))

from sdkwork_skill_core import AuthClient, ProgressiveEndpointResolver, request_with_transparent_auth
```

This keeps authentication and endpoint resolution reusable across all skills.
