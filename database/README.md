# Skills Database

Owner: skills-platform

Skills Hub system-of-record uses intelligence-domain `ai_*` tables and shared `c_category`
for skill market classification.

## Tables

| Table | Purpose |
| --- | --- |
| `c_category` | Skill market and collection categories |
| `ai_agent_skill_package` | Skill package metadata and kernel runtime binding |
| `ai_agent_skill` | Skill marketplace entries |
| `ai_user_agent_skill` | Per-user install and enablement |
| `ai_skill_asset` | Icons, covers, media |
| `ai_skill_artifact` | Versioned release artifacts |
| `ai_skill_action` | Download, favorite, rating actions |

## Verification

```bash
pnpm db:validate
pnpm db:plan
```
