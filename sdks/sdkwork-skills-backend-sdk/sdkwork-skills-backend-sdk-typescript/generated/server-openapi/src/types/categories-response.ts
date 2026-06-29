import type { CategoriesResourceData } from './categories-resource-data';

export interface CategoriesResponse {
  code: 0;
  data: unknown & CategoriesResourceData;
  /** Server-owned request correlation id. */
  traceId: string;
}
