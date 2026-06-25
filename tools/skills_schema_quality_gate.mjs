#!/usr/bin/env node
import { existsSync, readFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const HTTP_METHODS = new Set(["get", "post", "put", "patch", "delete"]);
const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const workspaceRoot = path.resolve(scriptDir, "..");

function fail(message) {
  process.stderr.write(`[skills_schema_quality_gate] ${message}\n`);
  process.exit(1);
}

function readJson(filePath) {
  if (!existsSync(filePath)) {
    fail(`missing OpenAPI file: ${filePath}`);
  }
  return JSON.parse(readFileSync(filePath, "utf8"));
}

function operations(document) {
  return Object.entries(document.paths ?? {}).flatMap(([pathKey, pathItem]) =>
    Object.entries(pathItem ?? {})
      .filter(([method]) => HTTP_METHODS.has(method))
      .map(([method, operation]) => ({ method, operation, pathKey })),
  );
}

function checkDocument(filePath, authority, prefix) {
  const document = readJson(filePath);
  const openapiVersion = String(document.openapi ?? "");
  if (!openapiVersion.startsWith("3.1.")) {
    fail(`${filePath} must use OpenAPI 3.1.x`);
  }
  if (document["x-sdkwork-owner"] && document["x-sdkwork-owner"] !== "sdkwork-skills") {
    fail(`${filePath} owner drift`);
  }
  if (document["x-sdkwork-api-authority"] && document["x-sdkwork-api-authority"] !== authority) {
    fail(`${filePath} authority drift`);
  }
  for (const { operation, pathKey } of operations(document)) {
    if (!pathKey.startsWith(prefix) && !["/livez", "/readyz", "/healthz"].includes(pathKey)) {
      fail(`${filePath} has invalid path prefix ${pathKey}`);
    }
    if (!/^[a-z][A-Za-z0-9]*(\.[a-z][A-Za-z0-9]*)+$/u.test(operation.operationId ?? "")) {
      fail(`${filePath} invalid operationId ${operation.operationId}`);
    }
    if (!operation["x-sdkwork-api-surface"]) {
      fail(`${filePath} missing x-sdkwork-api-surface on ${operation.operationId}`);
    }
  }
}

function parseArg(name) {
  const index = process.argv.indexOf(name);
  if (index === -1) {
    return null;
  }
  return process.argv[index + 1] ?? null;
}

const appPath = parseArg("--app-openapi") ?? path.join(workspaceRoot, "apis/app-api/skills/skills-app-api.openapi.json");
const backendPath =
  parseArg("--backend-openapi") ?? path.join(workspaceRoot, "apis/backend-api/skills/skills-backend-api.openapi.json");

checkDocument(appPath, "sdkwork-skills.app", "/app/v3/api");
checkDocument(backendPath, "sdkwork-skills.backend", "/backend/v3/api");
process.stdout.write("[skills_schema_quality_gate] ok\n");
