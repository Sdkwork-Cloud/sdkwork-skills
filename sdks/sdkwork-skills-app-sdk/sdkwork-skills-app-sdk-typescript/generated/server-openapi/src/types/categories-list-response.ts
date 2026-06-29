import type { CategoriesPageData } from './categories-page-data';

export interface CategoriesListResponse {
  code: 0;
  data: unknown & CategoriesPageData;
  /** Server-owned request correlation id. */
  traceId: string;
}
