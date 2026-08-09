import { useEffect, useState, type FormEvent } from 'react';
import {
  createSkillCapability,
  listSkillCapabilities,
  updateSkillCapability,
  type SkillCapabilityRecord,
} from '@sdkwork/skills-pc-admin-core';
import { useSkillsClients } from '@sdkwork/skills-pc-core';

export function SkillCapabilitiesPage() {
  const clients = useSkillsClients();
  const [capabilities, setCapabilities] = useState<SkillCapabilityRecord[]>([]);
  const [error, setError] = useState<string | null>(null);
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
    <section>
      <h2>Skill Capabilities</h2>
      {error ? <p role="alert">{error}</p> : null}
      <form onSubmit={onSubmit} style={{ display: 'grid', gap: 8, maxWidth: 480, marginBottom: 24 }}>
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
        <button type="submit">Create Capability</button>
      </form>
      {capabilities.length === 0 ? (
        <p>No capability definitions yet.</p>
      ) : (
        <ul>
          {capabilities.map((item) => (
            <li key={item.id}>
              {item.displayName} ({item.capabilityKey}) - {item.riskLevel}
              <select
                value={item.riskLevel}
                onChange={(event) =>
                  onSetRiskLevel(
                    item,
                    event.target.value as 'standard' | 'sensitive' | 'privileged',
                  )
                }
                style={{ marginLeft: 8 }}
              >
                <option value="standard">standard</option>
                <option value="sensitive">sensitive</option>
                <option value="privileged">privileged</option>
              </select>
            </li>
          ))}
        </ul>
      )}
    </section>
  );
}
