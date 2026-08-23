import { Navigate } from 'react-router-dom';

/** Deep-link compatibility: create opens the list drawer via query. */
export function CreateSkillPage() {
  return <Navigate to="/console/skills?create=1" replace />;
}
