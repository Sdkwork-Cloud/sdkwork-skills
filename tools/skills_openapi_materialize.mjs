#!/usr/bin/env node
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { sdkWorkEnvelopeComponentSchemas } from "../../sdkwork-specs/tools/lib/openapi-envelope-schemas.mjs";

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const workspaceRoot = path.resolve(scriptDir, "..");
const OWNER = "sdkwork-skills";
const DOMAIN = "skills";
const TAG = "skills";

const invocationKind = ["local-workflow", "process-adapter", "mcp-tool", "kernel-provider"];
const lifecycleStatus = ["draft", "active", "disabled", "archived", "deleted"];
const visibility = ["private", "tenant", "organization", "public"];
const artifactStatus = ["draft", "published", "yanked"];
const subjectKind = ["user", "workspace", "project", "agent"];
const capabilityRisk = ["standard", "sensitive", "privileged"];
const categoryType = ["skill_market", "skills_collection"];

function uint64StringProperty() {
  return {
    type: "string",
    format: "int64",
    pattern: "^[0-9]+$",
    minLength: 1,
    "x-sdkwork-int64-string": true,
    "x-sdkwork-rust-type": "u64",
  };
}

function nullable(schema) {
  return { ...schema, nullable: true };
}

function strictObject(required, properties) {
  return { type: "object", additionalProperties: false, required, properties };
}

const timestamp = { type: "string", format: "date-time" };
const optionalTimestamp = nullable(timestamp);
const stringArray = { type: "array", items: { type: "string" }, default: [] };
const jsonObject = { type: "object", additionalProperties: true };
const driveUri = {
  type: "string",
  minLength: 1,
  maxLength: 2048,
  pattern: "^drive://spaces/[^/]+/nodes/[^/]+$",
};

