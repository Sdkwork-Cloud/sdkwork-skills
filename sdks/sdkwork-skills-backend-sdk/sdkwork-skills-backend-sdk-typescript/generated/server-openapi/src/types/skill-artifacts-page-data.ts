import type { PageInfo } from './page-info';
import type { SkillArtifactRecord } from './skill-artifact-record';

export interface SkillArtifactsPageData {
  items: SkillArtifactRecord[];
  pageInfo: PageInfo;
}
