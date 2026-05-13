# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## 项目概述

**传媒技术学院 2026 届毕业设计展网站** — 公开展示软件工程、电子信息工程、广播电视工程三个专业的毕业设计作品。

**主题**：智媒融合 · 创启未来

## 技术栈

**前端**：Nuxt 3 + Vue 3 + TypeScript + Tailwind CSS + Pinia + Three.js + GSAP  
**后端**：Rust + axum 0.7 + SQLx + MySQL 8.0 + Redis  
**部署**：Nginx 静态托管（SSG）+ Rust 后端 API  
**包管理器**：pnpm（前端必须使用 pnpm）

## 项目特点

- **无需登录**：游客直接浏览、点赞、评论
- **智能防刷**：Redis 限流 + 指纹识别 + 敏感词过滤
- **高性能**：Rust 后端 + SSG 静态页面 + 游标分页
- **安全可靠**：XSS 防护 + SQL 注入防护 + 数据哈希存储
- **响应式设计**：PC、平板、手机全适配

## 快速启动

### 前置条件
- MySQL 8.0（本地或 Docker，端口 3306）
- Redis 7（本地或 Docker，端口 6379）
- Node.js >= 18 + pnpm >= 9
- Rust >= 1.70

### 手动启动

```bash
# 1. 创建数据库
mysql -uroot -p123456 -e "CREATE DATABASE IF NOT EXISTS graduation_exhibition CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;"

# 2. 运行迁移脚本
for file in backend/migrations/*.sql; do
  mysql -uroot -p123456 --default-character-set=utf8mb4 graduation_exhibition < "$file"
done

# 3. 导入作品数据
mysql -uroot -p123456 --default-character-set=utf8mb4 graduation_exhibition < backend/backend/import_works.sql

# 4. 启动 Redis（Docker）
docker run -d -p 6379:6379 redis:7-alpine

# 5. 启动后端
cd backend && cargo run

# 6. 前端开发模式
cd frontend && pnpm install && pnpm dev
```

访问 http://localhost:3000

## 常用命令

### 前端开发
```bash
cd frontend
pnpm install          # 安装依赖
pnpm dev              # 开发模式 (localhost:3000)
pnpm generate         # SSG 静态生成（输出到 .output/public/）
pnpm build            # SSR 生产构建
pnpm preview          # 预览生产构建
```

### 后端开发
```bash
cd backend
cargo run             # 开发服务器 (localhost:8080)
cargo build --release # 生产构建
cargo test            # 运行测试
cargo fmt             # 格式化代码
cargo clippy          # 代码检查
```

### Docker 管理
```bash
docker-compose up -d       # 启动 MySQL + Redis
docker-compose down        # 停止服务
docker-compose down -v     # 停止并删除数据卷（清空数据库）
```

### 数据库操作
```bash
# 导入作品数据（注意字符集）
mysql -uroot -p123456 --default-character-set=utf8mb4 graduation_exhibition < backend/backend/import_works.sql

# 查看作品数量
mysql -uroot -p123456 graduation_exhibition -e "SELECT COUNT(*) FROM works"

# 查看点赞数
mysql -uroot -p123456 graduation_exhibition -e "SELECT COUNT(*) FROM likes WHERE is_active = 1"
```

## 架构概览

### 项目结构
```
├── frontend/                  # Nuxt 3 前端
│   ├── pages/                # 路由页面（4个）
│   ├── components/           # Vue 组件
│   ├── stores/               # Pinia 状态管理（visitor, interactions）
│   ├── composables/          # 组合式函数（useWorks, useWaveBackground）
│   ├── types/                # TypeScript 类型定义
│   └── nuxt.config.ts        # Nuxt 配置（含 SSG prerender）
├── backend/                  # Rust axum 后端
│   ├── src/
│   │   ├── main.rs           # 入口，路由注册，CORS
│   │   ├── routes/           # API 路由（health, works, likes, comments, interactions, rankings）
│   │   ├── models/           # 数据模型
│   │   ├── services/         # 业务逻辑（fingerprint, rate_limit, sanitize）
│   │   └── utils/            # 工具函数
│   ├── migrations/           # 数据库迁移脚本
│   └── backend/              # SQL 导入脚本
├── nginx/                    # Nginx 配置
│   ├── nginx-local.conf      # 本地测试配置
│   └── graduation.conf       # 生产环境配置
└── docker-compose.yml        # Docker 编排（MySQL + Redis）
```

### 前端页面路由

| 路由 | 说明 |
|------|------|
| `/` | 首页（3D 科技立方体、专业入口、热榜预览、关于） |
| `/major/software` | 软件工程作品列表 |
| `/major/electronic` | 电子信息工程作品列表 |
| `/major/broadcast` | 广播电视工程作品列表 |
| `/works/:workId` | 作品详情（视频播放、点赞、评论） |
| `/ranking` | 点赞热榜（今日/本周/总榜） |

### 后端 API

| 方法 | 路径 | 说明 |
|------|------|------|
| GET | `/api/health` | 健康检查 |
| GET | `/api/works` | 作品列表（分页、专业筛选） |
| GET | `/api/works/:id` | 作品详情 |
| GET | `/api/interactions/summary` | 批量交互统计（含 liked_by_me） |
| GET | `/api/works/:workId/interaction` | 单个作品交互状态 |
| POST | `/api/works/:workId/like/toggle` | 切换点赞状态（日重置） |
| GET | `/api/works/:workId/comments` | 评论列表（游标分页） |
| POST | `/api/works/:workId/comments` | 提交评论 |
| GET | `/api/rankings/likes` | 点赞热榜（range=all/today/week） |

