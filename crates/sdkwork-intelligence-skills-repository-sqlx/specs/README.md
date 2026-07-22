# SDKWork Skills SQLx Repository Specs

`component.spec.json` defines the integration contract for the Skills-owned SQLx repository.
The repository consumes the process-shared SDKWork database pool and Snowflake ID generator, and
must provide equivalent PostgreSQL and SQLite behavior without projections, shadow persistence, or
in-process pagination.

Global requirements remain authoritative through the relative `canonicalSpecs` links in the
component manifest.
