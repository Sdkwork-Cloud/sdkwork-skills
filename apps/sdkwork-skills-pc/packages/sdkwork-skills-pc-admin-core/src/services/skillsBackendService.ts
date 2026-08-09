import type {
  CreateSkillArtifactCommand,
  CreateSkillCapabilityCommand,
  CreateSkillCategoryCommand,
  CreateSkillPackageCommand,
  SkillArtifactRecord,
  SkillArtifactsPageData,
  SkillCapabilitiesPageData,
  SkillCapabilityRecord,
  SkillCategoriesPageData,
  SkillCategoryRecord,
  SkillPackageRecord,
  SkillPackagesPageData,
  UpdateSkillCapabilityCommand,
  UpdateSkillPackageCommand,
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

export async function updateSkillPackage(
  clients: SkillsBackendClients,
  packageId: string,
  input: UpdateSkillPackageCommand,
): Promise<SkillPackageRecord> {
  return clients.backend.skills.skillPackages.update(packageId, input);
}

export async function deleteSkillPackage(
  clients: SkillsBackendClients,
  packageId: string,
): Promise<void> {
  return clients.backend.skills.skillPackages.delete(packageId);
}

export async function listPackageArtifacts(
  clients: SkillsBackendClients,
  packageId: string,
): Promise<SkillArtifactsPageData> {
  return clients.backend.skills.skillPackages.artifacts.list(packageId);
}

export async function createSkillArtifact(
  clients: SkillsBackendClients,
  packageId: string,
  input: CreateSkillArtifactCommand,
): Promise<SkillArtifactRecord> {
  return clients.backend.skills.skillPackages.artifacts.create(packageId, input);
}

export async function listSkillCapabilities(
  clients: SkillsBackendClients,
): Promise<SkillCapabilitiesPageData> {
  return clients.backend.skills.skillCapabilities.list();
}

export async function createSkillCapability(
  clients: SkillsBackendClients,
  input: CreateSkillCapabilityCommand,
): Promise<SkillCapabilityRecord> {
  return clients.backend.skills.skillCapabilities.create(input);
}

export async function updateSkillCapability(
  clients: SkillsBackendClients,
  capabilityId: string,
  input: UpdateSkillCapabilityCommand,
): Promise<SkillCapabilityRecord> {
  return clients.backend.skills.skillCapabilities.update(capabilityId, input);
}

export async function createSkillCategory(
  clients: SkillsBackendClients,
  input: CreateSkillCategoryCommand,
): Promise<SkillCategoryRecord> {
  return clients.backend.skills.skillCategories.create(input);
}
