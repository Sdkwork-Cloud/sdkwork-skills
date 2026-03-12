from __future__ import annotations

import json
from datetime import datetime, timedelta, timezone
from pathlib import Path
from typing import Dict, Mapping, Optional
from urllib import error, request

from .http import normalize_base_url


DEFAULT_DOMAIN_ENDPOINTS: Dict[str, Dict[str, str]] = {
    "auth": {
        "login": "/app/v3/api/auth/login",
        "register": "/app/v3/api/auth/register",
        "logout": "/app/v3/api/auth/logout",
        "refresh": "/app/v3/api/auth/refresh",
        "verify_send": "/app/v3/api/auth/verify/send",
        "verify_check": "/app/v3/api/auth/verify/check",
        "password_reset_request": "/app/v3/api/auth/password/reset/request",
        "password_reset": "/app/v3/api/auth/password/reset",
        "qr_generate": "/app/v3/api/auth/qr/generate",
        "qr_status": "/app/v3/api/auth/qr/status/{qrKey}",
        "qr_entry": "/app/v3/api/auth/qr/entry/{qrKey}",
        "qr_confirm": "/app/v3/api/auth/qr/confirm",
        "phone_login": "/app/v3/api/auth/phone/login",
        "oauth_url": "/app/v3/api/auth/oauth/url",
        "oauth_login": "/app/v3/api/auth/oauth/login",
    },
    "email": {
        "account_get": "/app/v3/api/email/account",
        "send": "/app/v3/api/email/send",
        "receive": "/app/v3/api/email/receive",
        "message_list": "/app/v3/api/email/messages",
        "message_get": "/app/v3/api/email/messages/{messageId}",
        "message_read": "/app/v3/api/email/messages/{messageId}/read",
        "message_delete": "/app/v3/api/email/messages/{messageId}",
        "sync": "/app/v3/api/email/sync",
    },
}

_OPENAPI_PATHS = (
    "/v3/api-docs/app",
    "/v3/api-docs/business",
    "/v3/api-docs/all",
    "/v3/api-docs?group=app",
    "/v3/api-docs?group=business",
    "/v3/api-docs?group=all",
    "/api/v3/api-docs/app",
    "/api/v3/api-docs/business",
    "/api/v3/api-docs/all",
    "/api/v3/api-docs?group=app",
    "/api/v3/api-docs?group=business",
    "/api/v3/api-docs?group=all",
    "/v3/api-docs",
    "/openapi.json",
    "/api/openapi.json",
)


