import type { SkillsAppClients } from '@sdkwork/skills-pc-core';
import { formatDriveArtifactRef } from '@sdkwork/skills-pc-commons/driveUri';
import { hexEncode, Sha256Hasher } from '@sdkwork/utils';
import {
  resolveSkillsDriveParentNodeId,
  resolveSkillsDriveSpaceId,
} from '@sdkwork/skills-pc-commons/runtime';

type SkillsDriveAppClient = SkillsAppClients['drive'];

export type SkillPackageUploadOptions = {
  spaceId?: string;
  parentNodeId?: string;
};

export interface SkillArtifactUploadResult {
  artifactRef: string;
  checksumSha256: string;
  sizeBytes: string;
}

async function calculateSha256(file: File): Promise<string> {
  const hasher = new Sha256Hasher();
  const reader = file.stream().getReader();

  try {
    while (true) {
      const { done, value } = await reader.read();
      if (done) {
        break;
      }
      hasher.update(value);
    }
  } finally {
    reader.releaseLock();
  }

  return hexEncode(hasher.digest());
}

export async function uploadSkillPackageArchive(
  driveClient: SkillsDriveAppClient,
  file: File,
  options: SkillPackageUploadOptions = {},
): Promise<SkillArtifactUploadResult> {
  const spaceId = options.spaceId ?? resolveSkillsDriveSpaceId();
  if (!spaceId) {
    throw new Error(
      'VITE_SDKWORK_SKILLS_DRIVE_SPACE_ID is required before uploading skill packages through sdkwork-drive.',
    );
  }

  const checksumSha256 = await calculateSha256(file);
  const uploadResult = await driveClient.uploader.upload({
    file,
    appResourceType: 'skills-pc-package-upload',
    appResourceId: file.name,
    scene: 'skills_admin_package_upload',
    source: 'pc_local_file',
    spaceId,
    parentNodeId: options.parentNodeId ?? resolveSkillsDriveParentNodeId(),
    uploadProfileCode: 'archive',
    originalFileName: file.name,
    contentType: file.type || 'application/octet-stream',
    checksumSha256Hex: `sha256:${checksumSha256}`,
  });

  return {
    artifactRef: formatDriveArtifactRef(
      uploadResult.uploadItem.spaceId,
      uploadResult.uploadItem.nodeId,
    ),
    checksumSha256,
    sizeBytes: String(file.size),
  };
}
