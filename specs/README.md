# Repository Component Specs

This directory holds the SDKWork Skills application component contract (`component.spec.json`)
and topology narrowing rules for the `sdkwork-skills` application root.

Governed by [COMPONENT_SPEC.md](../../sdkwork-specs/COMPONENT_SPEC.md).

## Owned Contract

`sdkwork-skills` is the sole write authority for the `intelligence / skills`
bounded context. The machine-readable ownership contract is
[domain-ownership.spec.json](./domain-ownership.spec.json).

| Contract | Authority |
| --- | --- |
| Database | Exactly 10 Skills-owned `ai_*` tables in `database/contract/table-registry.json` |
| App API | 8 operations in `apis/app-api/skills/skills-app-api.openapi.json` |
| Backend API | 16 operations in `apis/backend-api/skills/skills-backend-api.openapi.json` |
| Open API | None, 0 operations |

Skills owns packages, marketplace entries, immutable artifacts, capabilities,
installations, assets, and marketplace actions. Agent Project, Session, Turn,
Session Item, and Interaction belong to `sdkwork-agents`. Conversation,
Message, Member, and ReadCursor belong to `sdkwork-im`. Artifact bytes and
storage lifecycle belong to `sdkwork-drive`.

## Layout

| Path | Purpose |
| --- | --- |
| `component.spec.json` | Application component identity and capability boundaries |
| `domain-ownership.spec.json` | Skills persistence, API, dependency, and semantic ownership |
| `topology.spec.json` | Gateway / runtime topology contract (see also repo `specs/topology.spec.json`) |

PC surface component specs live under `apps/sdkwork-skills-pc/specs/`.
