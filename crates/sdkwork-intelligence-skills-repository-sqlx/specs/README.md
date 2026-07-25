# SDKWork Skills SQLx Repository Specs

`component.spec.json` defines the integration contract for the Skills-owned SQLx repository.
The repository consumes the injected process-shared PostgreSQL `PgPool` and Snowflake ID generator.
It operates without projections, shadow persistence, or in-process pagination. Installation
persistence stores `package_id` and the selected immutable
`artifact_id`; the response-only `skill_id` is derived by joining the package's canonical
marketplace entry. Concurrent installs use the database uniqueness boundary and atomic conflict
handling, so duplicate rows and duplicate install-count side effects are impossible.

Global requirements remain authoritative through the relative `canonicalSpecs` links in the
component manifest.

## PostgreSQL Contract Test

Set `SDKWORK_SKILLS_POSTGRES_URL` to a disposable PostgreSQL database before running
`cargo test -p sdkwork-intelligence-skills-repository-sqlx`. The contract test creates a unique
schema, verifies marketplace, isolation, concurrency, lifecycle, and integrity behavior, then drops
that schema. Without the variable, local unit tests still run and the external database contract is
reported as skipped.
