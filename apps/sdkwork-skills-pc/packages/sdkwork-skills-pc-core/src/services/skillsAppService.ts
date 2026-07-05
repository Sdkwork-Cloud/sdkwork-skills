import type {
  CategoriesPageData,
  SkillPackagesPageData,
  SkillRecord,
  SkillsPageData,
  UserSkillInstallRecord,
} from '@sdkwork/skills-app-sdk';

import type { SkillsAppClients } from '../clients';

export async function listPublishedSkills(clients: SkillsAppClients): Promise<SkillsPageData> {
  return clients.app.skills.list();
}

export async function retrievePublishedSkill(
  clients: SkillsAppClients,
  skillKey: string,
): Promise<SkillRecord> {
  return clients.app.skills.retrieve(skillKey);
}

export async function listSkillPackages(clients: SkillsAppClients): Promise<SkillPackagesPageData> {
  return clients.app.skills.skillPackages.list();
}

export async function listSkillCategories(clients: SkillsAppClients): Promise<CategoriesPageData> {
  return clients.app.skills.categories.list();
}

export async function installUserSkill(
  clients: SkillsAppClients,
  skillId: string,
): Promise<UserSkillInstallRecord> {
  return clients.app.skills.userSkills.install({ skill_id: skillId });
}
