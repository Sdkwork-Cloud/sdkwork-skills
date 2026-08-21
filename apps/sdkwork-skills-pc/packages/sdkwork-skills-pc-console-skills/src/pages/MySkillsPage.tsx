import { useEffect, useState, type FormEvent } from 'react';
import { isBlank, trim } from '@sdkwork/utils';
import { isDriveArtifactRef } from '@sdkwork/skills-pc-commons/driveUri';
import {
  createOwnSkillPackage,
  deleteOwnSkillPackage,
  listOwnedSkillPackages,
  uploadSkillPackageArchive,
  useSkillsClients,
  type SkillPackageRecord,
} from '@sdkwork/skills-pc-core';
import { Link } from 'react-router-dom';

export function MySkillsPage() {
  const clients = useSkillsClients();
  const [packages, setPackages] = useState<SkillPackageRecord[]>([]);
  const [error, setError] = useState<string | null>(null);

  async function reload() {
    const page = await listOwnedSkillPackages(clients);
    setPackages(page.items);
  }

  useEffect(() => {
    reload().catch((cause: Error) => setError(cause.message));
  }, [clients]);

  async function onDelete(packageId: string) {
    setError(null);
    try {
      await deleteOwnSkillPackage(clients, packageId);
      await reload();
    } catch (cause) {
      setError(cause instanceof Error ? cause.message : String(cause));
    }
  }

  return (
    <section>
      <h2>My Skills</h2>
      <p>
        Skill packages you created are active in your workspace. Marketplace publication is
        managed by administrators.
      </p>
      <p>
        <Link to="/console/skills/create">Create and upload a skill package</Link>
      </p>
      {error ? <p role="alert">{error}</p> : null}
      {packages.length === 0 ? (
        <p>You have not created any skill packages yet.</p>
      ) : (
        <ul>
          {packages.map((item) => (
            <li key={item.id}>
              {item.displayName} ({item.skillKey}) - {item.status} [{item.visibility}]
              <Link to={`/console/skills/edit/${encodeURIComponent(item.id)}`} style={{ marginLeft: 8 }}>
                Edit
              </Link>
              <button type="button" onClick={() => onDelete(item.id)} style={{ marginLeft: 8 }}>
                Delete
              </button>
            </li>
          ))}
        </ul>
      )}
    </section>
  );
}
