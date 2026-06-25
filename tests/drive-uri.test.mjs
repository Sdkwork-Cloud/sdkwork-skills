import test from 'node:test';
import assert from 'node:assert/strict';

test('drive package ref helpers roundtrip canonical drive uri', async () => {
  const { formatDrivePackageRef, parseDrivePackageRef, isDrivePackageRef } = await import(
    '../apps/sdkwork-skills-pc/packages/sdkwork-skills-pc-commons/src/driveUri.ts'
  );
  const packageRef = formatDrivePackageRef('skills-space', 'node-42');
  assert.equal(packageRef, 'drive://spaces/skills-space/nodes/node-42');
  assert.equal(parseDrivePackageRef(packageRef).nodeId, 'node-42');
  assert.equal(isDrivePackageRef(packageRef), true);
  assert.equal(isDrivePackageRef('file://legacy'), false);
});
