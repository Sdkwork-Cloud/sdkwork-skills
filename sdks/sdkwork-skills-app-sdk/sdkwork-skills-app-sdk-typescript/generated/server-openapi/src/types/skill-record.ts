export interface SkillRecord {
  id: string;
  skill_key: string;
  name: string;
  summary?: string | null;
  description?: string | null;
  runtime?: string | null;
  entrypoint?: string | null;
  market_status: string;
  visibility: string;
  enabled: boolean;
  featured: boolean;
  install_count: string;
  tags?: string[];
  capabilities?: string[];
}