class ProgressiveEndpointResolver:
    """
    Load OpenAPI spec lazily and progressively resolve endpoint paths by domain.

    - Lazy: fetches OpenAPI only when a domain is first requested.
    - Progressive: only resolves the requested domain (auth/email/...) and caches it.
    - Safe fallback: when OpenAPI is unavailable, returns static defaults.
    """

    def __init__(
        self,
        base_url: str,
        cache_file: Optional[Path] = None,
        ttl_seconds: int = 600,
        scan_all_sources_on_gap: bool = False,
    ) -> None:
        self.base_url = normalize_base_url(base_url)
        self.cache_file = cache_file or (Path.home() / ".sdkwork" / "cache" / "openapi-app-v3.json")
        self.ttl_seconds = ttl_seconds
        self.scan_all_sources_on_gap = scan_all_sources_on_gap
        self._spec: Optional[Dict[str, object]] = None
        self._openapi_source: Optional[str] = None
        self._spec_by_source: Dict[str, Dict[str, object]] = {}
        self._resolved_domains: Dict[str, Dict[str, str]] = {}
        self._discovered_domains: Dict[str, Dict[str, str]] = {}
        self._domain_openapi_source: Dict[str, Optional[str]] = {}
        self._domain_checked_sources: Dict[str, list[str]] = {}

    def resolve(self, domain: str, endpoint_key: str) -> str:
        domain_map = self._resolve_domain(domain)
        if endpoint_key in domain_map:
            return domain_map[endpoint_key]
        defaults = DEFAULT_DOMAIN_ENDPOINTS.get(domain, {})
        if endpoint_key in defaults:
            return defaults[endpoint_key]
        raise KeyError(f"Unknown endpoint key '{endpoint_key}' for domain '{domain}'")

    def resolve_domain(self, domain: str) -> Mapping[str, str]:
        return dict(self._resolve_domain(domain))

    def domain_report(self, domain: str) -> Dict[str, object]:
        resolved = self._resolve_domain(domain)
        discovered = dict(self._discovered_domains.get(domain, {}))
        required = set(DEFAULT_DOMAIN_ENDPOINTS.get(domain, {}).keys())
        return {
            "baseUrl": self.base_url,
            "domain": domain,
            "openapiLoaded": self._load_openapi_spec() is not None,
            "openapiSource": self._openapi_source,
            "domainOpenapiSource": self._domain_openapi_source.get(domain),
            "domainCheckedOpenapiSources": self._domain_checked_sources.get(domain, []),
            "resolved": dict(resolved),
            "discovered": discovered,
            "requiredKeys": sorted(required),
            "missingRequiredKeys": sorted(required - set(resolved.keys())),
            "resolvedByOpenapi": sorted(set(discovered.keys())),
            "resolvedByFallback": sorted(set(resolved.keys()) - set(discovered.keys())),
        }

    def clear_cache(self) -> None:
        self._spec = None
        self._openapi_source = None
        self._spec_by_source.clear()
        self._resolved_domains.clear()
        self._discovered_domains.clear()
        self._domain_openapi_source.clear()
        self._domain_checked_sources.clear()
        try:
            if self.cache_file.exists():
                self.cache_file.unlink()
        except Exception:
            return

    def _resolve_domain(self, domain: str) -> Dict[str, str]:
        if domain in self._resolved_domains:
            return self._resolved_domains[domain]
        defaults = dict(DEFAULT_DOMAIN_ENDPOINTS.get(domain, {}))
        resolved = dict(defaults)
        discovered: Dict[str, str] = {}
        discovered_source: Optional[str] = None
        required_keys = set(defaults.keys())
        checked_sources: list[str] = []
        spec = self._load_openapi_spec()
        if spec:
            paths = spec.get("paths")
            if isinstance(paths, dict):
                discovered = self._discover_domain_paths(domain, paths)
                discovered_source = self._openapi_source
                if self._openapi_source:
                    checked_sources.append(self._openapi_source)
        if self.scan_all_sources_on_gap and required_keys and len(discovered) < len(required_keys):
            discovered, discovered_source, scanned_sources = self._discover_domain_with_additional_specs(
                domain=domain,
                current_discovered=discovered,
                current_source=discovered_source,
                required_keys=required_keys,
            )
            checked_sources.extend(scanned_sources)
        resolved.update(discovered)
        self._discovered_domains[domain] = discovered
        self._domain_openapi_source[domain] = discovered_source
        self._domain_checked_sources[domain] = list(dict.fromkeys(checked_sources))
        self._resolved_domains[domain] = resolved
        return resolved

    def _discover_domain_with_additional_specs(
        self,
        domain: str,
        current_discovered: Dict[str, str],
        current_source: Optional[str],
        required_keys: set[str],
    ) -> tuple[Dict[str, str], Optional[str], list[str]]:
        best_discovered = dict(current_discovered)
        best_source = current_source
        exclude_sources = {current_source} if current_source else set()
        scanned_sources = [
            f"{self.base_url}{path}"
            for path in _OPENAPI_PATHS
            if f"{self.base_url}{path}" not in exclude_sources
        ]
        for source, spec in self._iter_openapi_specs(exclude_sources=exclude_sources):
            paths = spec.get("paths")
            if not isinstance(paths, dict):
                continue
            candidate = self._discover_domain_paths(domain, paths)
            if len(candidate) > len(best_discovered):
                best_discovered = candidate
                best_source = source
            if required_keys.issubset(candidate.keys()):
                return candidate, source, list(dict.fromkeys(scanned_sources))
        return best_discovered, best_source, list(dict.fromkeys(scanned_sources))

    def _load_openapi_spec(self, force_refresh: bool = False) -> Optional[Dict[str, object]]:
        if self._spec is not None and not force_refresh:
            return self._spec
        if force_refresh:
            self._spec = None
            self._openapi_source = None
        else:
            cached_payload = self._load_cached_spec()
            if cached_payload is not None:
                cached, source = cached_payload
                self._spec = cached
                self._openapi_source = source
                if source:
                    self._spec_by_source[source] = cached
                return cached
        fetched = self._fetch_openapi_spec()
        if fetched is not None:
            spec, source = fetched
            self._spec = spec
            self._openapi_source = source
            self._spec_by_source[source] = spec
            self._save_cache(spec, source)
            return spec
        return None

    def _load_cached_spec(self) -> Optional[tuple[Dict[str, object], Optional[str]]]:
        if not self.cache_file.exists():
            return None
        try:
            content = json.loads(self.cache_file.read_text(encoding="utf-8"))
            fetched_at = content.get("fetchedAt")
            spec = content.get("spec")
            source_url = content.get("sourceUrl")
            if not isinstance(fetched_at, str) or not isinstance(spec, dict):
                return None
            if source_url is not None and not isinstance(source_url, str):
                source_url = None
            fetched_time = datetime.fromisoformat(fetched_at)
            if datetime.now(timezone.utc) - fetched_time > timedelta(seconds=self.ttl_seconds):
                return None
            return spec, source_url
        except Exception:
            return None

    def _save_cache(self, spec: Dict[str, object], source_url: Optional[str]) -> None:
        try:
            self.cache_file.parent.mkdir(parents=True, exist_ok=True)
            payload = {
                "fetchedAt": datetime.now(timezone.utc).isoformat(),
                "sourceUrl": source_url,
                "spec": spec,
            }
            self.cache_file.write_text(json.dumps(payload, ensure_ascii=False, indent=2), encoding="utf-8")
        except Exception:
            # Cache write failures should not break skill execution.
            return

    def _iter_openapi_specs(self, exclude_sources: set[str]) -> list[tuple[str, Dict[str, object]]]:
        specs: list[tuple[str, Dict[str, object]]] = []
        for path in _OPENAPI_PATHS:
            source = f"{self.base_url}{path}"
            if source in exclude_sources:
                continue
            cached = self._spec_by_source.get(source)
            if cached is not None:
                specs.append((source, cached))
                continue
            fetched = self._fetch_openapi_spec_by_url(source)
            if fetched is None:
                continue
            spec, resolved_source = fetched
            self._spec_by_source[resolved_source] = spec
            specs.append((resolved_source, spec))
        return specs

    def _fetch_openapi_spec_by_url(self, source_url: str) -> Optional[tuple[Dict[str, object], str]]:
        req = request.Request(url=source_url, method="GET", headers={"Accept": "application/json"})
        try:
            with request.urlopen(req, timeout=15) as resp:
                raw = resp.read().decode("utf-8", errors="replace")
                body = json.loads(raw)
                if isinstance(body, dict) and "paths" in body:
                    return body, source_url
        except (error.HTTPError, error.URLError, json.JSONDecodeError, TimeoutError):
            return None
        return None

    def _fetch_openapi_spec(self) -> Optional[tuple[Dict[str, object], str]]:
        for path in _OPENAPI_PATHS:
            url = f"{self.base_url}{path}"
            cached = self._spec_by_source.get(url)
            if cached is not None:
                return cached, url
            fetched = self._fetch_openapi_spec_by_url(url)
            if fetched is None:
                continue
            spec, source = fetched
            self._spec_by_source[source] = spec
            return spec, source
        return None

    def _discover_domain_paths(self, domain: str, paths: Dict[str, object]) -> Dict[str, str]:
        resolved: Dict[str, str] = {}
        prefix = f"/app/v3/api/{domain}/"
        root = f"/app/v3/api/{domain}"
        for path, item in paths.items():
            if not isinstance(path, str):
                continue
            if not (path == root or path.startswith(prefix)):
                continue
            if not isinstance(item, dict):
                continue
            for method in ("get", "post", "put", "patch", "delete"):
                operation = item.get(method)
                if not isinstance(operation, dict):
                    continue
                key = self._to_endpoint_key(domain, method, path, operation)
                if key:
                    resolved[key] = path
        return resolved

    def _to_endpoint_key(self, domain: str, method: str, path: str, operation: Mapping[str, object]) -> Optional[str]:
        operation_id = operation.get("operationId")
        operation_id_str = str(operation_id) if isinstance(operation_id, str) else None
        if domain == "auth":
            return self._auth_key(method, path, operation_id_str)
        if domain == "email":
            return self._email_key(method, path, operation_id_str)
        return None

    @staticmethod
    def _auth_key(method: str, path: str, operation_id: Optional[str]) -> Optional[str]:
        by_operation_id = ProgressiveEndpointResolver._auth_key_by_operation_id(operation_id)
        if by_operation_id:
            return by_operation_id
        mapping = {
            ("post", "/app/v3/api/auth/login"): "login",
            ("post", "/app/v3/api/auth/register"): "register",
            ("post", "/app/v3/api/auth/logout"): "logout",
            ("post", "/app/v3/api/auth/refresh"): "refresh",
            ("post", "/app/v3/api/auth/sms/send"): "verify_send",
            ("post", "/app/v3/api/auth/verify/send"): "verify_send",
            ("post", "/app/v3/api/auth/sms/verify"): "verify_check",
            ("post", "/app/v3/api/auth/verify/check"): "verify_check",
            ("post", "/app/v3/api/auth/password/reset/request"): "password_reset_request",
            ("post", "/app/v3/api/auth/password/reset"): "password_reset",
            ("post", "/app/v3/api/auth/qr/generate"): "qr_generate",
            ("get", "/app/v3/api/auth/qr/status/{qrKey}"): "qr_status",
            ("get", "/app/v3/api/auth/qr/entry/{qrKey}"): "qr_entry",
            ("post", "/app/v3/api/auth/qr/confirm"): "qr_confirm",
            ("post", "/app/v3/api/auth/phone/login"): "phone_login",
            ("post", "/app/v3/api/auth/oauth/url"): "oauth_url",
            ("post", "/app/v3/api/auth/oauth/login"): "oauth_login",
        }
        return mapping.get((method, path))

    @staticmethod
    def _email_key(method: str, path: str, operation_id: Optional[str]) -> Optional[str]:
        by_operation_id = ProgressiveEndpointResolver._email_key_by_operation_id(operation_id)
        if by_operation_id:
            return by_operation_id
        mapping = {
            ("get", "/app/v3/api/email/account"): "account_get",
            ("post", "/app/v3/api/email/send"): "send",
            ("post", "/app/v3/api/email/receive"): "receive",
            ("get", "/app/v3/api/email/messages"): "message_list",
            ("get", "/app/v3/api/email/messages/{messageId}"): "message_get",
            ("post", "/app/v3/api/email/messages/{messageId}/read"): "message_read",
            ("delete", "/app/v3/api/email/messages/{messageId}"): "message_delete",
            ("post", "/app/v3/api/email/sync"): "sync",
        }
        return mapping.get((method, path))

    @staticmethod
    def _auth_key_by_operation_id(operation_id: Optional[str]) -> Optional[str]:
        if not operation_id:
            return None
        normalized = operation_id.strip().lower()
        suffix = normalized.split("__", 1)[1] if "__" in normalized else normalized
        mapping = {
            "login": "login",
            "register": "register",
            "logout": "logout",
            "refresh": "refresh",
            "sendsms": "verify_send",
            "sendverifycode": "verify_send",
            "verifysms": "verify_check",
            "checkverifycode": "verify_check",
            "passwordresetrequest": "password_reset_request",
            "passwordreset": "password_reset",
            "generateqrcode": "qr_generate",
            "checkqrcodestatus": "qr_status",
            "qrauthentry": "qr_entry",
            "confirmqrcodelogin": "qr_confirm",
            "phonelogin": "phone_login",
            "getoauthurl": "oauth_url",
            "oauthlogin": "oauth_login",
        }
        return mapping.get(suffix)

    @staticmethod
    def _email_key_by_operation_id(operation_id: Optional[str]) -> Optional[str]:
        if not operation_id:
            return None
        normalized = operation_id.strip().lower()
        suffix = normalized.split("__", 1)[1] if "__" in normalized else normalized
        mapping = {
            "getaccountconfig": "account_get",
            "send": "send",
            "receive": "receive",
            "listmessages": "message_list",
            "getmessage": "message_get",
            "markread": "message_read",
            "deletemessage": "message_delete",
            "sync": "sync",
        }
        return mapping.get(suffix)
