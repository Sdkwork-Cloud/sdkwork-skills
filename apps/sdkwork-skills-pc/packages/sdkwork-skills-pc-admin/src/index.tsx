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
  updateSkillPackage,
} from '@sdkwork/skills-pc-admin-core';

import { uploadSkillPackageArchive } from './services/skillPackageUploadService';
import { ConfirmModal, SurfaceDrawer } from './components/SurfaceOverlay.tsx';

export function AdminSkillsPage({
  grantedPermissions = [],
  roleCodes = [],
  initialEditPackageId = null,
}: {
  grantedPermissions?: readonly string[];
  roleCodes?: readonly string[];
  initialEditPackageId?: string | null;
}) {
  const clients = useSkillsClients();
  const fileInputRef = useRef<HTMLInputElement>(null);
  const [packages, setPackages] = useState<SkillPackageRecord[]>([]);
  const [categories, setCategories] = useState<SkillCategoryRecord[]>([]);
  const [error, setError] = useState<string | null>(null);
  const [uploading, setUploading] = useState(false);
  const [selectedFileName, setSelectedFileName] = useState<string | null>(null);
  const [createOpen, setCreateOpen] = useState(false);
  const [editTarget, setEditTarget] = useState<SkillPackageRecord | null>(null);
  const [deleteTarget, setDeleteTarget] = useState<SkillPackageRecord | null>(null);
  const [deleting, setDeleting] = useState(false);
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

  useEffect(() => {
    if (!initialEditPackageId || packages.length === 0) return;
    const found = packages.find((item) => item.id === initialEditPackageId);
    if (found) setEditTarget(found);
  }, [initialEditPackageId, packages]);

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
      setCreateOpen(false);
      await reload();
    } catch (cause) {
      setError(cause instanceof Error ? cause.message : String(cause));
    }
  }

  async function confirmDelete() {
    if (!deleteTarget) return;
    setDeleting(true);
    setError(null);
    try {
      await deleteSkillPackage(clients, deleteTarget.id);
      setDeleteTarget(null);
      await reload();
    } catch (cause) {
      setError(cause instanceof Error ? cause.message : String(cause));
    } finally {
      setDeleting(false);
    }
  }

  return (
    <section className="skills-console-page">
      <header className="skills-console-header" style={{ marginBottom: 0 }}>
        <h2>Admin Skills</h2>
        <button type="button" className="skills-console-primary" onClick={() => setCreateOpen(true)}>
          Create package
        </button>
      </header>
      {error ? <p role="alert">{error}</p> : null}
      <div className="data-surface">
        <div className="table-frame">
          {packages.length === 0 ? (
            <div className="empty-state">
              <span>No skill packages yet.</span>
              <button type="button" className="skills-console-primary" onClick={() => setCreateOpen(true)}>
                Create package
              </button>
            </div>
          ) : (
            <table>
              <thead>
                <tr>
                  <th>Name</th>
                  <th>Skill key</th>
                  <th>Categories</th>
                  <th>Actions</th>
                </tr>
              </thead>
              <tbody>
                {packages.map((item) => (
                  <tr key={item.id}>
                    <td>{item.displayName}</td>
                    <td>{item.skillKey}</td>
                    <td>{item.categories.length > 0 ? item.categories.join(', ') : '—'}</td>
                    <td>
                      <div className="skills-console-actions">
                        <button type="button" onClick={() => setEditTarget(item)}>
                          Edit
                        </button>
                        <button type="button" onClick={() => setDeleteTarget(item)}>
                          Delete
                        </button>
                      </div>
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          )}
        </div>
      </div>
      <SurfaceDrawer
        open={createOpen}
        title="Create package and artifact"
        onClose={() => setCreateOpen(false)}
      >
      <form onSubmit={onSubmit} style={{ display: 'grid', gap: 8 }}>
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
      </SurfaceDrawer>
      <SurfaceDrawer
        open={editTarget != null}
        title={editTarget ? `Update ${editTarget.skillKey}` : 'Update package'}
        onClose={() => setEditTarget(null)}
      >
        {editTarget ? (
          <form
            onSubmit={(event) => {
              event.preventDefault();
              const current = editTarget;
              setError(null);
              void updateSkillPackage(clients, current.id, {
                version: current.version,
                ...(current.displayName ? { displayName: current.displayName } : {}),
                ...(current.summary ? { summary: current.summary } : {}),
                ...(current.description ? { description: current.description } : {}),
                ...(current.categories.length > 0 ? { categories: current.categories } : {}),
                ...(current.tags.length > 0 ? { tags: current.tags } : {}),
              })
                .then(async () => {
                  setEditTarget(null);
                  await reload();
                })
                .catch((cause: unknown) => {
                  setError(cause instanceof Error ? cause.message : String(cause));
                });
            }}
            style={{ display: 'grid', gap: 8 }}
          >
            <input
              value={editTarget.displayName}
              onChange={(event) => setEditTarget({ ...editTarget, displayName: event.target.value })}
              placeholder="display name"
              required
            />
            <input
              value={editTarget.summary ?? ''}
              onChange={(event) => setEditTarget({ ...editTarget, summary: event.target.value || null })}
              placeholder="summary"
            />
            <textarea
              value={editTarget.description ?? ''}
              onChange={(event) => setEditTarget({ ...editTarget, description: event.target.value || null })}
              placeholder="description"
              rows={4}
            />
            <div className="sdkwork-surface-drawer-form-actions">
              <button type="button" onClick={() => setEditTarget(null)}>Cancel</button>
              <button type="submit">Save Changes</button>
            </div>
          </form>
        ) : null}
      </SurfaceDrawer>
      <ConfirmModal
        open={deleteTarget != null}
        title="Delete skill package?"
        description={`Delete “${deleteTarget?.displayName ?? ''}”. This cannot be undone.`}
        confirmLabel="Delete"
        cancelLabel="Cancel"
        busy={deleting}
        onCancel={() => setDeleteTarget(null)}
        onConfirm={() => {
          void confirmDelete();
        }}
      />
    </section>
  );
}

export function AdminCategoriesPage() {
  const clients = useSkillsClients();
  const [categories, setCategories] = useState<SkillCategoryRecord[]>([]);
  const [error, setError] = useState<string | null>(null);
  const [createOpen, setCreateOpen] = useState(false);
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
      setCreateOpen(false);
      await reload();
    } catch (cause) {
      setError(cause instanceof Error ? cause.message : String(cause));
    }
  }

  return (
    <section className="skills-console-page">
      <header className="skills-console-header" style={{ marginBottom: 0 }}>
        <h2>Admin Categories</h2>
        <button type="button" className="skills-console-primary" onClick={() => setCreateOpen(true)}>
          Create category
        </button>
      </header>
      {error ? <p role="alert">{error}</p> : null}
      <div className="data-surface">
        <div className="table-frame">
          {categories.length === 0 ? (
            <div className="empty-state">
              <span>No categories yet.</span>
              <button type="button" className="skills-console-primary" onClick={() => setCreateOpen(true)}>
                Create category
              </button>
            </div>
          ) : (
            <table>
              <thead>
                <tr>
                  <th>Name</th>
                  <th>Code</th>
                  <th>Permission</th>
                </tr>
              </thead>
              <tbody>
                {categories.map((item) => (
                  <tr key={item.id}>
                    <td>{item.name}</td>
                    <td>{item.code}</td>
                    <td>{item.permissionCode}</td>
                  </tr>
                ))}
              </tbody>
            </table>
          )}
        </div>
      </div>
      <SurfaceDrawer open={createOpen} title="Create category" onClose={() => setCreateOpen(false)}>
        <form onSubmit={onSubmit} style={{ display: 'grid', gap: 8 }}>
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
          <div className="sdkwork-surface-drawer-form-actions">
            <button type="button" onClick={() => setCreateOpen(false)}>Cancel</button>
            <button type="submit">Create Category</button>
          </div>
        </form>
      </SurfaceDrawer>
    </section>
  );
}
