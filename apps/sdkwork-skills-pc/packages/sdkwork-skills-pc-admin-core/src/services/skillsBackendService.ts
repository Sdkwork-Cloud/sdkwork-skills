import type {
  CreateSkillCategoryCommand,
  CreateSkillPackageCommand,
  SkillCategoriesPageData,
  SkillCategoryRecord,
  SkillPackageRecord,
  SkillPackagesPageData,
} from '@sdkwork/skills-backend-sdk';

import type { SkillsBackendClients } from '../clients';

export async function listManagedSkillPackages(
  clients: SkillsBackendClients,
): Promise<SkillPackagesPageData> {
  return clients.backend.skills.skillPackages.list();
}

export async function listManagedSkillCategories(
  clients: SkillsBackendClients,
): Promise<SkillCategoriesPageData> {
  return clients.backend.skills.skillCategories.list();
}

export async function createSkillPackage(
  clients: SkillsBackendClients,
  input: CreateSkillPackageCommand,
): Promise<SkillPackageRecord> {
  return clients.backend.skills.skillPackages.create(input);
}

export async function deleteSkillPackage(
  clients: SkillsBackendClients,
  packageId: string,
): Promise<void> {
  return clients.backend.skills.skillPackages.delete(packageId);
}

export async function createSkillCategory(
  clients: SkillsBackendClients,
  input: CreateSkillCategoryCommand,
): Promise<SkillCategoryRecord> {
  return clients.backend.skills.skillCategories.create(input);
}
