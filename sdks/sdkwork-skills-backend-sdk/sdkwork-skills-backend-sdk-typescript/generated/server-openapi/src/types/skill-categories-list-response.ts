import type { SkillCategoriesPageData } from './skill-categories-page-data';

export interface SkillCategoriesListResponse {
  code: 0;
  data: unknown & SkillCategoriesPageData;
  /** Server-owned request correlation id. */
  traceId: string;
}
