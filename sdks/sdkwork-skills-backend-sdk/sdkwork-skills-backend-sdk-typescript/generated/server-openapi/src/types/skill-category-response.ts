import type { SkillCategoryResourceData } from './skill-category-resource-data';

export interface SkillCategoryResponse {
  code: 0;
  data: unknown & SkillCategoryResourceData;
  /** Server-owned request correlation id. */
  traceId: string;
}