const domainSchemas = {
  SkillRecord: strictObject(
    [
      "id", "uuid", "tenantId", "organizationId", "skillKey", "packageId", "name",
      "marketStatus", "visibility", "reviewStatus", "categories", "enabled", "featured",
      "installCount", "tags", "version", "createdAt", "updatedAt",
    ],
    {
      id: uint64StringProperty(),
      uuid: { type: "string" },
      tenantId: uint64StringProperty(),
      organizationId: uint64StringProperty(),
      skillKey: { type: "string", pattern: "^skill\\.[a-z0-9_-]+(?:\\.[a-z0-9_-]+)*$" },
      packageId: uint64StringProperty(),
      name: { type: "string" },
      summary: nullable({ type: "string" }),
      description: nullable({ type: "string" }),
      marketStatus: { type: "string" },
      visibility: { type: "string", enum: visibility },
      reviewStatus: { type: "string" },
      categories: stringArray,
      enabled: { type: "boolean" },
      featured: { type: "boolean" },
      installCount: uint64StringProperty(),
      tags: stringArray,
      version: uint64StringProperty(),
      createdAt: timestamp,
      updatedAt: timestamp,
      deletedAt: optionalTimestamp,
    },
  ),
  SkillPackageRecord: strictObject(
    [
      "id", "uuid", "tenantId", "organizationId", "ownerUserId", "skillKey", "packageKey",
      "code", "displayName", "categories", "tags", "status", "visibility", "featured",
      "sortWeight", "version", "createdAt", "updatedAt",
    ],
    {
      id: uint64StringProperty(),
      uuid: { type: "string" },
      tenantId: uint64StringProperty(),
      organizationId: uint64StringProperty(),
      ownerUserId: uint64StringProperty(),
      skillKey: { type: "string", pattern: "^skill\\.[a-z0-9_-]+(?:\\.[a-z0-9_-]+)*$" },
      packageKey: { type: "string", minLength: 1, maxLength: 128 },
      code: { type: "string", minLength: 1, maxLength: 128 },
      displayName: { type: "string", minLength: 1, maxLength: 255 },
      summary: nullable({ type: "string" }),
      description: nullable({ type: "string" }),
      categories: stringArray,
      tags: stringArray,
      status: { type: "string", enum: lifecycleStatus },
      visibility: { type: "string", enum: visibility },
      featured: { type: "boolean" },
      sortWeight: { type: "integer", format: "int32" },
      version: uint64StringProperty(),
      createdAt: timestamp,
      updatedAt: timestamp,
      deletedAt: optionalTimestamp,
    },
  ),
  SkillArtifactRecord: strictObject(
    [
      "id", "uuid", "tenantId", "packageId", "versionLabel", "artifactRef",
      "checksumSha256", "invocationKind", "entrypoint", "inputSchema", "outputSchema",
      "configSchema", "defaultConfig", "status", "capabilityKeys", "createdAt",
    ],
    {
      id: uint64StringProperty(),
      uuid: { type: "string" },
      tenantId: uint64StringProperty(),
      packageId: uint64StringProperty(),
      versionLabel: { type: "string", minLength: 1, maxLength: 128 },
      artifactRef: driveUri,
      checksumSha256: { type: "string", pattern: "^[0-9a-f]{64}$" },
      sizeBytes: nullable(uint64StringProperty()),
      invocationKind: { type: "string", enum: invocationKind },
      entrypoint: { type: "string", minLength: 1, maxLength: 255 },
      inputSchema: jsonObject,
      outputSchema: jsonObject,
      configSchema: jsonObject,
      defaultConfig: jsonObject,
      securityProfileId: nullable({ type: "string" }),
      status: { type: "string", enum: artifactStatus },
      capabilityKeys: stringArray,
      publishedAt: optionalTimestamp,
      yankedAt: optionalTimestamp,
      createdAt: timestamp,
    },
  ),
  SkillCategoryRecord: strictObject(
    [
      "id", "uuid", "tenantId", "organizationId", "categoryType", "code", "name",
      "sortWeight", "permissionCode", "visible", "status", "version", "createdAt", "updatedAt",
    ],
    {
      id: uint64StringProperty(),
      uuid: { type: "string" },
      tenantId: uint64StringProperty(),
      organizationId: uint64StringProperty(),
      categoryType: { type: "string", enum: categoryType },
      code: { type: "string", minLength: 1, maxLength: 128 },
      name: { type: "string", minLength: 1, maxLength: 255 },
      description: nullable({ type: "string" }),
      parentId: nullable(uint64StringProperty()),
      sortWeight: { type: "integer", format: "int32" },
      permissionCode: { type: "string", minLength: 1 },
      visible: { type: "boolean" },
      status: { type: "integer", format: "int16", enum: [0, 1] },
      version: uint64StringProperty(),
      createdAt: timestamp,
      updatedAt: timestamp,
    },
  ),
  SkillCapabilityRecord: strictObject(
    [
      "id", "uuid", "tenantId", "organizationId", "capabilityKey", "displayName",
      "riskLevel", "status", "version", "createdAt", "updatedAt",
    ],
    {
      id: uint64StringProperty(),
      uuid: { type: "string" },
      tenantId: uint64StringProperty(),
      organizationId: uint64StringProperty(),
      capabilityKey: { type: "string", pattern: "^[a-z0-9_-]+(?:\\.[a-z0-9_-]+)+$" },
      displayName: { type: "string", minLength: 1, maxLength: 255 },
      description: nullable({ type: "string" }),
      riskLevel: { type: "string", enum: capabilityRisk },
      status: { type: "integer", format: "int16", enum: [0, 1] },
      version: uint64StringProperty(),
      createdAt: timestamp,
      updatedAt: timestamp,
    },
  ),
  SkillInstallationRecord: strictObject(
    [
      "id", "uuid", "tenantId", "organizationId", "subjectKind", "subjectId", "skillId",
      "packageId", "artifactId", "installedByUserId", "installStatus", "enabled", "config",
      "version", "installedAt", "updatedAt",
    ],
    {
      id: uint64StringProperty(),
      uuid: { type: "string" },
      tenantId: uint64StringProperty(),
      organizationId: uint64StringProperty(),
      subjectKind: { type: "string", enum: subjectKind },
      subjectId: uint64StringProperty(),
      skillId: uint64StringProperty(),
      packageId: uint64StringProperty(),
      artifactId: uint64StringProperty(),
      installedByUserId: uint64StringProperty(),
      installStatus: { type: "string" },
      enabled: { type: "boolean" },
      config: jsonObject,
      version: uint64StringProperty(),
      installedAt: timestamp,
      updatedAt: timestamp,
    },
  ),
  SkillInstallationTargetCommand: strictObject(["kind", "id"], {
    kind: { type: "string", enum: subjectKind },
    id: uint64StringProperty(),
  }),
  CreateSkillInstallationCommand: strictObject(["artifactId"], {
    artifactId: uint64StringProperty(),
    target: { $ref: "#/components/schemas/SkillInstallationTargetCommand" },
    config: { ...jsonObject, default: {} },
  }),
  CreateSkillArtifactCommand: strictObject(
    ["versionLabel", "artifactRef", "checksumSha256", "invocationKind", "entrypoint"],
    {
      versionLabel: { type: "string", minLength: 1, maxLength: 128 },
      artifactRef: driveUri,
      checksumSha256: { type: "string", pattern: "^[0-9a-f]{64}$" },
      sizeBytes: nullable(uint64StringProperty()),
      invocationKind: { type: "string", enum: invocationKind },
      entrypoint: { type: "string", minLength: 1, maxLength: 255 },
      inputSchema: { ...jsonObject, default: {} },
      outputSchema: { ...jsonObject, default: {} },
      configSchema: { ...jsonObject, default: {} },
      defaultConfig: { ...jsonObject, default: {} },
      securityProfileId: nullable({ type: "string" }),
      status: { type: "string", enum: artifactStatus, default: "draft" },
      capabilityKeys: stringArray,
    },
  ),
  CreateSkillPackageCommand: strictObject(
    ["skillKey", "code", "displayName", "initialArtifact"],
    {
      skillKey: { type: "string", pattern: "^skill\\.[a-z0-9_-]+(?:\\.[a-z0-9_-]+)*$" },
      packageKey: { type: "string", minLength: 1, maxLength: 128 },
      code: { type: "string", minLength: 1, maxLength: 128 },
      displayName: { type: "string", minLength: 1, maxLength: 255 },
      summary: nullable({ type: "string" }),
      description: nullable({ type: "string" }),
      categories: stringArray,
      tags: stringArray,
      status: { type: "string", enum: lifecycleStatus, default: "draft" },
      visibility: { type: "string", enum: visibility, default: "tenant" },
      featured: { type: "boolean", default: false },
      sortWeight: { type: "integer", format: "int32", default: 0 },
      initialArtifact: { $ref: "#/components/schemas/CreateSkillArtifactCommand" },
    },
  ),
  UpdateSkillPackageCommand: strictObject(["version"], {
    version: uint64StringProperty(),
    displayName: { type: "string", minLength: 1, maxLength: 255 },
    summary: nullable({ type: "string" }),
    description: nullable({ type: "string" }),
    categories: stringArray,
    tags: stringArray,
    status: { type: "string", enum: lifecycleStatus },
    visibility: { type: "string", enum: visibility },
    featured: { type: "boolean" },
    sortWeight: { type: "integer", format: "int32" },
  }),
  CreateSkillCategoryCommand: strictObject(["code", "name"], {
    categoryType: { type: "string", enum: categoryType, default: "skill_market" },
    code: { type: "string", minLength: 1, maxLength: 128 },
    name: { type: "string", minLength: 1, maxLength: 255 },
    description: nullable({ type: "string" }),
    parentId: nullable(uint64StringProperty()),
    sortWeight: { type: "integer", format: "int32", default: 0 },
    permissionCode: { type: "string" },
    visible: { type: "boolean", default: true },
    status: { type: "integer", format: "int16", enum: [0, 1], default: 1 },
  }),
  UpdateSkillCategoryCommand: strictObject(["version"], {
    version: uint64StringProperty(),
    name: { type: "string", minLength: 1, maxLength: 255 },
    description: nullable({ type: "string" }),
    parentId: nullable(uint64StringProperty()),
    sortWeight: { type: "integer", format: "int32" },
    permissionCode: { type: "string", minLength: 1 },
    visible: { type: "boolean" },
    status: { type: "integer", format: "int16", enum: [0, 1] },
  }),
  CreateSkillCapabilityCommand: strictObject(["capabilityKey", "displayName"], {
    capabilityKey: { type: "string", pattern: "^[a-z0-9_-]+(?:\\.[a-z0-9_-]+)+$" },
    displayName: { type: "string", minLength: 1, maxLength: 255 },
    description: nullable({ type: "string" }),
    riskLevel: { type: "string", enum: capabilityRisk, default: "standard" },
    status: { type: "integer", format: "int16", enum: [0, 1], default: 1 },
  }),
  UpdateSkillCapabilityCommand: strictObject(["version"], {
    version: uint64StringProperty(),
    displayName: { type: "string", minLength: 1, maxLength: 255 },
    description: nullable({ type: "string" }),
    riskLevel: { type: "string", enum: capabilityRisk },
    status: { type: "integer", format: "int16", enum: [0, 1] },
  }),
};

