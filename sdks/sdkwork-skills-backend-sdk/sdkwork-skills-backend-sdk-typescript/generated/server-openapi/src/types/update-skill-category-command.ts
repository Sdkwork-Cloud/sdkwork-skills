export interface UpdateSkillCategoryCommand {
  version: string;
  name?: string;
  description?: string | null;
  parentId?: string | null;
  sortWeight?: number;
  permissionCode?: string;
  visible?: boolean;
  status?: 0 | 1;
}
