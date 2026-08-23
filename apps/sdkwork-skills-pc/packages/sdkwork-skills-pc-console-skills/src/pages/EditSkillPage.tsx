import { Navigate, useParams } from 'react-router-dom';

/** Deep-link compatibility: edit opens the list drawer via query. */
export function EditSkillPage() {
  const { packageId: routePackageId = '' } = useParams<{ packageId: string }>();
  const packageId = decodeURIComponent(routePackageId);
  const target = packageId
    ? `/console/skills?edit=${encodeURIComponent(packageId)}`
    : '/console/skills';
  return <Navigate to={target} replace />;
}
