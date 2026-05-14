-- 数据迁移脚本：将作品数据导入 works 表
-- 使用方法：mysql -h localhost -u root -p graduation_exhibition < scripts/migrate_works.sql

-- 示例数据（根据实际 works.json 内容调整）
INSERT INTO works (
  id,
  title,
  major_code,
  major_name,
  theme,
  tags,
  authors,
  teacher,
  poster_url,
  video_url,
  duration,
  introduction
) VALUES
(
  'work001',
  '智能校园导航系统',
  'software',
  '软件工程',
  '数智焕新',
  '["人工智能", "Web应用"]',
  '["张三", "李四"]',
  '王老师',
  '/data/work001.webp',
  '/static/videos/work001.mp4',
  '05:30',
  '基于深度学习的智能校园导航系统，提供室内外一体化导航服务。'
),
(
  'work002',
  '智能家居控制平台',
  'electronic',
  '电子信息工程',
  '芯火智造',
  '["物联网", "智能硬件"]',
  '["赵五", "钱六"]',
  '刘老师',
  '/data/work002.webp',
  '/static/videos/work002.mp4',
  '04:45',
  '基于物联网技术的智能家居控制平台，实现远程控制和智能联动。'
),
(
  'work003',
  '虚拟演播室系统',
  'broadcast',
  '广播电视工程',
  '虚实共生',
  '["虚拟制作", "影像交互"]',
  '["孙七", "周八"]',
  '陈老师',
  '/data/work003.webp',
  '/static/videos/work003.mp4',
  '06:15',
  '基于虚拟现实技术的演播室系统，实现实时虚拟场景合成。'
);

-- 注意：
-- 1. 这只是示例数据，需要根据实际的 works.json 内容生成完整的 INSERT 语句
-- 2. tags 和 authors 字段使用 JSON 格式
-- 3. poster_url 和 video_url 使用相对路径，指向 /static 目录
-- 4. 执行前请确保 static/posters 和 static/videos 目录中已有对应的文件
