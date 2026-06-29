import type { SkillsManagementPageData } from './skills-management-page-data';

export interface SkillsManagementListResponse {
  code: 0;
  data: unknown & SkillsManagementPageData;
  /** Server-owned request correlation id. */
  traceId: string;
}
