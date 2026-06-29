import type { CategoriesUpdateResourceData } from './categories-update-resource-data';

export interface CategoriesUpdateResponse {
  code: 0;
  data: unknown & CategoriesUpdateResourceData;
  /** Server-owned request correlation id. */
  traceId: string;
}
