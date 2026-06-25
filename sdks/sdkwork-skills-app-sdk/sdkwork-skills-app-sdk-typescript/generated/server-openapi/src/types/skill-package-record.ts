export interface SkillPackageRecord {
  id: string;
  skill_id: string;
  code: string;
  display_name: string;
  summary?: string | null;
  invocation_kind: string;
  /** Drive-backed package reference (sdkwork-drive). */
  package_ref: string;
  entrypoint: string;
  status: string;
  visibility: string;
}
