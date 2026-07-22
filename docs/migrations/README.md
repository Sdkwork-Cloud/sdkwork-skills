# Skills Migration Records

Migration records describe boundary or schema cutovers that materially affect
the Skills owner contract. The current pre-launch cutover is documented in
[MIG-2026-0010-skills-greenfield-boundary-cutover.md](MIG-2026-0010-skills-greenfield-boundary-cutover.md).

Database evolution after GA must use forward migrations governed by
`DATABASE_SPEC.md`; compatibility schemas and double writes are not migration
mechanisms.
