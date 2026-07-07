#!/usr/bin/env node
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import {
  sdkWorkEnvelopeComponentSchemas,
} from "../../sdkwork-specs/tools/lib/openapi-envelope-schemas.mjs";

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const workspaceRoot = path.resolve(scriptDir, "..");
const OWNER = "sdkwork-skills";
const DOMAIN = "skills";
const TAG = "skills";

const domainSchemas = {
  SkillRecord: {
    type: "object",
    additionalProperties: true,
    required: ["id", "skill_key", "name", "market_status", "visibility", "enabled", "featured", "install_count"],
    properties: {
      id: int64StringProperty(),
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
      install_count: int64StringProperty(),
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
      id: int64StringProperty(),
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
      id: int64StringProperty(),
      code: { type: "string" },
      name: { type: "string" },
      category_type: { type: "string" },
      description: { type: "string", nullable: true },
      parent_id: { ...int64StringProperty(), nullable: true },
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
      id: int64StringProperty(),
      skill_id: int64StringProperty(),
      install_status: { type: "string" },
      enabled: { type: "boolean" },
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
      skill_id: int64StringProperty(),
      package_id: int64StringProperty(),
    },
  },
  UpdateSkillPackageCommand: {
    type: "object",
    additionalProperties: true,
    properties: {
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
      visibility: { type: "string" },
    },
  },
  UpdateSkillCategoryCommand: {
    type: "object",
    additionalProperties: true,
    properties: {
      name: { type: "string" },
      description: { type: "string" },
      sort_weight: { type: "integer" },
      permission_code: { type: "string" },
    },
  },
};

function int64StringProperty() {
  return {
    type: "string",
    format: "int64",
    pattern: "^-?[0-9]+$",
    "x-sdkwork-int64-string": true,
    "x-sdkwork-rust-type": "i64",
  };
}

function listQueryParameters() {
  return [
    {
      name: "page",
      in: "query",
      required: false,
      description: "One-based page index for offset pagination. Default 1.",
      schema: { type: "integer", format: "int32", minimum: 1, default: 1 },
    },
    {
      name: "page_size",
      in: "query",
      required: false,
      description: "Page size for offset pagination. Default 20, maximum 200.",
      schema: { type: "integer", format: "int32", minimum: 1, maximum: 200, default: 20 },
    },
    {
      name: "cursor",
      in: "query",
      required: false,
      description: "Opaque cursor token from a previous list response pageInfo.nextCursor.",
      schema: { type: "string", minLength: 1, maxLength: 512 },
    },
    {
      name: "q",
      in: "query",
      required: false,
      description: "Generic free-text search keyword.",
      schema: { type: "string", maxLength: 256 },
    },
  ];
}

function registerNamedListSchemas(prefix, itemSchemaName) {
  const pageSchemaName = `${prefix}PageData`;
  const responseSchemaName = `${prefix}ListResponse`;
  return {
    [pageSchemaName]: {
      type: "object",
      additionalProperties: false,
      required: ["items", "pageInfo"],
      properties: {
        items: { type: "array", items: { $ref: `#/components/schemas/${itemSchemaName}` } },
        pageInfo: { $ref: "#/components/schemas/PageInfo" },
      },
    },
    [responseSchemaName]: {
      allOf: [
        { $ref: "#/components/schemas/SdkWorkApiResponse" },
        {
          type: "object",
          required: ["data"],
          properties: {
            data: { $ref: `#/components/schemas/${pageSchemaName}` },
          },
        },
      ],
    },
  };
}

function registerNamedResourceSchemas(prefix, itemSchemaName) {
  const dataSchemaName = `${prefix}ResourceData`;
  const responseSchemaName = `${prefix}Response`;
  return {
    [dataSchemaName]: {
      type: "object",
      additionalProperties: false,
      required: ["item"],
      properties: {
        item: { $ref: `#/components/schemas/${itemSchemaName}` },
      },
    },
    [responseSchemaName]: {
      allOf: [
        { $ref: "#/components/schemas/SdkWorkApiResponse" },
        {
          type: "object",
          required: ["data"],
          properties: {
            data: { $ref: `#/components/schemas/${dataSchemaName}` },
          },
        },
      ],
    },
  };
}

const listResponseSchemas = {
  ...registerNamedListSchemas("Skills", "SkillRecord"),
  ...registerNamedListSchemas("SkillPackages", "SkillPackageRecord"),
  ...registerNamedListSchemas("Categories", "SkillCategoryRecord"),
  ...registerNamedListSchemas("SkillsManagement", "SkillRecord"),
  ...registerNamedListSchemas("SkillPackagesManagement", "SkillPackageRecord"),
  ...registerNamedListSchemas("CategoriesManagement", "SkillCategoryRecord"),
};

