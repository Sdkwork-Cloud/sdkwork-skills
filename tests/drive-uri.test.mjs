import test from 'node:test';
import assert from 'node:assert/strict';

test('drive artifact ref helpers roundtrip canonical drive uri', async () => {
  const { formatDriveArtifactRef, parseDriveArtifactRef, isDriveArtifactRef } = await import(
    '../apps/sdkwork-skills-pc/packages/sdkwork-skills-pc-commons/src/driveUri.ts'
  );
  const artifactRef = formatDriveArtifactRef('skills-space', 'node-42');
  assert.equal(artifactRef, 'drive://spaces/skills-space/nodes/node-42');
  assert.deepEqual(parseDriveArtifactRef(artifactRef), {
    spaceId: 'skills-space',
    nodeId: 'node-42',
  });
  assert.equal(isDriveArtifactRef(artifactRef), true);
  assert.equal(isDriveArtifactRef('file://legacy'), false);
  assert.equal(isDriveArtifactRef('drive://spaces/skills-space/nodes/'), false);
  assert.throws(() => formatDriveArtifactRef(' ', 'node-42'), /spaceId and nodeId are required/u);
});
