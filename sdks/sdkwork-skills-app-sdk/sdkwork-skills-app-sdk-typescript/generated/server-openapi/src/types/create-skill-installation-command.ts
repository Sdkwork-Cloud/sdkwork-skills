import type { SkillInstallationTargetCommand } from './skill-installation-target-command';

export interface CreateSkillInstallationCommand {
  artifactId: string;
  target?: SkillInstallationTargetCommand;
  config?: Record<string, unknown>;
}
