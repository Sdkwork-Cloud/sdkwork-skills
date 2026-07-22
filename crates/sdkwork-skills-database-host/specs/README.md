# SDKWork Skills Database Host Specs

`component.spec.json` defines the runtime database-host contract. The host owns Skills lifecycle
bootstrap and exposes the process-shared `DatabasePool`, Snowflake generator, and live node lease to
the API assembly. PostgreSQL and SQLite are both supported through the same host boundary.
