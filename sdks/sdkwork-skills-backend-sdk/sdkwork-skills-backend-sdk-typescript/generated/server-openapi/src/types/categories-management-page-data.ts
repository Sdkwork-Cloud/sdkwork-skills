import type { PageInfo } from './page-info';
import type { SkillCategoryRecord } from './skill-category-record';

export interface CategoriesManagementPageData {
  items: SkillCategoryRecord[];
  pageInfo: PageInfo;
}
