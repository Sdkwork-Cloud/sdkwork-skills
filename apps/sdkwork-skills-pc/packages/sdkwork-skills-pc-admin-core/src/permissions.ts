import { isBlank, trim } from '@sdkwork/utils';
import type { SkillCategoryRecord } from '@sdkwork/skills-backend-sdk';

type CategoryPermissionView = Pick<SkillCategoryRecord, 'code' | 'permissionCode'>;

export const SKILLS_ADMIN_PERMISSIONS = {
  packageManage: 'skills.packages.manage',
  categoryManage: 'skills.categories.manage',
  capabilityManage: 'skills.capabilities.manage',
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

export function resolveCategoryPackagePermission(category: CategoryPermissionView): string {
  const explicit = category.permissionCode ? trim(category.permissionCode) : '';
  return !isBlank(explicit) ? explicit : packageManagePermissionForCategory(category.code);
}

export function canManagePackagesInCategories(
  granted: readonly string[],
  roleCodes: readonly string[],
  categoryCodes: readonly string[],
  categories: readonly CategoryPermissionView[] = [],
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
      resolveCategoryPackagePermission(
        categoryByCode.get(code) ?? { code, permissionCode: '' },
      ),
    ),
  );
}
