#!/usr/bin/env python3
"""
Reusable SDKWORK auth client for app-v3 skills.

This script only manages auth lifecycle:
- register
- login
- refresh token
- inspect saved auth context
"""

from __future__ import annotations

import argparse
import sys
from pathlib import Path

SKILLS_ROOT = Path(__file__).resolve().parents[2]
SHARED_ROOT = SKILLS_ROOT / "shared"
if str(SHARED_ROOT) not in sys.path:
    sys.path.insert(0, str(SHARED_ROOT))

from sdkwork_skill_core import AuthClient, DEFAULT_AUTH_FILE, print_json  # noqa: E402


DEFAULT_BASE_URL = "http://127.0.0.1:8080"


def run_login(args: argparse.Namespace) -> None:
    client = AuthClient(base_url=args.base_url, auth_file=Path(args.auth_file))
    ctx = client.login(username=args.username, password=args.password, captcha=args.captcha, base_url=args.base_url)
    print(f"Saved auth to: {client.auth_file}")
    print_json(
        {
            "base_url": ctx.base_url,
            "username": ctx.username,
            "password": ctx.password,
            "authToken": ctx.auth_token,
            "refreshToken": ctx.refresh_token,
            "tokenType": ctx.token_type,
            "expiresIn": ctx.expires_in,
            "savedAt": ctx.saved_at,
        }
    )


def run_register(args: argparse.Namespace) -> None:
    client = AuthClient(base_url=args.base_url, auth_file=Path(args.auth_file))
    register_data = client.register_and_login(
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
    run_show(args)


def run_refresh(args: argparse.Namespace) -> None:
    client = AuthClient(base_url=args.base_url, auth_file=Path(args.auth_file))
    ctx = client.refresh(base_url=args.base_url)
    print(f"Refreshed auth in: {client.auth_file}")
    print_json(
        {
            "base_url": ctx.base_url,
            "username": ctx.username,
            "password": ctx.password,
            "authToken": ctx.auth_token,
            "refreshToken": ctx.refresh_token,
            "tokenType": ctx.token_type,
            "expiresIn": ctx.expires_in,
            "savedAt": ctx.saved_at,
        }
    )


def run_show(args: argparse.Namespace) -> None:
    client = AuthClient(base_url=args.base_url, auth_file=Path(args.auth_file))
    print_json(client.store.load(required=True))


def run_headers(args: argparse.Namespace) -> None:
    client = AuthClient(base_url=args.base_url, auth_file=Path(args.auth_file))
    ctx = client.load_context(required=True, base_url_override=args.base_url)
    assert ctx is not None
    print_json(AuthClient.build_auth_headers(ctx))


def add_base_url_arg(parser: argparse.ArgumentParser) -> None:
    parser.add_argument("--base-url", default=None, help=f"API base URL (default: {DEFAULT_BASE_URL})")


def add_auth_file_arg(parser: argparse.ArgumentParser) -> None:
    parser.add_argument(
        "--auth-file",
        default=str(DEFAULT_AUTH_FILE),
        help=f"Auth file path (default: {DEFAULT_AUTH_FILE})",
    )


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(description="SDKWORK reusable auth client")
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

    refresh_parser = subparsers.add_parser("refresh", help="Refresh token and persist")
    add_base_url_arg(refresh_parser)
    add_auth_file_arg(refresh_parser)
    refresh_parser.set_defaults(func=run_refresh)

    show_parser = subparsers.add_parser("show", help="Show persisted auth context")
    add_base_url_arg(show_parser)
    add_auth_file_arg(show_parser)
    show_parser.set_defaults(func=run_show)

    headers_parser = subparsers.add_parser("headers", help="Print authorization headers for API requests")
    add_base_url_arg(headers_parser)
    add_auth_file_arg(headers_parser)
    headers_parser.set_defaults(func=run_headers)

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
