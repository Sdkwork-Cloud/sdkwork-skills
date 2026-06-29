export interface UpdateSkillPackageCommand {
  package_key?: string;
  code?: string;
  display_name?: string;
  summary?: string;
  invocation_kind?: string;
  package_ref?: string;
  entrypoint?: string;
  capability_ids?: string[];
  categories?: string[];
  tags?: string[];
  visibility?: string;
}
