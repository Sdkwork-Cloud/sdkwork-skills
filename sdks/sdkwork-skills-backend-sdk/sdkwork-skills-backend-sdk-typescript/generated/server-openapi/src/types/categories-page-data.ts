import type { PageInfo } from './page-info';
import type { SkillCategoryRecord } from './skill-category-record';

export interface CategoriesPageData {
  items: SkillCategoryRecord[];
  pageInfo: PageInfo;
}
