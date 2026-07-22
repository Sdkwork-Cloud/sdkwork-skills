export interface SkillInstallationTargetCommand {
  kind: 'user' | 'workspace' | 'project' | 'agent';
  id: string;
}