function listQueryParameters() {
  return [
    { name: "page", in: "query", required: false, schema: { type: "integer", format: "int32", minimum: 1, default: 1 } },
    { name: "page_size", in: "query", required: false, schema: { type: "integer", format: "int32", minimum: 1, maximum: 200, default: 20 } },
    { name: "cursor", in: "query", required: false, schema: { type: "string", minLength: 1, maxLength: 512 } },
    { name: "q", in: "query", required: false, schema: { type: "string", maxLength: 256 } },
  ];
}

function categoryListParameters() {
  return [
    ...listQueryParameters(),
    { name: "category_type", in: "query", required: false, schema: { type: "string", enum: categoryType } },
  ];
}

function installationListParameters() {
  return [
    ...listQueryParameters(),
    { name: "subject_kind", in: "query", required: false, schema: { type: "string", enum: subjectKind } },
    { name: "subject_id", in: "query", required: false, schema: uint64StringProperty() },
  ];
}

function registerListSchemas(prefix, itemSchemaName) {
  const page = `${prefix}PageData`;
  const response = `${prefix}ListResponse`;
  return {
    [page]: strictObject(["items", "pageInfo"], {
      items: { type: "array", items: { $ref: `#/components/schemas/${itemSchemaName}` } },
      pageInfo: { $ref: "#/components/schemas/PageInfo" },
    }),
    [response]: {
      allOf: [
        { $ref: "#/components/schemas/SdkWorkApiResponse" },
        { type: "object", required: ["data"], properties: { data: { $ref: `#/components/schemas/${page}` } } },
      ],
    },
  };
}

