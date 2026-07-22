import type { AuthTokenManager } from '@sdkwork/sdk-common';
import {
  createClient as createBackendSdkClient,
  type SdkworkBackendClient,
} from '@sdkwork/skills-backend-sdk';
import { normalizeApiBaseUrl, readRuntimeEnv } from '@sdkwork/skills-pc-commons/runtime';

export type SkillsBackendClientConfig = {
  backendApiBaseUrl?: string;
  tokenManager: AuthTokenManager;
};

export type SkillsBackendClients = {
  backend: SdkworkBackendClient;
};

function resolveBackendApiBaseUrl(config: SkillsBackendClientConfig): string {
  return normalizeApiBaseUrl(
    config.backendApiBaseUrl ?? readRuntimeEnv('VITE_SDKWORK_SKILLS_BACKEND_API_BASE_URL') ?? '',
  );
}

export function createSkillsBackendClients(config: SkillsBackendClientConfig): SkillsBackendClients {
  const backend = createBackendSdkClient({
    baseUrl: resolveBackendApiBaseUrl(config),
    authMode: 'dual-token',
    platform: 'pc',
    tokenManager: config.tokenManager,
  });
  backend.setTokenManager(config.tokenManager);
  return { backend };
}
