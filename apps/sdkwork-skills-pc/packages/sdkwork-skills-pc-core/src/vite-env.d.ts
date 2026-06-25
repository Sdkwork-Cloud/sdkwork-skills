/// <reference types="vite/client" />

interface ImportMetaEnv {
  readonly VITE_SDKWORK_SKILLS_APP_API_BASE_URL?: string;
  readonly VITE_SDKWORK_SKILLS_BACKEND_API_BASE_URL?: string;
  readonly VITE_SDKWORK_SKILLS_TENANT_ID?: string;
}

interface ImportMeta {
  readonly env: ImportMetaEnv;
}
