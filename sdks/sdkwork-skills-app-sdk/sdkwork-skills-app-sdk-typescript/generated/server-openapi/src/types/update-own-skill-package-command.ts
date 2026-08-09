export interface UpdateOwnSkillPackageCommand {
  version: string;
  displayName?: string;
  summary?: string | null;
  description?: string | null;
  categories?: string[];
  tags?: string[];
}
