import { Navigate, Route, Routes } from 'react-router-dom';
import { SkillsClientsProvider } from '@sdkwork/skills-pc-core';
import { SKILLS_ADMIN_PERMISSIONS } from '@sdkwork/skills-pc-admin-core';
import { AdminCategoriesPage, AdminSkillsPage } from '@sdkwork/skills-pc-admin';
import { ConsoleSkillsPage } from '@sdkwork/skills-pc-console';
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
            <Route
              path="/admin/skills"
              element={
                <AdminPermissionGate
                  permission={SKILLS_ADMIN_PERMISSIONS.packageManage}
                  runtime={runtime}
                >
                  <AdminSkillsPage />
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
          </Route>
        </Routes>
      </AuthGate>
    </SkillsClientsProvider>
  );
}
