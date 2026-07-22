import { isBlank, trim } from '@sdkwork/utils';

const DRIVE_URI_PREFIX = 'drive://spaces/';

export function formatDriveArtifactRef(spaceId: string, nodeId: string): string {
  const normalizedSpaceId = trim(spaceId);
  const normalizedNodeId = trim(nodeId);
  if (isBlank(normalizedSpaceId) || isBlank(normalizedNodeId)) {
    throw new Error('Drive spaceId and nodeId are required to build artifactRef.');
  }
  return `${DRIVE_URI_PREFIX}${normalizedSpaceId}/nodes/${normalizedNodeId}`;
}

export function parseDriveArtifactRef(artifactRef: string): { spaceId: string; nodeId: string } {
  const normalized = trim(artifactRef);
  if (!normalized.startsWith(DRIVE_URI_PREFIX)) {
    throw new Error('artifactRef must use drive://spaces/{spaceId}/nodes/{nodeId}');
  }
  const remainder = normalized.slice(DRIVE_URI_PREFIX.length);
  const [spaceId, nodePart] = remainder.split('/nodes/');
  if (isBlank(spaceId) || isBlank(nodePart)) {
    throw new Error('artifactRef is missing Drive space or node identifiers.');
  }
  return { spaceId, nodeId: nodePart };
}

export function isDriveArtifactRef(artifactRef: string): boolean {
  try {
    parseDriveArtifactRef(artifactRef);
    return true;
  } catch {
    return false;
  }
}
