import type { CreateSkillArtifactCommand } from './create-skill-artifact-command';

export interface CreateSkillPackageCommand {
  skillKey: string;
  packageKey?: string;
  code: string;
  displayName: string;
  summary?: string | null;
  description?: string | null;
  categories?: string[];
  tags?: string[];
  status?: 'draft' | 'active' | 'disabled' | 'archived' | 'deleted';
  visibility?: 'private' | 'tenant' | 'organization' | 'public';
  featured?: boolean;
  sortWeight?: number;
  initialArtifact: CreateSkillArtifactCommand;
}
