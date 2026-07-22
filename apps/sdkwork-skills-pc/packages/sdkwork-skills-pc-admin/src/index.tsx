import { useEffect, useRef, useState, type FormEvent } from 'react';
import { isBlank, trim } from '@sdkwork/utils';
import { isDriveArtifactRef } from '@sdkwork/skills-pc-commons/driveUri';
import {
  useSkillsClients,
  type CreateCategoryInput,
  type CreatePackageInput,
  type SkillCategoryRecord,
  type SkillPackageRecord,
} from '@sdkwork/skills-pc-core';
import {
  canManagePackagesInCategories,
  createSkillCategory,
  createSkillPackage,
  deleteSkillPackage,
  listManagedSkillCategories,
  listManagedSkillPackages,
  packageManagePermissionForCategory,
} from '@sdkwork/skills-pc-admin-core';

import { uploadSkillPackageArchive } from './services/skillPackageUploadService';

export function AdminSkillsPage({
  grantedPermissions = [],
  roleCodes = [],
}: {
  grantedPermissions?: readonly string[];
  roleCodes?: readonly string[];
}) {
  const clients = useSkillsClients();
  const fileInputRef = useRef<HTMLInputElement>(null);
  const [packages, setPackages] = useState<SkillPackageRecord[]>([]);
  const [categories, setCategories] = useState<SkillCategoryRecord[]>([]);
  const [error, setError] = useState<string | null>(null);
  const [uploading, setUploading] = useState(false);
  const [selectedFileName, setSelectedFileName] = useState<string | null>(null);
  const [form, setForm] = useState<CreatePackageInput>({
    skillKey: 'skill.demo.sample',
    packageKey: 'demo-sample',
    code: 'demo-sample',
    displayName: 'Demo Sample Skill',
    summary: 'Skill package uploaded through sdkwork-drive',
    categories: [],
    tags: ['demo'],
    status: 'active',
    visibility: 'tenant',
    initialArtifact: {
      versionLabel: '1.0.0',
      artifactRef: '',
      checksumSha256: '',
      sizeBytes: null,
      invocationKind: 'local-workflow',
      entrypoint: 'run',
      inputSchema: {},
      outputSchema: {},
      configSchema: {},
      defaultConfig: {},
      status: 'published',
      capabilityKeys: [],
    },
  });

  async function reload() {
    const [packagePage, categoryPage] = await Promise.all([
      listManagedSkillPackages(clients),
      listManagedSkillCategories(clients),
    ]);
    setPackages(packagePage.items);
    setCategories(categoryPage.items);
  }

  useEffect(() => {
    reload().catch((cause: Error) => setError(cause.message));
  }, [clients]);

  async function onUploadSelectedFile() {
    const file = fileInputRef.current?.files?.[0];
    if (!file) {
      setError('Select a Skill package archive to upload through sdkwork-drive.');
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
      setError('Upload an artifact through sdkwork-drive before creating the Skill package.');
      return;
    }
    if (!/^[0-9a-f]{64}$/.test(form.initialArtifact.checksumSha256)) {
      setError('The uploaded artifact is missing a valid SHA-256 checksum.');
      return;
    }
    const selectedCategories = form.categories ?? [];
    if (
      !canManagePackagesInCategories(
        grantedPermissions,
        roleCodes,
        selectedCategories,
        categories,
      )
    ) {
      setError('You do not have package-manage permission for the selected categories.');
      return;
    }
    try {
      await createSkillPackage(clients, form);
      await reload();
    } catch (cause) {
      setError(cause instanceof Error ? cause.message : String(cause));
    }
  }

  async function onDelete(packageId: string) {
    setError(null);
    try {
      await deleteSkillPackage(clients, packageId);
      await reload();
    } catch (cause) {
      setError(cause instanceof Error ? cause.message : String(cause));
    }
  }

  return (
    <section>
      <h2>Admin Skills</h2>
      {error ? <p role="alert">{error}</p> : null}
      <form onSubmit={onSubmit} style={{ display: 'grid', gap: 8, maxWidth: 640, marginBottom: 24 }}>
        <input
          value={form.skillKey}
          onChange={(event) => setForm({ ...form, skillKey: event.target.value })}
          placeholder="skill key"
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
          placeholder="artifact version"
          required
        />
        <fieldset style={{ display: 'grid', gap: 8, border: '1px solid #ddd', padding: 12 }}>
          <legend>Categories</legend>
          {categories.map((category) => (
            <label key={category.id} style={{ display: 'flex', gap: 8, alignItems: 'center' }}>
              <input
                type="checkbox"
                checked={(form.categories ?? []).includes(category.code)}
                onChange={(event) => {
                  const current = new Set(form.categories ?? []);
                  if (event.target.checked) {
                    current.add(category.code);
                  } else {
                    current.delete(category.code);
                  }
                  setForm({ ...form, categories: [...current] });
                }}
              />
              <span>
                {category.name} ({category.code}) - {category.permissionCode}
              </span>
            </label>
          ))}
        </fieldset>
        <div style={{ display: 'grid', gap: 8 }}>
          <input ref={fileInputRef} type="file" accept=".zip,.tar,.gz,.tgz,.skillpkg,application/zip" />
          <button type="button" onClick={onUploadSelectedFile} disabled={uploading}>
            {uploading ? 'Uploading...' : 'Upload Artifact via sdkwork-drive'}
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
          Create Package And Artifact
        </button>
      </form>
      <ul>
        {packages.map((item) => (
          <li key={item.id}>
            {item.displayName} ({item.skillKey})
            {item.categories.length > 0 ? ` [${item.categories.join(', ')}]` : ''}
            <button type="button" onClick={() => onDelete(item.id)} style={{ marginLeft: 8 }}>
              Delete
            </button>
          </li>
        ))}
      </ul>
    </section>
  );
}

