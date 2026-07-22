import type {
  SkillArtifactsPageData,
  SkillCategoriesPageData,
  SkillInstallationRecord,
  SkillPackagesPageData,
  SkillRecord,
  SkillsPageData,
} from '@sdkwork/skills-app-sdk';

import type { SkillsAppClients } from '../clients';

export async function listPublishedSkills(clients: SkillsAppClients): Promise<SkillsPageData> {
  return clients.app.skills.marketplace.list();
}

export async function retrievePublishedSkill(
  clients: SkillsAppClients,
  skillKey: string,
): Promise<SkillRecord> {
  return clients.app.skills.marketplace.retrieve(skillKey);
}

export async function listSkillPackages(clients: SkillsAppClients): Promise<SkillPackagesPageData> {
  return clients.app.skills.skillPackages.list();
}

export async function listSkillCategories(
  clients: SkillsAppClients,
): Promise<SkillCategoriesPageData> {
  return clients.app.skills.skillCategories.list();
}

export async function listInstallableSkillArtifacts(
  clients: SkillsAppClients,
  packageId: string,
): Promise<SkillArtifactsPageData> {
  return clients.app.skills.skillPackages.artifacts.list(packageId);
}

export async function installUserSkill(
  clients: SkillsAppClients,
  packageId: string,
  artifactId: string,
  config?: Record<string, unknown>,
): Promise<SkillInstallationRecord> {
  return clients.app.skills.skillPackages.installations.create(packageId, {
    artifactId,
    ...(config ? { config } : {}),
  });
}
