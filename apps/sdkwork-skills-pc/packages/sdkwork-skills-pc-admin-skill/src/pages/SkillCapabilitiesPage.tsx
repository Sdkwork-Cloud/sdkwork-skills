import { useEffect, useState, type FormEvent } from 'react';
import {
  createSkillCapability,
  listSkillCapabilities,
  updateSkillCapability,
  type SkillCapabilityRecord,
} from '@sdkwork/skills-pc-admin-core';
import { useSkillsClients } from '@sdkwork/skills-pc-core';
import { SurfaceDrawer } from '../components/SurfaceOverlay.tsx';

export function SkillCapabilitiesPage() {
  const clients = useSkillsClients();
  const [capabilities, setCapabilities] = useState<SkillCapabilityRecord[]>([]);
  const [error, setError] = useState<string | null>(null);
  const [createOpen, setCreateOpen] = useState(false);
  const [form, setForm] = useState({
    capabilityKey: 'capability.basic.run',
    displayName: 'Basic Run',
    description: 'Standard capability',
    riskLevel: 'standard' as 'standard' | 'sensitive' | 'privileged',
  });

  async function reload() {
    const page = await listSkillCapabilities(clients);
    setCapabilities(page.items);
  }

  useEffect(() => {
    reload().catch((cause: Error) => setError(cause.message));
  }, [clients]);

  async function onSubmit(event: FormEvent) {
    event.preventDefault();
    setError(null);
    try {
      await createSkillCapability(clients, form);
      setForm({
        capabilityKey: 'capability.basic.run',
        displayName: 'Basic Run',
        description: 'Standard capability',
        riskLevel: 'standard',
      });
      setCreateOpen(false);
      await reload();
    } catch (cause) {
      setError(cause instanceof Error ? cause.message : String(cause));
    }
  }

  async function onSetRiskLevel(
    item: SkillCapabilityRecord,
    riskLevel: 'standard' | 'sensitive' | 'privileged',
  ) {
    setError(null);
    try {
      await updateSkillCapability(clients, item.id, {
        version: item.version,
        riskLevel,
      });
      await reload();
    } catch (cause) {
      setError(cause instanceof Error ? cause.message : String(cause));
    }
  }

  return (
    <section className="skills-console-page">
      <header className="skills-console-header" style={{ marginBottom: 0 }}>
        <h2>Skill Capabilities</h2>
        <button type="button" className="skills-console-primary" onClick={() => setCreateOpen(true)}>
          Create capability
        </button>
      </header>
      {error ? <p role="alert">{error}</p> : null}
      <div className="data-surface">
        <div className="table-frame">
          {capabilities.length === 0 ? (
            <div className="empty-state">
              <span>No capability definitions yet.</span>
              <button type="button" className="skills-console-primary" onClick={() => setCreateOpen(true)}>
                Create capability
              </button>
            </div>
          ) : (
            <table>
              <thead>
                <tr>
                  <th>Name</th>
                  <th>Key</th>
                  <th>Risk</th>
                </tr>
              </thead>
              <tbody>
                {capabilities.map((item) => (
                  <tr key={item.id}>
                    <td>{item.displayName}</td>
                    <td>{item.capabilityKey}</td>
                    <td>
                      <select
                        value={item.riskLevel}
                        onChange={(event) =>
                          onSetRiskLevel(
                            item,
                            event.target.value as 'standard' | 'sensitive' | 'privileged',
                          )
                        }
                      >
                        <option value="standard">standard</option>
                        <option value="sensitive">sensitive</option>
                        <option value="privileged">privileged</option>
                      </select>
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
        title="Create capability"
        onClose={() => setCreateOpen(false)}
      >
        <form onSubmit={onSubmit} style={{ display: 'grid', gap: 8 }}>
          <input
            value={form.capabilityKey}
            onChange={(event) => setForm({ ...form, capabilityKey: event.target.value })}
            placeholder="capability key"
            required
          />
          <input
            value={form.displayName}
            onChange={(event) => setForm({ ...form, displayName: event.target.value })}
            placeholder="display name"
            required
          />
          <select
            value={form.riskLevel}
            onChange={(event) =>
              setForm({ ...form, riskLevel: event.target.value as typeof form.riskLevel })
            }
          >
            <option value="standard">standard</option>
            <option value="sensitive">sensitive</option>
            <option value="privileged">privileged</option>
          </select>
          <div className="sdkwork-surface-drawer-form-actions">
            <button type="button" onClick={() => setCreateOpen(false)}>
              Cancel
            </button>
            <button type="submit">Create Capability</button>
          </div>
        </form>
      </SurfaceDrawer>
    </section>
  );
}
