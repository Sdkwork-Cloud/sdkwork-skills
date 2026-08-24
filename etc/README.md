# SDKWork Skills Source Configuration

`sdkwork.deployment.config.json` is the source-controlled profile index for SDKWork Skills. It
selects one typed profile under `topology/`; the application topology contract is
[`../specs/topology.spec.json`](../specs/topology.spec.json).

## Profiles

The supported lifecycle environments are `development`, `test`, `staging`, and `production` for
both `standalone` and `cloud` deployment profiles.

- `standalone.*` owns one application ingress. App API and Backend API share that origin; the
  standalone gateway is the only local Skills HTTP process.
- `cloud.*` starts no local Skills or platform gateway process. It consumes an explicitly deployed
  Skills application origin and the separately owned platform API origin.
- Development uses loopback/private-network CORS policy. Production-like profiles declare one
  exact safe template origin; `.invalid` values must be replaced before deployment.

The platform cloud gateway, IAM, and Drive own their runtime configuration in their respective
repositories. This repository declares only the Skills endpoints that its PC bootstrap consumes.

## Materialization

The selected profile provides lifecycle environment, deployment profile, runtime target, gateway
placement, application ingress, platform ingress, CORS projection, and safe browser `VITE_*`
inputs. Process environment and CLI values are explicit runtime overrides, not a second checked-in
configuration authority.

Committed files contain no credentials. Local overrides use ignored `*.local.*` files; database
credentials, tokens, private keys, and other secrets come from protected process configuration or
mounted secret files.

Validate before development, packaging, or deployment:

```powershell
node ../sdkwork-specs/tools/check-source-config-standard.mjs --root .
pnpm topology:validate
```

<!-- SDKWORK-DEPLOY-LAYOUT: v1 -->
## Installed Runtime Paths

Authority: `APPLICATION_DEPLOY_LAYOUT_SPEC.md` (`../sdkwork-specs/`).

| Item | Value |
| --- | --- |
| `appId` | `sdkwork-skills` |
| `runtimeCode` | `skills` |
| Config root | `/etc/sdkwork/skills/` |
| Runtime TOML | `/etc/sdkwork/skills/config.toml` |
| Secrets | `/etc/sdkwork/skills/secrets/` |
| Override | `SDKWORK_SKILLS_CONFIG_FILE` |

Source profiles live under `etc/` (`sdkwork.deployment.config.json` index). Deploy manifest: `deployments/deploy.yaml`. Web data-plane source: `deployments/webserver/` (`SDKWORK_WEBSERVER_SPEC.md` layout v3).

```bash
node ../sdkwork-specs/tools/check-source-config-standard.mjs --root .
node ../sdkwork-specs/tools/check-application-deploy-layout.mjs --root .
node ../sdkwork-specs/tools/check-webserver-toml-standard.mjs --root deployments/webserver
```
<!-- /SDKWORK-DEPLOY-LAYOUT -->


