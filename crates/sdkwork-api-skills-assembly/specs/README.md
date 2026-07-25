# sdkwork-api-skills-assembly Specs

Component root: `crates/sdkwork-api-skills-assembly`

API assembly manifest, business-router composition, and verification contract.

The assembly exports one host-neutral unit containing the raw App/Backend business router, the
combined `HttpRouteManifest`, OpenAPI contribution, permission catalog, Skills domain-context
injectors, and database readiness contribution. It does not bind a listener, mount infrastructure
probes, or install a `WebFrameworkLayer`; those process-wide responsibilities belong to the
standalone or cloud host.

`assemble_app_api_contribution()` is fail-closed for Agents `project` and `agent` installation
targets. A composing gateway enables those targets only through
`assemble_app_api_contribution_with_target_authorizer(...)`, using an Agents-owned implementation
of the exported `SkillInstallationTargetAuthorizer` port. Skills never imports Agents. IAM `user`
and `organization` targets are validated directly against the verified request context.
