#!/usr/bin/env node
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import path from 'node:path';
import { validateDatabaseFramework } from '../../../sdkwork-specs/tools/check-database-framework-standard.mjs';

const result = validateDatabaseFramework(process.cwd());
assert.equal(result.skipped, false, 'application must own database/');
assert.equal(result.ok, true, `database framework validation failed: ${result.failures.join('; ')}`);

const manifest = JSON.parse(
  readFileSync(path.join(process.cwd(), 'database/database.manifest.json'), 'utf8'),
);
assert.equal(manifest.databaseRole, 'authoritative-server');
assert.deepEqual(manifest.engines, ['postgres']);
assert.equal(manifest.defaultEngine, 'postgres');
assert.equal(
  manifest.lifecycle.autoMigrate,
  false,
  'authoritative startup must not execute pending migrations implicitly',
);
assert.equal(
  manifest.lifecycle.seedOnBoot,
  false,
  'authoritative startup must not mutate reference data implicitly',
);

process.stdout.write('database-framework.contract.test.mjs passed\n');
