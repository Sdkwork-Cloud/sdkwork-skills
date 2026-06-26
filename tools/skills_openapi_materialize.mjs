#!/usr/bin/env node
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const workspaceRoot = path.resolve(scriptDir, "..");
const OWNER = "sdkwork-skills";
const DOMAIN = "skills";
const TAG = "skills";

const schemas = {
  ProblemDetail: {
    type: "object",
    additionalProperties: true,
    required: ["type", "title", "status"],
    properties: {
      type: { type: "string", format: "uri-reference" },
      title: { type: "string" },
      status: { type: "integer", minimum: 100, maximum: 599 },
      detail: { type: "string" },
      requestId: { type: "string", format: "uuid" },
    },
  },
  SkillRecord: {
    type: "object",
    additionalProperties: true,
    required: ["id", "skill_key", "name", "market_status", "visibility", "enabled", "featured", "install_count"],
    properties: {
      id: { type: "integer", format: "int64" },
      skill_key: { type: "string" },
      name: { type: "string" },
      summary: { type: "string", nullable: true },
      description: { type: "string", nullable: true },
      runtime: { type: "string", nullable: true },
      entrypoint: { type: "string", nullable: true },
      market_status: { type: "string" },
      visibility: { type: "string" },
      enabled: { type: "boolean" },
      featured: { type: "boolean" },
      install_count: { type: "integer", format: "int64" },
      tags: { type: "array", items: { type: "string" } },
      capabilities: { type: "array", items: { type: "string" } },
      categories: { type: "array", items: { type: "string" } },
    },
  },
  SkillPackageRecord: {
    type: "object",
    additionalProperties: true,
    required: ["id", "skill_id", "code", "display_name", "invocation_kind", "package_ref", "entrypoint", "status", "visibility"],
    properties: {
      id: { type: "integer", format: "int64" },
      skill_id: { type: "string" },
      code: { type: "string" },
      display_name: { type: "string" },
      summary: { type: "string", nullable: true },
      invocation_kind: { type: "string" },
      package_ref: {
        type: "string",
        description: "Canonical sdkwork-drive package reference.",
        pattern: "^drive://spaces/[^/]+/nodes/[^/]+$",
      },
      entrypoint: { type: "string" },
      status: { type: "string" },
      visibility: { type: "string" },
      categories: { type: "array", items: { type: "string" } },
      tags: { type: "array", items: { type: "string" } },
    },
  },
  SkillCategoryRecord: {
    type: "object",
    additionalProperties: true,
    required: ["id", "code", "name", "category_type", "permission_code", "sort_weight", "visible", "status"],
    properties: {
      id: { type: "integer", format: "int64" },
      code: { type: "string" },
      name: { type: "string" },
      category_type: { type: "string" },
      description: { type: "string", nullable: true },
      parent_id: { type: "integer", format: "int64", nullable: true },
      permission_code: { type: "string" },
      sort_weight: { type: "integer" },
      visible: { type: "boolean" },
      status: { type: "integer" },
    },
  },
  UserSkillInstallRecord: {
    type: "object",
    additionalProperties: true,
    required: ["id", "skill_id", "install_status", "enabled"],
    properties: {
      id: { type: "integer", format: "int64" },
      skill_id: { type: "integer", format: "int64" },
      install_status: { type: "string" },
      enabled: { type: "boolean" },
    },
  },
  SkillListResponse: {
    type: "object",
    additionalProperties: false,
    required: ["items"],
    properties: {
      items: { type: "array", items: { $ref: "#/components/schemas/SkillRecord" } },
    },
  },
  SkillPackageListResponse: {
    type: "object",
    additionalProperties: false,
    required: ["items"],
    properties: {
      items: { type: "array", items: { $ref: "#/components/schemas/SkillPackageRecord" } },
    },
  },
  SkillCategoryListResponse: {
    type: "object",
    additionalProperties: false,
    required: ["items"],
    properties: {
      items: { type: "array", items: { $ref: "#/components/schemas/SkillCategoryRecord" } },
    },
  },
  SkillRecordResponse: {
    type: "object",
    additionalProperties: false,
    required: ["data"],
    properties: {
      data: { $ref: "#/components/schemas/SkillRecord" },
    },
  },
  SkillPackageRecordResponse: {
    type: "object",
    additionalProperties: false,
    required: ["data"],
    properties: {
      data: { $ref: "#/components/schemas/SkillPackageRecord" },
    },
  },
  SkillCategoryRecordResponse: {
    type: "object",
    additionalProperties: false,
    required: ["data"],
    properties: {
      data: { $ref: "#/components/schemas/SkillCategoryRecord" },
    },
  },
  UserSkillInstallRecordResponse: {
    type: "object",
    additionalProperties: false,
    required: ["data"],
    properties: {
      data: { $ref: "#/components/schemas/UserSkillInstallRecord" },
    },
  },
  CreateSkillPackageCommand: {
    type: "object",
    additionalProperties: true,
    required: ["skill_id", "code", "display_name", "invocation_kind", "package_ref", "entrypoint"],
    properties: {
      skill_id: { type: "string" },
      package_key: { type: "string" },
      code: { type: "string" },
      display_name: { type: "string" },
      summary: { type: "string" },
      invocation_kind: { type: "string" },
      package_ref: { type: "string", pattern: "^drive://spaces/[^/]+/nodes/[^/]+$" },
      entrypoint: { type: "string" },
      capability_ids: { type: "array", items: { type: "string" } },
      categories: { type: "array", items: { type: "string" } },
      tags: { type: "array", items: { type: "string" } },
    },
  },
  CreateSkillCategoryCommand: {
    type: "object",
    additionalProperties: true,
    required: ["code", "name"],
    properties: {
      code: { type: "string" },
      name: { type: "string" },
      description: { type: "string" },
      sort_weight: { type: "integer" },
      permission_code: { type: "string" },
    },
  },
  InstallSkillCommand: {
    type: "object",
    additionalProperties: true,
    required: ["skill_id"],
    properties: {
      skill_id: { type: "integer", format: "int64" },
      package_id: { type: "integer", format: "int64" },
    },
  },
};

