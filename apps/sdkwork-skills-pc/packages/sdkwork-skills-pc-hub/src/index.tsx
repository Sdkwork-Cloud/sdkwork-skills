import { useEffect, useState } from 'react';
import { Link, useParams } from 'react-router-dom';
import {
  installUserSkill,
  listPublishedSkills,
  retrievePublishedSkill,
  useSkillsClients,
  type SkillRecord,
} from '@sdkwork/skills-pc-core';

export function SkillsHubPage() {
  const clients = useSkillsClients();
  const [skills, setSkills] = useState<SkillRecord[]>([]);
  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    let active = true;
    listPublishedSkills(clients)
      .then((page) => {
        if (active) {
          setSkills(page.items);
          setError(null);
        }
      })
      .catch((cause: Error) => {
        if (active) {
          setError(cause.message);
        }
      })
      .finally(() => {
        if (active) {
          setLoading(false);
        }
      });
    return () => {
      active = false;
    };
  }, [clients]);

  if (loading) {
    return <p>Loading skills...</p>;
  }

  if (error) {
    return <p role="alert">Failed to load skills: {error}</p>;
  }

  return (
    <section>
      <h2>Skills Hub</h2>
      <ul>
        {skills.map((skill) => (
          <li key={skill.id}>
            <Link to={`/skills-hub/${encodeURIComponent(skill.skill_key)}`}>{skill.name}</Link>
            {skill.summary ? <span> — {skill.summary}</span> : null}
          </li>
        ))}
      </ul>
      {skills.length === 0 ? <p>No skills published yet.</p> : null}
    </section>
  );
}

export function SkillDetailPage() {
  const clients = useSkillsClients();
  const { skillId = '' } = useParams();
  const [skill, setSkill] = useState<SkillRecord | null>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    if (!skillId) {
      return;
    }
    let active = true;
    retrievePublishedSkill(clients, skillId)
      .then((nextSkill) => {
        if (active) {
          setSkill(nextSkill);
          setError(null);
        }
      })
      .catch((cause: Error) => {
        if (active) {
          setError(cause.message);
        }
      });
    return () => {
      active = false;
    };
  }, [clients, skillId]);

  if (error) {
    return <p role="alert">Failed to load skill: {error}</p>;
  }

  if (!skill) {
    return <p>Loading skill...</p>;
  }

  return (
    <section>
      <h2>{skill.name}</h2>
      <p>
        <strong>Key:</strong> {skill.skill_key}
      </p>
      {skill.summary ? <p>{skill.summary}</p> : null}
      {skill.description ? <p>{skill.description}</p> : null}
      <p>
        <strong>Runtime:</strong> {skill.runtime ?? 'n/a'}
      </p>
      <p>
        <strong>Capabilities:</strong> {skill.capabilities?.join(', ') || 'none'}
      </p>
      <InstallSkillButton clients={clients} skillId={skill.id} />
    </section>
  );
}

function InstallSkillButton({
  clients,
  skillId,
}: {
  clients: ReturnType<typeof useSkillsClients>;
  skillId: string;
}) {
  const [status, setStatus] = useState<'idle' | 'installing' | 'done' | 'error'>('idle');
  const [message, setMessage] = useState<string | null>(null);

  async function onInstall() {
    setStatus('installing');
    setMessage(null);
    try {
      await installUserSkill(clients, skillId);
      setStatus('done');
      setMessage('Skill installed.');
    } catch (cause) {
      setStatus('error');
      setMessage(cause instanceof Error ? cause.message : String(cause));
    }
  }

  return (
    <div>
      <button type="button" onClick={onInstall} disabled={status === 'installing' || status === 'done'}>
        {status === 'done' ? 'Installed' : status === 'installing' ? 'Installing…' : 'Install Skill'}
      </button>
      {message ? <p role="status">{message}</p> : null}
    </div>
  );
}
