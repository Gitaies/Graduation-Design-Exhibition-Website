# 腾讯云 COS 资源迁移指南

## 迁移概述

本次迁移将视频和封面资源从本地后端静态文件迁移到腾讯云 COS 对象存储。

**COS 基础 URL**: `https://whcm-1353140174.cos.ap-nanjing.myqcloud.com`

## 资源路径映射

### 专业目录对应关系
- **软件工程** (software) → `resource/Se/` (001-012.mp4，共12个视频)
- **电子信息工程** (electronic) → `resource/Ele/` (001-011.mp4，共11个视频)
- **广播电视工程** (broadcast) → `resource/Rte/` (001-010.mp4，共10个视频)

**总计**: 33个作品视频，对应33个数据库作品记录

### URL 格式示例
```
软件工程作品 w001:
- 封面: https://whcm-1353140174.cos.ap-nanjing.myqcloud.com/resource/Se/001.mp4
- 视频: https://whcm-1353140174.cos.ap-nanjing.myqcloud.com/resource/Se/001.mp4

电子信息工程作品 w013:
- 封面: https://whcm-1353140174.cos.ap-nanjing.myqcloud.com/resource/Ele/001.mp4
- 视频: https://whcm-1353140174.cos.ap-nanjing.myqcloud.com/resource/Ele/001.mp4

广播电视工程作品 w024:
- 封面: https://whcm-1353140174.cos.ap-nanjing.myqcloud.com/resource/Rte/001.mp4
- 视频: https://whcm-1353140174.cos.ap-nanjing.myqcloud.com/resource/Rte/001.mp4
```

## 已完成的修改

### 1. 环境变量配置

#### 后端 (backend/.env)
```env
COS_BASE_URL=https://whcm-1353140174.cos.ap-nanjing.myqcloud.com
```

#### 前端 (frontend/.env)
```env
NUXT_PUBLIC_COS_BASE_URL=https://whcm-1353140174.cos.ap-nanjing.myqcloud.com
```

### 2. 数据库迁移脚本

创建了 `backend/migrations/0007_migrate_to_cos.sql`，用于更新现有数据库中的作品 URL。

### 3. 导入脚本更新

更新了 `backend/backend/import_works.sql`，新导入的作品将直接使用 COS URL。

## 迁移步骤

### 方案 A：更新现有数据（推荐）

如果数据库中已有作品数据和用户交互数据（点赞、评论），使用此方案：

```bash
# 1. 运行迁移脚本
mysql -uroot -p123456 graduation_exhibition < backend/migrations/0007_migrate_to_cos.sql

# 2. 验证更新结果
mysql -uroot -p123456 graduation_exhibition -e "SELECT id, poster_url, video_url FROM works LIMIT 3;"
```

### 方案 B：重新导入数据

如果是全新部署或可以清空现有数据，使用此方案：

```bash
# 1. 清空作品表（会级联删除点赞和评论数据）
mysql -uroot -p123456 graduation_exhibition -e "DELETE FROM works;"

# 2. 重新导入作品数据
mysql -uroot -p123456 --default-character-set=utf8mb4 graduation_exhibition < backend/backend/import_works.sql

# 3. 验证导入结果
mysql -uroot -p123456 graduation_exhibition -e "SELECT id, poster_url, video_url FROM works LIMIT 3;"
```

## 验证步骤

### 1. 验证数据库 URL

```bash
# 检查软件工程专业作品
mysql -uroot -p123456 graduation_exhibition -e "SELECT id, poster_url FROM works WHERE major_code='software' LIMIT 2;"

# 检查电子信息工程专业作品
mysql -uroot -p123456 graduation_exhibition -e "SELECT id, poster_url FROM works WHERE major_code='electronic' LIMIT 2;"

# 检查广播电视工程专业作品
mysql -uroot -p123456 graduation_exhibition -e "SELECT id, poster_url FROM works WHERE major_code='broadcast' LIMIT 2;"
```

### 2. 测试 COS 资源可访问性

```bash
# 测试视频文件（Se/001-012, Ele/001-011, Rte/001-010）
curl -I https://whcm-1353140174.cos.ap-nanjing.myqcloud.com/resource/Se/001.mp4
curl -I https://whcm-1353140174.cos.ap-nanjing.myqcloud.com/resource/Ele/001.mp4
curl -I https://whcm-1353140174.cos.ap-nanjing.myqcloud.com/resource/Rte/001.mp4
```

### 3. 启动服务并测试

```bash
# 启动后端（新终端）
cd backend
cargo run

# 启动前端（新终端）
cd frontend
pnpm dev

# 访问测试
# 1. 打开浏览器访问 http://localhost:3000
# 2. 进入任意专业展区
# 3. 点击作品卡片查看详情
# 4. 验证视频和封面是否正常加载
```

### 4. API 测试

```bash
# 测试作品列表 API（软件工程 12个作品）
curl http://localhost:8080/api/works?major_code=software | jq '.data.items[0] | {id, poster_url, video_url}'

# 测试作品详情 API
curl http://localhost:8080/api/works/w001 | jq '.data | {id, poster_url, video_url}'
```

## 注意事项

### 1. 本地静态文件
迁移后，`backend/static/videos/` 和 `backend/static/posters/` 目录中的文件不再使用，但建议保留作为备份。

### 2. Nuxt 代理配置
`frontend/nuxt.config.ts` 中的 `/static` 代理配置不再需要，但保留也不影响（因为不会再访问这个路径）。

### 3. CORS 配置
确保腾讯云 COS 存储桶已配置 CORS，允许前端域名访问：
```json
{
  "AllowedOrigins": ["http://localhost:3000", "https://your-domain.com"],
  "AllowedMethods": ["GET", "HEAD"],
  "AllowedHeaders": ["*"],
  "MaxAgeSeconds": 3600
}
```

### 4. CDN 加速
未来有域名后，可以配置腾讯云 CDN 加速：
- 创建 CDN 域名（如 `cdn.your-domain.com`）
- 源站配置为 COS 存储桶
- 更新环境变量中的 `COS_BASE_URL` 为 CDN 域名

## 回滚方案

如果需要回滚到本地静态文件：

```bash
# 1. 恢复环境变量（删除 COS_BASE_URL）
# 编辑 backend/.env 和 frontend/.env

# 2. 运行回滚 SQL
mysql -uroot -p123456 graduation_exhibition << EOF
UPDATE works SET
  poster_url = CONCAT('/static/posters/', id, '.jpg'),
  video_url = CONCAT('/static/videos/', id, '.mp4');
EOF

# 3. 重启服务
```

## 故障排查

### 问题 1：视频无法播放
- 检查浏览器控制台是否有 CORS 错误
- 验证 COS URL 是否可访问
- 检查数据库中的 URL 格式是否正确

### 问题 2：封面图片不显示
- 检查图片文件是否已上传到 COS
- 验证文件路径大小写是否正确（SE/Ele/RTE）
- 检查浏览器网络请求状态码

### 问题 3：API 返回旧的 URL
- 确认已运行数据库迁移脚本
- 重启后端服务
- 清除浏览器缓存

## 相关文件

- `backend/.env` - 后端环境变量
- `frontend/.env` - 前端环境变量
- `backend/migrations/0007_migrate_to_cos.sql` - 数据库迁移脚本
- `backend/backend/import_works.sql` - 作品数据导入脚本
- `test_cos_migration.sh` - 迁移测试脚本

## 联系支持

如有问题，请检查：
1. 腾讯云 COS 控制台确认文件已上传
2. 存储桶权限配置（公有读）
3. CORS 配置是否正确
