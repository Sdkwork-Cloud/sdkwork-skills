import { FormEvent, useEffect, useRef, useState } from 'react';
import { isBlank, trim } from '@sdkwork/utils';
import { isDrivePackageRef } from '@sdkwork/skills-pc-commons/driveUri';
import {
  useSkillsClients,
  type CreateCategoryInput,
  type CreatePackageInput,
  type SkillCategoryRecord,
  type SkillPackageRecord,
} from '@sdkwork/skills-pc-core';

import { uploadSkillPackageArchive } from './services/skillPackageUploadService';

export function AdminSkillsPage() {
  const clients = useSkillsClients();
  const fileInputRef = useRef<HTMLInputElement>(null);
  const [packages, setPackages] = useState<SkillPackageRecord[]>([]);
  const [error, setError] = useState<string | null>(null);
  const [uploading, setUploading] = useState(false);
  const [selectedFileName, setSelectedFileName] = useState<string | null>(null);
  const [form, setForm] = useState<CreatePackageInput>({
    skill_id: 'skill.demo.sample',
    package_key: 'demo-sample',
    code: 'demo-sample',
    display_name: 'Demo Sample Skill',
    invocation_kind: 'local-workflow',
    package_ref: '',
    entrypoint: 'run',
    summary: 'Skill package uploaded through sdkwork-drive',
    capability_ids: ['cap.demo.sample.run'],
    tags: ['demo'],
  });

  async function reload() {
    const response = await clients.backend.skills.skillPackages.management.list();
    setPackages(response.items);
  }

  useEffect(() => {
    reload().catch((cause: Error) => setError(cause.message));
  }, [clients]);

  async function onUploadSelectedFile() {
    const file = fileInputRef.current?.files?.[0];
    if (!file) {
      setError('Select a skill package archive to upload through sdkwork-drive.');
      return;
    }
    setUploading(true);
    setError(null);
    try {
      const packageRef = await uploadSkillPackageArchive(clients.drive, file);
      setForm((current) => ({ ...current, package_ref: packageRef }));
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
    if (!isDrivePackageRef(form.package_ref)) {
      setError('Upload a package through sdkwork-drive before creating the skill package record.');
      return;
    }
    try {
      await clients.backend.skills.skillPackages.create(form);
      await reload();
    } catch (cause) {
      setError(cause instanceof Error ? cause.message : String(cause));
    }
  }

  async function onDelete(skillId: string) {
    setError(null);
    try {
      await clients.backend.skills.skillPackages.delete(skillId);
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
          value={form.skill_id}
          onChange={(event) => setForm({ ...form, skill_id: event.target.value })}
          placeholder="skill_id"
          required
        />
        <input
          value={form.display_name}
          onChange={(event) => setForm({ ...form, display_name: event.target.value })}
          placeholder="display_name"
          required
        />
        <div style={{ display: 'grid', gap: 8 }}>
          <input ref={fileInputRef} type="file" accept=".zip,.tar,.gz,.tgz,.skillpkg,application/zip" />
          <button type="button" onClick={onUploadSelectedFile} disabled={uploading}>
            {uploading ? 'Uploading…' : 'Upload Package via sdkwork-drive'}
          </button>
          {selectedFileName ? <p>Uploaded file: {selectedFileName}</p> : null}
          <input
            value={form.package_ref}
            readOnly
            placeholder="package_ref (drive://spaces/.../nodes/...)"
            required
          />
        </div>
        <button type="submit" disabled={isBlank(trim(form.package_ref))}>
          Create Package
        </button>
      </form>
      <ul>
        {packages.map((item) => (
          <li key={item.id}>
            {item.display_name} ({item.skill_id})
            <button type="button" onClick={() => onDelete(item.skill_id)} style={{ marginLeft: 8 }}>
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
    sort_weight: 0,
  });

  async function reload() {
    const response = await clients.backend.skills.categories.management.list();
    setCategories(response.items);
  }

  useEffect(() => {
    reload().catch((cause: Error) => setError(cause.message));
  }, [clients]);

  async function onSubmit(event: FormEvent) {
    event.preventDefault();
    setError(null);
    try {
      await clients.backend.skills.categories.create(form);
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
          onChange={(event) => setForm({ ...form, code: event.target.value })}
          placeholder="code"
          required
        />
        <input
          value={form.name}
          onChange={(event) => setForm({ ...form, name: event.target.value })}
          placeholder="name"
          required
        />
        <button type="submit">Create Category</button>
      </form>
      <ul>
        {categories.map((item) => (
          <li key={item.id}>
            {item.name} ({item.code})
          </li>
        ))}
      </ul>
    </section>
  );
}
