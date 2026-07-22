export interface CreateSkillCategoryCommand {
  categoryType?: 'skill_market' | 'skills_collection';
  code: string;
  name: string;
  description?: string | null;
  parentId?: string | null;
  sortWeight?: number;
  permissionCode?: string;
  visible?: boolean;
  status?: 0 | 1;
}
