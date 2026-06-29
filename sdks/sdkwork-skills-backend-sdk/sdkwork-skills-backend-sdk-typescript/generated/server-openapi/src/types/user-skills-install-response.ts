import type { UserSkillsInstallResourceData } from './user-skills-install-resource-data';

export interface UserSkillsInstallResponse {
  code: 0;
  data: unknown & UserSkillsInstallResourceData;
  /** Server-owned request correlation id. */
  traceId: string;
}
