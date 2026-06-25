import { createClient as createAppbaseAppClient, type SdkworkAppClient } from '@sdkwork/iam-app-sdk';
import {
  createSdkworkAppbasePcAuthRuntime,
  type SdkworkAppbasePcAuthRuntimeComposition,
  type SdkworkAppbasePcAuthRuntimeSdkClient,
} from '@sdkwork/auth-runtime-pc-react';
import { createClient as createDriveSdkClient, type SdkworkDriveAppClient } from '@sdkwork/drive-app-sdk';
import type { IamAppContext, IamDeploymentMode, IamEnvironment } from '@sdkwork/iam-contracts';
import type { IamRuntime } from '@sdkwork/iam-runtime';
import { normalizeSdkworkApiBaseUrl } from '@sdkwork/runtime-bootstrap';
import { createClient as createSkillsAppSdkClient, type SdkworkAppClient as SkillsAppClient } from 'sdkwork-skills-app-sdk-generated-typescript/src/sdk';
import {
  createClient as createSkillsBackendSdkClient,
  type SdkworkBackendClient,
} from 'sdkwork-skills-backend-sdk-generated-typescript/src/sdk';

import {
  resolveAppbaseAppApiBaseUrl,
  type SdkworkSkillsPcRuntimeConfig,
} from './environment';
import {
  createSdkworkSkillsPcSessionStore,
  type SdkworkSkillsPcSessionSnapshot,
  type SdkworkSkillsPcSessionStore,
} from './sessionStore';
import { createSdkworkSkillsPcSessionTokenManager } from './sessionTokenManager';
import type { SdkworkSkillsPcSdkClientInventory } from './sdkClients';

const APPBASE_APP_SDK_FAMILY_ID = 'sdkwork-iam-app-sdk';
const APP_API_PREFIX = '/app/v3/api';
const BACKEND_API_PREFIX = '/backend/v3/api';

export type SdkworkSkillsPcIamRuntime = IamRuntime & {
  composition: SdkworkAppbasePcAuthRuntimeComposition;
  session: SdkworkSkillsPcSessionStore;
};

export interface CreateSdkworkSkillsPcIamRuntimeOptions {
  config: SdkworkSkillsPcRuntimeConfig;
  sdkClients: SdkworkSkillsPcSdkClientInventory;
  session?: SdkworkSkillsPcSessionStore;
}

interface SkillsIamSessionLike {
  accessToken?: string;
  authToken?: string;
  refreshToken?: string;
  sessionId?: string;
  context?: IamAppContext;
}

export function createSdkworkSkillsPcIamRuntime(
  options: CreateSdkworkSkillsPcIamRuntimeOptions,
): SdkworkSkillsPcIamRuntime {
  const session = options.session ?? createSdkworkSkillsPcSessionStore(resolveSessionStorage());
  const tokenManager = createSdkworkSkillsPcSessionTokenManager(session);
  const appbaseAppClient = createAppbaseGeneratedAppClient(options.config, tokenManager);
  const composition = createSdkworkAppbasePcAuthRuntime({
    app: {
      appId: options.config.appKey,
      deploymentMode: toIamDeploymentMode(options.config.deploymentMode),
      environment: toIamEnvironment(options.config.environment),
      platform: 'pc',
    },
    baseUrls: {
      appbaseAppApiBaseUrl: resolveAppbaseAppApiBaseUrl(options.config),
    },
    createAppbaseAppClient: () => appbaseAppClient,
    localeProvider: () => options.config.i18n.defaultLocale,
    sdkClients: [
      options.sdkClients.app,
      options.sdkClients.backend,
      options.sdkClients.drive,
    ] as SdkworkAppbasePcAuthRuntimeSdkClient[],
    sessionBridge: {
      clearSession: () => {
        session.clearSession();
      },
      commitSession: (nextSession) =>
        commitSkillsIamRuntimeSession(session, nextSession as SkillsIamSessionLike),
      readSession: () => toSkillsIamBridgeSession(session.getSnapshot()),
    },
    tokenManager,
  });

  return {
    ...composition.runtime,
    composition,
    session,
  };
}

export function createSdkworkSkillsPcSdkClientsWithTokenManager(
  config: SdkworkSkillsPcRuntimeConfig,
  tokenManager: ReturnType<typeof createSdkworkSkillsPcSessionTokenManager>,
): SdkworkSkillsPcSdkClientInventory {
  const tenantHeader = config.defaultTenantId;
  const authenticatedConfig = (baseUrl: string) => ({
    authMode: 'dual-token' as const,
    baseUrl: normalizeGeneratedSdkBaseUrl(baseUrl, APP_API_PREFIX),
    headers: {
      'x-sdkwork-tenant-id': tenantHeader,
    },
    platform: 'pc' as const,
    tokenManager,
  });

  const app = createSkillsAppSdkClient(
    authenticatedConfig(normalizeSdkworkApiBaseUrl(config.appApiBaseUrl, 'app')),
  );
  app.setTokenManager(tokenManager);

  const backend = createSkillsBackendSdkClient({
    ...authenticatedConfig(normalizeSdkworkApiBaseUrl(config.backendApiBaseUrl, 'backend')),
    baseUrl: normalizeGeneratedSdkBaseUrl(
      normalizeSdkworkApiBaseUrl(config.backendApiBaseUrl, 'backend'),
      BACKEND_API_PREFIX,
    ),
  });
  backend.setTokenManager(tokenManager);

  const drive = createDriveSdkClient(
    authenticatedConfig(normalizeSdkworkApiBaseUrl(config.driveAppApiBaseUrl, 'app')),
  );
  drive.setTokenManager(tokenManager);

  return {
    appApiBaseUrl: normalizeSdkworkApiBaseUrl(config.appApiBaseUrl, 'app'),
    backendApiBaseUrl: normalizeSdkworkApiBaseUrl(config.backendApiBaseUrl, 'backend'),
    driveAppApiBaseUrl: normalizeSdkworkApiBaseUrl(config.driveAppApiBaseUrl, 'app'),
    app,
    backend,
    drive,
    sdkFamilies: {
      app: ['sdkwork-skills-app-sdk', APPBASE_APP_SDK_FAMILY_ID, 'sdkwork-drive-app-sdk'],
      backend: ['sdkwork-skills-backend-sdk'],
    },
  };
}

