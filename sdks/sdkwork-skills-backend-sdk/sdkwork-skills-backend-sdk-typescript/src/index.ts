import {
  createClient as createGeneratedSkillsBackendClient,
  SdkworkBackendClient,
} from '../generated/server-openapi/src/index';
import type { SdkworkBackendConfig } from '../generated/server-openapi/src/types/common';

export { SdkworkBackendClient, createGeneratedSkillsBackendClient };
export type { SdkworkBackendConfig };
export * from '../generated/server-openapi/src/types';
export * from '../generated/server-openapi/src/api';
export * from '../generated/server-openapi/src/http';
export * from '../generated/server-openapi/src/auth';

export type SdkworkSkillsBackendClient = SdkworkBackendClient;

export function createSkillsBackendClient(config: SdkworkBackendConfig): SdkworkSkillsBackendClient {
  return createGeneratedSkillsBackendClient(config);
}

export function createClient(config: SdkworkBackendConfig): SdkworkSkillsBackendClient {
  return createSkillsBackendClient(config);
}
