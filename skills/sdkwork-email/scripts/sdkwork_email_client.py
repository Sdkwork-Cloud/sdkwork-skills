#!/usr/bin/env python3
"""
SDKWORK Email skill operational client.

Capabilities:
- Reuse shared auth/session module (stored in ~/.sdkwork/user/auth.json)
- Resolve auth/email endpoints progressively from OpenAPI 3.x docs
- Operate send/receive/list/read/delete lifecycle using SaaS-managed account config
"""

from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path
from typing import Any, Dict, List, Optional
from urllib import parse

SKILLS_ROOT = Path(__file__).resolve().parents[2]
SHARED_ROOT = SKILLS_ROOT / "shared"
if str(SHARED_ROOT) not in sys.path:
    sys.path.insert(0, str(SHARED_ROOT))

from sdkwork_skill_core import (  # noqa: E402
    AuthClient,
    DEFAULT_AUTH_FILE,
    ProgressiveEndpointResolver,
    print_json,
    request_with_transparent_auth,
)

DEFAULT_BASE_URL = "http://127.0.0.1:8080"


def parse_emails(csv_value: Optional[str]) -> List[str]:
    if not csv_value:
        return []
    values = [item.strip() for item in csv_value.split(",")]
    return [item for item in values if item]


def build_resolver(base_url: Optional[str], scan_all_sources_on_gap: bool = False) -> ProgressiveEndpointResolver:
    return ProgressiveEndpointResolver(
        base_url=base_url or DEFAULT_BASE_URL,
        scan_all_sources_on_gap=scan_all_sources_on_gap,
    )


def build_runtime(args: argparse.Namespace) -> tuple[ProgressiveEndpointResolver, AuthClient]:
    resolver = build_resolver(args.base_url)
    auth = AuthClient(
        base_url=args.base_url,
        auth_file=Path(args.auth_file),
        endpoint_resolver=resolver,
    )
    return resolver, auth


def run_login(args: argparse.Namespace) -> None:
    _, auth_client = build_runtime(args)
    context = auth_client.login(
        username=args.username,
        password=args.password,
        captcha=args.captcha,
        base_url=args.base_url,
    )
    print(f"Saved auth to: {auth_client.auth_file}")
    print_json(
        {
            "base_url": context.base_url,
            "username": context.username,
            "password": context.password,
            "authToken": context.auth_token,
            "refreshToken": context.refresh_token,
            "tokenType": context.token_type,
            "expiresIn": context.expires_in,
            "savedAt": context.saved_at,
        }
    )


def run_register(args: argparse.Namespace) -> None:
    _, auth_client = build_runtime(args)
    register_data = auth_client.register_and_login(
        username=args.username,
        password=args.password,
        confirm_password=args.confirm_password,
        email=args.email,
        phone=args.phone,
        register_type=args.register_type,
        verification_code=args.verification_code,
        base_url=args.base_url,
    )
    print("Register success:")
    print_json(register_data)
    context = auth_client.load_context(required=True, base_url_override=args.base_url)
    assert context is not None
    print(f"Saved auth to: {auth_client.auth_file}")
    print_json(
        {
            "base_url": context.base_url,
            "username": context.username,
            "password": context.password,
            "authToken": context.auth_token,
            "refreshToken": context.refresh_token,
            "tokenType": context.token_type,
            "expiresIn": context.expires_in,
            "savedAt": context.saved_at,
        }
    )


def run_refresh_auth(args: argparse.Namespace) -> None:
    _, auth_client = build_runtime(args)
    context = auth_client.refresh(base_url=args.base_url)
    print(f"Refreshed auth in: {auth_client.auth_file}")
    print_json(
        {
            "base_url": context.base_url,
            "username": context.username,
            "password": context.password,
            "authToken": context.auth_token,
            "refreshToken": context.refresh_token,
            "tokenType": context.token_type,
            "expiresIn": context.expires_in,
            "savedAt": context.saved_at,
        }
    )


def run_show_auth(args: argparse.Namespace) -> None:
    _, auth_client = build_runtime(args)
    raw = auth_client.store.load(required=True)
    print_json(raw)


def repo_root() -> Path:
    # .../spring-ai-plus-business/spring-ai-plus-app-api/skills/sdkwork-email/scripts
    return Path(__file__).resolve().parents[4]


def app_api_root() -> Path:
    return repo_root() / "spring-ai-plus-app-api"


