import { useEffect, useState, type FormEvent } from 'react';
import { useParams } from 'react-router-dom';
import {
  listManagedSkillPackages,
  updateSkillPackage,
} from '@sdkwork/skills-pc-admin-core';
import { useSkillsClients, type SkillPackageRecord } from '@sdkwork/skills-pc-core';

export function UpdateSkillPackagePage() {
  const clients = useSkillsClients();
  const { packageId = '' } = useParams();
  const [record, setRecord] = useState<SkillPackageRecord | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [saved, setSaved] = useState(false);

  useEffect(() => {
    listManagedSkillPackages(clients)
      .then((page) => {
        const found = page.items.find((item) => item.id === packageId);
        if (!found) {
          setError(`Skill package ${packageId} was not found.`);
          return;
        }
        setRecord(found);
      })
      .catch((cause: Error) => setError(cause.message));
  }, [clients, packageId]);

  async function onSubmit(event: FormEvent) {
    event.preventDefault();
    if (!record) {
      return;
    }
    setError(null);
    setSaved(false);
    try {
      await updateSkillPackage(clients, packageId, {
        version: record.version,
        ...(record.displayName ? { displayName: record.displayName } : {}),
        ...(record.summary ? { summary: record.summary } : {}),
        ...(record.description ? { description: record.description } : {}),
        ...(record.categories.length > 0 ? { categories: record.categories } : {}),
        ...(record.tags.length > 0 ? { tags: record.tags } : {}),
      });
      setSaved(true);
    } catch (cause) {
      setError(cause instanceof Error ? cause.message : String(cause));
    }
  }

  if (error && !record) {
    return (
      <section>
        <p role="alert">{error}</p>
      </section>
    );
  }
  if (!record) {
    return (
      <section>
        <p>Loading skill package {packageId}…</p>
      </section>
    );
  }

  return (
    <section>
      <h2>Update Skill Package {record.skillKey}</h2>
      {error ? <p role="alert">{error}</p> : null}
      {saved ? <p role="status">Skill package updated.</p> : null}
      <form onSubmit={onSubmit} style={{ display: 'grid', gap: 8, maxWidth: 640 }}>
        <input
          value={record.displayName}
          onChange={(event) => setRecord({ ...record, displayName: event.target.value })}
          placeholder="display name"
          required
        />
        <input
          value={record.summary ?? ''}
          onChange={(event) => setRecord({ ...record, summary: event.target.value || null })}
          placeholder="summary"
        />
        <textarea
          value={record.description ?? ''}
          onChange={(event) => setRecord({ ...record, description: event.target.value || null })}
          placeholder="description"
          rows={4}
        />
        <input
          value={record.categories.join(', ')}
          onChange={(event) =>
            setRecord({
              ...record,
              categories: event.target.value
                .split(',')
                .map((value) => value.trim())
                .filter(Boolean),
            })
          }
          placeholder="categories (comma separated)"
        />
        <input
          value={record.tags.join(', ')}
          onChange={(event) =>
            setRecord({
              ...record,
              tags: event.target.value
                .split(',')
                .map((value) => value.trim())
                .filter(Boolean),
            })
          }
          placeholder="tags (comma separated)"
        />
        <button type="submit">Save Changes</button>
      </form>
    </section>
  );
}