function createAppbaseGeneratedAppClient(
  config: SdkworkSkillsPcRuntimeConfig,
  tokenManager: ReturnType<typeof createSdkworkSkillsPcSessionTokenManager>,
): SdkworkAppClient {
  return createAppbaseAppClient({
    authMode: 'dual-token',
    baseUrl: normalizeGeneratedSdkBaseUrl(resolveAppbaseAppApiBaseUrl(config), APP_API_PREFIX),
    platform: 'pc',
    tokenManager,
  });
}

function normalizeGeneratedSdkBaseUrl(baseUrl: string, apiPrefix: string): string {
  const normalizedBaseUrl = baseUrl.replace(/\/+$/u, '');
  const normalizedApiPrefix = apiPrefix.replace(/\/+$/u, '');
  if (normalizedBaseUrl.endsWith(normalizedApiPrefix)) {
    return normalizedBaseUrl.slice(0, -normalizedApiPrefix.length) || normalizedBaseUrl;
  }
  return normalizedBaseUrl;
}

function commitSkillsIamRuntimeSession(
  session: SdkworkSkillsPcSessionStore,
  iamSession: SkillsIamSessionLike,
): SkillsIamSessionLike | undefined {
  const nextSession: SdkworkSkillsPcSessionSnapshot = {
    ...session.getSnapshot(),
    accessToken: iamSession.accessToken,
    authToken: iamSession.authToken,
    refreshToken: iamSession.refreshToken,
    sessionId: iamSession.sessionId ?? iamSession.context?.sessionId,
    context: iamSession.context
      ? {
          tenantId: iamSession.context.tenantId,
          userId: iamSession.context.userId,
          organizationId: iamSession.context.organizationId,
          sessionId: iamSession.context.sessionId,
          appId: iamSession.context.appId,
          environment: iamSession.context.environment,
          deploymentMode: iamSession.context.deploymentMode,
          permissionScope: [...iamSession.context.permissionScope],
          standardRoleCodes: iamSession.context.standardRoleCodes,
        }
      : undefined,
  };

  if (!nextSession.context) {
    delete nextSession.context;
  }

  session.setSession(nextSession);
  return toSkillsIamBridgeSession(session.getSnapshot()) ?? undefined;
}

function toSkillsIamBridgeSession(
  snapshot: SdkworkSkillsPcSessionSnapshot,
): SkillsIamSessionLike | null {
  if (!snapshot.authToken && !snapshot.accessToken && !snapshot.refreshToken) {
    return null;
  }

  return {
    ...(snapshot.accessToken ? { accessToken: snapshot.accessToken } : {}),
    ...(snapshot.authToken ? { authToken: snapshot.authToken } : {}),
    ...(snapshot.refreshToken ? { refreshToken: snapshot.refreshToken } : {}),
    ...(snapshot.sessionId ? { sessionId: snapshot.sessionId } : {}),
    ...(snapshot.context?.tenantId && snapshot.context.userId
      ? {
          context: {
            tenantId: snapshot.context.tenantId,
            userId: snapshot.context.userId,
            organizationId: snapshot.context.organizationId,
            sessionId: snapshot.context.sessionId ?? snapshot.sessionId ?? '',
            appId: snapshot.context.appId ?? '',
            environment: (snapshot.context.environment ?? 'dev') as IamEnvironment,
            deploymentMode: (snapshot.context.deploymentMode ?? 'saas') as IamDeploymentMode,
            authLevel: 'password',
            dataScope: [],
            permissionScope: snapshot.context.permissionScope ?? [],
            standardRoleCodes: snapshot.context.standardRoleCodes,
          } as IamAppContext,
        }
      : {}),
  };
}

function resolveSessionStorage(): Storage | undefined {
  if (typeof window === 'undefined') {
    return undefined;
  }
  return window.sessionStorage;
}

function toIamDeploymentMode(value: SdkworkSkillsPcRuntimeConfig['deploymentMode']): IamDeploymentMode {
  return value === 'web' ? 'saas' : 'local';
}

function toIamEnvironment(value: SdkworkSkillsPcRuntimeConfig['environment']): IamEnvironment {
  if (value === 'development') {
    return 'dev';
  }
  if (value === 'production') {
    return 'prod';
  }
  if (value === 'staging') {
    return 'test';
  }
  return 'test';
}

export type { SkillsAppClient, SdkworkDriveAppClient, SdkworkBackendClient };
