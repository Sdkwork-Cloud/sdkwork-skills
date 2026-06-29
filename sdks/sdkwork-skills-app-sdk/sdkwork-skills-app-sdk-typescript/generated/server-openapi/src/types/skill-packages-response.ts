import type { SkillPackagesResourceData } from './skill-packages-resource-data';

export interface SkillPackagesResponse {
  code: 0;
  data: unknown & SkillPackagesResourceData;
  /** Server-owned request correlation id. */
  traceId: string;
}
