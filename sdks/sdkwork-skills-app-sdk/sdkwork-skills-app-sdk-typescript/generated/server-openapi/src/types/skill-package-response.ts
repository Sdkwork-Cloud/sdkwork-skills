import type { SkillPackageResourceData } from './skill-package-resource-data';

export interface SkillPackageResponse {
  code: 0;
  data: unknown & SkillPackageResourceData;
  /** Server-owned request correlation id. */
  traceId: string;
}