def domain_controller_file(domain: str) -> Optional[Path]:
    if domain == "auth":
        return app_api_root() / "src" / "main" / "java" / "com" / "sdkwork" / "ai" / "gateway" / "api" / "app" / "v3" / "auth" / "AuthAppApiController.java"
    if domain == "email":
        return app_api_root() / "src" / "main" / "java" / "com" / "sdkwork" / "ai" / "gateway" / "api" / "app" / "v3" / "email" / "EmailAppApiController.java"
    return None


def parse_controller_routes(controller_file: Path) -> List[Dict[str, str]]:
    if not controller_file.exists():
        return []
    text = controller_file.read_text(encoding="utf-8", errors="ignore")
    base_path_match = re.search(r'@RequestMapping\(\s*(?:value\s*=\s*)?"([^"]+)"', text)
    base_path = base_path_match.group(1).strip() if base_path_match else ""
    route_pattern = re.compile(r'@(Get|Post|Put|Patch|Delete)Mapping\((.*?)\)', re.S)
    routes: List[Dict[str, str]] = []
    for match in route_pattern.finditer(text):
        http_method = match.group(1).upper()
        arg_text = match.group(2)
        paths = re.findall(r'"([^"]+)"', arg_text)
        if not paths:
            paths = [""]
        for sub_path in paths:
            resolved = join_path(base_path, sub_path.strip())
            routes.append(
                {
                    "method": http_method,
                    "path": resolved,
                }
            )
    return routes


def join_path(base_path: str, sub_path: str) -> str:
    base = base_path or ""
    sub = sub_path or ""
    if not base and not sub:
        return "/"
    if not base:
        return sub if sub.startswith("/") else "/" + sub
    if not sub:
        return base
    if base.endswith("/") and sub.startswith("/"):
        return base[:-1] + sub
    if not base.endswith("/") and not sub.startswith("/"):
        return base + "/" + sub
    return base + sub


def build_source_contract(domain: str, report: Dict[str, Any]) -> Dict[str, Any]:
    controller_file = domain_controller_file(domain)
    if controller_file is None:
        return {
            "available": False,
            "reason": f"unsupported domain: {domain}",
        }
    routes = parse_controller_routes(controller_file)
    discovered_keys = set((report.get("discovered") or {}).keys())
    resolved_keys = set((report.get("resolved") or {}).keys())
    source_keys: set[str] = set()
    unmapped_routes: List[Dict[str, str]] = []
    for route in routes:
        route_key = route_to_endpoint_key(domain, route["method"], route["path"])
        if route_key is None:
            unmapped_routes.append(route)
            continue
        source_keys.add(route_key)
    return {
        "available": controller_file.exists(),
        "controllerFile": str(controller_file),
        "routeCount": len(routes),
        "routes": routes,
        "sourceEndpointKeys": sorted(source_keys),
        "missingInOpenapiDiscoveryKeys": sorted(source_keys - discovered_keys),
        "coveredByResolvedFallbackKeys": sorted((source_keys & resolved_keys) - discovered_keys),
        "unmappedRoutes": unmapped_routes,
    }


def route_to_endpoint_key(domain: str, method: str, path: str) -> Optional[str]:
    method_upper = method.upper()
    if domain == "email":
        mapping = {
            ("GET", "/app/v3/api/email/account"): "account_get",
            ("POST", "/app/v3/api/email/send"): "send",
            ("POST", "/app/v3/api/email/receive"): "receive",
            ("GET", "/app/v3/api/email/messages"): "message_list",
            ("GET", "/app/v3/api/email/messages/{messageId}"): "message_get",
            ("POST", "/app/v3/api/email/messages/{messageId}/read"): "message_read",
            ("DELETE", "/app/v3/api/email/messages/{messageId}"): "message_delete",
            ("POST", "/app/v3/api/email/sync"): "sync",
        }
        return mapping.get((method_upper, path))
    if domain == "auth":
        mapping = {
            ("POST", "/app/v3/api/auth/login"): "login",
            ("POST", "/app/v3/api/auth/register"): "register",
            ("POST", "/app/v3/api/auth/logout"): "logout",
            ("POST", "/app/v3/api/auth/refresh"): "refresh",
            ("POST", "/app/v3/api/auth/sms/send"): "verify_send",
            ("POST", "/app/v3/api/auth/verify/send"): "verify_send",
            ("POST", "/app/v3/api/auth/sms/verify"): "verify_check",
            ("POST", "/app/v3/api/auth/verify/check"): "verify_check",
            ("POST", "/app/v3/api/auth/password/reset/request"): "password_reset_request",
            ("POST", "/app/v3/api/auth/password/reset"): "password_reset",
            ("POST", "/app/v3/api/auth/qr/generate"): "qr_generate",
            ("GET", "/app/v3/api/auth/qr/status/{qrKey}"): "qr_status",
            ("GET", "/app/v3/api/auth/qr/entry/{qrKey}"): "qr_entry",
            ("POST", "/app/v3/api/auth/qr/confirm"): "qr_confirm",
            ("POST", "/app/v3/api/auth/phone/login"): "phone_login",
            ("POST", "/app/v3/api/auth/oauth/url"): "oauth_url",
            ("POST", "/app/v3/api/auth/oauth/login"): "oauth_login",
        }
        return mapping.get((method_upper, path))
    return None


