import type { PageInfo } from './page-info';
import type { SkillRecord } from './skill-record';

export interface SkillsManagementPageData {
  items: SkillRecord[];
  pageInfo: PageInfo;
}
