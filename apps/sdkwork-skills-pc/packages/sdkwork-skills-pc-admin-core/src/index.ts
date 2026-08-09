export {
  SKILLS_ADMIN_PERMISSIONS,
  SKILLS_ADMIN_ROLES,
  canManagePackagesInCategories,
  packageManagePermissionForCategory,
  resolveCategoryPackagePermission,
  type SkillsAdminPermission,
} from './permissions';

export type { SkillCapabilityRecord } from '@sdkwork/skills-backend-sdk';

export {
  createSkillsBackendClients,
  type SkillsBackendClients,
  type SkillsBackendClientConfig,
} from './clients';

export {
  createSkillArtifact,
  createSkillCapability,
  createSkillCategory,
  createSkillPackage,
  deleteSkillPackage,
  listManagedSkillCategories,
  listManagedSkillPackages,
  listPackageArtifacts,
  listSkillCapabilities,
  updateSkillCapability,
  updateSkillPackage,
} from './services';
