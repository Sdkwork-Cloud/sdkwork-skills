# SDKWork Skills SQLx Repository Specs

`component.spec.json` defines the integration contract for the Skills-owned SQLx repository.
The repository consumes the process-shared SDKWork database pool and Snowflake ID generator, and
must provide equivalent PostgreSQL and SQLite behavior without projections, shadow persistence, or
in-process pagination. Installation persistence stores `package_id` and the selected immutable
`artifact_id`; the response-only `skill_id` is derived by joining the package's canonical
marketplace entry. Concurrent installs use the database uniqueness boundary and atomic conflict
handling, so duplicate rows and duplicate install-count side effects are impossible.

Global requirements remain authoritative through the relative `canonicalSpecs` links in the
component manifest.
