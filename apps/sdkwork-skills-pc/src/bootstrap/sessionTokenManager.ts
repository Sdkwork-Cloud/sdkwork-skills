import { createTokenManager, type AuthTokenManager } from '@sdkwork/sdk-common';

import type { SdkworkSkillsPcSessionStore } from './sessionStore';

export function createSdkworkSkillsPcSessionTokenManager(
  session: SdkworkSkillsPcSessionStore,
): AuthTokenManager {
  const tokenManager = createTokenManager();

  const hydrate = () => {
    const snapshot = session.getSnapshot();
    tokenManager.setTokens({
      accessToken: snapshot.accessToken,
      authToken: snapshot.authToken,
      refreshToken: snapshot.refreshToken,
    });
  };

  hydrate();
  session.subscribe(() => {
    hydrate();
  });

  return tokenManager;
}
