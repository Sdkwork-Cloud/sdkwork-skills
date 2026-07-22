#!/usr/bin/env node
import { resolveFamilySdkRoot, runSkillsSdkGenerator } from "../../../tools/skills_sdk_generator_runner.mjs";

runSkillsSdkGenerator(
  {
    apiAuthority: "sdkwork-skills-backend-api",
    apiPrefix: "/backend/v3/api",
    defaultBaseUrl: "http://127.0.0.1:18092",
    defaultOpenapiRelativePath: "backend-api/skills/skills-backend-api.openapi.json",
    sdkName: "sdkwork-skills-backend-sdk",
    sdkRoot: resolveFamilySdkRoot(import.meta.url),
    sdkType: "backend",
  },
  process.argv.slice(2),
);
