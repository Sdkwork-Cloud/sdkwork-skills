import type { SkillInstallationResourceData } from './skill-installation-resource-data';

export interface SkillInstallationResponse {
  code: 0;
  data: unknown & SkillInstallationResourceData;
  /** Server-owned request correlation id. */
  traceId: string;
}
