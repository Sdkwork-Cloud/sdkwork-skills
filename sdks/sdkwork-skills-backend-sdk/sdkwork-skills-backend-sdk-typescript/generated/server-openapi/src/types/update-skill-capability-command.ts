export interface UpdateSkillCapabilityCommand {
  version: string;
  displayName?: string;
  description?: string | null;
  riskLevel?: 'standard' | 'sensitive' | 'privileged';
  status?: 0 | 1;
}
