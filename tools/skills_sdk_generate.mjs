#!/usr/bin/env node
import { spawnSync } from "node:child_process";
import path from "node:path";
import { fileURLToPath } from "node:url";

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const workspaceRoot = path.resolve(scriptDir, "..");
const appPath = path.join(workspaceRoot, "apis/app-api/skills/skills-app-api.openapi.json");
const backendPath = path.join(workspaceRoot, "apis/backend-api/skills/skills-backend-api.openapi.json");

function run(script, args) {
  const result = spawnSync("node", [path.join(workspaceRoot, script), ...args], {
    cwd: workspaceRoot,
    stdio: "inherit",
  });
  if (result.status !== 0) {
    process.exit(result.status ?? 1);
  }
}

const check = process.argv.includes("--check");
run("tools/skills_openapi_materialize.mjs", check ? ["--check"] : []);
run("tools/skills_schema_quality_gate.mjs", [
  "--app-openapi",
  appPath,
  "--backend-openapi",
  backendPath,
]);

if (!check) {
  run("sdks/sdkwork-skills-app-sdk/bin/generate-sdk.mjs", ["--input", appPath]);
  run("sdks/sdkwork-skills-backend-sdk/bin/generate-sdk.mjs", ["--input", backendPath]);
}

process.stdout.write(`[skills_sdk_generate] ${check ? "check passed" : "generation completed"}\n`);
