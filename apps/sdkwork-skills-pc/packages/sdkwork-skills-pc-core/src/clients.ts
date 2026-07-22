import { createClient as createDriveSdkClient, type SdkworkDriveAppClient } from '@sdkwork/drive-app-sdk';
import type { AuthTokenManager } from '@sdkwork/sdk-common';
import { createClient as createAppSdkClient, type SdkworkAppClient } from '@sdkwork/skills-app-sdk';
import type { SdkworkBackendClient } from '@sdkwork/skills-backend-sdk';
import { normalizeApiBaseUrl, readRuntimeEnv } from '@sdkwork/skills-pc-commons/runtime';

import { createSkillsTokenManager } from './session';

export type SkillsAppClientConfig = {
  appApiBaseUrl?: string;
  driveAppApiBaseUrl?: string;
  tokenManager?: AuthTokenManager;
};

export type SkillsAppClients = {
  app: SdkworkAppClient;
  drive: SdkworkDriveAppClient;
};

/** Full PC runtime client inventory (app bootstrap composes app + backend surfaces). */
export type SkillsClients = SkillsAppClients & {
  backend: SdkworkBackendClient;
};

export type SkillsClientConfig = SkillsAppClientConfig & {
  backendApiBaseUrl?: string;
};

function resolveAppApiBaseUrl(config?: SkillsAppClientConfig): string {
  return normalizeApiBaseUrl(
    config?.appApiBaseUrl ?? readRuntimeEnv('VITE_SDKWORK_SKILLS_APP_API_BASE_URL') ?? '',
  );
}

function resolveDriveAppApiBaseUrl(config?: SkillsAppClientConfig): string {
  return normalizeApiBaseUrl(
    config?.driveAppApiBaseUrl ??
      readRuntimeEnv('VITE_SDKWORK_DRIVE_APP_API_BASE_URL') ??
      readRuntimeEnv('VITE_SDKWORK_SKILLS_APP_API_BASE_URL') ??
      '',
  );
}

function createAuthenticatedClientConfig(
  baseUrl: string,
  tokenManager: AuthTokenManager,
) {
  return {
    baseUrl,
    authMode: 'dual-token' as const,
    platform: 'pc' as const,
    tokenManager,
  };
}

export function createSkillsAppClients(config: SkillsAppClientConfig = {}): SkillsAppClients {
  const tokenManager = config.tokenManager ?? createSkillsTokenManager();

  const app = createAppSdkClient(
    createAuthenticatedClientConfig(resolveAppApiBaseUrl(config), tokenManager),
  );
  app.setTokenManager(tokenManager);

  const drive = createDriveSdkClient(
    createAuthenticatedClientConfig(resolveDriveAppApiBaseUrl(config), tokenManager),
  );
  drive.setTokenManager(tokenManager);

  return { app, drive };
}
