INSERT INTO ai_skill_category (
    uuid,
    tenant_id,
    organization_id,
    category_type,
    code,
    name,
    description,
    sort_weight,
    permission_code
)
VALUES
  (
    'cat.skill.market.default',
    100001,
    0,
    'skill_market',
    'default',
    '默认分类',
    'Skills Hub 默认市场分类',
    0,
    'skills.packages.manage.default'
  ),
  (
    'cat.skill.collection.official',
    100001,
    0,
    'skills_collection',
    'official',
    '官方精选',
    '官方技能合集',
    0,
    'skills.packages.manage.official'
  )
ON CONFLICT DO NOTHING;
