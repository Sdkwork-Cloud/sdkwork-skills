INSERT INTO c_category (uuid, tenant_id, organization_id, category_type, code, name, description, sort_weight)
VALUES
  ('cat.skill.market.default', 100001, 0, 'skill_market', 'default', '默认分类', 'Skills Hub 默认市场分类', 0),
  ('cat.skill.collection.official', 100001, 0, 'skills_collection', 'official', '官方精选', '官方技能合集', 0)
ON CONFLICT DO NOTHING;
