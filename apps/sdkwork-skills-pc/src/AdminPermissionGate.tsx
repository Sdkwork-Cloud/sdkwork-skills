import type { ReactNode } from 'react';
import { Navigate, useLocation } from 'react-router-dom';
import { hasPermissionInScope } from '@sdkwork/iam-contracts';
import {
  SKILLS_ADMIN_PERMISSIONS,
  SKILLS_ADMIN_ROLES,
  type SkillsAdminPermission,
} from '@sdkwork/skills-pc-admin-core';

import {
  buildSdkworkSkillsPcAuthLoginRedirect,
  hasSdkworkSkillsPcAuthenticatedSession,
} from './authGateLogic';
import type { SdkworkSkillsPcRuntime } from './bootstrap/runtime';

export interface AdminPermissionGateProps {
  children: ReactNode;
  permission: SkillsAdminPermission;
  runtime: SdkworkSkillsPcRuntime;
}

export function AdminPermissionGate({
  children,
  permission,
  runtime,
}: AdminPermissionGateProps) {
  const location = useLocation();
  const snapshot = runtime.session.getSnapshot();

  if (!hasSdkworkSkillsPcAuthenticatedSession(snapshot)) {
    return <Navigate replace to={buildSdkworkSkillsPcAuthLoginRedirect(location)} />;
  }

  const granted = snapshot.context?.permissionScope ?? [];
  const roleCodes = snapshot.context?.standardRoleCodes ?? [];
  const allowed =
    hasPermissionInScope(granted, permission) ||
    roleCodes.includes(SKILLS_ADMIN_ROLES.superAdmin) ||
    (permission === SKILLS_ADMIN_PERMISSIONS.marketplaceRead &&
      roleCodes.includes(SKILLS_ADMIN_ROLES.operator));

  if (!allowed) {
    return (
      <section style={{ maxWidth: 560, margin: '48px auto' }}>
        <h1>Access denied</h1>
        <p>You do not have permission to access this skills admin surface.</p>
      </section>
    );
  }

  return <>{children}</>;
}
