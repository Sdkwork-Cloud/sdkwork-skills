import { isBlank, trim } from '@sdkwork/utils';
import { createTokenManager, type AuthTokenManager } from '@sdkwork/sdk-common';

const AUTH_TOKEN_KEY = 'sdkwork-skills-auth-token';
const ACCESS_TOKEN_KEY = 'sdkwork-skills-access-token';

function resolveStorage(): Storage | undefined {
  if (typeof window === 'undefined') {
    return undefined;
  }
  migrateLegacyToken(AUTH_TOKEN_KEY);
  migrateLegacyToken(ACCESS_TOKEN_KEY);
  return window.localStorage;
}

function migrateLegacyToken(key: string): void {
  const legacyToken = window.sessionStorage.getItem(key);
  if (legacyToken && !window.localStorage.getItem(key)) {
    window.localStorage.setItem(key, legacyToken);
  }
  if (legacyToken) {
    window.sessionStorage.removeItem(key);
  }
}

function readToken(key: string): string | undefined {
  const storage = resolveStorage();
  const fromStorage = trim(storage?.getItem(key) ?? '');
  return isBlank(fromStorage) ? undefined : fromStorage;
}

export function readStoredAuthToken(): string | undefined {
  return readToken(AUTH_TOKEN_KEY);
}

export function readStoredAccessToken(): string | undefined {
  return readToken(ACCESS_TOKEN_KEY);
}

export function clearStoredTokens(): void {
  resolveStorage()?.removeItem(AUTH_TOKEN_KEY);
  resolveStorage()?.removeItem(ACCESS_TOKEN_KEY);
}

export function createSkillsTokenManager(): AuthTokenManager {
  return createTokenManager();
}

export function hasStoredSession(): boolean {
  return !isBlank(readStoredAuthToken()) && !isBlank(readStoredAccessToken());
}
