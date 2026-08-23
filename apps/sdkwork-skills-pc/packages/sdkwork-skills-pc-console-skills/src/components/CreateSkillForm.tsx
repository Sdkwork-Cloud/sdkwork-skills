import { useRef, useState, type FormEvent, type ReactNode } from 'react';
import { isBlank, trim } from '@sdkwork/utils';
import { isDriveArtifactRef } from '@sdkwork/skills-pc-commons/driveUri';
import {
  createOwnSkillPackage,
  uploadSkillPackageArchive,
  useSkillsClients,
} from '@sdkwork/skills-pc-core';
import { useSkillsConsoleT } from '../locale.tsx';

function createEmptyForm(t: ReturnType<typeof useSkillsConsoleT>) {
  return {
    skillKey: 'skill.selfservice.sample',
    code: 'selfservice-sample',
    displayName: t('create.default.displayName'),
    summary: t('create.default.summary'),
    categories: [] as string[],
    tags: ['self-service'] as string[],
    initialArtifact: {
      versionLabel: '1.0.0',
      artifactRef: '',
      checksumSha256: '',
      sizeBytes: null as string | null,
      invocationKind: 'local-workflow' as const,
      entrypoint: 'run',
      inputSchema: {} as Record<string, unknown>,
      outputSchema: {} as Record<string, unknown>,
      configSchema: {} as Record<string, unknown>,
      defaultConfig: {} as Record<string, unknown>,
      capabilityKeys: [] as string[],
    },
  };
}

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

export interface CreateSkillFormProps {
  onSuccess?: (packageId: string) => void;
  onCancel?: () => void;
}

export function CreateSkillForm({ onSuccess, onCancel }: CreateSkillFormProps) {
  const t = useSkillsConsoleT();
  const clients = useSkillsClients();
  const fileInputRef = useRef<HTMLInputElement>(null);
  const [form, setForm] = useState(() => createEmptyForm(t));
  const [error, setError] = useState<string | null>(null);
  const [uploading, setUploading] = useState(false);
  const [selectedFileName, setSelectedFileName] = useState<string | null>(null);
  const [submitting, setSubmitting] = useState(false);

  async function onUploadSelectedFile() {
    const file = fileInputRef.current?.files?.[0];
    if (!file) {
      setError(t('create.error.selectFile'));
      return;
    }
    setUploading(true);
    setError(null);
    try {
      const artifact = await uploadSkillPackageArchive(clients.drive, file);
      setForm((current) => ({
        ...current,
        initialArtifact: {
          ...current.initialArtifact,
          artifactRef: artifact.artifactRef,
          checksumSha256: artifact.checksumSha256,
          sizeBytes: artifact.sizeBytes,
        },
      }));
      setSelectedFileName(file.name);
    } catch (cause) {
      setError(cause instanceof Error ? cause.message : String(cause));
    } finally {
      setUploading(false);
    }
  }

  async function onSubmit(event: FormEvent) {
    event.preventDefault();
    setError(null);
    if (!isDriveArtifactRef(form.initialArtifact.artifactRef)) {
      setError(t('create.error.needArtifact'));
      return;
    }
    if (!/^[0-9a-f]{64}$/.test(form.initialArtifact.checksumSha256)) {
      setError(t('create.error.checksum'));
      return;
    }
    setSubmitting(true);
    try {
      const record = await createOwnSkillPackage(clients, form);
      setForm(createEmptyForm(t));
      setSelectedFileName(null);
      if (fileInputRef.current) {
        fileInputRef.current.value = '';
      }
      onSuccess?.(record.id);
    } catch (cause) {
      setError(cause instanceof Error ? cause.message : String(cause));
    } finally {
      setSubmitting(false);
    }
  }

  return (
    <form className="skills-console-form" onSubmit={onSubmit}>
      {error ? (
        <p className="skills-console-error" role="alert">
          {error}
        </p>
      ) : null}
      <Field label={t('create.field.skillKey')}>
        <input
          value={form.skillKey}
          onChange={(event) => setForm({ ...form, skillKey: event.target.value })}
          placeholder={t('create.placeholder.skillKey')}
          required
        />
      </Field>
      <Field label={t('create.field.code')}>
        <input
          value={form.code}
          onChange={(event) => setForm({ ...form, code: event.target.value })}
          placeholder={t('create.placeholder.code')}
          required
        />
      </Field>
      <Field label={t('create.field.displayName')}>
        <input
          value={form.displayName}
          onChange={(event) => setForm({ ...form, displayName: event.target.value })}
          placeholder={t('create.placeholder.displayName')}
          required
        />
      </Field>
      <Field label={t('create.field.version')}>
        <input
          value={form.initialArtifact.versionLabel}
          onChange={(event) =>
            setForm({
              ...form,
              initialArtifact: {
                ...form.initialArtifact,
                versionLabel: event.target.value,
              },
            })
          }
          placeholder={t('create.placeholder.version')}
          required
        />
      </Field>
      <Field label={t('create.field.entrypoint')}>
        <input
          value={form.initialArtifact.entrypoint}
          onChange={(event) =>
            setForm({
              ...form,
              initialArtifact: {
                ...form.initialArtifact,
                entrypoint: event.target.value,
              },
            })
          }
          placeholder={t('create.placeholder.entrypoint')}
          required
        />
      </Field>
      <Field label={t('create.field.archive')}>
        <input
          ref={fileInputRef}
          type="file"
          accept=".zip,.tar,.gz,.tgz,.skillpkg,application/zip"
          aria-label={t('create.field.archive')}
        />
        <button type="button" onClick={onUploadSelectedFile} disabled={uploading || submitting}>
          {uploading ? t('create.uploading') : t('create.upload')}
        </button>
        {selectedFileName ? <p>{t('create.uploadedFile', { name: selectedFileName })}</p> : null}
      </Field>
      <Field label={t('create.field.artifactRef')}>
        <input
          value={form.initialArtifact.artifactRef}
          readOnly
          placeholder={t('create.placeholder.artifactRef')}
          required
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
          disabled={isBlank(trim(form.initialArtifact.artifactRef)) || uploading || submitting}
        >
          {t('create.submit')}
        </button>
      </div>
    </form>
  );
}
