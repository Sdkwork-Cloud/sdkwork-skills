export const SKILLS_ADMIN_PERMISSIONS = {
  packageManage: 'skills.admin.package.manage',
  categoryManage: 'skills.admin.category.manage',
  marketplaceRead: 'skills.admin.marketplace.read',
} as const;

export const SKILLS_ADMIN_ROLES = {
  operator: 'skills-admin-operator',
  superAdmin: 'skills-admin-super',
} as const;

export type SkillsAdminPermission =
  (typeof SKILLS_ADMIN_PERMISSIONS)[keyof typeof SKILLS_ADMIN_PERMISSIONS];
