# SDKWork Skills Database Host Specs

`component.spec.json` defines the runtime database-host contract. The host owns Skills lifecycle
bootstrap and exposes the process-shared `DatabasePool`, Snowflake generator, and live node lease to
the API assembly. The host requires `DatabasePool::Postgres`, exposes its shared `PgPool` to the
repository, and fails closed before lifecycle work when any other engine is configured.