function registerResourceSchemas(prefix, itemSchemaName) {
  const data = `${prefix}ResourceData`;
  const response = `${prefix}Response`;
  return {
    [data]: strictObject(["item"], { item: { $ref: `#/components/schemas/${itemSchemaName}` } }),
    [response]: {
      allOf: [
        { $ref: "#/components/schemas/SdkWorkApiResponse" },
        { type: "object", required: ["data"], properties: { data: { $ref: `#/components/schemas/${data}` } } },
      ],
    },
  };
}

const envelopeSchemas = structuredClone(sdkWorkEnvelopeComponentSchemas);

const appSchemas = {
  ...envelopeSchemas,
  SkillRecord: domainSchemas.SkillRecord,
  SkillPackageRecord: domainSchemas.SkillPackageRecord,
  SkillArtifactRecord: domainSchemas.SkillArtifactRecord,
  SkillCategoryRecord: domainSchemas.SkillCategoryRecord,
  SkillInstallationRecord: domainSchemas.SkillInstallationRecord,
  SkillInstallationTargetCommand: domainSchemas.SkillInstallationTargetCommand,
  CreateSkillInstallationCommand: domainSchemas.CreateSkillInstallationCommand,
  ...registerListSchemas("Skills", "SkillRecord"),
  ...registerResourceSchemas("Skill", "SkillRecord"),
  ...registerListSchemas("SkillPackages", "SkillPackageRecord"),
  ...registerResourceSchemas("SkillPackage", "SkillPackageRecord"),
  ...registerListSchemas("SkillArtifacts", "SkillArtifactRecord"),
  ...registerListSchemas("SkillCategories", "SkillCategoryRecord"),
  ...registerListSchemas("SkillInstallations", "SkillInstallationRecord"),
  ...registerResourceSchemas("SkillInstallation", "SkillInstallationRecord"),
};

