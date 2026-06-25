import type { SdkworkDriveAppClient } from '@sdkwork/drive-app-sdk';
import { formatDrivePackageRef } from '@sdkwork/skills-pc-commons/driveUri';
import {
  resolveSkillsDriveParentNodeId,
  resolveSkillsDriveSpaceId,
} from '@sdkwork/skills-pc-commons/runtime';

export type SkillPackageUploadOptions = {
  spaceId?: string;
  parentNodeId?: string;
};

export async function uploadSkillPackageArchive(
  driveClient: SdkworkDriveAppClient,
  file: File,
  options: SkillPackageUploadOptions = {},
): Promise<string> {
  const spaceId = options.spaceId ?? resolveSkillsDriveSpaceId();
  if (!spaceId) {
    throw new Error(
      'VITE_SDKWORK_SKILLS_DRIVE_SPACE_ID is required before uploading skill packages through sdkwork-drive.',
    );
  }

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
  });

  return formatDrivePackageRef(uploadResult.uploadItem.spaceId, uploadResult.uploadItem.nodeId);
}
