export interface SkillCategoryRecord {
  id: string;
  code: string;
  name: string;
  description?: string | null;
  sort_weight: number;
  visible: boolean;
  status: number;
}