const backendSchemas = {
  ...structuredClone(sdkWorkEnvelopeComponentSchemas),
  SkillRecord: domainSchemas.SkillRecord,
  SkillPackageRecord: domainSchemas.SkillPackageRecord,
  SkillArtifactRecord: domainSchemas.SkillArtifactRecord,
  SkillCategoryRecord: domainSchemas.SkillCategoryRecord,
  SkillCapabilityRecord: domainSchemas.SkillCapabilityRecord,
  CreateSkillArtifactCommand: domainSchemas.CreateSkillArtifactCommand,
  CreateSkillPackageCommand: domainSchemas.CreateSkillPackageCommand,
  UpdateSkillPackageCommand: domainSchemas.UpdateSkillPackageCommand,
  CreateSkillCategoryCommand: domainSchemas.CreateSkillCategoryCommand,
  UpdateSkillCategoryCommand: domainSchemas.UpdateSkillCategoryCommand,
  CreateSkillCapabilityCommand: domainSchemas.CreateSkillCapabilityCommand,
  UpdateSkillCapabilityCommand: domainSchemas.UpdateSkillCapabilityCommand,
  ...registerListSchemas("Skills", "SkillRecord"),
  ...registerResourceSchemas("Skill", "SkillRecord"),
  ...registerListSchemas("SkillPackages", "SkillPackageRecord"),
  ...registerResourceSchemas("SkillPackage", "SkillPackageRecord"),
  ...registerListSchemas("SkillArtifacts", "SkillArtifactRecord"),
  ...registerResourceSchemas("SkillArtifact", "SkillArtifactRecord"),
  ...registerListSchemas("SkillCategories", "SkillCategoryRecord"),
  ...registerResourceSchemas("SkillCategory", "SkillCategoryRecord"),
  ...registerListSchemas("SkillCapabilities", "SkillCapabilityRecord"),
  ...registerResourceSchemas("SkillCapability", "SkillCapabilityRecord"),
};

function listResponse(name) {
  return { $ref: `#/components/schemas/${name}ListResponse` };
}

function resourceResponse(name) {
  return { $ref: `#/components/schemas/${name}Response` };
}

function pathParam(name, schema = { type: "string", minLength: 1 }) {
  return { name, in: "path", required: true, schema };
}

function problemResponse(description = "Problem detail") {
  return {
    description,
    content: { "application/problem+json": { schema: { $ref: "#/components/schemas/ProblemDetail" } } },
  };
}

