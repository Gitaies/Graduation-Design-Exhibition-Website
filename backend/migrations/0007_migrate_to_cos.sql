-- 迁移作品资源到腾讯云 COS + 本地 WebP 封面
-- poster_url：本地 WebP 封面（frontend/public/data/）
-- video_url：腾讯云 COS 对象存储
-- 注意：封面使用本地 WebP 图片，视频保留在 COS

USE graduation_exhibition;

-- 更新软件工程专业作品 (w001-w012)
-- 视频保留在 COS，封面使用本地 WebP
UPDATE works SET
  poster_url = CONCAT('/data/', id, '.webp'),
  video_url = CONCAT('https://whcm-1353140174.cos.ap-nanjing.myqcloud.com/resource/Se/', LPAD(SUBSTRING(id, 2), 3, '0'), '.mp4')
WHERE major_code = 'software';

-- 更新电子信息工程专业作品 (w013-w023)
UPDATE works SET
  poster_url = CONCAT('/data/', id, '.webp'),
  video_url = CONCAT('https://whcm-1353140174.cos.ap-nanjing.myqcloud.com/resource/Ele/', LPAD(SUBSTRING(id, 2), 3, '0'), '.mp4')
WHERE major_code = 'electronic';

-- 更新广播电视工程专业作品 (w024-w033)
UPDATE works SET
  poster_url = CONCAT('/data/', id, '.webp'),
  video_url = CONCAT('https://whcm-1353140174.cos.ap-nanjing.myqcloud.com/resource/Rte/', LPAD(SUBSTRING(id, 2), 3, '0'), '.mp4')
WHERE major_code = 'broadcast';
