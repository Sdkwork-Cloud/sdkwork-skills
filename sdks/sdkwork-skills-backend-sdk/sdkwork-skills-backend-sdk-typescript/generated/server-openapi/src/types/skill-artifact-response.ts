import type { SkillArtifactResourceData } from './skill-artifact-resource-data';

export interface SkillArtifactResponse {
  code: 0;
  data: unknown & SkillArtifactResourceData;
  /** Server-owned request correlation id. */
  traceId: string;
}
