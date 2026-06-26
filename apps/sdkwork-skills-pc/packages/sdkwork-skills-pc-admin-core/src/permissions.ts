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

export function packageManagePermissionForCategory(categoryCode: string): string {
  return `skills.admin.package.manage.${categoryCode}`;
}

export function resolveCategoryPackagePermission(category: {
  code: string;
  permission_code?: string | null;
}): string {
  const explicit = category.permission_code?.trim();
  return explicit || packageManagePermissionForCategory(category.code);
}

export function canManagePackagesInCategories(
  granted: readonly string[],
  roleCodes: readonly string[],
  categoryCodes: readonly string[],
  categories: ReadonlyArray<{ code: string; permission_code?: string | null }> = [],
): boolean {
  if (roleCodes.includes(SKILLS_ADMIN_ROLES.superAdmin)) {
    return true;
  }
  if (granted.includes(SKILLS_ADMIN_PERMISSIONS.packageManage)) {
    return true;
  }
  if (categoryCodes.length === 0) {
    return granted.includes(SKILLS_ADMIN_PERMISSIONS.packageManage);
  }
  const categoryByCode = new Map(categories.map((item) => [item.code, item]));
  return categoryCodes.every((code) =>
    granted.includes(
      resolveCategoryPackagePermission(categoryByCode.get(code) ?? { code }),
    ),
  );
}
