import type { SdkworkAppClient as SkillsAppClient } from 'sdkwork-skills-app-sdk-generated-typescript/src/sdk';
import type { SdkworkBackendClient } from 'sdkwork-skills-backend-sdk-generated-typescript/src/sdk';
import type { SdkworkDriveAppClient } from '@sdkwork/drive-app-sdk';

export interface SdkworkSkillsPcSdkClientInventory {
  appApiBaseUrl: string;
  backendApiBaseUrl: string;
  driveAppApiBaseUrl: string;
  app: SkillsAppClient;
  backend: SdkworkBackendClient;
  drive: SdkworkDriveAppClient;
  sdkFamilies: {
    app: string[];
    backend: string[];
  };
}
