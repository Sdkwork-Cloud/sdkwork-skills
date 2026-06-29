import type { SkillPackagesUpdateResourceData } from './skill-packages-update-resource-data';

export interface SkillPackagesUpdateResponse {
  code: 0;
  data: unknown & SkillPackagesUpdateResourceData;
  /** Server-owned request correlation id. */
  traceId: string;
}
