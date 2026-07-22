import type { SkillArtifactsPageData } from './skill-artifacts-page-data';

export interface SkillArtifactsListResponse {
  code: 0;
  data: unknown & SkillArtifactsPageData;
  /** Server-owned request correlation id. */
  traceId: string;
}
