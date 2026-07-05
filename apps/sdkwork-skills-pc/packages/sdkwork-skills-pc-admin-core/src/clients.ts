import type { AuthTokenManager } from '@sdkwork/sdk-common';
import {
  createClient as createBackendSdkClient,
  type SdkworkBackendClient,
} from '@sdkwork/skills-backend-sdk';
import { isBlank, trim } from '@sdkwork/utils';
import { normalizeApiBaseUrl, readRuntimeEnv } from '@sdkwork/skills-pc-commons/runtime';

export type SkillsBackendClientConfig = {
  backendApiBaseUrl?: string;
  tenantId?: string;
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

function resolveTenantHeader(config: SkillsBackendClientConfig): string {
  const tenantId = trim(config.tenantId ?? readRuntimeEnv('VITE_SDKWORK_SKILLS_TENANT_ID') ?? '100001');
  return isBlank(tenantId) ? '100001' : tenantId;
}

export function createSkillsBackendClients(config: SkillsBackendClientConfig): SkillsBackendClients {
  const backend = createBackendSdkClient({
    baseUrl: resolveBackendApiBaseUrl(config),
    authMode: 'dual-token',
    platform: 'pc',
    headers: {
      'x-sdkwork-tenant-id': resolveTenantHeader(config),
    },
    tokenManager: config.tokenManager,
  });
  backend.setTokenManager(config.tokenManager);
  return { backend };
}
