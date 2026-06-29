import type { PageInfo } from './page-info';
import type { SkillPackageRecord } from './skill-package-record';

export interface SkillPackagesManagementPageData {
  items: SkillPackageRecord[];
  pageInfo: PageInfo;
}