def run_check_openapi(args: argparse.Namespace) -> None:
    resolver = build_resolver(args.base_url, scan_all_sources_on_gap=True)
    if args.refresh_openapi:
        resolver.clear_cache()
    domains = ["auth", "email"] if args.domain == "all" else [args.domain]
    reports: Dict[str, Any] = {}
    strict_failed_domains: List[str] = []
    for domain in domains:
        report = resolver.domain_report(domain)
        if args.include_source:
            report["sourceContract"] = build_source_contract(domain, report)
            source_contract = report.get("sourceContract") or {}
            source_missing = source_contract.get("missingInOpenapiDiscoveryKeys") or []
            if source_contract.get("available") and source_missing:
                report["runtimeOpenapiDrift"] = {
                    "detected": True,
                    "missingKeys": source_missing,
                    "message": (
                        "Runtime OpenAPI does not expose some controller routes. "
                        "Likely stale deployment or OpenAPI grouping mismatch. "
                        "Rebuild/restart server and re-check with --refresh-openapi."
                    ),
                }
            else:
                report["runtimeOpenapiDrift"] = {"detected": False}
        reports[domain] = report
        if report.get("missingRequiredKeys") or report.get("resolvedByFallback"):
            strict_failed_domains.append(domain)
    print_json(
        {
            "baseUrl": resolver.base_url,
            "refreshOpenapi": args.refresh_openapi,
            "domains": reports,
        }
    )
    if args.strict and strict_failed_domains:
        raise RuntimeError(
            "OpenAPI strict check failed for domains: "
            + ",".join(strict_failed_domains)
            + ". See `missingRequiredKeys`, `resolvedByFallback`, and `runtimeOpenapiDrift`."
        )


def request_via_shared_auth(
    args: argparse.Namespace,
    domain: str,
    endpoint_key: str,
    method: str,
    payload: Optional[Dict[str, Any]] = None,
    query: Optional[str] = None,
    path_replacements: Optional[Dict[str, str]] = None,
) -> Any:
    _, auth_client = build_runtime(args)
    return request_with_transparent_auth(
        auth_client=auth_client,
        domain=domain,
        endpoint_key=endpoint_key,
        method=method,
        payload=payload,
        query=query,
        path_replacements=path_replacements,
        base_url=args.base_url,
        username=getattr(args, "username", None),
        password=getattr(args, "password", None),
        captcha=getattr(args, "captcha", None),
    )


def run_get_account(args: argparse.Namespace) -> None:
    print_json(request_via_shared_auth(args, "email", "account_get", "GET"))


def run_send(args: argparse.Namespace) -> None:
    to = parse_emails(args.to)
    if not to:
        raise RuntimeError("--to is required, comma separated emails")
    payload = {
        "to": to,
        "cc": parse_emails(args.cc),
        "bcc": parse_emails(args.bcc),
        "subject": args.subject,
        "content": args.content,
        "contentType": args.content_type,
    }
    payload = {k: v for k, v in payload.items() if v is not None}
    print_json(request_via_shared_auth(args, "email", "send", "POST", payload=payload))


def run_receive(args: argparse.Namespace) -> None:
    payload = {
        "from": args.sender,
        "to": parse_emails(args.to),
        "cc": parse_emails(args.cc),
        "subject": args.subject,
        "content": args.content,
        "contentType": args.content_type,
    }
    payload = {k: v for k, v in payload.items() if v is not None}
    print_json(request_via_shared_auth(args, "email", "receive", "POST", payload=payload))


def run_sync(args: argparse.Namespace) -> None:
    payload = {
        "folder": args.folder,
        "maxMessages": args.max_messages,
    }
    payload = {k: v for k, v in payload.items() if v is not None}
    print_json(request_via_shared_auth(args, "email", "sync", "POST", payload=payload))


