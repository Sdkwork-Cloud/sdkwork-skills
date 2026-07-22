import { backendApiPath } from './paths';
import type { HttpClient } from '../http/client';

import type { CreateSkillArtifactCommand, CreateSkillCapabilityCommand, CreateSkillCategoryCommand, CreateSkillPackageCommand, SkillArtifactRecord, SkillArtifactsPageData, SkillCapabilitiesPageData, SkillCapabilityRecord, SkillCategoriesPageData, SkillCategoryRecord, SkillPackageRecord, SkillPackagesPageData, SkillsPageData, UpdateSkillCapabilityCommand, UpdateSkillCategoryCommand, UpdateSkillPackageCommand } from '../types';


export interface SkillsSkillCategoriesListParams {
  page?: number;
  pageSize?: number;
  cursor?: string;
  q?: string;
  categoryType?: 'skill_market' | 'skills_collection';
}

export class SkillsSkillCategoriesApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** skillCategories.list */
  async list(params?: SkillsSkillCategoriesListParams): Promise<SkillCategoriesPageData> {
    const query = buildQueryString([
      { name: 'page', value: params?.page, style: 'form', explode: true, allowReserved: false },
      { name: 'page_size', value: params?.pageSize, style: 'form', explode: true, allowReserved: false },
      { name: 'cursor', value: params?.cursor, style: 'form', explode: true, allowReserved: false },
      { name: 'q', value: params?.q, style: 'form', explode: true, allowReserved: false },
      { name: 'category_type', value: params?.categoryType, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.get<SkillCategoriesPageData>(appendQueryString(backendApiPath(`/skill_categories`), query));
  }

/** skillCategories.create */
  async create(body: CreateSkillCategoryCommand): Promise<SkillCategoryRecord> {
    return this.client.post<SkillCategoryRecord>(backendApiPath(`/skill_categories`), body, undefined, undefined, 'application/json');
  }

/** skillCategories.retrieve */
  async retrieve(categoryId: string): Promise<SkillCategoryRecord> {
    return this.client.get<SkillCategoryRecord>(backendApiPath(`/skill_categories/${serializePathParameter(categoryId, { name: 'categoryId', style: 'simple', explode: false })}`));
  }

/** skillCategories.update */
  async update(categoryId: string, body: UpdateSkillCategoryCommand): Promise<SkillCategoryRecord> {
    return this.client.patch<SkillCategoryRecord>(backendApiPath(`/skill_categories/${serializePathParameter(categoryId, { name: 'categoryId', style: 'simple', explode: false })}`), body, undefined, undefined, 'application/json');
  }
}

export interface SkillsSkillCapabilitiesListParams {
  page?: number;
  pageSize?: number;
  cursor?: string;
  q?: string;
}

export class SkillsSkillCapabilitiesApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** skillCapabilities.list */
  async list(params?: SkillsSkillCapabilitiesListParams): Promise<SkillCapabilitiesPageData> {
    const query = buildQueryString([
      { name: 'page', value: params?.page, style: 'form', explode: true, allowReserved: false },
      { name: 'page_size', value: params?.pageSize, style: 'form', explode: true, allowReserved: false },
      { name: 'cursor', value: params?.cursor, style: 'form', explode: true, allowReserved: false },
      { name: 'q', value: params?.q, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.get<SkillCapabilitiesPageData>(appendQueryString(backendApiPath(`/skill_capabilities`), query));
  }

/** skillCapabilities.create */
  async create(body: CreateSkillCapabilityCommand): Promise<SkillCapabilityRecord> {
    return this.client.post<SkillCapabilityRecord>(backendApiPath(`/skill_capabilities`), body, undefined, undefined, 'application/json');
  }

/** skillCapabilities.retrieve */
  async retrieve(capabilityId: string): Promise<SkillCapabilityRecord> {
    return this.client.get<SkillCapabilityRecord>(backendApiPath(`/skill_capabilities/${serializePathParameter(capabilityId, { name: 'capabilityId', style: 'simple', explode: false })}`));
  }

/** skillCapabilities.update */
  async update(capabilityId: string, body: UpdateSkillCapabilityCommand): Promise<SkillCapabilityRecord> {
    return this.client.patch<SkillCapabilityRecord>(backendApiPath(`/skill_capabilities/${serializePathParameter(capabilityId, { name: 'capabilityId', style: 'simple', explode: false })}`), body, undefined, undefined, 'application/json');
  }
}

export interface SkillsSkillPackagesArtifactsListParams {
  page?: number;
  pageSize?: number;
  cursor?: string;
  q?: string;
}

export class SkillsSkillPackagesArtifactsApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** skillPackages.artifacts.list */
  async list(packageId: string, params?: SkillsSkillPackagesArtifactsListParams): Promise<SkillArtifactsPageData> {
    const query = buildQueryString([
      { name: 'page', value: params?.page, style: 'form', explode: true, allowReserved: false },
      { name: 'page_size', value: params?.pageSize, style: 'form', explode: true, allowReserved: false },
      { name: 'cursor', value: params?.cursor, style: 'form', explode: true, allowReserved: false },
      { name: 'q', value: params?.q, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.get<SkillArtifactsPageData>(appendQueryString(backendApiPath(`/skill_packages/${serializePathParameter(packageId, { name: 'packageId', style: 'simple', explode: false })}/artifacts`), query));
  }

/** skillPackages.artifacts.create */
  async create(packageId: string, body: CreateSkillArtifactCommand): Promise<SkillArtifactRecord> {
    return this.client.post<SkillArtifactRecord>(backendApiPath(`/skill_packages/${serializePathParameter(packageId, { name: 'packageId', style: 'simple', explode: false })}/artifacts`), body, undefined, undefined, 'application/json');
  }
}

export interface SkillsSkillPackagesListParams {
  page?: number;
  pageSize?: number;
  cursor?: string;
  q?: string;
}

export class SkillsSkillPackagesApi {
  private client: HttpClient;
  public readonly artifacts: SkillsSkillPackagesArtifactsApi;

  constructor(client: HttpClient) {
    this.client = client;
    this.artifacts = new SkillsSkillPackagesArtifactsApi(client);
  }


/** skillPackages.list */
  async list(params?: SkillsSkillPackagesListParams): Promise<SkillPackagesPageData> {
    const query = buildQueryString([
      { name: 'page', value: params?.page, style: 'form', explode: true, allowReserved: false },
      { name: 'page_size', value: params?.pageSize, style: 'form', explode: true, allowReserved: false },
      { name: 'cursor', value: params?.cursor, style: 'form', explode: true, allowReserved: false },
      { name: 'q', value: params?.q, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.get<SkillPackagesPageData>(appendQueryString(backendApiPath(`/skill_packages`), query));
  }

/** skillPackages.create */
  async create(body: CreateSkillPackageCommand): Promise<SkillPackageRecord> {
    return this.client.post<SkillPackageRecord>(backendApiPath(`/skill_packages`), body, undefined, undefined, 'application/json');
  }

/** skillPackages.retrieve */
  async retrieve(packageId: string): Promise<SkillPackageRecord> {
    return this.client.get<SkillPackageRecord>(backendApiPath(`/skill_packages/${serializePathParameter(packageId, { name: 'packageId', style: 'simple', explode: false })}`));
  }

/** skillPackages.update */
  async update(packageId: string, body: UpdateSkillPackageCommand): Promise<SkillPackageRecord> {
    return this.client.patch<SkillPackageRecord>(backendApiPath(`/skill_packages/${serializePathParameter(packageId, { name: 'packageId', style: 'simple', explode: false })}`), body, undefined, undefined, 'application/json');
  }

/** skillPackages.delete */
  async delete(packageId: string): Promise<void> {
    return this.client.delete<void>(backendApiPath(`/skill_packages/${serializePathParameter(packageId, { name: 'packageId', style: 'simple', explode: false })}`));
  }
}

export interface SkillsMarketplaceListParams {
  page?: number;
  pageSize?: number;
  cursor?: string;
  q?: string;
}

export class SkillsMarketplaceApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** marketplace.list */
  async list(params?: SkillsMarketplaceListParams): Promise<SkillsPageData> {
    const query = buildQueryString([
      { name: 'page', value: params?.page, style: 'form', explode: true, allowReserved: false },
      { name: 'page_size', value: params?.pageSize, style: 'form', explode: true, allowReserved: false },
      { name: 'cursor', value: params?.cursor, style: 'form', explode: true, allowReserved: false },
      { name: 'q', value: params?.q, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.get<SkillsPageData>(appendQueryString(backendApiPath(`/skills`), query));
  }
}

export class SkillsApi {

  public readonly marketplace: SkillsMarketplaceApi;
  public readonly skillPackages: SkillsSkillPackagesApi;
  public readonly skillCapabilities: SkillsSkillCapabilitiesApi;
  public readonly skillCategories: SkillsSkillCategoriesApi;

  constructor(client: HttpClient) {

    this.marketplace = new SkillsMarketplaceApi(client);
    this.skillPackages = new SkillsSkillPackagesApi(client);
    this.skillCapabilities = new SkillsSkillCapabilitiesApi(client);
    this.skillCategories = new SkillsSkillCategoriesApi(client);
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
interface QueryParameterSpec {
  name: string;
  value: unknown;
  style: string;
  explode: boolean;
  allowReserved: boolean;
  contentType?: string;
}

function buildQueryString(parameters: QueryParameterSpec[]): string {
  const pairs: string[] = [];
  for (const parameter of parameters) {
    appendSerializedParameter(pairs, parameter);
  }
  return pairs.join('&');
}

function appendSerializedParameter(pairs: string[], parameter: QueryParameterSpec): void {
  if (parameter.value === undefined || parameter.value === null) {
    return;
  }

  if (parameter.contentType) {
    pairs.push(`${encodeQueryComponent(parameter.name)}=${encodeQueryValue(JSON.stringify(parameter.value), parameter.allowReserved)}`);
    return;
  }

  const style = parameter.style || 'form';
  if (style === 'deepObject') {
    appendDeepObjectParameter(pairs, parameter.name, parameter.value, parameter.allowReserved);
    return;
  }

  if (Array.isArray(parameter.value)) {
    appendArrayParameter(pairs, parameter.name, parameter.value, style, parameter.explode, parameter.allowReserved);
    return;
  }

  if (typeof parameter.value === 'object') {
    appendObjectParameter(pairs, parameter.name, parameter.value as Record<string, unknown>, style, parameter.explode, parameter.allowReserved);
    return;
  }

  pairs.push(`${encodeQueryComponent(parameter.name)}=${encodeQueryValue(serializePrimitive(parameter.value), parameter.allowReserved)}`);
}

function appendArrayParameter(
  pairs: string[],
  name: string,
  value: unknown[],
  style: string,
  explode: boolean,
  allowReserved: boolean,
): void {
  const values = value
    .filter((item) => item !== undefined && item !== null)
    .map((item) => serializePrimitive(item));
  if (values.length === 0) {
    return;
  }

  if (style === 'form' && explode) {
    for (const item of values) {
      pairs.push(`${encodeQueryComponent(name)}=${encodeQueryValue(item, allowReserved)}`);
    }
    return;
  }

  pairs.push(`${encodeQueryComponent(name)}=${encodeQueryValue(values.join(','), allowReserved)}`);
}

function appendObjectParameter(
  pairs: string[],
  name: string,
  value: Record<string, unknown>,
  style: string,
  explode: boolean,
  allowReserved: boolean,
): void {
  const entries = Object.entries(value).filter(([, entryValue]) => entryValue !== undefined && entryValue !== null);
  if (entries.length === 0) {
    return;
  }

  if (style === 'form' && explode) {
    for (const [key, entryValue] of entries) {
      pairs.push(`${encodeQueryComponent(key)}=${encodeQueryValue(serializePrimitive(entryValue), allowReserved)}`);
    }
    return;
  }

  const serialized = entries.flatMap(([key, entryValue]) => [key, serializePrimitive(entryValue)]).join(',');
  pairs.push(`${encodeQueryComponent(name)}=${encodeQueryValue(serialized, allowReserved)}`);
}

function appendDeepObjectParameter(
  pairs: string[],
  name: string,
  value: unknown,
  allowReserved: boolean,
): void {
  if (!value || typeof value !== 'object' || Array.isArray(value)) {
    pairs.push(`${encodeQueryComponent(name)}=${encodeQueryValue(serializePrimitive(value), allowReserved)}`);
    return;
  }

  for (const [key, entryValue] of Object.entries(value as Record<string, unknown>)) {
    if (entryValue === undefined || entryValue === null) {
      continue;
    }
    pairs.push(`${encodeQueryComponent(`${name}[${key}]`)}=${encodeQueryValue(serializePrimitive(entryValue), allowReserved)}`);
  }
}

function serializePrimitive(value: unknown): string {
  if (value instanceof Date) {
    return value.toISOString();
  }
  if (typeof value === 'object') {
    return JSON.stringify(value);
  }
  return String(value);
}

function encodeQueryComponent(value: string): string {
  return encodeURIComponent(value);
}

function encodeQueryValue(value: string, allowReserved: boolean): string {
  const encoded = encodeURIComponent(value);
  if (!allowReserved) {
    return encoded;
  }
  return encoded.replace(/%3A/gi, ':')
    .replace(/%2F/gi, '/')
    .replace(/%3F/gi, '?')
    .replace(/%23/gi, '#')
    .replace(/%5B/gi, '[')
    .replace(/%5D/gi, ']')
    .replace(/%40/gi, '@')
    .replace(/%21/gi, '!')
    .replace(/%24/gi, '$')
    .replace(/%26/gi, '&')
    .replace(/%27/gi, "'")
    .replace(/%28/gi, '(')
    .replace(/%29/gi, ')')
    .replace(/%2A/gi, '*')
    .replace(/%2B/gi, '+')
    .replace(/%2C/gi, ',')
    .replace(/%3B/gi, ';')
    .replace(/%3D/gi, '=');
}
