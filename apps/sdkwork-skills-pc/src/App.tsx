import { Navigate, Route, Routes } from 'react-router-dom';
import { SkillsClientsProvider } from '@sdkwork/skills-pc-core';
import { SKILLS_ADMIN_PERMISSIONS } from '@sdkwork/skills-pc-admin-core';
import { AdminCategoriesPage, AdminSkillsPage } from '@sdkwork/skills-pc-admin';
import { ConsoleSkillsPage } from '@sdkwork/skills-pc-console';
import { CreateSkillPage, MySkillsPage } from '@sdkwork/skills-pc-console-skills';
import {
  PackageArtifactsPage,
  SkillCapabilitiesPage,
  UpdateSkillPackagePage,
} from '@sdkwork/skills-pc-admin-skill';
import { SkillDetailPage, SkillsHubPage } from '@sdkwork/skills-pc-hub';
import { SkillsShell } from '@sdkwork/skills-pc-shell';

import { AdminPermissionGate } from './AdminPermissionGate';
import { AuthGate } from './AuthGate';
import { createSdkworkSkillsPcRuntime } from './bootstrap/runtime';

const runtime = createSdkworkSkillsPcRuntime();

export function App() {
  return (
    <SkillsClientsProvider clients={runtime.sdkClients}>
      <AuthGate runtime={runtime}>
        <Routes>
          <Route element={<SkillsShell />}>
            <Route path="/" element={<Navigate to="/skills-hub" replace />} />
            <Route path="/skills-hub" element={<SkillsHubPage />} />
            <Route path="/skills-hub/:skillId" element={<SkillDetailPage />} />
            <Route path="/console/skills" element={<ConsoleSkillsPage />} />
            <Route path="/console/skills/mine" element={<MySkillsPage />} />
            <Route path="/console/skills/create" element={<CreateSkillPage />} />
            <Route
              path="/admin/skills"
              element={
                <AdminPermissionGate
                  permission={SKILLS_ADMIN_PERMISSIONS.packageManage}
                  runtime={runtime}
                >
                  <AdminSkillsPage
                    grantedPermissions={runtime.session.getSnapshot().context?.permissionScope ?? []}
                    roleCodes={runtime.session.getSnapshot().context?.standardRoleCodes ?? []}
                  />
                </AdminPermissionGate>
              }
            />
            <Route
              path="/admin/skills/:packageId/edit"
              element={
                <AdminPermissionGate
                  permission={SKILLS_ADMIN_PERMISSIONS.packageManage}
                  runtime={runtime}
                >
                  <UpdateSkillPackagePage />
                </AdminPermissionGate>
              }
            />
            <Route
              path="/admin/skills/:packageId/artifacts"
              element={
                <AdminPermissionGate
                  permission={SKILLS_ADMIN_PERMISSIONS.packageManage}
                  runtime={runtime}
                >
                  <PackageArtifactsPage />
                </AdminPermissionGate>
              }
            />
            <Route
              path="/admin/categories"
              element={
                <AdminPermissionGate
                  permission={SKILLS_ADMIN_PERMISSIONS.categoryManage}
                  runtime={runtime}
                >
                  <AdminCategoriesPage />
                </AdminPermissionGate>
              }
            />
            <Route
              path="/admin/capabilities"
              element={
                <AdminPermissionGate
                  permission={SKILLS_ADMIN_PERMISSIONS.capabilityManage}
                  runtime={runtime}
                >
                  <SkillCapabilitiesPage />
                </AdminPermissionGate>
              }
            />
          </Route>
        </Routes>
      </AuthGate>
    </SkillsClientsProvider>
  );
}
