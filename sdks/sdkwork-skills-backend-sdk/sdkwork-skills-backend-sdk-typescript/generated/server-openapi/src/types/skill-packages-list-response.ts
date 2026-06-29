import type { SkillPackagesPageData } from './skill-packages-page-data';

export interface SkillPackagesListResponse {
  code: 0;
  data: unknown & SkillPackagesPageData;
  /** Server-owned request correlation id. */
  traceId: string;
}
