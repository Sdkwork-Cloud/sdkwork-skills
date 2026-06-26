export interface SkillCategoryRecord {
  id: string;
  code: string;
  name: string;
  category_type: string;
  description?: string | null;
  parent_id?: string | null;
  permission_code: string;
  sort_weight: number;
  visible: boolean;
  status: number;
}
