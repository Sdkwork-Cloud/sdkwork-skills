-- Public Skills marketplace records used by SDKWork Agents PC.
INSERT INTO skills_package (
    id, uuid, tenant_id, organization_id, owner_user_id, package_key, code,
    display_name, summary, description, tags_json, status, visibility,
    featured, sort_weight, version, created_at, updated_at
)
VALUES
    (9200000000001, 'pkg.agents.storyboard', 100001, 0, 9001, 'agents-storyboard', 'agents_storyboard', '叙事短片导演分镜', '从创意到镜头表，生成可执行的短片分镜方案。', '输出导演意图、镜头节奏、逐镜提示词与制作建议。', '["author:森海荧光","storyboard","video"]', 1, 3, 1, 100, 1, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
    (9200000000002, 'pkg.agents.image-to-film', 100001, 0, 9001, 'agents-image-to-film', 'agents_image_to_film', '一图成片导演', '基于参考图片生成电影短片与品牌视频方案。', '完成视觉分析、镜头设计、提示词和成片规划。', '["author:渊静-中意","video","director"]', 1, 3, 1, 95, 1, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
    (9200000000003, 'pkg.agents.world-design', 100001, 0, 9001, 'agents-world-design', 'agents_world_design', '世界观美术设定', '建立稳定的世界规则、场景语言与角色视觉体系。', '适用于影视、游戏和系列化内容的美术设定。', '["author:慕影-中意","worldbuilding","art"]', 1, 3, 0, 90, 1, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
    (9200000000004, 'pkg.agents.pop-ad', 100001, 0, 9001, 'agents-pop-ad', 'agents_pop_ad', '波普视觉广告导演', '输出高频闪切、节奏明确的快消品短视频分镜。', '面向品牌广告和社媒短片的视觉创意技能。', '["author:即梦AI","marketing","video"]', 1, 3, 1, 85, 1, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
    (9200000000005, 'pkg.agents.jewelry-commerce', 100001, 0, 9001, 'agents-jewelry-commerce', 'agents_jewelry_commerce', '珠宝电商图文视频', '面向珠宝品类生成图片、文案与视频素材。', '覆盖商品卖点、平台规格和系列化内容交付。', '["author:地质大学博士说AI","commerce","jewelry"]', 1, 3, 0, 80, 1, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
    (9200000000006, 'pkg.agents.story-ad', 100001, 0, 9001, 'agents-story-ad', 'agents_story_ad', '反差叙事剧情广告', '根据产品卖点生成短时长反差剧情广告。', '构建人物、冲突、产品转折与收束文案。', '["author:话神闲","marketing","story"]', 1, 3, 0, 75, 1, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
    (9200000000007, 'pkg.agents.series-images', 100001, 0, 9001, 'agents-series-images', 'agents_series_images', '系列套图生成', '把参考图和品牌资料抽象为稳定的系列视觉规则。', '支持角色、产品和品牌视觉的一致性批量创作。', '["author:渊静-中意","image","creative"]', 1, 3, 1, 70, 1, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
    (9200000000008, 'pkg.agents.pixel-animation', 100001, 0, 9001, 'agents-pixel-animation', 'agents_pixel_animation', '角色精灵图动画产线', '端到端生成游戏角色像素帧动画。', '从角色参考到动作拆分、帧序列和预览视频。', '["author:AIGC炼丹师","pixel","animation"]', 1, 3, 0, 65, 1, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
    (9200000000009, 'pkg.agents.jewelry-design', 100001, 0, 9001, 'agents-jewelry-design', 'agents_jewelry_design', '珠宝设计出款', '专业珠宝设计与系列化批量出款技能。', '支持文字描述、草图与风格参考驱动的设计方案。', '["author:地质大学博士说AI","design","jewelry"]', 1, 3, 0, 60, 1, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
ON CONFLICT (id) DO UPDATE SET
    display_name = EXCLUDED.display_name,
    summary = EXCLUDED.summary,
    description = EXCLUDED.description,
    tags_json = EXCLUDED.tags_json,
    status = EXCLUDED.status,
    visibility = EXCLUDED.visibility,
    featured = EXCLUDED.featured,
    sort_weight = EXCLUDED.sort_weight,
    updated_at = CURRENT_TIMESTAMP;

INSERT INTO skills_definition (
    id, uuid, tenant_id, organization_id, skill_key, package_id, market_status,
    review_status, enabled, featured, recommend_weight, install_count,
    rating_avg, rating_count, version, created_at, updated_at
)
VALUES
    (9300000000001, 'skill.agents.storyboard', 100001, 0, 'skill.agents.storyboard', 9200000000001, 'published', 'approved', 1, 1, 100, 162, 4.90, 78, 1, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
    (9300000000002, 'skill.agents.image_to_film', 100001, 0, 'skill.agents.image_to_film', 9200000000002, 'published', 'approved', 1, 1, 95, 107, 4.80, 54, 1, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
    (9300000000003, 'skill.agents.world_design', 100001, 0, 'skill.agents.world_design', 9200000000003, 'published', 'approved', 1, 0, 90, 38, 4.70, 19, 1, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
    (9300000000004, 'skill.agents.pop_ad', 100001, 0, 'skill.agents.pop_ad', 9200000000004, 'published', 'approved', 1, 1, 85, 58, 4.80, 31, 1, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
    (9300000000005, 'skill.agents.jewelry_commerce', 100001, 0, 'skill.agents.jewelry_commerce', 9200000000005, 'published', 'approved', 1, 0, 80, 36, 4.60, 17, 1, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
    (9300000000006, 'skill.agents.story_ad', 100001, 0, 'skill.agents.story_ad', 9200000000006, 'published', 'approved', 1, 0, 75, 13, 4.50, 8, 1, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
    (9300000000007, 'skill.agents.series_images', 100001, 0, 'skill.agents.series_images', 9200000000007, 'published', 'approved', 1, 1, 70, 44, 4.80, 23, 1, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
    (9300000000008, 'skill.agents.pixel_animation', 100001, 0, 'skill.agents.pixel_animation', 9200000000008, 'published', 'approved', 1, 0, 65, 20, 4.60, 11, 1, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
    (9300000000009, 'skill.agents.jewelry_design', 100001, 0, 'skill.agents.jewelry_design', 9200000000009, 'published', 'approved', 1, 0, 60, 15, 4.70, 9, 1, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
ON CONFLICT (id) DO UPDATE SET
    market_status = EXCLUDED.market_status,
    review_status = EXCLUDED.review_status,
    enabled = EXCLUDED.enabled,
    featured = EXCLUDED.featured,
    recommend_weight = EXCLUDED.recommend_weight,
    install_count = EXCLUDED.install_count,
    rating_avg = EXCLUDED.rating_avg,
    rating_count = EXCLUDED.rating_count,
    updated_at = CURRENT_TIMESTAMP;

INSERT INTO skills_category_binding (id, tenant_id, skill_id, category_id)
VALUES
    (9400000000001, 100001, 9300000000001, 9100000000002),
    (9400000000002, 100001, 9300000000002, 9100000000002),
    (9400000000003, 100001, 9300000000003, 9100000000002),
    (9400000000004, 100001, 9300000000004, 9100000000003),
    (9400000000005, 100001, 9300000000005, 9100000000003),
    (9400000000006, 100001, 9300000000006, 9100000000003),
    (9400000000007, 100001, 9300000000007, 9100000000004),
    (9400000000008, 100001, 9300000000008, 9100000000004),
    (9400000000009, 100001, 9300000000009, 9100000000004)
ON CONFLICT (skill_id, category_id) DO NOTHING;
