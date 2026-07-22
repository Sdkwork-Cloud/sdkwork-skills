export interface CreateSkillCapabilityCommand {
  capabilityKey: string;
  displayName: string;
  description?: string | null;
  riskLevel?: 'standard' | 'sensitive' | 'privileged';
  status?: 0 | 1;
}
