import type { SkillResourceData } from './skill-resource-data';

export interface SkillResponse {
  code: 0;
  data: unknown & SkillResourceData;
  /** Server-owned request correlation id. */
  traceId: string;
}
