import { useEffect, useState, type FormEvent } from 'react';
import { Link, useNavigate, useParams } from 'react-router-dom';
import { isBlank, trim } from '@sdkwork/utils';
import {
  listOwnedSkillPackages,
  updateOwnSkillPackage,
  useSkillsClients,
  type SkillPackageRecord,
} from '@sdkwork/skills-pc-core';

export function EditSkillPage() {
  const { packageId: routePackageId = '' } = useParams<{ packageId: string }>();
  const packageId = decodeURIComponent(routePackageId);
  const clients = useSkillsClients();
  const navigate = useNavigate();
  const [record, setRecord] = useState<SkillPackageRecord | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [form, setForm] = useState({
    displayName: '',
    summary: '',
    description: '',
    categories: '',
    tags: '',
  });

  useEffect(() => {
    let active = true;
    setError(null);
    void listOwnedSkillPackages(clients)
      .then((page) => {
        if (!active) return;
        const found = page.items.find((item) => item.id === packageId) ?? null;
        setRecord(found);
        if (!found) {
          setError(`Skill package ${packageId} was not found in your workspace.`);
          return;
        }
        setForm({
          displayName: found.displayName ?? '',
          summary: found.summary ?? '',
          description: found.description ?? '',
          categories: (found.categories ?? []).join(', '),
          tags: (found.tags ?? []).join(', '),
        });
      })
      .catch((cause: unknown) => {
        if (active) {
          setError(cause instanceof Error ? cause.message : String(cause));
        }
      });
    return () => {
      active = false;
    };
  }, [clients, packageId]);

  async function onSubmit(event: FormEvent) {
    event.preventDefault();
    if (!record) return;
    setError(null);
    try {
      await updateOwnSkillPackage(clients, packageId, {
        version: record.version,
        displayName: trim(form.displayName),
        summary: trim(form.summary) || null,
        description: trim(form.description) || null,
        categories: form.categories
          .split(',')
          .map((value) => trim(value))
          .filter((value) => value.length > 0),
        tags: form.tags
          .split(',')
          .map((value) => trim(value))
          .filter((value) => value.length > 0),
      });
      navigate('/console/skills');
    } catch (cause) {
      setError(cause instanceof Error ? cause.message : String(cause));
    }
  }

  return (
    <section>
      <h2>Edit Skill Package</h2>
      <p>
        <Link to="/console/skills">Back to My Skills</Link>
      </p>
      {error ? <p role="alert">{error}</p> : null}
      {!record && !error ? <p>Loading…</p> : null}
      {record ? (
        <form onSubmit={onSubmit} style={{ display: 'grid', gap: 8, maxWidth: 640 }}>
          <input value={record.skillKey} disabled readOnly aria-label="skill key" />
          <input
            value={form.displayName}
            onChange={(event) => setForm({ ...form, displayName: event.target.value })}
            placeholder="display name"
            required
          />
          <input
            value={form.summary}
            onChange={(event) => setForm({ ...form, summary: event.target.value })}
            placeholder="summary"
          />
          <textarea
            value={form.description}
            onChange={(event) => setForm({ ...form, description: event.target.value })}
            placeholder="description"
            rows={4}
          />
          <input
            value={form.categories}
            onChange={(event) => setForm({ ...form, categories: event.target.value })}
            placeholder="categories (comma separated)"
          />
          <input
            value={form.tags}
            onChange={(event) => setForm({ ...form, tags: event.target.value })}
            placeholder="tags (comma separated)"
          />
          <button type="submit" disabled={isBlank(trim(form.displayName))}>
            Save changes
          </button>
        </form>
      ) : null}
    </section>
  );
}
