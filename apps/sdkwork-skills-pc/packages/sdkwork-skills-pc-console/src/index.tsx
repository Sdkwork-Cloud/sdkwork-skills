import { useEffect, useState } from 'react';
import {
  listSkillPackages,
  useSkillsClients,
  type SkillPackageRecord,
} from '@sdkwork/skills-pc-core';

export function ConsoleSkillsPage() {
  const clients = useSkillsClients();
  const [packages, setPackages] = useState<SkillPackageRecord[]>([]);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    listSkillPackages(clients)
      .then((page) => setPackages(page.items))
      .catch((cause: Error) => setError(cause.message));
  }, [clients]);

  return (
    <section>
      <h2>Console Skills</h2>
      {error ? <p role="alert">{error}</p> : null}
      <ul>
        {packages.map((item) => (
          <li key={item.id}>
            {item.displayName} - {item.status}
          </li>
        ))}
      </ul>
    </section>
  );
}
