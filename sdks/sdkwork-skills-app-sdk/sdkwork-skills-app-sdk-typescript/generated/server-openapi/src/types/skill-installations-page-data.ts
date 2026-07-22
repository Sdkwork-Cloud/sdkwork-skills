import type { PageInfo } from './page-info';
import type { SkillInstallationRecord } from './skill-installation-record';

export interface SkillInstallationsPageData {
  items: SkillInstallationRecord[];
  pageInfo: PageInfo;
}
