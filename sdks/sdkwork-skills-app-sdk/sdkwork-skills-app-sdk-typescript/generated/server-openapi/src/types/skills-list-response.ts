import type { SkillsPageData } from './skills-page-data';

export interface SkillsListResponse {
  code: 0;
  data: unknown & SkillsPageData;
  /** Server-owned request correlation id. */
  traceId: string;
}
