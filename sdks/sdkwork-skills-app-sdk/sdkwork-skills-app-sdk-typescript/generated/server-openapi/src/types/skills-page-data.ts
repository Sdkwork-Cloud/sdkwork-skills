import type { PageInfo } from './page-info';
import type { SkillRecord } from './skill-record';

export interface SkillsPageData {
  items: SkillRecord[];
  pageInfo: PageInfo;
}