### 数据库表

- **works**：作品信息（id, title, major_code, tags, authors, teacher, poster_url, video_url, introduction）
- **likes**：点赞记录（软删除，is_active 字段，唯一约束 work_id + visitor_fingerprint_hash）
- **comments**：评论记录（游标分页，status 字段）
- **abuse_logs**：风控日志

## 核心功能实现

### 游客身份识别
- 前端：`localStorage` 存储 `visitor_id`，使用 `generateUUID()`（兼容非 secure context 的 fallback）
- 后端：SHA256(work_id + visitor_id + ip + 日期 + server_salt) 生成指纹哈希
- 日期参与哈希，实现每日重置（同一用户每天可重新点赞）
- 自动生成中文昵称：形容词 + 名词 + 4位数字（如"快乐熊猫4821"）

### 限流策略（Redis 滑动窗口）
- 点赞：10秒/次（同一指纹对同一作品）
- 评论：30秒/次（同一指纹）
- IP：60秒/20次（同一IP）

### 点赞系统
- 原子化切换：`INSERT ... ON DUPLICATE KEY UPDATE is_active = NOT is_active`
- 日重置：指纹含日期，每天凌晨后同一用户可对同一作品再次点赞
- 点赞数为跨天累计（所有 is_active=TRUE 记录总和）

### SSG 预渲染
- `nuxt.config.ts` 中配置 `nitro.prerender` 自动预渲染所有页面
- 主要页面 + 33个作品详情页全部静态生成
- 需在构建时运行后端以获取作品数据
- 输出到 `.output/public/`，由 Nginx 直接托管

## 环境配置

### 前端 (.env)
```env
NUXT_PUBLIC_API_BASE=/api
NUXT_PUBLIC_COS_BASE_URL=https://whcm-1353140174.cos.ap-nanjing.myqcloud.com
```

### 后端 (.env)
```env
APP_HOST=0.0.0.0
APP_PORT=8080
DATABASE_URL=mysql://root:123456@localhost:3306/graduation_exhibition
REDIS_URL=redis://localhost:6379
SERVER_SALT=your-secret-salt-here  # 生产环境必须修改
CORS_ALLOWED_ORIGINS=http://localhost:3000
TRUST_PROXY=true
COS_BASE_URL=https://whcm-1353140174.cos.ap-nanjing.myqcloud.com
```

**注意**：
- `NUXT_PUBLIC_API_BASE` 设为 `/api`（相对路径），避免硬编码 localhost 导致外部设备无法访问
- 数据库密码默认为 `123456`（开发环境）
- 数据库名必须是 `graduation_exhibition`

## 部署方案

### 本地 Nginx 静态托管
```bash
# 1. 构建静态文件（确保后端运行中）
cd frontend && pnpm generate

# 2. 启动 Nginx（配置指向 .output/public/）
nginx.exe

# 3. 访问 http://localhost
```

Nginx 配置文件：[nginx/nginx-local.conf](nginx/nginx-local.conf)（本地）、[nginx/graduation.conf](nginx/graduation.conf)（生产）

### 公网测试（ngrok）
```bash
ngrok http 80
# 将生成的 https://xxx.ngrok-free.app 分享给外部测试者
```

### 生产部署
详见 [DEPLOYMENT.md](DEPLOYMENT.md)

## 代码风格

### 前端
- Composition API + `<script setup>` 语法
- Pinia stores：`visitor.ts`、`interactions.ts`
- Tailwind CSS 工具类优先
- GSAP 动画使用 `transform` 和 `opacity`（GPU 加速）

### 后端
- 模块化路由：`routes/` 下每个功能独立文件
- 数据模型使用 SQLx `FromRow` derive
- 错误处理使用 thiserror
- 数据库查询必须参数化（SQLx 编译时检查）

## 设计规范

- 浅色科技感 + 玻璃拟态（glassmorphism）
- 蓝白配色：`#1466ff`（主蓝）、`#37c8ff`（青色）
- 3D 科技立方体、粒子光点、数据网格背景
- PC 最大宽度 1200-1440px，平板/手机响应式

## 故障排查

### 前端启动失败
```bash
cd frontend
rm -rf node_modules .nuxt .output pnpm-lock.yaml
pnpm install
```

### 数据库连接失败
- 检查 MySQL 是否运行：`docker ps | grep mysql`
- 检查 `backend/.env` 中的 `DATABASE_URL`

### 外部设备无法访问 API
- 检查 `frontend/.env` 中 `NUXT_PUBLIC_API_BASE` 应为 `/api`（相对路径）
- 确认 Nginx 正确代理 `/api/` 到后端 8080

### 作品详情页无数据
- SSG 模式下需确保构建时后端正在运行（用于获取作品数据生成静态页）
- 清理构建缓存：`rm -rf .nuxt .output && pnpm generate`

## 参考文档

- [README.md](README.md) — 项目说明
- [DEPLOYMENT.md](DEPLOYMENT.md) — 部署指南（含 Nginx 配置）
- [COS_MIGRATION.md](COS_MIGRATION.md) — COS 资源说明
- [毕业设计作品汇总.md](毕业设计作品汇总.md) — 33个作品源信息
