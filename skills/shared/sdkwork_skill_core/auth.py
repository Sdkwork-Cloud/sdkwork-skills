from __future__ import annotations

import json
import random
import string
from dataclasses import dataclass
from datetime import datetime, timezone
from pathlib import Path
from typing import Any, Dict, Mapping, Optional

from .crypto import decrypt_text, encrypt_text, load_or_create_key
from .http import normalize_base_url, request_json, unwrap_plus_result
from .openapi import ProgressiveEndpointResolver


DEFAULT_AUTH_FILE = Path.home() / ".sdkwork" / "user" / "auth.json"
AUTH_ERROR_MARKERS = (
    "code=401",
    "code=4010",
    "code=4011",
    "401 unauthorized",
    "authentication failed",
    "token",
    "not login",
    "not logged in",
    "unauthorized",
    "http 401",
)
AUTH_HEADER_ALIASES = (
    "Authorization",
    "AuthorizationT",
    "T-Auth-Token",
    "Auth-Token",
)
ACCESS_HEADER_ALIASES = (
    "Access-Token",
    "access-token",
    "access_token",
    "X-ACCESS-TOKEN",
)

@dataclass
class AuthContext:
    base_url: str
    username: Optional[str]
    password: Optional[str]
    auth_token: Optional[str]
    refresh_token: Optional[str]
    token_type: str
    expires_in: Optional[int]
    saved_at: Optional[str]

    def require_token(self) -> str:
        if not self.auth_token:
            raise RuntimeError("authToken is missing. Run login/register first.")
        return self.auth_token

    def auth_header_value(self) -> str:
        token = self.require_token()
        token_type = self.token_type or "Bearer"
        return f"{token_type} {token}".strip()


class AuthStore:
    def __init__(self, auth_file: Path = DEFAULT_AUTH_FILE) -> None:
        self.auth_file = auth_file
        self._key = load_or_create_key()

    def load(self, required: bool = True) -> Optional[Dict[str, Any]]:
        if not self.auth_file.exists():
            if required:
                raise RuntimeError(f"Auth file does not exist: {self.auth_file}")
            return None
        try:
            raw = json.loads(self.auth_file.read_text(encoding="utf-8"))
        except json.JSONDecodeError as exc:
            raise RuntimeError(f"Auth file JSON is invalid: {exc}") from exc
        return self._decrypt_if_needed(raw)

    def save(self, auth_data: Dict[str, Any]) -> None:
        self.auth_file.parent.mkdir(parents=True, exist_ok=True)
        payload = self._encrypt_payload(auth_data)
        self.auth_file.write_text(json.dumps(payload, ensure_ascii=False, indent=2), encoding="utf-8")

    def _encrypt_payload(self, auth_data: Dict[str, Any]) -> Dict[str, Any]:
        payload = dict(auth_data)
        encrypted_fields = {}
        for field in ("username", "password", "authToken", "refreshToken"):
            value = payload.pop(field, None)
            if value is not None:
                encrypted_fields[field] = encrypt_text(str(value), self._key)
        payload["encrypted"] = True
        payload["encVersion"] = 1
        payload["sensitive"] = encrypted_fields
        return payload

    def _decrypt_if_needed(self, raw: Dict[str, Any]) -> Dict[str, Any]:
        if not isinstance(raw, dict):
            raise RuntimeError("Auth file JSON must be an object")
        if not raw.get("encrypted"):
            return raw
        payload = dict(raw)
        sensitive = payload.pop("sensitive", {})
        if isinstance(sensitive, dict):
            for field, cipher in sensitive.items():
                if cipher is None:
                    payload[field] = None
                else:
                    payload[field] = decrypt_text(str(cipher), self._key)
        payload.pop("encrypted", None)
        payload.pop("encVersion", None)
        return payload