function route(
  method,
  pathKey,
  operationId,
  responseSchema,
  parameters = [],
  bodySchemaName = null,
  permission = null,
  { successStatus = 200, rateLimitTier = null } = {},
) {
  const successResponse = successStatus === 204
    ? { description: "No Content" }
    : {
        description: successStatus === 201 ? "Created" : "OK",
        content: { "application/json": { schema: responseSchema } },
      };
  return {
    method,
    path: pathKey,
    operation: {
      tags: [TAG],
      summary: operationId,
      operationId,
      parameters,
      ...(bodySchemaName ? {
        requestBody: {
          required: true,
          content: { "application/json": { schema: { $ref: `#/components/schemas/${bodySchemaName}` } } },
        },
      } : {}),
      responses: {
        [successStatus]: successResponse,
        400: problemResponse("Invalid request"),
        401: problemResponse("Authentication required"),
        403: problemResponse("Permission denied"),
        404: problemResponse("Resource not found"),
        409: problemResponse("Resource state conflict"),
        429: problemResponse("Rate limit exceeded"),
        500: problemResponse("Internal server error"),
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
      ...(rateLimitTier ? { "x-sdkwork-rate-limit-tier": rateLimitTier } : {}),
    },
  };
}

const appRoutes = [
  route("get", "/app/v3/api/skills", "marketplace.list", listResponse("Skills"), listQueryParameters(), null, "skills.marketplace.read"),
  route("get", "/app/v3/api/skills/{skillKey}", "marketplace.retrieve", resourceResponse("Skill"), [pathParam("skillKey")], null, "skills.marketplace.read"),
  route("get", "/app/v3/api/skill_packages", "skillPackages.list", listResponse("SkillPackages"), listQueryParameters(), null, "skills.marketplace.read"),
  route("get", "/app/v3/api/skill_packages/{packageId}", "skillPackages.retrieve", resourceResponse("SkillPackage"), [pathParam("packageId", uint64StringProperty())], null, "skills.marketplace.read"),
  route("get", "/app/v3/api/skill_packages/{packageId}/artifacts", "skillPackages.artifacts.list", listResponse("SkillArtifacts"), [pathParam("packageId", uint64StringProperty()), ...listQueryParameters()], null, "skills.marketplace.read"),
  route("get", "/app/v3/api/skill_categories", "skillCategories.list", listResponse("SkillCategories"), listQueryParameters(), null, "skills.marketplace.read"),
  route("post", "/app/v3/api/skill_packages/{packageId}/installations", "skillPackages.installations.create", resourceResponse("SkillInstallation"), [pathParam("packageId", uint64StringProperty())], "CreateSkillInstallationCommand", "skills.packages.install", { successStatus: 201 }),
  route("get", "/app/v3/api/skill_installations", "skillInstallations.list", listResponse("SkillInstallations"), installationListParameters(), null, "skills.installations.read"),
];

const backendRoutes = [
  route("get", "/backend/v3/api/skills", "marketplace.list", listResponse("Skills"), listQueryParameters(), null, "skills.marketplace.read"),
  route("get", "/backend/v3/api/skill_packages", "skillPackages.list", listResponse("SkillPackages"), listQueryParameters(), null, "skills.packages.manage"),
  route("post", "/backend/v3/api/skill_packages", "skillPackages.create", resourceResponse("SkillPackage"), [], "CreateSkillPackageCommand", "skills.packages.manage", { successStatus: 201 }),
  route("get", "/backend/v3/api/skill_packages/{packageId}", "skillPackages.retrieve", resourceResponse("SkillPackage"), [pathParam("packageId", uint64StringProperty())], null, "skills.packages.manage"),
  route("patch", "/backend/v3/api/skill_packages/{packageId}", "skillPackages.update", resourceResponse("SkillPackage"), [pathParam("packageId", uint64StringProperty())], "UpdateSkillPackageCommand", "skills.packages.manage"),
  route("delete", "/backend/v3/api/skill_packages/{packageId}", "skillPackages.delete", null, [pathParam("packageId", uint64StringProperty())], null, "skills.packages.manage", { successStatus: 204, rateLimitTier: "auth_critical" }),
  route("get", "/backend/v3/api/skill_packages/{packageId}/artifacts", "skillPackages.artifacts.list", listResponse("SkillArtifacts"), [pathParam("packageId", uint64StringProperty()), ...listQueryParameters()], null, "skills.artifacts.manage"),
  route("post", "/backend/v3/api/skill_packages/{packageId}/artifacts", "skillPackages.artifacts.create", resourceResponse("SkillArtifact"), [pathParam("packageId", uint64StringProperty())], "CreateSkillArtifactCommand", "skills.artifacts.manage", { successStatus: 201 }),
  route("get", "/backend/v3/api/skill_capabilities", "skillCapabilities.list", listResponse("SkillCapabilities"), listQueryParameters(), null, "skills.capabilities.manage"),
  route("post", "/backend/v3/api/skill_capabilities", "skillCapabilities.create", resourceResponse("SkillCapability"), [], "CreateSkillCapabilityCommand", "skills.capabilities.manage", { successStatus: 201 }),
  route("get", "/backend/v3/api/skill_capabilities/{capabilityId}", "skillCapabilities.retrieve", resourceResponse("SkillCapability"), [pathParam("capabilityId", uint64StringProperty())], null, "skills.capabilities.manage"),
  route("patch", "/backend/v3/api/skill_capabilities/{capabilityId}", "skillCapabilities.update", resourceResponse("SkillCapability"), [pathParam("capabilityId", uint64StringProperty())], "UpdateSkillCapabilityCommand", "skills.capabilities.manage"),
  route("get", "/backend/v3/api/skill_categories", "skillCategories.list", listResponse("SkillCategories"), categoryListParameters(), null, "skills.categories.manage"),
  route("post", "/backend/v3/api/skill_categories", "skillCategories.create", resourceResponse("SkillCategory"), [], "CreateSkillCategoryCommand", "skills.categories.manage", { successStatus: 201 }),
  route("get", "/backend/v3/api/skill_categories/{categoryId}", "skillCategories.retrieve", resourceResponse("SkillCategory"), [pathParam("categoryId", uint64StringProperty())], null, "skills.categories.manage"),
  route("patch", "/backend/v3/api/skill_categories/{categoryId}", "skillCategories.update", resourceResponse("SkillCategory"), [pathParam("categoryId", uint64StringProperty())], "UpdateSkillCategoryCommand", "skills.categories.manage"),
];

function documentFor({ authority, componentSchemas, routes, serverUrl, title, surface }) {
  const paths = {};
  for (const item of routes) {
    paths[item.path] ??= {};
    item.operation["x-sdkwork-api-authority"] = authority;
    item.operation["x-sdkwork-api-surface"] = surface;
    paths[item.path][item.method] = item.operation;
  }
  return {
    openapi: "3.1.2",
    info: { title, version: "1.0.0", "x-sdkwork-owner": OWNER, "x-sdkwork-api-authority": authority },
    servers: [{ url: serverUrl }],
    tags: [{ name: TAG, description: "Skills marketplace and package resources.", "x-sdk-nested-resource-surface": true }],
    paths,
    components: {
      securitySchemes: {
        AuthToken: { type: "http", scheme: "bearer", bearerFormat: "JWT" },
        AccessToken: { type: "apiKey", in: "header", name: "Access-Token" },
      },
      schemas: componentSchemas,
    },
    "x-sdkwork-owner": OWNER,
    "x-sdkwork-api-authority": authority,
    "x-sdkwork-domain": DOMAIN,
    "x-sdkwork-standard-profile": "sdkwork-v3",
  };
}

const outputs = [
  {
    file: path.join(workspaceRoot, "apis/app-api/skills/skills-app-api.openapi.json"),
    document: documentFor({
      authority: "sdkwork-skills-app-api",
      componentSchemas: appSchemas,
      routes: appRoutes,
      serverUrl: "http://127.0.0.1:18092",
      title: "SDKWork Skills App API",
      surface: "app-api",
    }),
  },
  {
    file: path.join(workspaceRoot, "apis/backend-api/skills/skills-backend-api.openapi.json"),
    document: documentFor({
      authority: "sdkwork-skills-backend-api",
      componentSchemas: backendSchemas,
      routes: backendRoutes,
      serverUrl: "http://127.0.0.1:18092",
      title: "SDKWork Skills Backend API",
      surface: "backend-api",
    }),
  },
];

const checkOnly = process.argv.includes("--check");
let stale = false;
for (const { file, document } of outputs) {
  const expected = `${JSON.stringify(document, null, 2)}\n`;
  if (checkOnly) {
    let actual = "";
    try {
      actual = readFileSync(file, "utf8");
    } catch {
      console.error(`missing OpenAPI authority: ${file}`);
      stale = true;
      continue;
    }
    if (actual !== expected) {
      console.error(`stale OpenAPI authority: ${file}`);
      stale = true;
    }
  } else {
    mkdirSync(path.dirname(file), { recursive: true });
    writeFileSync(file, expected, "utf8");
  }
}

if (stale) {
  process.exit(1);
}
process.stdout.write(`[skills_openapi_materialize] ok app=${appRoutes.length} backend=${backendRoutes.length}\n`);
