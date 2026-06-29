import type { SkillsResourceData } from './skills-resource-data';

export interface SkillsResponse {
  code: 0;
  data: unknown & SkillsResourceData;
  /** Server-owned request correlation id. */
  traceId: string;
}
