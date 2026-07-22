export interface SkillCapabilityRecord {
  id: string;
  uuid: string;
  tenantId: string;
  organizationId: string;
  capabilityKey: string;
  displayName: string;
  description?: string | null;
  riskLevel: 'standard' | 'sensitive' | 'privileged';
  status: 0 | 1;
  version: string;
  createdAt: string;
  updatedAt: string;
}
