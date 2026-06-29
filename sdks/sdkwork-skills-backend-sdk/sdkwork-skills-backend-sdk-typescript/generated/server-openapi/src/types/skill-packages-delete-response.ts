import type { SkillPackagesDeleteResourceData } from './skill-packages-delete-resource-data';

export interface SkillPackagesDeleteResponse {
  code: 0;
  data: unknown & SkillPackagesDeleteResourceData;
  /** Server-owned request correlation id. */
  traceId: string;
}
