import type { SkillCapabilitiesPageData } from './skill-capabilities-page-data';

export interface SkillCapabilitiesListResponse {
  code: 0;
  data: unknown & SkillCapabilitiesPageData;
  /** Server-owned request correlation id. */
  traceId: string;
}
