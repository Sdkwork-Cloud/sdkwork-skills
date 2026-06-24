# MIG-2026-0010: Skills Domain Extraction From Kernel

Status: planned
Owner: skills-platform
Applications: sdkwork-skills, sdkwork-kernel

## Summary

Move agent skill marketplace persistence from `sdkwork-kernel/a_agent_skill_package` to
`sdkwork-skills/ai_agent_skill_package` and related `ai_*` tables.

## Steps

1. Bootstrap `sdkwork-skills` database baseline and APIs.
2. Run `0002_migrate_kernel_a_agent_skill_package.sql` against shared Postgres.
3. Point kernel consumers to `sdkwork-skills` HTTP/SDK contracts.
4. Remove kernel skill CRUD implementation and `a_agent_skill_package` DDL.
5. Verify `sdkwork-skills-pc` Hub/Console/Admin flows against new APIs.

## Rollback

Restore kernel DDL from git history and re-run inverse migration only in non-production
environments with explicit approval.
