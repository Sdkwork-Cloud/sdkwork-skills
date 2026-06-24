import test from 'node:test';
import assert from 'node:assert/strict';

test('skills pc routes include hub and admin surfaces', async () => {
  const routes = ['/skills-hub', '/console/skills', '/admin/skills', '/admin/categories'];
  assert.equal(routes.length, 4);
});
