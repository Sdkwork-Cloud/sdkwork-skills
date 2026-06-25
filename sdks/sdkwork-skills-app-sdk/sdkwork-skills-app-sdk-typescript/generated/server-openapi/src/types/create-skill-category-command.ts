export interface CreateSkillCategoryCommand {
  code: string;
  name: string;
  description?: string;
  sort_weight?: number;
}
