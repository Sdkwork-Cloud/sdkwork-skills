import { register } from 'node:module';
import path from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const hooksPath = pathToFileURL(
  path.join(path.dirname(fileURLToPath(import.meta.url)), 'workspace-import-hooks.mjs'),
).href;

register(hooksPath);
