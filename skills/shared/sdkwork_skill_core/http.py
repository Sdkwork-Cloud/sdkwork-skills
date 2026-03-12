from __future__ import annotations

import json
from typing import Any, Dict, Mapping, Optional
from urllib import error, request


def normalize_base_url(base_url: str) -> str:
    return (base_url or "").rstrip("/")


def print_json(value: Any) -> None:
    print(json.dumps(value, ensure_ascii=False, indent=2))


def read_json_response(raw: bytes) -> Dict[str, Any]:
    if not raw:
        return {}
    text = raw.decode("utf-8", errors="replace")
    if not text.strip():
        return {}
    try:
        data = json.loads(text)
    except json.JSONDecodeError as exc:
        raise RuntimeError(f"Invalid JSON response: {text[:300]}") from exc
    if isinstance(data, dict):
        return data
    return {"data": data}


def request_json(
    method: str,
    base_url: str,
    path: str,
    payload: Optional[Dict[str, Any]] = None,
    headers: Optional[Mapping[str, str]] = None,
    timeout: int = 30,
) -> Dict[str, Any]:
    url = f"{normalize_base_url(base_url)}{path}"
    body: Optional[bytes] = None
    final_headers: Dict[str, str] = {"Accept": "application/json"}
    if headers:
        final_headers.update({k: v for k, v in headers.items() if v is not None})
    if payload is not None:
        body = json.dumps(payload, ensure_ascii=False).encode("utf-8")
        final_headers["Content-Type"] = "application/json"
    req = request.Request(url=url, data=body, method=method.upper(), headers=final_headers)
    try:
        with request.urlopen(req, timeout=timeout) as resp:
            return read_json_response(resp.read())
    except error.HTTPError as exc:
        response_body = read_json_response(exc.read())
        raise RuntimeError(
            f"HTTP {exc.code} {exc.reason} for {method.upper()} {path}: "
            f"{json.dumps(response_body, ensure_ascii=False)}"
        ) from exc
    except error.URLError as exc:
        reason = getattr(exc, "reason", exc)
        raise RuntimeError(f"Request failed for {method.upper()} {path}: {reason}") from exc


def unwrap_plus_result(result: Dict[str, Any]) -> Any:
    code = str(result.get("code", ""))
    success = result.get("success")
    if success is False:
        msg = result.get("msg") or result.get("message") or "request failed"
        raise RuntimeError(f"API request failed: {msg} (code={code or 'unknown'})")
    if code and code not in {"0", "2000"}:
        msg = result.get("msg") or result.get("message") or "request failed"
        raise RuntimeError(f"API request failed: {msg} (code={code})")
    return result.get("data")