const appRoutes = [
  route("get", "/app/v3/api/skills", "skills.list", ref("SkillListResponse")),
  route("get", "/app/v3/api/skills/{skillKey}", "skills.retrieve", ref("SkillRecordResponse"), [pathParam("skillKey")]),
  route("get", "/app/v3/api/skill_packages", "skillPackages.list", ref("SkillPackageListResponse")),
  route("get", "/app/v3/api/skill_packages/{skillId}", "skillPackages.retrieve", ref("SkillPackageRecordResponse"), [pathParam("skillId")]),
  route("get", "/app/v3/api/categories", "categories.list", ref("SkillCategoryListResponse")),
  route("post", "/app/v3/api/user/skills/install", "userSkills.install", ref("UserSkillInstallRecordResponse"), [], "InstallSkillCommand"),
];

const backendRoutes = [
  route("get", "/backend/v3/api/skill", "skills.management.list", ref("SkillListResponse")),
  route("get", "/backend/v3/api/skill/package", "skillPackages.management.list", ref("SkillPackageListResponse")),
  route("post", "/backend/v3/api/skill/package", "skillPackages.create", ref("SkillPackageRecordResponse"), [], "CreateSkillPackageCommand"),
  route("put", "/backend/v3/api/skill/package/{skillId}", "skillPackages.update", ref("SkillPackageRecordResponse"), [pathParam("skillId")], "CreateSkillPackageCommand"),
  route("delete", "/backend/v3/api/skill/package/{skillId}", "skillPackages.delete", ref("SkillPackageRecordResponse"), [pathParam("skillId")]),
  route("get", "/backend/v3/api/category", "categories.management.list", ref("SkillCategoryListResponse")),
  route("post", "/backend/v3/api/category", "categories.create", ref("SkillCategoryRecordResponse"), [], "CreateSkillCategoryCommand"),
  route("put", "/backend/v3/api/category/{categoryId}", "categories.update", ref("SkillCategoryRecordResponse"), [pathParamInt("categoryId")], "CreateSkillCategoryCommand"),
];

