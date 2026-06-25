export interface SdkworkSkillsPcSessionContext {
  tenantId?: string;
  userId?: string;
  organizationId?: string;
  sessionId?: string;
  appId?: string;
  environment?: string;
  deploymentMode?: string;
  permissionScope?: string[];
  standardRoleCodes?: readonly string[];
}

export interface SdkworkSkillsPcSessionSnapshot {
  accessToken?: string;
  authToken?: string;
  refreshToken?: string;
  sessionId?: string;
  context?: SdkworkSkillsPcSessionContext;
  updatedAt?: string;
}

export interface SdkworkSkillsPcSessionStorageLike {
  getItem(key: string): string | null;
  setItem(key: string, value: string): void;
  removeItem(key: string): void;
}

export interface SdkworkSkillsPcSessionStore {
  clearSession(): void;
  getSnapshot(): SdkworkSkillsPcSessionSnapshot;
  refreshSession(): SdkworkSkillsPcSessionSnapshot;
  setSession(nextSession: SdkworkSkillsPcSessionSnapshot): void;
  subscribe(listener: (snapshot: SdkworkSkillsPcSessionSnapshot) => void): () => void;
}

export const SDKWORK_SKILLS_PC_SESSION_STORAGE_KEY = 'sdkwork-skills-pc-session';

function readInitialSession(
  storage: SdkworkSkillsPcSessionStorageLike | undefined,
  storageKey: string,
): SdkworkSkillsPcSessionSnapshot {
  if (!storage) {
    return {};
  }

  try {
    const raw = storage.getItem(storageKey);
    return raw ? (JSON.parse(raw) as SdkworkSkillsPcSessionSnapshot) : {};
  } catch {
    return {};
  }
}

export function createSdkworkSkillsPcSessionStore(
  storage?: SdkworkSkillsPcSessionStorageLike,
  storageKey = SDKWORK_SKILLS_PC_SESSION_STORAGE_KEY,
): SdkworkSkillsPcSessionStore {
  let snapshot = readInitialSession(storage, storageKey);
  const listeners = new Set<(nextSnapshot: SdkworkSkillsPcSessionSnapshot) => void>();

  const emit = () => {
    for (const listener of listeners) {
      listener(snapshot);
    }
  };

  const persist = () => {
    if (!storage) {
      return;
    }

    if (!snapshot.authToken && !snapshot.accessToken && !snapshot.refreshToken) {
      storage.removeItem(storageKey);
      return;
    }

    storage.setItem(storageKey, JSON.stringify(snapshot));
  };

  return {
    clearSession() {
      snapshot = {};
      persist();
      emit();
    },
    getSnapshot() {
      return snapshot;
    },
    refreshSession() {
      snapshot = readInitialSession(storage, storageKey);
      emit();
      return snapshot;
    },
    setSession(nextSession) {
      snapshot = {
        ...nextSession,
        updatedAt: new Date().toISOString(),
      };
      persist();
      emit();
    },
    subscribe(listener) {
      listeners.add(listener);
      return () => {
        listeners.delete(listener);
      };
    },
  };
}

export function hasSdkworkSkillsPcIamSession(snapshot: SdkworkSkillsPcSessionSnapshot): boolean {
  return Boolean(snapshot.authToken && snapshot.accessToken && snapshot.context?.tenantId);
}
