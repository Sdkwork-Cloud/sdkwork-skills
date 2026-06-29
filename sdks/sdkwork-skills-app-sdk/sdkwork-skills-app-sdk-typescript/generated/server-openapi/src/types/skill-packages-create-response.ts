import type { SkillPackagesCreateResourceData } from './skill-packages-create-resource-data';

export interface SkillPackagesCreateResponse {
  code: 0;
  data: unknown & SkillPackagesCreateResourceData;
  /** Server-owned request correlation id. */
  traceId: string;
}
