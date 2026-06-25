import { appApiPath } from './paths';
import type { HttpClient } from '../http/client';

import type { InstallSkillCommand, SkillCategoryListResponse, SkillListResponse, SkillPackageListResponse, SkillPackageRecordResponse, SkillRecordResponse, UserSkillInstallRecordResponse } from '../types';


export class SkillsUserSkillsApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Skills userSkills.install */
  async install(body: InstallSkillCommand): Promise<UserSkillInstallRecordResponse> {
    return this.client.post<UserSkillInstallRecordResponse>(appApiPath(`/user/skills/install`), body, undefined, undefined, 'application/json');
  }
}

export class SkillsCategoriesApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Skills categories.list */
  async list(): Promise<SkillCategoryListResponse> {
    return this.client.get<SkillCategoryListResponse>(appApiPath(`/categories`));
  }
}

export class SkillsSkillPackagesApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Skills skillPackages.list */
  async list(): Promise<SkillPackageListResponse> {
    return this.client.get<SkillPackageListResponse>(appApiPath(`/skill_packages`));
  }

/** Skills skillPackages.retrieve */
  async retrieve(skillId: string): Promise<SkillPackageRecordResponse> {
    return this.client.get<SkillPackageRecordResponse>(appApiPath(`/skill_packages/${serializePathParameter(skillId, { name: 'skillId', style: 'simple', explode: false })}`));
  }
}

export class SkillsApi {
  private client: HttpClient;
  public readonly skillPackages: SkillsSkillPackagesApi;
  public readonly categories: SkillsCategoriesApi;
  public readonly userSkills: SkillsUserSkillsApi;

  constructor(client: HttpClient) {
    this.client = client;
    this.skillPackages = new SkillsSkillPackagesApi(client);
    this.categories = new SkillsCategoriesApi(client);
    this.userSkills = new SkillsUserSkillsApi(client);
  }


/** Skills skills.list */
  async list(): Promise<SkillListResponse> {
    return this.client.get<SkillListResponse>(appApiPath(`/skills`));
  }

/** Skills skills.retrieve */
  async retrieve(skillKey: string): Promise<SkillRecordResponse> {
    return this.client.get<SkillRecordResponse>(appApiPath(`/skills/${serializePathParameter(skillKey, { name: 'skillKey', style: 'simple', explode: false })}`));
  }
}

export function createSkillsApi(client: HttpClient): SkillsApi {
  return new SkillsApi(client);
}

function appendQueryString(path: string, rawQueryString: string): string {
  const query = rawQueryString.replace(/^\?+/, '');
  if (!query) {
    return path;
  }
  return path.includes('?') ? `${path}&${query}` : `${path}?${query}`;
}

interface PathParameterSpec {
  name: string;
  style: string;
  explode: boolean;
}

function serializePathParameter(value: unknown, spec: PathParameterSpec): string {
  if (value === undefined || value === null) {
    return '';
  }

  const style = spec.style || 'simple';
  if (Array.isArray(value)) {
    return serializePathArray(spec.name, value, style, spec.explode);
  }
  if (typeof value === 'object') {
    return serializePathObject(spec.name, value as Record<string, unknown>, style, spec.explode);
  }
  return pathPrefix(spec.name, style, false) + encodePathValue(serializePathPrimitive(value));
}

function serializePathArray(name: string, values: unknown[], style: string, explode: boolean): string {
  const serialized = values
    .filter((item) => item !== undefined && item !== null)
    .map((item) => encodePathValue(serializePathPrimitive(item)));
  if (serialized.length === 0) {
    return pathPrefix(name, style, false);
  }
  if (style === 'matrix') {
    return explode
      ? serialized.map((item) => `;${name}=${item}`).join('')
      : `;${name}=${serialized.join(',')}`;
  }
  return pathPrefix(name, style, false) + serialized.join(explode ? '.' : ',');
}

function serializePathObject(name: string, value: Record<string, unknown>, style: string, explode: boolean): string {
  const entries = Object.entries(value).filter(([, entryValue]) => entryValue !== undefined && entryValue !== null);
  if (entries.length === 0) {
    return pathPrefix(name, style, true);
  }
  if (style === 'matrix') {
    return explode
      ? entries.map(([key, entryValue]) => `;${encodePathValue(key)}=${encodePathValue(serializePathPrimitive(entryValue))}`).join('')
      : `;${name}=${entries.flatMap(([key, entryValue]) => [encodePathValue(key), encodePathValue(serializePathPrimitive(entryValue))]).join(',')}`;
  }
  const serialized = explode
    ? entries.map(([key, entryValue]) => `${encodePathValue(key)}=${encodePathValue(serializePathPrimitive(entryValue))}`).join(style === 'label' ? '.' : ',')
    : entries.flatMap(([key, entryValue]) => [encodePathValue(key), encodePathValue(serializePathPrimitive(entryValue))]).join(',');
  return pathPrefix(name, style, true) + serialized;
}

function pathPrefix(name: string, style: string, _objectValue: boolean): string {
  if (style === 'label') return '.';
  if (style === 'matrix') return `;${name}`;
  return '';
}

function encodePathValue(value: string): string {
  return encodeURIComponent(value);
}

function serializePathPrimitive(value: unknown): string {
  if (value instanceof Date) {
    return value.toISOString();
  }
  if (typeof value === 'object') {
    return JSON.stringify(value);
  }
  return String(value);
}
