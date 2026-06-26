# Skills Database

Owner: skills-platform

Skills Hub system-of-record uses intelligence-domain `ai_*` tables. Skill market
classification is owned by `ai_skill_category`; packages and skills bind categories
through `categories_json` category codes.

## Tables

| Table | Purpose |
| --- | --- |
| `ai_skill_category` | Skill market and collection taxonomy with IAM permission binding |
| `ai_agent_skill_package` | Skill package metadata and kernel runtime binding |
| `ai_agent_skill` | Skill marketplace entries |
| `ai_user_agent_skill` | Per-user install and enablement |
| `ai_skill_asset` | Icons, covers, media |
| `ai_skill_artifact` | Versioned release artifacts |
| `ai_skill_action` | Download, favorite, rating actions |

## Category Permissions

Each `ai_skill_category.permission_code` defines the IAM scope required to manage
packages in that category. Default pattern: `skills.admin.package.manage.<category_code>`.

Global admin scopes:

- `skills.admin.category.manage` — manage category taxonomy
- `skills.admin.package.manage` — manage packages across all categories
- `skills.admin.marketplace.read` — read marketplace admin surfaces

## Verification

```bash
pnpm db:validate
pnpm db:plan
```
