import { isBlank, trim } from '@sdkwork/utils';

export const SKILLS_ADMIN_PERMISSIONS = {
  packageManage: 'skills.packages.manage',
  categoryManage: 'skills.categories.manage',
  marketplaceRead: 'skills.marketplace.read',
  packagesInstall: 'skills.packages.install',
} as const;

export const SKILLS_ADMIN_ROLES = {
  operator: 'skills_admin_operator',
  superAdmin: 'org_admin',
} as const;

export type SkillsAdminPermission =
  (typeof SKILLS_ADMIN_PERMISSIONS)[keyof typeof SKILLS_ADMIN_PERMISSIONS];

export function packageManagePermissionForCategory(categoryCode: string): string {
  return `skills.packages.manage.${categoryCode}`;
}

export function resolveCategoryPackagePermission(category: {
  code: string;
  permission_code?: string | null;
}): string {
  const explicit = category.permission_code ? trim(category.permission_code) : '';
  return !isBlank(explicit) ? explicit : packageManagePermissionForCategory(category.code);
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
