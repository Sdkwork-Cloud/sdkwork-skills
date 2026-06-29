import type { SkillPackagesManagementPageData } from './skill-packages-management-page-data';

export interface SkillPackagesManagementListResponse {
  code: 0;
  data: unknown & SkillPackagesManagementPageData;
  /** Server-owned request correlation id. */
  traceId: string;
}