function ref(name) {
  return { $ref: `#/components/schemas/${name}` };
}

function pathParam(name) {
  return { name, in: "path", required: true, schema: { type: "string" } };
}

function pathParamInt(name) {
  return { name, in: "path", required: true, schema: { type: "integer", format: "int64" } };
}

function problemResponse() {
  return {
    description: "Problem detail",
    content: {
      "application/problem+json": {
        schema: ref("ProblemDetail"),
      },
    },
  };
}

function route(method, pathKey, operationId, responseSchema, parameters = [], bodySchemaName = null) {
  return {
    method,
    path: pathKey,
    operation: {
      tags: [TAG],
      summary: `Skills ${operationId}`,
      operationId,
      parameters,
      ...(bodySchemaName
        ? {
            requestBody: {
              required: true,
              content: {
                "application/json": {
                  schema: ref(bodySchemaName),
                },
              },
            },
          }
        : {}),
      responses: {
        200: {
          description: "OK",
          content: {
            "application/json": {
              schema: responseSchema,
            },
          },
        },
        400: problemResponse(),
        401: problemResponse(),
        404: problemResponse(),
      },
      security: [{ AuthToken: [], AccessToken: [] }],
      "x-sdkwork-owner": OWNER,
      "x-sdkwork-api-authority": "",
      "x-sdkwork-domain": DOMAIN,
      "x-sdkwork-resource": operationId.split(".")[0],
      "x-sdkwork-public": false,
      "x-sdkwork-api-surface": "",
      "x-sdkwork-request-context": "WebRequestContext",
    },
  };
}

function documentFor({ authority, routes, serverUrl, title, surface }) {
  const paths = {};
  for (const item of routes) {
    paths[item.path] ??= {};
    item.operation["x-sdkwork-api-authority"] = authority;
    item.operation["x-sdkwork-api-surface"] = surface;
    paths[item.path][item.method] = item.operation;
  }
  return {
    openapi: "3.1.2",
    info: {
      title,
      version: "0.1.0",
      "x-sdkwork-owner": OWNER,
      "x-sdkwork-api-authority": authority,
    },
    servers: [{ url: serverUrl }],
    tags: [{ name: TAG, description: "Skills API resources.", "x-sdk-nested-resource-surface": true }],
    paths,
    components: {
      securitySchemes: {
        AuthToken: { type: "http", scheme: "bearer", bearerFormat: "JWT" },
        AccessToken: { type: "apiKey", in: "header", name: "Access-Token" },
      },
      schemas,
    },
    "x-sdkwork-owner": OWNER,
    "x-sdkwork-api-authority": authority,
    "x-sdkwork-domain": DOMAIN,
    "x-sdkwork-standard-profile": "sdkwork-v3",
  };
}

const checkOnly = process.argv.includes("--check");
const outputs = [
  {
    file: path.join(workspaceRoot, "apis/app-api/skills/skills-app-api.openapi.json"),
    document: documentFor({
      authority: "sdkwork-skills.app",
      routes: appRoutes,
      serverUrl: "http://127.0.0.1:18090",
      title: "SDKWork Skills App API",
      surface: "app-api",
    }),
  },
  {
    file: path.join(workspaceRoot, "apis/backend-api/skills/skills-backend-api.openapi.json"),
    document: documentFor({
      authority: "sdkwork-skills.backend",
      routes: backendRoutes,
      serverUrl: "http://127.0.0.1:18091",
      title: "SDKWork Skills Backend API",
      surface: "backend-api",
    }),
  },
];

if (!checkOnly) {
  for (const { file, document } of outputs) {
    mkdirSync(path.dirname(file), { recursive: true });
    writeFileSync(file, `${JSON.stringify(document, null, 2)}\n`, "utf8");
  }
} else {
  for (const { file } of outputs) {
    try {
      readFileSync(file, "utf8");
    } catch {
      console.error(`missing openapi: ${file}`);
      process.exit(1);
    }
  }
}

process.stdout.write(`[skills_openapi_materialize] ok app=${appRoutes.length} backend=${backendRoutes.length}\n`);
