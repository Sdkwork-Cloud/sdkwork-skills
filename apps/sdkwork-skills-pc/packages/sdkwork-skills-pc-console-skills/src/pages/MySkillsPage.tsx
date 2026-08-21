import { useEffect, useState } from 'react';
import { Link } from 'react-router-dom';
import {
  deleteOwnSkillPackage,
  listOwnedSkillPackages,
  useSkillsClients,
  type SkillPackageRecord,
} from '@sdkwork/skills-pc-core';

export function MySkillsPage() {
  const clients = useSkillsClients();
  const [packages, setPackages] = useState<SkillPackageRecord[]>([]);
  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState(true);

  async function reload() {
    setLoading(true);
    try {
      const page = await listOwnedSkillPackages(clients);
      setPackages(page.items);
      setError(null);
    } finally {
      setLoading(false);
    }
  }

  useEffect(() => {
    reload().catch((cause: Error) => {
      setError(cause.message);
      setLoading(false);
    });
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

  if (loading) {
    return <p className="skills-console-status">Loading your skill packages…</p>;
  }

  return (
    <section className="skills-console-page">
      <header className="skills-console-header">
        <div>
          <h2>My Skills</h2>
          <p>
            Skill packages you create stay active in your workspace. Marketplace publication is
            managed by administrators.
          </p>
        </div>
        <Link className="skills-console-primary" to="/console/skills/create">
          Create skill package
        </Link>
      </header>
      {error ? (
        <p className="skills-console-error" role="alert">
          {error}
        </p>
      ) : null}
      {packages.length === 0 ? (
        <div className="skills-console-empty">
          <h3>No skill packages yet</h3>
          <p>Create and upload a skill archive to manage it here.</p>
          <Link to="/console/skills/create">Create and upload a skill package</Link>
        </div>
      ) : (
        <ul className="skills-console-list">
          {packages.map((item) => (
            <li key={item.id}>
              <div>
                <strong>{item.displayName}</strong>
                <span>
                  {item.skillKey} · {item.status} · {item.visibility}
                </span>
              </div>
              <div className="skills-console-actions">
                <Link to={`/console/skills/edit/${encodeURIComponent(item.id)}`}>Edit</Link>
                <button type="button" onClick={() => onDelete(item.id)}>
                  Delete
                </button>
              </div>
            </li>
          ))}
        </ul>
      )}
    </section>
  );
}