const resourceResponseSchemas = {
  ...registerNamedResourceSchemas("Skills", "SkillRecord"),
  ...registerNamedResourceSchemas("SkillPackages", "SkillPackageRecord"),
  ...registerNamedResourceSchemas("Categories", "SkillCategoryRecord"),
  ...registerNamedResourceSchemas("UserSkillsInstall", "UserSkillInstallRecord"),
  ...registerNamedResourceSchemas("SkillPackagesCreate", "SkillPackageRecord"),
  ...registerNamedResourceSchemas("SkillPackagesUpdate", "SkillPackageRecord"),
  ...registerNamedResourceSchemas("SkillPackagesDelete", "SkillPackageRecord"),
  ...registerNamedResourceSchemas("CategoriesCreate", "SkillCategoryRecord"),
  ...registerNamedResourceSchemas("CategoriesUpdate", "SkillCategoryRecord"),
};

const schemas = {
  ...structuredClone(sdkWorkEnvelopeComponentSchemas),
  ...domainSchemas,
  ...listResponseSchemas,
  ...resourceResponseSchemas,
};

function listResponse(responseSchemaName) {
  return { $ref: `#/components/schemas/${responseSchemaName}` };
}

function resourceResponse(responseSchemaName) {
  return { $ref: `#/components/schemas/${responseSchemaName}` };
}
const appRoutes = [
  route("get", "/app/v3/api/skills", "skills.list", listResponse("SkillsListResponse"), listQueryParameters(), null, "skills.marketplace.read"),
  route("get", "/app/v3/api/skills/{skillKey}", "skills.retrieve", resourceResponse("SkillsResponse"), [pathParam("skillKey")], null, "skills.marketplace.read"),
  route("get", "/app/v3/api/skill_packages", "skillPackages.list", listResponse("SkillPackagesListResponse"), listQueryParameters(), null, "skills.marketplace.read"),
  route("get", "/app/v3/api/skill_packages/{skillId}", "skillPackages.retrieve", resourceResponse("SkillPackagesResponse"), [pathParam("skillId")], null, "skills.marketplace.read"),
  route("get", "/app/v3/api/categories", "categories.list", listResponse("CategoriesListResponse"), listQueryParameters(), null, "skills.marketplace.read"),
  route("post", "/app/v3/api/user/skills/install", "userSkills.install", resourceResponse("UserSkillsInstallResponse"), [], "InstallSkillCommand", "skills.packages.install"),
];

const backendRoutes = [
  route("get", "/backend/v3/api/skill", "skills.management.list", listResponse("SkillsManagementListResponse"), listQueryParameters(), null, "skills.marketplace.read"),
  route("get", "/backend/v3/api/skill/package", "skillPackages.management.list", listResponse("SkillPackagesManagementListResponse"), listQueryParameters(), null, "skills.packages.manage"),
  route("post", "/backend/v3/api/skill/package", "skillPackages.create", resourceResponse("SkillPackagesCreateResponse"), [], "CreateSkillPackageCommand", "skills.packages.manage"),
  route("put", "/backend/v3/api/skill/package/{skillId}", "skillPackages.update", resourceResponse("SkillPackagesUpdateResponse"), [pathParam("skillId")], "UpdateSkillPackageCommand", "skills.packages.manage"),
  route("delete", "/backend/v3/api/skill/package/{skillId}", "skillPackages.delete", resourceResponse("SkillPackagesDeleteResponse"), [pathParam("skillId")], null, "skills.packages.manage"),
  route("get", "/backend/v3/api/category", "categories.management.list", listResponse("CategoriesManagementListResponse"), listQueryParameters(), null, "skills.categories.manage"),
  route("post", "/backend/v3/api/category", "categories.create", resourceResponse("CategoriesCreateResponse"), [], "CreateSkillCategoryCommand", "skills.categories.manage"),
  route("put", "/backend/v3/api/category/{categoryId}", "categories.update", resourceResponse("CategoriesUpdateResponse"), [pathParamInt("categoryId")], "UpdateSkillCategoryCommand", "skills.categories.manage"),
];

function pathParam(name) {
  return { name, in: "path", required: true, schema: { type: "string" } };
}

function pathParamInt(name) {
  return { name, in: "path", required: true, schema: int64StringProperty() };
}

function problemResponse() {
  return {
    description: "Problem detail",
    content: {
      "application/problem+json": {
        schema: { $ref: "#/components/schemas/ProblemDetail" },
      },
    },
  };
}

function route(method, pathKey, operationId, responseSchema, parameters = [], bodySchemaName = null, permission = null) {
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
                  schema: { $ref: `#/components/schemas/${bodySchemaName}` },
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
        403: problemResponse(),
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
      ...(permission ? { "x-sdkwork-permission": permission } : {}),
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
