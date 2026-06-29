import type { CategoriesCreateResourceData } from './categories-create-resource-data';

export interface CategoriesCreateResponse {
  code: 0;
  data: unknown & CategoriesCreateResourceData;
  /** Server-owned request correlation id. */
  traceId: string;
}
