import type { PageInfo } from './page-info';
import type { SkillPackageRecord } from './skill-package-record';

export interface SkillPackagesPageData {
  items: SkillPackageRecord[];
  pageInfo: PageInfo;
}
