import { useEffect, useState, type FormEvent, type ReactNode } from 'react';
import { isBlank, trim } from '@sdkwork/utils';
import {
  listOwnedSkillPackages,
  updateOwnSkillPackage,
  useSkillsClients,
  type SkillPackageRecord,
} from '@sdkwork/skills-pc-core';
import { useSkillsConsoleT } from '../locale.tsx';

function Field({
  hint,
  label,
  children,
}: {
  hint?: string;
  label: string;
  children: ReactNode;
}) {
  return (
    <label className="skills-console-field">
      <span className="skills-console-field-label">{label}</span>
      {children}
      {hint ? <small className="skills-console-field-hint">{hint}</small> : null}
    </label>
  );
}

export interface EditSkillFormProps {
  packageId: string;
  onSuccess?: () => void;
  onCancel?: () => void;
}

export function EditSkillForm({ packageId, onSuccess, onCancel }: EditSkillFormProps) {
  const t = useSkillsConsoleT();
  const clients = useSkillsClients();
  const [record, setRecord] = useState<SkillPackageRecord | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [submitting, setSubmitting] = useState(false);
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
    setRecord(null);
    void listOwnedSkillPackages(clients)
      .then((page) => {
        if (!active) return;
        const found = page.items.find((item) => item.id === packageId) ?? null;
        setRecord(found);
        if (!found) {
          setError(t('edit.notFound', { id: packageId }));
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
  }, [clients, packageId, t]);

  async function onSubmit(event: FormEvent) {
    event.preventDefault();
    if (!record) return;
    setError(null);
    setSubmitting(true);
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
      onSuccess?.();
    } catch (cause) {
      setError(cause instanceof Error ? cause.message : String(cause));
    } finally {
      setSubmitting(false);
    }
  }

  if (!record && !error) {
    return <p className="skills-console-status">{t('edit.loading')}</p>;
  }

  if (!record) {
    return error ? (
      <p className="skills-console-error" role="alert">
        {error}
      </p>
    ) : null;
  }

  return (
    <form className="skills-console-form" onSubmit={onSubmit}>
      {error ? (
        <p className="skills-console-error" role="alert">
          {error}
        </p>
      ) : null}
      <Field label={t('edit.field.skillKey')}>
        <input value={record.skillKey} disabled readOnly aria-label={t('edit.aria.skillKey')} />
      </Field>
      <Field label={t('edit.field.displayName')}>
        <input
          value={form.displayName}
          onChange={(event) => setForm({ ...form, displayName: event.target.value })}
          placeholder={t('edit.placeholder.displayName')}
          required
        />
      </Field>
      <Field label={t('edit.field.summary')}>
        <input
          value={form.summary}
          onChange={(event) => setForm({ ...form, summary: event.target.value })}
          placeholder={t('edit.placeholder.summary')}
        />
      </Field>
      <Field label={t('edit.field.description')}>
        <textarea
          value={form.description}
          onChange={(event) => setForm({ ...form, description: event.target.value })}
          placeholder={t('edit.placeholder.description')}
          rows={4}
        />
      </Field>
      <Field label={t('edit.field.categories')} hint={t('edit.field.categories.hint')}>
        <input
          value={form.categories}
          onChange={(event) => setForm({ ...form, categories: event.target.value })}
          placeholder={t('edit.placeholder.categories')}
        />
      </Field>
      <Field label={t('edit.field.tags')} hint={t('edit.field.tags.hint')}>
        <input
          value={form.tags}
          onChange={(event) => setForm({ ...form, tags: event.target.value })}
          placeholder={t('edit.placeholder.tags')}
        />
      </Field>
      <div className="sdkwork-surface-drawer-form-actions">
        {onCancel ? (
          <button type="button" className="sdkwork-surface-modal-cancel" onClick={onCancel} disabled={submitting}>
            {t('dialog.cancel')}
          </button>
        ) : null}
        <button
          className="skills-console-primary"
          type="submit"
          disabled={isBlank(trim(form.displayName)) || submitting}
        >
          {t('edit.save')}
        </button>
      </div>
    </form>
  );
}