class AuthClient:
    def __init__(
        self,
        base_url: Optional[str] = None,
        auth_file: Path = DEFAULT_AUTH_FILE,
        endpoint_resolver: Optional[ProgressiveEndpointResolver] = None,
    ) -> None:
        self.store = AuthStore(auth_file=auth_file)
        self._base_url = normalize_base_url(base_url or "")
        self.resolver = endpoint_resolver or ProgressiveEndpointResolver(self._base_url or "http://127.0.0.1:8080")

    @property
    def auth_file(self) -> Path:
        return self.store.auth_file

    def load_context(self, required: bool = True, base_url_override: Optional[str] = None) -> Optional[AuthContext]:
        raw = self.store.load(required=required)
        if raw is None:
            return None
        base_url = normalize_base_url(base_url_override or raw.get("base_url") or self._base_url)
        return self._to_context(raw, base_url=base_url)

    def login(self, username: str, password: str, captcha: Optional[str] = None, base_url: Optional[str] = None) -> AuthContext:
        final_base_url = self._resolve_base_url(base_url)
        payload: Dict[str, Any] = {"username": username, "password": password}
        if captcha:
            payload["captcha"] = captcha
        path = self.resolver.resolve("auth", "login")
        result = request_json("POST", final_base_url, path, payload=payload)
        data = unwrap_plus_result(result) or {}
        context = self._to_context(
            {
                "base_url": final_base_url,
                "username": username,
                "password": password,
                "authToken": data.get("authToken"),
                "refreshToken": data.get("refreshToken"),
                "tokenType": data.get("tokenType") or "Bearer",
                "expiresIn": data.get("expiresIn"),
                "savedAt": datetime.now(timezone.utc).isoformat(),
            },
            base_url=final_base_url,
        )
        self.store.save(self._from_context(context))
        return context

    def register_and_login(
        self,
        username: str,
        password: str,
        confirm_password: Optional[str] = None,
        email: Optional[str] = None,
        phone: Optional[str] = None,
        register_type: Optional[str] = None,
        verification_code: Optional[str] = None,
        base_url: Optional[str] = None,
    ) -> Dict[str, Any]:
        final_base_url = self._resolve_base_url(base_url)
        payload: Dict[str, Any] = {
            "username": username,
            "password": password,
            "confirmPassword": confirm_password or password,
        }
        if email:
            payload["email"] = email
        if phone:
            payload["phone"] = phone
        if register_type:
            payload["type"] = register_type
        if verification_code:
            payload["verificationCode"] = verification_code
        register_path = self.resolver.resolve("auth", "register")
        register_result = request_json("POST", final_base_url, register_path, payload=payload)
        register_data = unwrap_plus_result(register_result)
        self.login(username=username, password=password, base_url=final_base_url)
        return register_data if isinstance(register_data, dict) else {"data": register_data}

    def refresh(self, base_url: Optional[str] = None) -> AuthContext:
        ctx = self.load_context(required=True, base_url_override=base_url)
        assert ctx is not None
        if not ctx.refresh_token:
            raise RuntimeError("refreshToken is missing in auth file.")
        final_base_url = ctx.base_url
        path = self.resolver.resolve("auth", "refresh")
        result = request_json("POST", final_base_url, path, payload={"refreshToken": ctx.refresh_token})
        data = unwrap_plus_result(result) or {}
        refreshed = AuthContext(
            base_url=final_base_url,
            username=ctx.username,
            password=ctx.password,
            auth_token=data.get("authToken"),
            refresh_token=data.get("refreshToken") or ctx.refresh_token,
            token_type=data.get("tokenType") or ctx.token_type or "Bearer",
            expires_in=data.get("expiresIn"),
            saved_at=datetime.now(timezone.utc).isoformat(),
        )
        self.store.save(self._from_context(refreshed))
        return refreshed

    def ensure_context(
        self,
        base_url: Optional[str] = None,
        username: Optional[str] = None,
        password: Optional[str] = None,
        captcha: Optional[str] = None,
        allow_login: bool = False,
        auto_register: bool = False,
    ) -> AuthContext:
        ctx = self.load_context(required=False, base_url_override=base_url)
        has_user_profile = bool(ctx and ctx.username and ctx.password)
        if ctx and ctx.auth_token and has_user_profile:
            return ctx

        login_candidates: list[tuple[str, str]] = []
        if username and password:
            login_candidates.append((username, password))
        if ctx and ctx.username and ctx.password and (ctx.username, ctx.password) not in login_candidates:
            login_candidates.append((ctx.username, ctx.password))

        if allow_login or auto_register:
            for login_username, login_password in login_candidates:
                try:
                    return self.login(
                        username=login_username,
                        password=login_password,
                        captcha=captcha,
                        base_url=base_url,
                    )
                except Exception:
                    continue

        if auto_register:
            register_username = username or (ctx.username if ctx else None) or self._generate_username()
            register_password = password or (ctx.password if ctx else None) or self._generate_password()
            try:
                self.register_and_login(
                    username=register_username,
                    password=register_password,
                    confirm_password=register_password,
                    base_url=base_url,
                )
            except Exception:
                # Existing user profile may be invalid or occupied; fallback to generated account once.
                fallback_username = self._generate_username()
                fallback_password = self._generate_password()
                self.register_and_login(
                    username=fallback_username,
                    password=fallback_password,
                    confirm_password=fallback_password,
                    base_url=base_url,
                )
            ctx = self.load_context(required=True, base_url_override=base_url)
            assert ctx is not None
            return ctx
        if allow_login and username and password:
            return self.login(username=username, password=password, captcha=captcha, base_url=base_url)
        missing_file = self.auth_file
        raise RuntimeError(
            f"No valid auth context. Run login/register first, or provide credentials. auth file: {missing_file}"
        )

    @staticmethod
    def build_auth_headers(context: AuthContext, include_access_token_alias: bool = True) -> Dict[str, str]:
        token = context.require_token()
        header_value = context.auth_header_value()
        headers = {header: header_value for header in AUTH_HEADER_ALIASES}
        if include_access_token_alias:
            for header in ACCESS_HEADER_ALIASES:
                headers[header] = token
        return headers

    def _resolve_base_url(self, base_url: Optional[str]) -> str:
        resolved = normalize_base_url(base_url or self._base_url)
        if not resolved:
            loaded = self.store.load(required=False)
            if loaded and loaded.get("base_url"):
                resolved = normalize_base_url(str(loaded["base_url"]))
        return resolved or "http://127.0.0.1:8080"

    @staticmethod
    def _to_context(raw: Dict[str, Any], base_url: Optional[str] = None) -> AuthContext:
        return AuthContext(
            base_url=normalize_base_url(base_url or raw.get("base_url") or ""),
            username=raw.get("username"),
            password=raw.get("password"),
            auth_token=raw.get("authToken"),
            refresh_token=raw.get("refreshToken"),
            token_type=raw.get("tokenType") or "Bearer",
            expires_in=raw.get("expiresIn"),
            saved_at=raw.get("savedAt"),
        )

    @staticmethod
    def _from_context(context: AuthContext) -> Dict[str, Any]:
        return {
            "base_url": context.base_url,
            "username": context.username,
            "password": context.password,
            "authToken": context.auth_token,
            "refreshToken": context.refresh_token,
            "tokenType": context.token_type,
            "expiresIn": context.expires_in,
            "savedAt": context.saved_at or datetime.now(timezone.utc).isoformat(),
        }

    @staticmethod
    def _generate_username() -> str:
        seed = datetime.now(timezone.utc).strftime("%Y%m%d%H%M%S")
        suffix = "".join(random.choices(string.ascii_lowercase + string.digits, k=6))
        return f"sdkwork_{seed}_{suffix}"

    @staticmethod
    def _generate_password() -> str:
        chars = string.ascii_letters + string.digits + "!@#$%^&*"
        # Ensure complexity: at least one upper/lower/digit/symbol.
        mandatory = [
            random.choice(string.ascii_uppercase),
            random.choice(string.ascii_lowercase),
            random.choice(string.digits),
            random.choice("!@#$%^&*"),
        ]
        tail = [random.choice(chars) for _ in range(12)]
        merged = mandatory + tail
        random.shuffle(merged)
        return "".join(merged)


