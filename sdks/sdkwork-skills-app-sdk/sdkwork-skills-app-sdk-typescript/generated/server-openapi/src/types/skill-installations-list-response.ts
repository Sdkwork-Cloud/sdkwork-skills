import type { SkillInstallationsPageData } from './skill-installations-page-data';

export interface SkillInstallationsListResponse {
  code: 0;
  data: unknown & SkillInstallationsPageData;
  /** Server-owned request correlation id. */
  traceId: string;
}
