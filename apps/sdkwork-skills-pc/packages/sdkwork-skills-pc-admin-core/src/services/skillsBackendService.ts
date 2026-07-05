import type {
  CategoriesManagementPageData,
  CreateSkillCategoryCommand,
  CreateSkillPackageCommand,
  SkillCategoryRecord,
  SkillPackageRecord,
  SkillPackagesManagementPageData,
} from '@sdkwork/skills-backend-sdk';

import type { SkillsBackendClients } from '../clients';

export async function listManagedSkillPackages(
  clients: SkillsBackendClients,
): Promise<SkillPackagesManagementPageData> {
  return clients.backend.skills.skillPackages.management.list();
}

export async function listManagedSkillCategories(
  clients: SkillsBackendClients,
): Promise<CategoriesManagementPageData> {
  return clients.backend.skills.categories.management.list();
}

export async function createSkillPackage(
  clients: SkillsBackendClients,
  input: CreateSkillPackageCommand,
): Promise<SkillPackageRecord> {
  return clients.backend.skills.skillPackages.create(input);
}

export async function deleteSkillPackage(
  clients: SkillsBackendClients,
  skillId: string,
): Promise<SkillPackageRecord> {
  return clients.backend.skills.skillPackages.delete(skillId);
}

export async function createSkillCategory(
  clients: SkillsBackendClients,
  input: CreateSkillCategoryCommand,
): Promise<SkillCategoryRecord> {
  return clients.backend.skills.categories.create(input);
}
