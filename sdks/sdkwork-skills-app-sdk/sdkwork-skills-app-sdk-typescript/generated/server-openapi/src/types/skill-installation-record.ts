export interface SkillInstallationRecord {
  id: string;
  uuid: string;
  tenantId: string;
  organizationId: string;
  subjectKind: 'user' | 'workspace' | 'project' | 'agent';
  subjectId: string;
  skillId: string;
  packageId: string;
  artifactId: string;
  installedByUserId: string;
  installStatus: string;
  enabled: boolean;
  config: Record<string, unknown>;
  version: string;
  installedAt: string;
  updatedAt: string;
}
