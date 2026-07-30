INSERT INTO skills_category (
    id,
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
    9100000000001,
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
    9100000000002,
    'cat.skill.market.video',
    100001,
    0,
    'skill_market',
    'video-production',
    '短剧影视',
    '短片、分镜与影视制作技能',
    100,
    'skills.packages.manage.video'
  ),
  (
    9100000000003,
    'cat.skill.market.marketing',
    100001,
    0,
    'skill_market',
    'marketing',
    '电商营销',
    '品牌、电商与营销内容技能',
    90,
    'skills.packages.manage.marketing'
  ),
  (
    9100000000004,
    'cat.skill.market.creative',
    100001,
    0,
    'skill_market',
    'creative-art',
    '创意艺术',
    '视觉设计与创意艺术技能',
    80,
    'skills.packages.manage.creative'
  ),
  (
    9100000000005,
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
ON CONFLICT (id) DO UPDATE SET
    name = EXCLUDED.name,
    description = EXCLUDED.description,
    sort_weight = EXCLUDED.sort_weight,
    permission_code = EXCLUDED.permission_code,
    visible = 1,
    status = 1,
    updated_at = CURRENT_TIMESTAMP;
