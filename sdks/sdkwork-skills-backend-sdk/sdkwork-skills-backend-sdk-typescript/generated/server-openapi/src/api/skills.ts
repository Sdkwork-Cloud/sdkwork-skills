import { backendApiPath } from './paths';
import type { HttpClient } from '../http/client';

import type { CreateSkillCategoryCommand, CreateSkillPackageCommand, SkillCategoryListResponse, SkillCategoryRecordResponse, SkillListResponse, SkillPackageListResponse, SkillPackageRecordResponse } from '../types';


export class SkillsCategoriesManagementApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Skills categories.management.list */
  async list(): Promise<SkillCategoryListResponse> {
    return this.client.get<SkillCategoryListResponse>(backendApiPath(`/category`));
  }
}

export class SkillsCategoriesApi {
  private client: HttpClient;
  public readonly management: SkillsCategoriesManagementApi;

  constructor(client: HttpClient) {
    this.client = client;
    this.management = new SkillsCategoriesManagementApi(client);
  }


/** Skills categories.create */
  async create(body: CreateSkillCategoryCommand): Promise<SkillCategoryRecordResponse> {
    return this.client.post<SkillCategoryRecordResponse>(backendApiPath(`/category`), body, undefined, undefined, 'application/json');
  }

/** Skills categories.update */
  async update(categoryId: string, body: CreateSkillCategoryCommand): Promise<SkillCategoryRecordResponse> {
    return this.client.put<SkillCategoryRecordResponse>(backendApiPath(`/category/${serializePathParameter(categoryId, { name: 'categoryId', style: 'simple', explode: false })}`), body, undefined, undefined, 'application/json');
  }
}

export class SkillsSkillPackagesManagementApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Skills skillPackages.management.list */
  async list(): Promise<SkillPackageListResponse> {
    return this.client.get<SkillPackageListResponse>(backendApiPath(`/skill/package`));
  }
}

export class SkillsSkillPackagesApi {
  private client: HttpClient;
  public readonly management: SkillsSkillPackagesManagementApi;

  constructor(client: HttpClient) {
    this.client = client;
    this.management = new SkillsSkillPackagesManagementApi(client);
  }


/** Skills skillPackages.create */
  async create(body: CreateSkillPackageCommand): Promise<SkillPackageRecordResponse> {
    return this.client.post<SkillPackageRecordResponse>(backendApiPath(`/skill/package`), body, undefined, undefined, 'application/json');
  }

/** Skills skillPackages.update */
  async update(skillId: string, body: CreateSkillPackageCommand): Promise<SkillPackageRecordResponse> {
    return this.client.put<SkillPackageRecordResponse>(backendApiPath(`/skill/package/${serializePathParameter(skillId, { name: 'skillId', style: 'simple', explode: false })}`), body, undefined, undefined, 'application/json');
  }

/** Skills skillPackages.delete */
  async delete(skillId: string): Promise<SkillPackageRecordResponse> {
    return this.client.delete<SkillPackageRecordResponse>(backendApiPath(`/skill/package/${serializePathParameter(skillId, { name: 'skillId', style: 'simple', explode: false })}`));
  }
}

export class SkillsManagementApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Skills skills.management.list */
  async list(): Promise<SkillListResponse> {
    return this.client.get<SkillListResponse>(backendApiPath(`/skill`));
  }
}

export class SkillsApi {
  private client: HttpClient;
  public readonly management: SkillsManagementApi;
  public readonly skillPackages: SkillsSkillPackagesApi;
  public readonly categories: SkillsCategoriesApi;

  constructor(client: HttpClient) {
    this.client = client;
    this.management = new SkillsManagementApi(client);
    this.skillPackages = new SkillsSkillPackagesApi(client);
    this.categories = new SkillsCategoriesApi(client);
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