def run_list(args: argparse.Namespace) -> None:
    params = {
        "folder": args.folder,
        "keyword": args.keyword,
        "unreadOnly": "true" if args.unread_only else None,
        "pageNum": str(args.page_num),
        "pageSize": str(args.page_size),
    }
    query = parse.urlencode({k: v for k, v in params.items() if v is not None})
    print_json(request_via_shared_auth(args, "email", "message_list", "GET", query=query))


def run_get(args: argparse.Namespace) -> None:
    print_json(
        request_via_shared_auth(
            args,
            "email",
            "message_get",
            "GET",
            path_replacements={"messageId": str(args.message_id)},
        )
    )


def run_read(args: argparse.Namespace) -> None:
    payload = {"read": not args.unread}
    print_json(
        request_via_shared_auth(
            args,
            "email",
            "message_read",
            "POST",
            payload=payload,
            path_replacements={"messageId": str(args.message_id)},
        )
    )


def run_delete(args: argparse.Namespace) -> None:
    print_json(
        request_via_shared_auth(
            args,
            "email",
            "message_delete",
            "DELETE",
            path_replacements={"messageId": str(args.message_id)},
        )
    )


def add_auth_file_arg(parser: argparse.ArgumentParser) -> None:
    parser.add_argument(
        "--auth-file",
        default=str(DEFAULT_AUTH_FILE),
        help=f"Auth file path (default: {DEFAULT_AUTH_FILE})",
    )


def add_base_url_arg(parser: argparse.ArgumentParser) -> None:
    parser.add_argument("--base-url", default=None, help=f"API base URL (default: {DEFAULT_BASE_URL})")