export function AdminCategoriesPage() {
  const clients = useSkillsClients();
  const [categories, setCategories] = useState<SkillCategoryRecord[]>([]);
  const [error, setError] = useState<string | null>(null);
  const [form, setForm] = useState<CreateCategoryInput>({
    code: 'general',
    name: 'General',
    description: 'Default category',
    sortWeight: 0,
    permissionCode: packageManagePermissionForCategory('general'),
  });

  async function reload() {
    const page = await listManagedSkillCategories(clients);
    setCategories(page.items);
  }

  useEffect(() => {
    reload().catch((cause: Error) => setError(cause.message));
  }, [clients]);

  async function onSubmit(event: FormEvent) {
    event.preventDefault();
    setError(null);
    try {
      await createSkillCategory(clients, form);
      await reload();
    } catch (cause) {
      setError(cause instanceof Error ? cause.message : String(cause));
    }
  }

  return (
    <section>
      <h2>Admin Categories</h2>
      {error ? <p role="alert">{error}</p> : null}
      <form onSubmit={onSubmit} style={{ display: 'grid', gap: 8, maxWidth: 480, marginBottom: 24 }}>
        <input
          value={form.code}
          onChange={(event) => {
            const code = event.target.value;
            setForm({
              ...form,
              code,
              permissionCode: packageManagePermissionForCategory(code),
            });
          }}
          placeholder="code"
          required
        />
        <input
          value={form.name}
          onChange={(event) => setForm({ ...form, name: event.target.value })}
          placeholder="name"
          required
        />
        <input
          value={form.permissionCode ?? ''}
          onChange={(event) => setForm({ ...form, permissionCode: event.target.value })}
          placeholder="permission code"
          required
        />
        <button type="submit">Create Category</button>
      </form>
      <ul>
        {categories.map((item) => (
          <li key={item.id}>
            {item.name} ({item.code}) - {item.permissionCode}
          </li>
        ))}
      </ul>
    </section>
  );
}
