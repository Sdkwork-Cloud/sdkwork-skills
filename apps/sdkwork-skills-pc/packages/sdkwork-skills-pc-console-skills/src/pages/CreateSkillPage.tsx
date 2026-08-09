import { useRef, useState, type FormEvent } from 'react';
import { isBlank, trim } from '@sdkwork/utils';
import { isDriveArtifactRef } from '@sdkwork/skills-pc-commons/driveUri';
import {
  createOwnSkillPackage,
  uploadSkillPackageArchive,
  useSkillsClients,
} from '@sdkwork/skills-pc-core';

const EMPTY_FORM = {
  skillKey: 'skill.selfservice.sample',
  code: 'selfservice-sample',
  displayName: 'Self-service Sample Skill',
  summary: 'Skill package uploaded by the workspace user',
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

export function CreateSkillPage() {
  const clients = useSkillsClients();
  const fileInputRef = useRef<HTMLInputElement>(null);
  const [form, setForm] = useState(EMPTY_FORM);
  const [error, setError] = useState<string | null>(null);
  const [uploading, setUploading] = useState(false);
  const [selectedFileName, setSelectedFileName] = useState<string | null>(null);
  const [created, setCreated] = useState<string | null>(null);

  async function onUploadSelectedFile() {
    const file = fileInputRef.current?.files?.[0];
    if (!file) {
      setError('Select a skill package archive to upload through sdkwork-drive.');
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
    setCreated(null);
    if (!isDriveArtifactRef(form.initialArtifact.artifactRef)) {
      setError('Upload an artifact through sdkwork-drive before creating the skill package.');
      return;
    }
    if (!/^[0-9a-f]{64}$/.test(form.initialArtifact.checksumSha256)) {
      setError('The uploaded artifact is missing a valid SHA-256 checksum.');
      return;
    }
    try {
      const record = await createOwnSkillPackage(clients, form);
      setCreated(record.id);
      setForm(EMPTY_FORM);
      setSelectedFileName(null);
      if (fileInputRef.current) {
        fileInputRef.current.value = '';
      }
    } catch (cause) {
      setError(cause instanceof Error ? cause.message : String(cause));
    }
  }

  return (
    <section>
      <h2>Create Skill Package</h2>
      <p>
        Upload a skill archive (through sdkwork-drive) and create an active package that your
        workspace can install immediately.
      </p>
      {error ? <p role="alert">{error}</p> : null}
      {created ? <p role="status">Created skill package {created}.</p> : null}
      <form onSubmit={onSubmit} style={{ display: 'grid', gap: 8, maxWidth: 640 }}>
        <input
          value={form.skillKey}
          onChange={(event) => setForm({ ...form, skillKey: event.target.value })}
          placeholder="skill key (skill.<segment>.<segment>)"
          required
        />
        <input
          value={form.code}
          onChange={(event) => setForm({ ...form, code: event.target.value })}
          placeholder="package code"
          required
        />
        <input
          value={form.displayName}
          onChange={(event) => setForm({ ...form, displayName: event.target.value })}
          placeholder="display name"
          required
        />
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
          placeholder="artifact version label"
          required
        />
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
          placeholder="entrypoint"
          required
        />
        <div style={{ display: 'grid', gap: 8 }}>
          <input
            ref={fileInputRef}
            type="file"
            accept=".zip,.tar,.gz,.tgz,.skillpkg,application/zip"
          />
          <button type="button" onClick={onUploadSelectedFile} disabled={uploading}>
            {uploading ? 'Uploading...' : 'Upload Archive via sdkwork-drive'}
          </button>
          {selectedFileName ? <p>Uploaded file: {selectedFileName}</p> : null}
          <input
            value={form.initialArtifact.artifactRef}
            readOnly
            placeholder="artifactRef (drive://spaces/.../nodes/...)"
            required
          />
        </div>
        <button
          type="submit"
          disabled={isBlank(trim(form.initialArtifact.artifactRef)) || uploading}
        >
          Create Package
        </button>
      </form>
    </section>
  );
}
