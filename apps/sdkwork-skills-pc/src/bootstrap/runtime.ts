import {
  resolveSdkworkSkillsPcRuntimeConfig,
  type SdkworkSkillsPcRuntimeConfig,
} from './environment';
import {
  createSdkworkSkillsPcIamRuntime,
  createSdkworkSkillsPcSdkClientsWithTokenManager,
  type SdkworkSkillsPcIamRuntime,
} from './iamRuntime';
import {
  createSdkworkSkillsPcSessionStore,
  type SdkworkSkillsPcSessionStore,
} from './sessionStore';
import { createSdkworkSkillsPcSessionTokenManager } from './sessionTokenManager';
import type { SdkworkSkillsPcSdkClientInventory } from './sdkClients';

export interface SdkworkSkillsPcRuntime {
  config: SdkworkSkillsPcRuntimeConfig;
  iamRuntime: SdkworkSkillsPcIamRuntime;
  sdkClients: SdkworkSkillsPcSdkClientInventory;
  session: SdkworkSkillsPcSessionStore;
}

export function createSdkworkSkillsPcRuntime(): SdkworkSkillsPcRuntime {
  const config = resolveSdkworkSkillsPcRuntimeConfig();
  const session = createSdkworkSkillsPcSessionStore(
    typeof window === 'undefined' ? undefined : window.sessionStorage,
  );
  const tokenManager = createSdkworkSkillsPcSessionTokenManager(session);
  const sdkClients = createSdkworkSkillsPcSdkClientsWithTokenManager(config, tokenManager);
  const iamRuntime = createSdkworkSkillsPcIamRuntime({
    config,
    sdkClients,
    session,
  });

  return {
    config,
    iamRuntime,
    sdkClients,
    session,
  };
}

export { resolveSdkworkSkillsPcRuntimeConfig } from './environment';
