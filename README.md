# SDKWork Skills
repository-kind: application

Skills Hub application root with PC client, Rust APIs, and `ai_*` intelligence-domain tables.

## Surfaces

| Surface | Path |
| --- | --- |
| Skills Hub | `apps/sdkwork-skills-pc` → `/skills-hub` |
| Console CRUD | `/console/skills` |
| Admin CRUD | `/admin/skills`, `/admin/categories` |

## Quick start

```bash
pnpm install
pnpm db:materialize:contract
pnpm api:check
cargo test --workspace
pnpm dev
```

## Documentation Canon

- [docs/README.md](docs/README.md)
- [docs/product/prd/PRD.md](docs/product/prd/PRD.md)
- [docs/architecture/tech/TECH_ARCHITECTURE.md](docs/architecture/tech/TECH_ARCHITECTURE.md)

## Verification

```bash
pnpm verify
node ../sdkwork-specs/tools/check-repository-docs-standard.mjs --root .
```

## Application Roots

- [apps directory index](apps/README.md)
