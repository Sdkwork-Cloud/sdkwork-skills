/** Minimal @sdkwork/utils shim for root workspace tests (no node_modules). */
export function isBlank(value) {
  return value == null || String(value).trim().length === 0;
}

export function trim(value) {
  return String(value ?? '').trim();
}
