#!/usr/bin/env node
import { existsSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const required = [
  'apis/app-api/skills/skills-app-api.openapi.json',
  'apis/backend-api/skills/skills-backend-api.openapi.json',
];

const checkOnly = process.argv.includes('--check');
for (const relativePath of required) {
  const absolutePath = path.join(root, relativePath);
  if (!existsSync(absolutePath)) {
    console.error(`missing api contract: ${relativePath}`);
    process.exit(1);
  }
}

if (checkOnly) {
  console.log('skills api contracts ok');
} else {
  console.log('skills api contracts present');
}
