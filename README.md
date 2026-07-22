# SDKWork Skills

repository-kind: application

Single-owner Skills application root with a PC client, Rust APIs, generated SDK
families, and a portable ten-table `ai_*` persistence module.

## Surfaces

| Surface | Path |
| --- | --- |
| Skills Hub | `apps/sdkwork-skills-pc` at `/skills-hub` |
| Tenant console | `/console/skills` |
| Admin CRUD | `/admin/skills`, `/admin/categories` |

The HTTP product exposes app-api and backend-api only. No public open-api is
declared until a separate public-product requirement is approved.

## Quick Start

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
