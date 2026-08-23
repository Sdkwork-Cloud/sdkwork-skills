import { useEffect, useState } from 'react';
import { useSearchParams } from 'react-router-dom';
import {
  deleteOwnSkillPackage,
  listOwnedSkillPackages,
  useSkillsClients,
  type SkillPackageRecord,
} from '@sdkwork/skills-pc-core';
import { CreateSkillForm } from '../components/CreateSkillForm.tsx';
import { EditSkillForm } from '../components/EditSkillForm.tsx';
import { ConfirmModal, SurfaceDrawer } from '../components/SurfaceOverlay.tsx';
import {
  formatSkillsStatusLocalized,
  formatSkillsVisibilityLocalized,
} from '../i18n.ts';
import { useSkillsConsoleLocale, useSkillsConsoleT } from '../locale.tsx';

type DrawerState =
  | { kind: 'create' }
  | { kind: 'edit'; packageId: string }
  | null;

export function MySkillsPage() {
  const t = useSkillsConsoleT();
  const locale = useSkillsConsoleLocale();
  const clients = useSkillsClients();
  const [searchParams, setSearchParams] = useSearchParams();
  const [packages, setPackages] = useState<SkillPackageRecord[]>([]);
  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState(true);
  const [drawer, setDrawer] = useState<DrawerState>(null);
  const [deleteTarget, setDeleteTarget] = useState<SkillPackageRecord | null>(null);
  const [deleting, setDeleting] = useState(false);

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

  useEffect(() => {
    const create = searchParams.get('create');
    const editId = searchParams.get('edit');
    if (create === '1' || create === 'true') {
      setDrawer({ kind: 'create' });
      return;
    }
    if (editId) {
      setDrawer({ kind: 'edit', packageId: editId });
    }
  }, [searchParams]);

  function clearOverlayParams() {
    if (!searchParams.has('create') && !searchParams.has('edit')) return;
    const next = new URLSearchParams(searchParams);
    next.delete('create');
    next.delete('edit');
    setSearchParams(next, { replace: true });
  }

  function closeDrawer() {
    setDrawer(null);
    clearOverlayParams();
  }

  async function confirmDelete() {
    if (!deleteTarget) return;
    setDeleting(true);
    setError(null);
    try {
      await deleteOwnSkillPackage(clients, deleteTarget.id);
      setDeleteTarget(null);
      await reload();
    } catch (cause) {
      setError(cause instanceof Error ? cause.message : String(cause));
    } finally {
      setDeleting(false);
    }
  }

  if (loading) {
    return <p className="skills-console-status">{t('mine.loading')}</p>;
  }

  return (
    <section className="skills-console-page">
      <header className="skills-console-header">
        <div>
          <h2>{t('mine.title')}</h2>
          <p>{t('mine.description')}</p>
        </div>
        <button
          type="button"
          className="skills-console-primary"
          onClick={() => setDrawer({ kind: 'create' })}
        >
          {t('mine.create')}
        </button>
      </header>
      {error ? (
        <p className="skills-console-error" role="alert">
          {error}
        </p>
      ) : null}
      {packages.length === 0 ? (
        <div className="data-surface">
          <div className="table-frame">
            <div className="empty-state">
              <h3>{t('mine.empty.title')}</h3>
              <p>{t('mine.empty.description')}</p>
              <button type="button" className="skills-console-primary" onClick={() => setDrawer({ kind: 'create' })}>
                {t('mine.empty.action')}
              </button>
            </div>
          </div>
        </div>
      ) : (
        <div className="data-surface">
          <div className="table-frame">
            <table>
              <thead>
                <tr>
                  <th>{t('mine.column.name')}</th>
                  <th>{t('mine.column.key')}</th>
                  <th>{t('mine.column.status')}</th>
                  <th>{t('mine.column.visibility')}</th>
                  <th>{t('mine.column.actions')}</th>
                </tr>
              </thead>
              <tbody>
                {packages.map((item) => (
                  <tr key={item.id}>
                    <td>{item.displayName}</td>
                    <td>{item.skillKey}</td>
                    <td>{formatSkillsStatusLocalized(locale, item.status)}</td>
                    <td>{formatSkillsVisibilityLocalized(locale, item.visibility)}</td>
                    <td>
                      <div className="skills-console-actions">
                        <button type="button" onClick={() => setDrawer({ kind: 'edit', packageId: item.id })}>
                          {t('mine.edit')}
                        </button>
                        <button type="button" onClick={() => setDeleteTarget(item)}>
                          {t('mine.delete')}
                        </button>
                      </div>
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </div>
      )}

      <SurfaceDrawer
        open={drawer?.kind === 'create'}
        title={t('create.title')}
        description={t('create.description')}
        onClose={closeDrawer}
      >
        <CreateSkillForm
          onCancel={closeDrawer}
          onSuccess={async () => {
            closeDrawer();
            await reload();
          }}
        />
      </SurfaceDrawer>

      <SurfaceDrawer
        open={drawer?.kind === 'edit'}
        title={t('edit.title')}
        onClose={closeDrawer}
      >
        {drawer?.kind === 'edit' ? (
          <EditSkillForm
            packageId={drawer.packageId}
            onCancel={closeDrawer}
            onSuccess={async () => {
              closeDrawer();
              await reload();
            }}
          />
        ) : null}
      </SurfaceDrawer>

      <ConfirmModal
        open={deleteTarget != null}
        title={t('mine.delete.confirmTitle')}
        description={t('mine.delete.confirmDescription', {
          name: deleteTarget?.displayName ?? '',
        })}
        confirmLabel={t('mine.delete')}
        cancelLabel={t('dialog.cancel')}
        busy={deleting}
        onCancel={() => setDeleteTarget(null)}
        onConfirm={() => {
          void confirmDelete();
        }}
      />
    </section>
  );
}
