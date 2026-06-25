import { createClient as createDriveSdkClient, type SdkworkDriveAppClient } from '@sdkwork/drive-app-sdk';
import type { AuthTokenManager } from '@sdkwork/sdk-common';
import { createClient as createAppSdkClient, type SdkworkAppClient } from 'sdkwork-skills-app-sdk-generated-typescript/src/sdk';
import {
  createClient as createBackendSdkClient,
  type SdkworkBackendClient,
} from 'sdkwork-skills-backend-sdk-generated-typescript/src/sdk';
import { isBlank, trim } from '@sdkwork/utils';
import { normalizeApiBaseUrl, readRuntimeEnv } from '@sdkwork/skills-pc-commons/runtime';

import { createSkillsTokenManager } from './session';

export type SkillsClientConfig = {
  appApiBaseUrl?: string;
  backendApiBaseUrl?: string;
  driveAppApiBaseUrl?: string;
  tenantId?: string;
  tokenManager?: AuthTokenManager;
};

export type SkillsClients = {
  app: SdkworkAppClient;
  backend: SdkworkBackendClient;
  drive: SdkworkDriveAppClient;
};

let cachedClients: SkillsClients | null = null;

function resolveAppApiBaseUrl(config?: SkillsClientConfig): string {
  return normalizeApiBaseUrl(
    config?.appApiBaseUrl ?? readRuntimeEnv('VITE_SDKWORK_SKILLS_APP_API_BASE_URL') ?? '',
  );
}

function resolveBackendApiBaseUrl(config?: SkillsClientConfig): string {
  return normalizeApiBaseUrl(
    config?.backendApiBaseUrl ?? readRuntimeEnv('VITE_SDKWORK_SKILLS_BACKEND_API_BASE_URL') ?? '',
  );
}

function resolveDriveAppApiBaseUrl(config?: SkillsClientConfig): string {
  return normalizeApiBaseUrl(
    config?.driveAppApiBaseUrl ??
      readRuntimeEnv('VITE_SDKWORK_DRIVE_APP_API_BASE_URL') ??
      readRuntimeEnv('VITE_SDKWORK_SKILLS_APP_API_BASE_URL') ??
      '',
  );
}

function resolveTenantHeader(config?: SkillsClientConfig): string {
  const tenantId = trim(config?.tenantId ?? readRuntimeEnv('VITE_SDKWORK_SKILLS_TENANT_ID') ?? '1');
  return isBlank(tenantId) ? '1' : tenantId;
}

function createAuthenticatedClientConfig(
  config: SkillsClientConfig,
  baseUrl: string,
  tokenManager: AuthTokenManager,
) {
  return {
    baseUrl,
    authMode: 'dual-token' as const,
    platform: 'pc' as const,
    headers: {
      'x-sdkwork-tenant-id': resolveTenantHeader(config),
    },
    tokenManager,
  };
}

export function createSkillsClients(config: SkillsClientConfig = {}): SkillsClients {
  const tokenManager = config.tokenManager ?? createSkillsTokenManager();

  const app = createAppSdkClient(
    createAuthenticatedClientConfig(config, resolveAppApiBaseUrl(config), tokenManager),
  );
  app.setTokenManager(tokenManager);

  const backend = createBackendSdkClient(
    createAuthenticatedClientConfig(config, resolveBackendApiBaseUrl(config), tokenManager),
  );
  backend.setTokenManager(tokenManager);

  const drive = createDriveSdkClient(
    createAuthenticatedClientConfig(config, resolveDriveAppApiBaseUrl(config), tokenManager),
  );
  drive.setTokenManager(tokenManager);

  return { app, backend, drive };
}

export function getSkillsClients(): SkillsClients {
  if (!cachedClients) {
    cachedClients = createSkillsClients();
  }
  return cachedClients;
}

export function resetSkillsClients(): void {
  cachedClients = null;
}
