export interface SkillPackageRecord {
  id: string;
  uuid: string;
  tenantId: string;
  organizationId: string;
  ownerUserId: string;
  skillKey: string;
  packageKey: string;
  code: string;
  displayName: string;
  summary?: string | null;
  description?: string | null;
  categories: string[];
  tags: string[];
  status: 'draft' | 'active' | 'disabled' | 'archived' | 'deleted';
  visibility: 'private' | 'tenant' | 'organization' | 'public';
  featured: boolean;
  sortWeight: number;
  version: string;
  createdAt: string;
  updatedAt: string;
  deletedAt?: string | null;
}
