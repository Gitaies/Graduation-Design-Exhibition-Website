-- 迁移作品资源到腾讯云 COS
-- 将本地静态资源路径更新为 COS 对象存储路径
-- 注意：封面使用视频文件本身（浏览器会自动显示视频第一帧）

USE graduation_exhibition;

-- 更新软件工程专业作品的视频和封面路径 (Se)
-- Se 文件夹有 001-012 共 12 个视频（013 是宣传视频不使用）
UPDATE works SET
  poster_url = CONCAT('https://whcm-1353140174.cos.ap-nanjing.myqcloud.com/resource/Se/', LPAD(SUBSTRING(id, 2), 3, '0'), '.mp4'),
  video_url = CONCAT('https://whcm-1353140174.cos.ap-nanjing.myqcloud.com/resource/Se/', LPAD(SUBSTRING(id, 2), 3, '0'), '.mp4')
WHERE major_code = 'software';

-- 更新电子信息工程专业作品的视频和封面路径 (Ele)
-- Ele 文件夹有 001-011 共 11 个视频
UPDATE works SET
  poster_url = CONCAT('https://whcm-1353140174.cos.ap-nanjing.myqcloud.com/resource/Ele/', LPAD(SUBSTRING(id, 2), 3, '0'), '.mp4'),
  video_url = CONCAT('https://whcm-1353140174.cos.ap-nanjing.myqcloud.com/resource/Ele/', LPAD(SUBSTRING(id, 2), 3, '0'), '.mp4')
WHERE major_code = 'electronic';

-- 更新广播电视工程专业作品的视频和封面路径 (Rte)
-- Rte 文件夹有 001-010 共 10 个视频
UPDATE works SET
  poster_url = CONCAT('https://whcm-1353140174.cos.ap-nanjing.myqcloud.com/resource/Rte/', LPAD(SUBSTRING(id, 2), 3, '0'), '.mp4'),
  video_url = CONCAT('https://whcm-1353140174.cos.ap-nanjing.myqcloud.com/resource/Rte/', LPAD(SUBSTRING(id, 2), 3, '0'), '.mp4')
WHERE major_code = 'broadcast';
