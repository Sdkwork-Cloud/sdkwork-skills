import type { PageInfo } from './page-info';
import type { SkillCategoryRecord } from './skill-category-record';

export interface SkillCategoriesPageData {
  items: SkillCategoryRecord[];
  pageInfo: PageInfo;
}
