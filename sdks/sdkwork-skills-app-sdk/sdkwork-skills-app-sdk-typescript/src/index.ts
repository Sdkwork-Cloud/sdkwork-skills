import {
  createClient as createGeneratedSkillsAppClient,
  SdkworkAppClient,
} from '../generated/server-openapi/src/index';
import type { SdkworkAppConfig } from '../generated/server-openapi/src/types/common';

export { SdkworkAppClient, createGeneratedSkillsAppClient };
export type { SdkworkAppConfig };
export * from '../generated/server-openapi/src/types';
export * from '../generated/server-openapi/src/api';
export * from '../generated/server-openapi/src/http';
export * from '../generated/server-openapi/src/auth';

export type SdkworkSkillsAppClient = SdkworkAppClient;

export function createSkillsAppClient(config: SdkworkAppConfig): SdkworkSkillsAppClient {
  return createGeneratedSkillsAppClient(config);
}

export function createClient(config: SdkworkAppConfig): SdkworkSkillsAppClient {
  return createSkillsAppClient(config);
}
