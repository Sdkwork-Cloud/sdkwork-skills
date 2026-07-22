#!/usr/bin/env node
import { resolveFamilySdkRoot, runSkillsSdkGenerator } from "../../../tools/skills_sdk_generator_runner.mjs";

runSkillsSdkGenerator(
  {
    apiAuthority: "sdkwork-skills-app-api",
    apiPrefix: "/app/v3/api",
    defaultBaseUrl: "http://127.0.0.1:18092",
    defaultOpenapiRelativePath: "app-api/skills/skills-app-api.openapi.json",
    sdkName: "sdkwork-skills-app-sdk",
    sdkRoot: resolveFamilySdkRoot(import.meta.url),
    sdkType: "app",
  },
  process.argv.slice(2),
);
