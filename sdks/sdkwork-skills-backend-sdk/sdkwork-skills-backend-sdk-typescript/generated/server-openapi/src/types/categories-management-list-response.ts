import type { CategoriesManagementPageData } from './categories-management-page-data';

export interface CategoriesManagementListResponse {
  code: 0;
  data: unknown & CategoriesManagementPageData;
  /** Server-owned request correlation id. */
  traceId: string;
}
