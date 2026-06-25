import type { Location } from 'react-router-dom';

import {
  hasSdkworkSkillsPcIamSession,
  type SdkworkSkillsPcSessionSnapshot,
} from './bootstrap/sessionStore';

export type SdkworkSkillsPcAuthGateDecision =
  | { kind: 'product-route' }
  | { kind: 'auth-route' }
  | { kind: 'redirect'; replace: true; to: string };

const AUTH_BASE_PATH = '/auth';
const AUTH_LOGIN_PATH = '/auth/login';
const DEFAULT_HOME_PATH = '/skills-hub';

const PUBLIC_PATH_PREFIXES = ['/skills-hub'];

const PROTECTED_PATH_PREFIXES = ['/console', '/admin'];

export function hasSdkworkSkillsPcAuthenticatedSession(
  snapshot: SdkworkSkillsPcSessionSnapshot,
): boolean {
  return hasSdkworkSkillsPcIamSession(snapshot);
}

export function buildSdkworkSkillsPcAuthLoginRedirect(
  location: Pick<Location, 'pathname' | 'search' | 'hash'>,
): string {
  const returnPath = `${normalizePathname(location.pathname)}${location.search ?? ''}${location.hash ?? ''}`;
  return `${AUTH_LOGIN_PATH}?redirect=${encodeURIComponent(returnPath)}`;
}

export function sanitizeSdkworkSkillsPcAuthRedirect(value: string | null | undefined): string {
  if (!value) {
    return DEFAULT_HOME_PATH;
  }

  let decoded = value;
  try {
    decoded = decodeURIComponent(value);
  } catch {
    return DEFAULT_HOME_PATH;
  }

  if (!decoded.startsWith('/') || decoded.startsWith('//')) {
    return DEFAULT_HOME_PATH;
  }

  const redirectUrl = new URL(decoded, 'http://sdkwork-skills.local');
  if (isAuthRoute(redirectUrl.pathname)) {
    return DEFAULT_HOME_PATH;
  }

  return `${redirectUrl.pathname}${redirectUrl.search}${redirectUrl.hash}`;
}

export function isSdkworkSkillsPcProtectedPath(pathname: string): boolean {
  const normalized = normalizePathname(pathname);
  return PROTECTED_PATH_PREFIXES.some(
    (prefix) => normalized === prefix || normalized.startsWith(`${prefix}/`),
  );
}

export function isSdkworkSkillsPcPublicPath(pathname: string): boolean {
  const normalized = normalizePathname(pathname);
  if (normalized === '/') {
    return true;
  }
  return PUBLIC_PATH_PREFIXES.some(
    (prefix) => normalized === prefix || normalized.startsWith(`${prefix}/`),
  );
}

export function resolveSdkworkSkillsPcAuthGateDecision({
  hasSession,
  homePath = DEFAULT_HOME_PATH,
  location,
}: {
  hasSession: boolean;
  homePath?: string;
  location: Pick<Location, 'pathname' | 'search' | 'hash'>;
}): SdkworkSkillsPcAuthGateDecision {
  const pathname = normalizePathname(location.pathname);

  if (isAuthRoute(pathname)) {
    if (!hasSession) {
      return { kind: 'auth-route' };
    }

    const redirect = new URLSearchParams((location.search ?? '').replace(/^\?/u, '')).get(
      'redirect',
    );
    return {
      kind: 'redirect',
      replace: true,
      to: sanitizeSdkworkSkillsPcAuthRedirect(redirect) || normalizePathname(homePath),
    };
  }

  if (!hasSession && isSdkworkSkillsPcProtectedPath(pathname)) {
    return {
      kind: 'redirect',
      replace: true,
      to: buildSdkworkSkillsPcAuthLoginRedirect(location),
    };
  }

  return { kind: 'product-route' };
}

function isAuthRoute(pathname: string): boolean {
  return pathname === AUTH_BASE_PATH || pathname.startsWith(`${AUTH_BASE_PATH}/`);
}

function normalizePathname(pathname: string): string {
  const normalized = pathname.trim();
  if (!normalized) {
    return '/';
  }
  return normalized.startsWith('/') ? normalized : `/${normalized}`;
}
