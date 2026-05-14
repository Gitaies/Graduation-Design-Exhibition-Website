# 腾讯云 COS 资源说明

项目**视频文件**存储在腾讯云 COS 对象存储。**作品封面图片**使用本地 WebP（`frontend/public/data/`），不走 COS。

## 基本信息

- **COS 基础 URL**：`https://whcm-1353140174.cos.ap-nanjing.myqcloud.com`
- **存储桶位置**：南京（ap-nanjing）

## 资源路径

| 专业 | 目录 | 文件范围 | 数量 |
|------|------|----------|------|
| 软件工程 | `resource/Se/` | 001-012.mp4 | 12 |
| 电子信息工程 | `resource/Ele/` | 001-011.mp4 | 11 |
| 广播电视工程 | `resource/Rte/` | 001-010.mp4 | 10 |

**总计 33 个视频**

## URL 格式

```
软件工程 (w001-w012)：https://whcm-1353140174.cos.ap-nanjing.myqcloud.com/resource/Se/{id}.mp4
电子信息 (w013-w023)：https://whcm-1353140174.cos.ap-nanjing.myqcloud.com/resource/Ele/{id}.mp4
广播电视 (w024-w033)：https://whcm-1353140174.cos.ap-nanjing.myqcloud.com/resource/Rte/{id}.mp4
```

数据库 `video_url` 存储 COS URL，`poster_url` 存储本地 WebP 路径（`/data/wXXX.webp`）。

## 环境配置

前端 `.env`（COS_BASE_URL 仅用于视频）：
```env
NUXT_PUBLIC_COS_BASE_URL=https://whcm-1353140174.cos.ap-nanjing.myqcloud.com
```

后端 `.env`：
```env
COS_BASE_URL=https://whcm-1353140174.cos.ap-nanjing.myqcloud.com
```

## CORS 配置

COS 存储桶需配置 CORS 允许前端域名：

```json
{
  "AllowedOrigins": ["*"],
  "AllowedMethods": ["GET", "HEAD"],
  "AllowedHeaders": ["*"],
  "MaxAgeSeconds": 3600
}
```

## 故障排查

**视频加载失败**：
1. 浏览器控制台查看是否为 CORS 错误
2. 检查 COS URL 是否可访问：`curl -I <cos-url>`
3. 确认数据库 `video_url` 格式正确
4. 检查存储桶权限是否为"公有读"
5. 确认 COS Referer 防盗链白名单包含当前域名

**封面图片加载失败**：
1. 检查 `frontend/public/data/wXXX.webp` 文件是否存在
2. 确认数据库 `poster_url` 格式为 `/data/wXXX.webp`
