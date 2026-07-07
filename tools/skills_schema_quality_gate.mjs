#!/usr/bin/env node
import { existsSync, readFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const HTTP_METHODS = new Set(["get", "post", "put", "patch", "delete"]);
const LIST_QUERY_PARAMS = new Set(["page", "page_size", "cursor", "q"]);
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

function readText(filePath) {
  if (!existsSync(filePath)) {
    fail(`missing file: ${filePath}`);
  }
  return readFileSync(filePath, "utf8");
}

function operations(document) {
  return Object.entries(document.paths ?? {}).flatMap(([pathKey, pathItem]) =>
    Object.entries(pathItem ?? {})
      .filter(([method]) => HTTP_METHODS.has(method))
      .map(([method, operation]) => ({ method, operation, pathKey })),
  );
}

function operationKey(method, pathKey, operationId) {
  return `${method.toUpperCase()} ${pathKey} ${operationId}`;
}

function parseRustRouteManifest(manifestPath) {
  const content = readText(manifestPath);
  const routes = [];
  const patterns = [
    /skills_(?:admin_abuse_|admin_)?route\(\s*HttpMethod::(\w+),\s*"([^"]+)",\s*"([^"]+)"/gu,
    /HttpRoute::dual_token\(\s*HttpMethod::(\w+),\s*"([^"]+)",\s*"[^"]+",\s*"([^"]+)"/gu,
  ];
  const seen = new Set();
  for (const pattern of patterns) {
    for (const match of content.matchAll(pattern)) {
      const route = {
        method: match[1].toLowerCase(),
        pathKey: match[2],
        operationId: match[3],
      };
      const key = operationKey(route.method, route.pathKey, route.operationId);
      if (seen.has(key)) {
        continue;
      }
      seen.add(key);
      routes.push(route);
    }
  }
  if (routes.length === 0) {
    fail(`${manifestPath} declares no HttpRoute entries`);
  }
  return routes;
}

function schemaRefName(schema) {
  if (!schema || typeof schema !== "object") {
    return null;
  }
  if (typeof schema.$ref === "string") {
    return schema.$ref.split("/").pop() ?? null;
  }
  if (Array.isArray(schema.allOf)) {
    for (const part of schema.allOf) {
      const ref = schemaRefName(part);
      if (ref) {
        return ref;
      }
    }
  }
  return null;
}

function usesSdkWorkEnvelope(schema, components) {
  const ref = schemaRefName(schema);
  if (!ref) {
    return false;
  }
  if (ref === "SdkWorkApiResponse" || ref.endsWith("Response")) {
    return true;
  }
  const component = components?.[ref];
  if (!component) {
    return false;
  }
  if (Array.isArray(component.allOf)) {
    return component.allOf.some(
      (part) => schemaRefName(part) === "SdkWorkApiResponse",
    );
  }
  return false;
}

function checkDocument(filePath, authority, prefix, manifestPath) {
  const document = readJson(filePath);
  const components = document.components?.schemas ?? {};
  const openapiVersion = String(document.openapi ?? "");
  if (!openapiVersion.startsWith("3.1.")) {
    fail(`${filePath} must use OpenAPI 3.1.x`);
  }
  if (document["x-sdkwork-standard-profile"] !== "sdkwork-v3") {
    fail(`${filePath} must declare x-sdkwork-standard-profile sdkwork-v3`);
  }
  if (document["x-sdkwork-owner"] && document["x-sdkwork-owner"] !== "sdkwork-skills") {
    fail(`${filePath} owner drift`);
  }
  if (document["x-sdkwork-api-authority"] && document["x-sdkwork-api-authority"] !== authority) {
    fail(`${filePath} authority drift`);
  }
  if (!components.ProblemDetail) {
    fail(`${filePath} must declare components.schemas.ProblemDetail`);
  }
  if (!components.SdkWorkApiResponse) {
    fail(`${filePath} must declare components.schemas.SdkWorkApiResponse`);
  }

  const openapiOps = new Map();
  for (const { method, operation, pathKey } of operations(document)) {
    if (!pathKey.startsWith(prefix) && !["/livez", "/readyz", "/healthz"].includes(pathKey)) {
      fail(`${filePath} has invalid path prefix ${pathKey}`);
    }
    if (!/^[a-z][A-Za-z0-9]*(\.[a-z][A-Za-z0-9]*)+$/u.test(operation.operationId ?? "")) {
      fail(`${filePath} invalid operationId ${operation.operationId}`);
    }
    if (!operation["x-sdkwork-api-surface"]) {
      fail(`${filePath} missing x-sdkwork-api-surface on ${operation.operationId}`);
    }

    const successSchema = operation.responses?.["200"]?.content?.["application/json"]?.schema;
    if (!successSchema) {
      fail(`${operation.operationId} must declare 200 application/json success schema`);
    }
    if (!usesSdkWorkEnvelope(successSchema, components)) {
      fail(`${operation.operationId} 200 must use SdkWorkApiResponse envelope`);
    }

    if (String(operation.operationId).endsWith(".list")) {
      const queryNames = new Set(
        (operation.parameters ?? [])
          .filter((parameter) => parameter.in === "query")
          .map((parameter) => parameter.name),
      );
      for (const required of LIST_QUERY_PARAMS) {
        if (!queryNames.has(required)) {
          fail(`${operation.operationId} must declare list query parameter ${required}`);
        }
      }
    }

    openapiOps.set(operationKey(method, pathKey, operation.operationId), true);
  }

  const manifestOps = parseRustRouteManifest(manifestPath);
  for (const route of manifestOps) {
    const key = operationKey(route.method, route.pathKey, route.operationId);
    if (!openapiOps.has(key)) {
      fail(`route manifest ${route.operationId} missing from ${filePath} (${route.method.toUpperCase()} ${route.pathKey})`);
    }
  }
  for (const key of openapiOps.keys()) {
    const manifestMatch = manifestOps.some(
      (route) => operationKey(route.method, route.pathKey, route.operationId) === key,
    );
    if (!manifestMatch) {
      fail(`OpenAPI operation not declared in ${manifestPath}: ${key}`);
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
const appManifest =
  parseArg("--app-manifest") ??
  path.join(workspaceRoot, "crates/sdkwork-routes-skills-app-api/src/http_route_manifest.rs");
const backendManifest =
  parseArg("--backend-manifest") ??
  path.join(workspaceRoot, "crates/sdkwork-routes-skills-backend-api/src/http_route_manifest.rs");

checkDocument(appPath, "sdkwork-skills.app", "/app/v3/api", appManifest);
checkDocument(backendPath, "sdkwork-skills.backend", "/backend/v3/api", backendManifest);
process.stdout.write("[skills_schema_quality_gate] ok\n");
