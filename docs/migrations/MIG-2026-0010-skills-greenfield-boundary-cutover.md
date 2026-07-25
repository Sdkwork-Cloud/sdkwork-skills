# MIG-2026-0010: Skills Greenfield Boundary Cutover

Status: completed
Owner: skills-platform
Application: sdkwork-skills

## Summary

The pre-launch schema and API boundary were replaced in place with the
canonical Skills owner model. No production data migration, dual write,
projection table, or compatibility period exists.

## Completed Cutover

1. Materialized the authoritative PostgreSQL ten-table baseline and database
   contract, with no server fallback or second Skills store.
2. Replaced user-only installation state with subject-scoped installation of an
   explicit immutable artifact.
3. Replaced category and capability authority arrays with normalized binding
   tables.
4. Materialized app-api and backend-api authorities and regenerated their SDK
   families.
5. Removed consumer-owned Skills routes, DTOs, storage bindings, and local
   schemas; consumers now use the canonical SDK family.
6. Verified SDK regeneration idempotence, PostgreSQL contract alignment,
   route/OpenAPI parity, pagination, response envelopes, and authorization tests.
7. Removed the redundant installation `skill_id` column, retained `skillId` as
   a canonical package join in API output, and added atomic subject/package
   installation conflict handling.
8. Added database checks for artifact lifecycle timestamps, package/artifact
   integrity, and exactly-one asset ownership.

## Rollback Policy

There is no legacy schema to restore. Before GA, defects are corrected directly
in the baseline and regenerated contracts. After GA, all schema evolution must
use forward migrations governed by `DATABASE_SPEC.md`.
