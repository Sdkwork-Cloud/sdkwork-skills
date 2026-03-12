from __future__ import annotations

import base64
import hashlib
import hmac
import os
from pathlib import Path


DEFAULT_KEY_FILE = Path.home() / ".sdkwork" / "user" / ".auth.key"


def load_or_create_key(key_file: Path = DEFAULT_KEY_FILE) -> bytes:
    if key_file.exists():
        raw = key_file.read_bytes()
        if raw:
            return raw
    key_file.parent.mkdir(parents=True, exist_ok=True)
    key = os.urandom(32)
    key_file.write_bytes(key)
    return key


def _keystream(key: bytes, nonce: bytes, length: int) -> bytes:
    blocks = []
    counter = 0
    while len(b"".join(blocks)) < length:
        msg = nonce + counter.to_bytes(4, "big")
        block = hmac.new(key, msg, hashlib.sha256).digest()
        blocks.append(block)
        counter += 1
    return b"".join(blocks)[:length]


def encrypt_text(text: str, key: bytes) -> str:
    if text is None:
        return ""
    data = text.encode("utf-8")
    nonce = os.urandom(16)
    stream = _keystream(key, nonce, len(data))
    encrypted = bytes(a ^ b for a, b in zip(data, stream))
    payload = nonce + encrypted
    return base64.urlsafe_b64encode(payload).decode("ascii")


def decrypt_text(cipher_text: str, key: bytes) -> str:
    if not cipher_text:
        return ""
    payload = base64.urlsafe_b64decode(cipher_text.encode("ascii"))
    if len(payload) < 16:
        return ""
    nonce = payload[:16]
    encrypted = payload[16:]
    stream = _keystream(key, nonce, len(encrypted))
    data = bytes(a ^ b for a, b in zip(encrypted, stream))
    return data.decode("utf-8", errors="replace")
