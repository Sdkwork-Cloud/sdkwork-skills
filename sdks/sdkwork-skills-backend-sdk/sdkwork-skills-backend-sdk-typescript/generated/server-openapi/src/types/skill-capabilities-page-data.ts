import type { PageInfo } from './page-info';
import type { SkillCapabilityRecord } from './skill-capability-record';

export interface SkillCapabilitiesPageData {
  items: SkillCapabilityRecord[];
  pageInfo: PageInfo;
}
