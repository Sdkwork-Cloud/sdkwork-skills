"""
Shared SDKWORK skill core utilities.

Provides reusable auth/session management, OpenAPI progressive endpoint
resolution, and HTTP helpers for all app-v3 skills.
"""

from .auth import (
    ACCESS_HEADER_ALIASES,
    AUTH_HEADER_ALIASES,
    AUTH_ERROR_MARKERS,
    AuthClient,
    AuthContext,
    AuthStore,
    DEFAULT_AUTH_FILE,
    is_auth_error,
    request_with_transparent_auth,
)
from .crypto import DEFAULT_KEY_FILE, decrypt_text, encrypt_text, load_or_create_key
from .http import normalize_base_url, request_json, unwrap_plus_result, print_json
from .openapi import ProgressiveEndpointResolver, DEFAULT_DOMAIN_ENDPOINTS

__all__ = [
    "AuthClient",
    "AuthContext",
    "AuthStore",
    "DEFAULT_AUTH_FILE",
    "AUTH_ERROR_MARKERS",
    "AUTH_HEADER_ALIASES",
    "ACCESS_HEADER_ALIASES",
    "request_with_transparent_auth",
    "is_auth_error",
    "DEFAULT_KEY_FILE",
    "load_or_create_key",
    "encrypt_text",
    "decrypt_text",
    "normalize_base_url",
    "request_json",
    "unwrap_plus_result",
    "print_json",
    "ProgressiveEndpointResolver",
    "DEFAULT_DOMAIN_ENDPOINTS",
]
