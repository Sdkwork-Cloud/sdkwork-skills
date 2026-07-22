export interface UpdateSkillPackageCommand {
  version: string;
  displayName?: string;
  summary?: string | null;
  description?: string | null;
  categories?: string[];
  tags?: string[];
  status?: 'draft' | 'active' | 'disabled' | 'archived' | 'deleted';
  visibility?: 'private' | 'tenant' | 'organization' | 'public';
  featured?: boolean;
  sortWeight?: number;
}
