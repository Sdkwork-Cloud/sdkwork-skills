import { Navigate, Route, Routes } from 'react-router-dom';
import { SkillsShell } from '@sdkwork/skills-pc-shell';

export function App() {
  return (
    <Routes>
      <Route element={<SkillsShell />}>
        <Route path="/" element={<Navigate to="/skills-hub" replace />} />
        <Route path="/skills-hub" element={<div>Skills Hub</div>} />
        <Route path="/skills-hub/:skillId" element={<div>Skill Detail</div>} />
        <Route path="/console/skills" element={<div>Console Skills CRUD</div>} />
        <Route path="/admin/skills" element={<div>Admin Skills CRUD</div>} />
        <Route path="/admin/categories" element={<div>Admin Categories</div>} />
      </Route>
    </Routes>
  );
}