def is_auth_error(exc: Exception) -> bool:
    text = str(exc).lower()
    return any(marker in text for marker in AUTH_ERROR_MARKERS)


def request_with_transparent_auth(
    auth_client: AuthClient,
    domain: str,
    endpoint_key: str,
    method: str,
    payload: Optional[Dict[str, Any]] = None,
    query: Optional[str] = None,
    path_replacements: Optional[Mapping[str, str]] = None,
    base_url: Optional[str] = None,
    username: Optional[str] = None,
    password: Optional[str] = None,
    captcha: Optional[str] = None,
    include_access_token_alias: bool = True,
) -> Any:
    """
    Send business request with transparent auth lifecycle:
    - load encrypted local auth context
    - auto login (if credentials available)
    - auto register+login when context/credentials missing
    - auto refresh and retry once when token expires
    """
    context = auth_client.ensure_context(
        base_url=base_url,
        username=username,
        password=password,
        captcha=captcha,
        allow_login=bool(username and password),
        auto_register=True,
    )
    auth_client.resolver.base_url = context.base_url
    request_base_url = context.base_url
    headers = AuthClient.build_auth_headers(context, include_access_token_alias=include_access_token_alias)
    path = auth_client.resolver.resolve(domain, endpoint_key)
    if path_replacements:
        for key, value in path_replacements.items():
            path = path.replace("{" + key + "}", str(value))
    if query:
        path = f"{path}?{query}"
    try:
        result = request_json(method, request_base_url, path, payload=payload, headers=headers)
        return unwrap_plus_result(result)
    except Exception as exc:
        if not is_auth_error(exc):
            raise
        try:
            refreshed = auth_client.refresh(base_url=request_base_url)
        except Exception:
            refreshed = auth_client.ensure_context(
                base_url=request_base_url,
                username=username,
                password=password,
                captcha=captcha,
                allow_login=bool(username and password),
                auto_register=True,
            )
        auth_client.resolver.base_url = refreshed.base_url
        retry_headers = AuthClient.build_auth_headers(refreshed, include_access_token_alias=include_access_token_alias)
        retry_result = request_json(method, refreshed.base_url, path, payload=payload, headers=retry_headers)
        return unwrap_plus_result(retry_result)


