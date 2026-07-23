import { isBlank, trim } from '@sdkwork/utils';
import manifest from '../../../../sdkwork.app.config.json';

export type SdkworkSkillsPcEnvironment = 'development' | 'test' | 'staging' | 'production';

export type SdkworkSkillsPcConfigProfile = 'dev' | 'test' | 'staging' | 'prod';

export type SdkworkSkillsPcDeploymentMode = 'web' | 'desktop';
export type SdkworkSkillsPcRuntimeTarget = 'browser' | 'desktop';

export interface SdkworkSkillsPcI18nRuntimeConfig {
  defaultLocale: string;
  fallbackLocale: string;
  supportedLocales: string[];
}

export interface SdkworkSkillsPcDependencySdkBaseUrls {
  appApiBaseUrl?: string;
  backendApiBaseUrl?: string;
}

export interface SdkworkSkillsPcSdkBaseUrls {
  appApiBaseUrl?: string;
  backendApiBaseUrl?: string;
  dependencySdkBaseUrls?: Record<string, SdkworkSkillsPcDependencySdkBaseUrls>;
  sdkBaseUrl?: string;
}

export interface SdkworkSkillsPcRuntimeConfig {
  appApiBaseUrl: string;
  appDisplayName: string;
  appKey: string;
  backendApiBaseUrl: string;
  buildMode: SdkworkSkillsPcEnvironment;
  configProfile: SdkworkSkillsPcConfigProfile;
  deploymentMode: SdkworkSkillsPcDeploymentMode;
  driveAppApiBaseUrl: string;
  environment: SdkworkSkillsPcEnvironment;
  i18n: SdkworkSkillsPcI18nRuntimeConfig;
  runtimeTarget: SdkworkSkillsPcRuntimeTarget;
  sdkBaseUrl?: string;
  sdkBaseUrls?: SdkworkSkillsPcSdkBaseUrls;
  version: string;
}

const environmentByMode: Record<string, SdkworkSkillsPcEnvironment> = {
  development: 'development',
  dev: 'development',
  production: 'production',
  prod: 'production',
  staging: 'staging',
  test: 'test',
};

const profileByEnvironment: Record<SdkworkSkillsPcEnvironment, SdkworkSkillsPcConfigProfile> = {
  development: 'dev',
  production: 'prod',
  staging: 'staging',
  test: 'test',
};

const APP_API_PREFIX = '/app/v3/api';
const BACKEND_API_PREFIX = '/backend/v3/api';
const APPBASE_APP_SDK_FAMILY_ID = 'sdkwork-iam-app-sdk';

function envValue(key: string): string | undefined {
  const value = import.meta.env[key];
  return typeof value === 'string' && !isBlank(value) ? trim(value) : undefined;
}

function resolveEnvironment(mode: string): SdkworkSkillsPcEnvironment {
  return environmentByMode[mode] ?? 'development';
}

function parseSdkBaseUrls(sdkBaseUrl?: string): SdkworkSkillsPcSdkBaseUrls | undefined {
  const raw = envValue('VITE_SDKWORK_SKILLS_PC_SDK_BASE_URLS_JSON');
  if (raw) {
    try {
      return JSON.parse(raw) as SdkworkSkillsPcSdkBaseUrls;
    } catch {
      return undefined;
    }
  }

  if (!sdkBaseUrl) {
    return undefined;
  }

  const normalizedSdkBaseUrl = sdkBaseUrl.replace(/\/+$/u, '');
  return {
    appApiBaseUrl: `${normalizedSdkBaseUrl}${APP_API_PREFIX}`,
    backendApiBaseUrl: `${normalizedSdkBaseUrl}${BACKEND_API_PREFIX}`,
    dependencySdkBaseUrls: {
      [APPBASE_APP_SDK_FAMILY_ID]: {
        appApiBaseUrl: `${normalizedSdkBaseUrl}${APP_API_PREFIX}`,
      },
    },
    sdkBaseUrl: normalizedSdkBaseUrl,
  };
}

export function resolveSdkworkSkillsPcRuntimeConfig(
  mode = import.meta.env.MODE,
): SdkworkSkillsPcRuntimeConfig {
  const environment = resolveEnvironment(mode);
  const sdkBaseUrl = envValue('VITE_SDKWORK_SKILLS_PC_SDK_BASE_URL');
  const sdkBaseUrls = parseSdkBaseUrls(sdkBaseUrl);
  return {
    appApiBaseUrl:
      envValue('VITE_SDKWORK_SKILLS_APP_API_BASE_URL') ??
      sdkBaseUrls?.appApiBaseUrl ??
      APP_API_PREFIX,
    appDisplayName: manifest.app.displayName,
    appKey: manifest.app.key,
    backendApiBaseUrl:
      envValue('VITE_SDKWORK_SKILLS_BACKEND_API_BASE_URL') ??
      sdkBaseUrls?.backendApiBaseUrl ??
      BACKEND_API_PREFIX,
    buildMode: environment,
    configProfile: profileByEnvironment[environment],
    deploymentMode: 'web',
    driveAppApiBaseUrl:
      envValue('VITE_SDKWORK_DRIVE_APP_API_BASE_URL') ??
      envValue('VITE_SDKWORK_SKILLS_APP_API_BASE_URL') ??
      sdkBaseUrls?.appApiBaseUrl ??
      APP_API_PREFIX,
    environment,
    i18n: {
      defaultLocale: envValue('VITE_SDKWORK_SKILLS_DEFAULT_LOCALE') ?? 'zh-CN',
      fallbackLocale: 'en-US',
      supportedLocales: ['zh-CN', 'en-US'],
    },
    runtimeTarget: 'browser',
    sdkBaseUrl,
    sdkBaseUrls,
    version: '0.1.0',
  };
}

export function resolveAppbaseAppApiBaseUrl(config: SdkworkSkillsPcRuntimeConfig): string {
  return (
    config.sdkBaseUrls?.dependencySdkBaseUrls?.[APPBASE_APP_SDK_FAMILY_ID]?.appApiBaseUrl ??
    config.appApiBaseUrl
  );
}
