import { useEffect, useState } from 'react';
import { Link, useParams } from 'react-router-dom';
import { useSkillsClients, type SkillRecord } from '@sdkwork/skills-pc-core';

export function SkillsHubPage() {
  const clients = useSkillsClients();
  const [skills, setSkills] = useState<SkillRecord[]>([]);
  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    let active = true;
    clients.app.skills
      .list()
      .then((response: { items: SkillRecord[] }) => {
        if (active) {
          setSkills(response.items);
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
    clients.app.skills
      .retrieve(skillId)
      .then((response: { data: SkillRecord }) => {
        if (active) {
          setSkill(response.data);
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
    </section>
  );
}
