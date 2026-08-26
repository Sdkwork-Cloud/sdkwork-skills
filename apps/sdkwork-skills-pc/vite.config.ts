import { resolveBrowserDistOutDir } from '../../../sdkwork-specs/tools/browser-dist-layout.mjs';
function resolveViteEnvironment(mode: string | undefined, processEnv = process.env) {
  const profileMatch = /^(standalone|cloud)\.(development|test|staging|production)$/u.exec(mode ?? '');
  return profileMatch?.[2]
    ?? (['development', 'test', 'staging', 'production'].includes(processEnv.SDKWORK_ENVIRONMENT ?? '')
      ? (processEnv.SDKWORK_ENVIRONMENT ?? 'production')
      : 'production');
}
import tailwindcss from '@tailwindcss/vite';
import react from '@vitejs/plugin-react';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { defineConfig, loadEnv } from 'vite';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(__dirname, '../..');
const appbaseRoot = path.resolve(repoRoot, '../sdkwork-appbase');
const iamRoot = path.resolve(repoRoot, '../sdkwork-iam');

export default defineConfig(({ mode }) => {
  const env = loadEnv(mode, __dirname, '');
  const platformApiGatewayTarget =
    env.VITE_SDKWORK_SKILLS_PLATFORM_API_GATEWAY_HTTP_URL ??
    'http://127.0.0.1:3900';

  return {
    build: {
      outDir: resolveBrowserDistOutDir(resolveViteEnvironment(mode, process.env)),
      emptyOutDir: true,
    },
    define: {
      'process.env.SDKWORK_ACCESS_TOKEN': JSON.stringify(env.SDKWORK_ACCESS_TOKEN ?? ''),
    },
    plugins: [react(), tailwindcss()],
    resolve: {
      alias: {
        '@': path.resolve(__dirname, '.'),
      },
    },
    server: {
      port: 5175,
      strictPort: true,
      proxy: {
        '/app/v3/api': {
          target: platformApiGatewayTarget,
          changeOrigin: true,
        },
        '/backend/v3/api': {
          target: platformApiGatewayTarget,
          changeOrigin: true,
        },
        '/app': {
          target: 'http://127.0.0.1:18090',
          changeOrigin: true,
        },
        '/backend': {
          target: 'http://127.0.0.1:18091',
          changeOrigin: true,
        },
      },
    },
  };
});
