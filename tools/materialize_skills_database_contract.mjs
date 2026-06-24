#!/usr/bin/env node
import { readFileSync, writeFileSync, existsSync, mkdirSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const baseline = path.join(root, 'database/ddl/baseline/postgres/0001_skills_baseline.sql');
const contractDir = path.join(root, 'database/contract');
mkdirSync(contractDir, { recursive: true });

const tableRegistry = JSON.parse(
  readFileSync(path.join(contractDir, 'table-registry.json'), 'utf8'),
);
const prefixRegistry = JSON.parse(
  readFileSync(path.join(contractDir, 'prefix-registry.json'), 'utf8'),
);

const schemaYaml = [
  'schemaVersion: 1',
  'moduleId: skills',
  'engine: postgres',
  'tables:',
  ...tableRegistry.tables.map((table) => `  - ${table.table_name}`),
  'prefixes:',
  ...prefixRegistry.prefixes.map((entry) => `  - ${entry.prefix}`),
  `baseline: ${path.relative(root, baseline).replaceAll('\\', '/')}`,
  '',
].join('\n');

const output = path.join(contractDir, 'schema.yaml');
const checkOnly = process.argv.includes('--check');
if (checkOnly) {
  const current = existsSync(output) ? readFileSync(output, 'utf8') : '';
  if (current !== schemaYaml) {
    console.error('database contract is out of date; run pnpm db:materialize:contract');
    process.exit(1);
  }
  console.log('skills database contract ok');
  process.exit(0);
}

writeFileSync(output, schemaYaml, 'utf8');
console.log(`materialized ${path.relative(root, output)}`);