def add_optional_login_args(parser: argparse.ArgumentParser) -> None:
    parser.add_argument("--username", default=None, help="Optional login username for auto-login fallback")
    parser.add_argument("--password", default=None, help="Optional login password for auto-login fallback")
    parser.add_argument("--captcha", default=None, help="Optional captcha for auto-login fallback")


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(description="SDKWORK email operational client")
    subparsers = parser.add_subparsers(dest="command", required=True)

    login_parser = subparsers.add_parser("login", help="Login and persist auth info")
    add_base_url_arg(login_parser)
    add_auth_file_arg(login_parser)
    login_parser.add_argument("--username", required=True, help="Login username")
    login_parser.add_argument("--password", required=True, help="Login password")
    login_parser.add_argument("--captcha", default=None, help="Optional captcha")
    login_parser.set_defaults(func=run_login)

    register_parser = subparsers.add_parser("register", help="Register then login and persist auth info")
    add_base_url_arg(register_parser)
    add_auth_file_arg(register_parser)
    register_parser.add_argument("--username", required=True, help="Register username")
    register_parser.add_argument("--password", required=True, help="Register password")
    register_parser.add_argument("--confirm-password", default=None, help="Confirm password")
    register_parser.add_argument("--email", default=None, help="Email address")
    register_parser.add_argument("--phone", default=None, help="Phone number")
    register_parser.add_argument("--register-type", default=None, help="Register type: EMAIL/PHONE/DEFAULT")
    register_parser.add_argument("--verification-code", default=None, help="Verification code if required")
    register_parser.set_defaults(func=run_register)

    refresh_parser = subparsers.add_parser("refresh-auth", help="Refresh auth token and persist")
    add_base_url_arg(refresh_parser)
    add_auth_file_arg(refresh_parser)
    refresh_parser.set_defaults(func=run_refresh_auth)

    get_account_parser = subparsers.add_parser("get-account", help="Get SaaS-managed email account summary")
    add_base_url_arg(get_account_parser)
    add_auth_file_arg(get_account_parser)
    add_optional_login_args(get_account_parser)
    get_account_parser.set_defaults(func=run_get_account)

    send_parser = subparsers.add_parser("send", help="Send email")
    add_base_url_arg(send_parser)
    add_auth_file_arg(send_parser)
    add_optional_login_args(send_parser)
    send_parser.add_argument("--to", required=True, help="Comma-separated recipients")
    send_parser.add_argument("--cc", default=None, help="Comma-separated cc recipients")
    send_parser.add_argument("--bcc", default=None, help="Comma-separated bcc recipients")
    send_parser.add_argument("--subject", required=True, help="Email subject")
    send_parser.add_argument("--content", required=True, help="Email content")
    send_parser.add_argument("--content-type", default="text/plain", help="Content type")
    send_parser.set_defaults(func=run_send)

    receive_parser = subparsers.add_parser("receive", help="Ingest inbound message into inbox")
    add_base_url_arg(receive_parser)
    add_auth_file_arg(receive_parser)
    add_optional_login_args(receive_parser)
    receive_parser.add_argument("--sender", required=True, help="Sender email")
    receive_parser.add_argument("--to", default=None, help="Comma-separated recipients")
    receive_parser.add_argument("--cc", default=None, help="Comma-separated cc recipients")
    receive_parser.add_argument("--subject", required=True, help="Email subject")
    receive_parser.add_argument("--content", required=True, help="Email content")
    receive_parser.add_argument("--content-type", default="text/plain", help="Content type")
    receive_parser.set_defaults(func=run_receive)

    sync_parser = subparsers.add_parser("sync", help="Manual inbox sync")
    add_base_url_arg(sync_parser)
    add_auth_file_arg(sync_parser)
    add_optional_login_args(sync_parser)
    sync_parser.add_argument("--folder", default="INBOX", help="Folder to sync (INBOX/SENT)")
    sync_parser.add_argument("--max-messages", type=int, default=50, help="Max messages to pull")
    sync_parser.set_defaults(func=run_sync)

    list_parser = subparsers.add_parser("list", help="List messages")
    add_base_url_arg(list_parser)
    add_auth_file_arg(list_parser)
    add_optional_login_args(list_parser)
    list_parser.add_argument("--folder", default="ALL", help="ALL/INBOX/SENT")
    list_parser.add_argument("--keyword", default=None, help="Search keyword")
    list_parser.add_argument("--unread-only", action="store_true", help="Unread only")
    list_parser.add_argument("--page-num", type=int, default=1, help="Page number")
    list_parser.add_argument("--page-size", type=int, default=20, help="Page size")
    list_parser.set_defaults(func=run_list)

    get_parser = subparsers.add_parser("get", help="Get message detail")
    add_base_url_arg(get_parser)
    add_auth_file_arg(get_parser)
    add_optional_login_args(get_parser)
    get_parser.add_argument("--message-id", type=int, required=True, help="Message id")
    get_parser.set_defaults(func=run_get)

    read_parser = subparsers.add_parser("read", help="Mark read/unread")
    add_base_url_arg(read_parser)
    add_auth_file_arg(read_parser)
    add_optional_login_args(read_parser)
    read_parser.add_argument("--message-id", type=int, required=True, help="Message id")
    read_parser.add_argument("--unread", action="store_true", help="Mark as unread")
    read_parser.set_defaults(func=run_read)

    delete_parser = subparsers.add_parser("delete", help="Delete message")
    add_base_url_arg(delete_parser)
    add_auth_file_arg(delete_parser)
    add_optional_login_args(delete_parser)
    delete_parser.add_argument("--message-id", type=int, required=True, help="Message id")
    delete_parser.set_defaults(func=run_delete)

    show_auth_parser = subparsers.add_parser("show-auth", help="Show auth file content")
    add_auth_file_arg(show_auth_parser)
    add_base_url_arg(show_auth_parser)
    show_auth_parser.set_defaults(func=run_show_auth)

    check_openapi_parser = subparsers.add_parser(
        "check-openapi",
        help="Check auth/email endpoint resolution from OpenAPI 3.x with fallback report",
    )
    add_base_url_arg(check_openapi_parser)
    check_openapi_parser.add_argument(
        "--domain",
        choices=["all", "auth", "email"],
        default="all",
        help="Target endpoint domain to verify",
    )
    check_openapi_parser.add_argument(
        "--strict",
        action="store_true",
        help="Exit with error when required keys are missing or resolved by fallback (not discovered from OpenAPI)",
    )
    check_openapi_parser.add_argument(
        "--include-source",
        action="store_true",
        help="Include source controller scan under spring-ai-plus-app-api/src/main/java for auth/email contract comparison",
    )
    check_openapi_parser.add_argument(
        "--refresh-openapi",
        action="store_true",
        help="Clear local OpenAPI cache and force refetch from runtime before checking",
    )
    check_openapi_parser.set_defaults(func=run_check_openapi)

    return parser


def main() -> int:
    parser = build_parser()
    args = parser.parse_args()
    try:
        args.func(args)
    except Exception as exc:
        print(f"[ERROR] {exc}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())

