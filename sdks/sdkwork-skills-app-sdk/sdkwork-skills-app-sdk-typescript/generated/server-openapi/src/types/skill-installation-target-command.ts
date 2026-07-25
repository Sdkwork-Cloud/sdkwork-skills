export interface SkillInstallationTargetCommand {
  kind: 'user' | 'organization' | 'project' | 'agent';
  id: string;
}
