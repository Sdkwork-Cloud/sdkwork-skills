import type { SkillCapabilityResourceData } from './skill-capability-resource-data';

export interface SkillCapabilityResponse {
  code: 0;
  data: unknown & SkillCapabilityResourceData;
  /** Server-owned request correlation id. */
  traceId: string;
}
