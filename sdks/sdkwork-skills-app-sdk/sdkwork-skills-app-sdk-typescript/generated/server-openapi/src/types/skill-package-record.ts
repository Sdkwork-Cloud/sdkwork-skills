export interface SkillPackageRecord {
  id: string;
  skill_id: string;
  code: string;
  display_name: string;
  summary?: string | null;
  invocation_kind: string;
  /** Canonical sdkwork-drive package reference. */
  package_ref: string;
  entrypoint: string;
  status: string;
  visibility: string;
  categories?: string[];
  tags?: string[];
}
